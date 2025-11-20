//! ESC/P2 Printer Driver
//!
//! This module provides a type-safe Rust driver for ESC/P2 printers.

use crate::errors::PrinterError;
use crate::io::{read_byte_with_timeout, write_all_with_retry};
use crate::types::PrinterStatus;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::time::Duration;

/// Represents a bidirectional connection to an ESC/P2 printer device
pub struct Printer<W: Write, R: Read> {
    writer: W,
    reader: R,
    max_graphics_width: u16,
}

impl<W: Write, R: Read> Printer<W, R> {
    /// Create a new Printer instance with custom Write and Read implementations
    ///
    /// # Arguments
    ///
    /// * `writer` - Output stream for sending commands to printer
    /// * `reader` - Input stream for receiving status responses from printer
    /// * `max_graphics_width` - Maximum graphics width in dots for validation
    pub fn new(writer: W, reader: R, max_graphics_width: u16) -> Self {
        Self {
            writer,
            reader,
            max_graphics_width,
        }
    }

    /// Send raw bytes to the printer
    ///
    /// This method handles partial writes automatically and ensures all data is sent.
    pub fn send(&mut self, data: &[u8]) -> Result<(), PrinterError> {
        write_all_with_retry(&mut self.writer, data)?;
        Ok(())
    }

    /// Send an ESC/P2 command starting with ESC byte
    ///
    /// This is a convenience method for commands that start with the ESC byte (0x1B).
    pub fn esc(&mut self, data: &[u8]) -> Result<(), PrinterError> {
        let mut cmd = Vec::with_capacity(1 + data.len());
        cmd.push(0x1B);
        cmd.extend_from_slice(data);
        self.send(&cmd)
    }

    /// Query printer status with timeout
    ///
    /// Sends a status query command and waits for the response, timing out if
    /// the printer doesn't respond within the specified duration.
    pub fn query_status(&mut self, timeout: Duration) -> Result<PrinterStatus, PrinterError> {
        // Send status query command (DLE EOT 1)
        self.send(&[0x10, 0x04, 0x01])?;

        // Read response with timeout
        let status_byte = read_byte_with_timeout(&mut self.reader, timeout).map_err(|e| {
            if e.kind() == io::ErrorKind::TimedOut {
                PrinterError::Timeout { timeout }
            } else if e.kind() == io::ErrorKind::UnexpectedEof {
                PrinterError::Disconnected
            } else {
                PrinterError::Io(e)
            }
        })?;

        Ok(PrinterStatus::from_byte(status_byte))
    }

    /// Reset printer to default state
    ///
    /// Sends the ESC @ command to reset all printer settings to their defaults.
    pub fn reset(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x40])
    }

    /// Get the maximum graphics width configured for this printer
    pub fn max_graphics_width(&self) -> u16 {
        self.max_graphics_width
    }

    /// Get a reference to the writer (test-only helper)
    #[cfg(test)]
    pub fn writer(&self) -> &W {
        &self.writer
    }
}

impl Printer<File, File> {
    /// Open a printer device file
    ///
    /// # Arguments
    ///
    /// * `path` - Device file path (e.g., "/dev/usb/lp0" on Linux)
    /// * `max_graphics_width` - Maximum graphics width in dots
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The device file doesn't exist (`DeviceNotFound`)
    /// - Permission is denied (`Permission` with remediation instructions)
    /// - Any other I/O error occurs
    pub fn open_device(path: &str, max_graphics_width: u16) -> Result<Self, PrinterError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .map_err(|e| {
                if e.kind() == io::ErrorKind::PermissionDenied {
                    PrinterError::Permission {
                        path: path.to_string(),
                        message: format!(
                            "Cannot access printer device '{}'. \n\
                             Solutions:\n\
                             - Add your user to 'lp' group: sudo usermod -aG lp $USER\n\
                             - Or run with sudo (not recommended)\n\
                             - Or adjust device permissions: sudo chmod 666 {}",
                            path, path
                        ),
                    }
                } else if e.kind() == io::ErrorKind::NotFound {
                    PrinterError::DeviceNotFound {
                        path: path.to_string(),
                    }
                } else {
                    PrinterError::Io(e)
                }
            })?;

        // Clone file handle for read and write
        let reader = file.try_clone()?;

        Ok(Self::new(file, reader, max_graphics_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::mock::{MockReader, MockWriter};

    #[test]
    fn test_printer_new() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let printer = Printer::new(writer, reader, 1440);

        assert_eq!(printer.max_graphics_width(), 1440);
    }

    #[test]
    fn test_send_data() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.send(b"Hello").unwrap();

        assert_eq!(printer.writer.written(), b"Hello");
    }

    #[test]
    fn test_esc_command() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.esc(&[0x40]).unwrap(); // ESC @

        assert_eq!(printer.writer.written(), &[0x1B, 0x40]);
    }

    #[test]
    fn test_reset() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.reset().unwrap();

        assert_eq!(printer.writer.written(), &[0x1B, 0x40]);
    }

    #[test]
    fn test_query_status() {
        let writer = MockWriter::new();
        // Simulate online status response
        let reader = MockReader::new(vec![0b0000_0000]);
        let mut printer = Printer::new(writer, reader, 1440);

        let status = printer.query_status(Duration::from_secs(1)).unwrap();

        assert!(status.online);
        assert!(!status.paper_out);
        assert!(!status.error);
        assert!(status.is_ready());
    }
}

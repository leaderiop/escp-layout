//! Graphics printing commands
//!
//! This module provides commands for printing bitmap graphics in various density modes.

use crate::errors::{PrinterError, ValidationError};
use crate::printer::Printer;
use crate::types::GraphicsMode;
use std::io::{Read, Write};

impl<W: Write, R: Read> Printer<W, R> {
    /// Print graphics in the specified density mode
    ///
    /// Sends bitmap graphics data to the printer in the specified density mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - Graphics density mode (SingleDensity, DoubleDensity, or HighDensity)
    /// * `width` - Width of the graphics in dots (must match data length)
    /// * `data` - Bitmap data (one byte per column of 8 vertical dots)
    ///
    /// # Errors
    ///
    /// Returns:
    /// - `ValidationError::GraphicsWidthExceeded` if width > max_graphics_width
    /// - `ValidationError::GraphicsWidthMismatch` if width != data.len()
    ///
    /// # Notes
    ///
    /// - SingleDensity: 60 DPI horizontal x 60 DPI vertical
    /// - DoubleDensity: 120 DPI horizontal x 60 DPI vertical
    /// - HighDensity: 180 DPI horizontal x 180 DPI vertical
    ///
    /// Each byte in `data` represents 8 vertical dots, with the least significant bit
    /// at the top and the most significant bit at the bottom.
    pub fn print_graphics(
        &mut self,
        mode: GraphicsMode,
        width: u16,
        data: &[u8],
    ) -> Result<(), PrinterError> {
        // Validate width against maximum
        if width > self.max_graphics_width() {
            return Err(ValidationError::GraphicsWidthExceeded {
                width,
                max_width: self.max_graphics_width(),
            }
            .into());
        }

        // Validate data length matches width
        if width as usize != data.len() {
            return Err(ValidationError::GraphicsWidthMismatch {
                width,
                data_len: data.len(),
            }
            .into());
        }

        // Construct command: ESC K/L/Y nL nH [data...]
        let mut cmd = Vec::with_capacity(4 + data.len());
        cmd.push(0x1B); // ESC
        cmd.push(mode.as_command_byte());
        cmd.push((width & 0xFF) as u8); // nL
        cmd.push(((width >> 8) & 0xFF) as u8); // nH
        cmd.extend_from_slice(data);

        self.send(&cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::mock::{MockReader, MockWriter};

    #[test]
    fn test_print_graphics_single_density() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        let data = vec![0xFF, 0x00, 0xFF, 0x00]; // Simple pattern
        printer
            .print_graphics(GraphicsMode::SingleDensity, 4, &data)
            .unwrap();

        let written = printer.writer().written();
        assert_eq!(written[0], 0x1B); // ESC
        assert_eq!(written[1], 0x4B); // K (Single Density)
        assert_eq!(written[2], 4); // Width low byte
        assert_eq!(written[3], 0); // Width high byte
        assert_eq!(&written[4..], &data);
    }

    #[test]
    fn test_print_graphics_double_density() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        let data = vec![0xAA; 8]; // 8 dots wide
        printer
            .print_graphics(GraphicsMode::DoubleDensity, 8, &data)
            .unwrap();

        let written = printer.writer().written();
        assert_eq!(written[0], 0x1B);
        assert_eq!(written[1], 0x4C); // L (Double Density)
        assert_eq!(written[2], 8);
        assert_eq!(written[3], 0);
    }

    #[test]
    fn test_print_graphics_high_density() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        let data = vec![0x55; 16]; // 16 dots wide
        printer
            .print_graphics(GraphicsMode::HighDensity, 16, &data)
            .unwrap();

        let written = printer.writer().written();
        assert_eq!(written[0], 0x1B);
        assert_eq!(written[1], 0x59); // Y (High Density)
        assert_eq!(written[2], 16);
        assert_eq!(written[3], 0);
    }

    #[test]
    fn test_print_graphics_width_validation() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let max_width = 100;
        let mut printer = Printer::new(writer, reader, max_width);

        let data = vec![0xFF; 200]; // Exceeds max width

        let result = printer.print_graphics(GraphicsMode::SingleDensity, 200, &data);

        assert!(matches!(
            result,
            Err(PrinterError::Validation(
                ValidationError::GraphicsWidthExceeded {
                    width: 200,
                    max_width: 100
                }
            ))
        ));
    }

    #[test]
    fn test_print_graphics_data_length_mismatch() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        let data = vec![0xFF; 10]; // Data is 10 bytes

        // Specify width as 20 (doesn't match data length)
        let result = printer.print_graphics(GraphicsMode::SingleDensity, 20, &data);

        assert!(matches!(
            result,
            Err(PrinterError::Validation(
                ValidationError::GraphicsWidthMismatch {
                    width: 20,
                    data_len: 10
                }
            ))
        ));
    }

    #[test]
    fn test_print_graphics_large_width() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        // Test with width that requires both nL and nH bytes
        let width = 300u16;
        let data = vec![0x00; 300];

        printer
            .print_graphics(GraphicsMode::HighDensity, width, &data)
            .unwrap();

        let written = printer.writer().written();
        assert_eq!(written[2], (width & 0xFF) as u8);
        assert_eq!(written[3], ((width >> 8) & 0xFF) as u8);
    }
}

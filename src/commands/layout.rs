//! Page layout control commands
//!
//! This module provides commands for controlling page dimensions, margins, and line spacing.

use crate::errors::{PrinterError, ValidationError};
use crate::printer::Printer;
use std::io::{Read, Write};

impl<W: Write, R: Read> Printer<W, R> {
    /// Set page length in lines
    ///
    /// Sends the ESC C n command to set the page length in lines (1-127).
    ///
    /// # Arguments
    ///
    /// * `lines` - Page length in lines (must be >= 1)
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidPageLength` if lines < 1
    pub fn set_page_length_lines(&mut self, lines: u8) -> Result<(), PrinterError> {
        if lines < 1 {
            return Err(ValidationError::InvalidPageLength { value: lines }.into());
        }
        self.send(&[0x1B, 0x43, lines])
    }

    /// Set page length in dots (1/360-inch units)
    ///
    /// Sends the ESC ( C command to set the page length in 1/360-inch units.
    ///
    /// # Arguments
    ///
    /// * `dots` - Page length in 1/360-inch units (must be >= 1)
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidPageLength` if dots < 1
    pub fn set_page_length_dots(&mut self, dots: u16) -> Result<(), PrinterError> {
        if dots < 1 {
            return Err(ValidationError::InvalidPageLength { value: 0 }.into());
        }

        let nl = (dots & 0xFF) as u8;
        let nh = ((dots >> 8) & 0xFF) as u8;

        self.send(&[0x1B, 0x28, 0x43, 0x02, 0x00, nl, nh])
    }

    /// Eject page (form feed)
    ///
    /// Sends the FF command (0x0C) to advance to the next page.
    pub fn form_feed(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x0C])
    }

    /// Line feed
    ///
    /// Sends the LF command (0x0A) to advance one line.
    pub fn line_feed(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x0A])
    }

    /// Carriage return
    ///
    /// Sends the CR command (0x0D) to return to the beginning of the current line.
    pub fn carriage_return(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x0D])
    }

    /// Set line spacing in 1/180-inch units
    ///
    /// Sends the ESC 3 n command to set custom line spacing.
    ///
    /// # Arguments
    ///
    /// * `dots` - Line spacing in 1/180-inch units (0-255)
    pub fn set_line_spacing(&mut self, dots: u8) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x33, dots])
    }

    /// Set default line spacing (1/6 inch)
    ///
    /// Sends the ESC 2 command to reset line spacing to the default 1/6 inch.
    pub fn set_default_line_spacing(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x32])
    }

    /// Set left margin in characters
    ///
    /// Sends the ESC l n command to set the left margin.
    ///
    /// # Arguments
    ///
    /// * `chars` - Left margin in characters (0-255)
    pub fn set_left_margin(&mut self, chars: u8) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x6C, chars])
    }

    /// Set right margin in characters
    ///
    /// Sends the ESC Q n command to set the right margin.
    ///
    /// # Arguments
    ///
    /// * `chars` - Right margin in characters (0-255)
    pub fn set_right_margin(&mut self, chars: u8) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x51, chars])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::mock::{MockReader, MockWriter};

    #[test]
    fn test_set_page_length_lines() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.set_page_length_lines(66).unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x43, 66]);
    }

    #[test]
    fn test_set_page_length_lines_validation() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        let result = printer.set_page_length_lines(0);

        assert!(matches!(
            result,
            Err(PrinterError::Validation(
                ValidationError::InvalidPageLength { value: 0 }
            ))
        ));
    }

    #[test]
    fn test_set_page_length_dots() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.set_page_length_dots(3960).unwrap(); // 11 inches * 360 dots/inch

        let written = printer.writer().written();
        assert_eq!(&written[0..5], &[0x1B, 0x28, 0x43, 0x02, 0x00]);
        assert_eq!(written[5], (3960 & 0xFF) as u8);
        assert_eq!(written[6], ((3960 >> 8) & 0xFF) as u8);
    }

    #[test]
    fn test_form_feed() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.form_feed().unwrap();

        assert_eq!(printer.writer().written(), &[0x0C]);
    }

    #[test]
    fn test_line_feed() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.line_feed().unwrap();

        assert_eq!(printer.writer().written(), &[0x0A]);
    }

    #[test]
    fn test_carriage_return() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.carriage_return().unwrap();

        assert_eq!(printer.writer().written(), &[0x0D]);
    }

    #[test]
    fn test_set_line_spacing() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.set_line_spacing(30).unwrap(); // 30/180 inch

        assert_eq!(printer.writer().written(), &[0x1B, 0x33, 30]);
    }

    #[test]
    fn test_set_default_line_spacing() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.set_default_line_spacing().unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x32]);
    }

    #[test]
    fn test_set_left_margin() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.set_left_margin(10).unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x6C, 10]);
    }

    #[test]
    fn test_set_right_margin() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.set_right_margin(80).unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x51, 80]);
    }
}

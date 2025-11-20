//! Positioning commands
//!
//! This module provides commands for micro-feeding and horizontal positioning.

use crate::errors::{PrinterError, ValidationError};
use crate::printer::Printer;
use std::io::{Read, Write};

impl<W: Write, R: Read> Printer<W, R> {
    /// Micro-forward feed (paper advance) in 1/180-inch units
    ///
    /// Sends the ESC J n command to advance the paper forward by n/180 inches.
    ///
    /// # Arguments
    ///
    /// * `units` - Number of 1/180-inch units to advance (must be 1-255)
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::MicroFeedZero` if units == 0
    pub fn micro_forward(&mut self, units: u8) -> Result<(), PrinterError> {
        if units == 0 {
            return Err(ValidationError::MicroFeedZero.into());
        }
        self.send(&[0x1B, 0x4A, units])
    }

    /// Micro-reverse feed (paper reverse) in 1/180-inch units
    ///
    /// Sends the ESC j n command to reverse the paper by n/180 inches.
    ///
    /// # Arguments
    ///
    /// * `units` - Number of 1/180-inch units to reverse (must be 1-255)
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::MicroFeedZero` if units == 0
    ///
    /// # Notes
    ///
    /// Maximum reverse movement is approximately 1.41 inches (254 units)
    pub fn micro_reverse(&mut self, units: u8) -> Result<(), PrinterError> {
        if units == 0 {
            return Err(ValidationError::MicroFeedZero.into());
        }
        self.send(&[0x1B, 0x6A, units])
    }

    /// Move to absolute horizontal position in 1/60-inch units
    ///
    /// Sends the ESC $ nL nH command to move the print position to an absolute horizontal coordinate.
    ///
    /// # Arguments
    ///
    /// * `position` - Absolute horizontal position in 1/60-inch units from the left margin
    pub fn move_absolute_x(&mut self, position: u16) -> Result<(), PrinterError> {
        let nl = (position & 0xFF) as u8;
        let nh = ((position >> 8) & 0xFF) as u8;
        self.send(&[0x1B, 0x24, nl, nh])
    }

    /// Move relative horizontal position in 1/120-inch units
    ///
    /// Sends the ESC \ nL nH command to move the print position relative to the current position.
    ///
    /// # Arguments
    ///
    /// * `offset` - Relative horizontal offset in 1/120-inch units (can be negative)
    pub fn move_relative_x(&mut self, offset: i16) -> Result<(), PrinterError> {
        let unsigned = offset as u16;
        let nl = (unsigned & 0xFF) as u8;
        let nh = ((unsigned >> 8) & 0xFF) as u8;
        self.send(&[0x1B, 0x5C, nl, nh])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::mock::{MockReader, MockWriter};

    #[test]
    fn test_micro_forward() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.micro_forward(10).unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x4A, 10]);
    }

    #[test]
    fn test_micro_forward_validation() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        let result = printer.micro_forward(0);

        assert!(matches!(
            result,
            Err(PrinterError::Validation(ValidationError::MicroFeedZero))
        ));
    }

    #[test]
    fn test_micro_reverse() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.micro_reverse(5).unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x6A, 5]);
    }

    #[test]
    fn test_micro_reverse_validation() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        let result = printer.micro_reverse(0);

        assert!(matches!(
            result,
            Err(PrinterError::Validation(ValidationError::MicroFeedZero))
        ));
    }

    #[test]
    fn test_move_absolute_x() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        let position = 120u16; // 2 inches * 60
        printer.move_absolute_x(position).unwrap();

        let written = printer.writer().written();
        assert_eq!(written[0], 0x1B);
        assert_eq!(written[1], 0x24);
        assert_eq!(written[2], (position & 0xFF) as u8);
        assert_eq!(written[3], ((position >> 8) & 0xFF) as u8);
    }

    #[test]
    fn test_move_relative_x_positive() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        let offset = 60i16; // 0.5 inches * 120
        printer.move_relative_x(offset).unwrap();

        let written = printer.writer().written();
        let unsigned = offset as u16;
        assert_eq!(written[0], 0x1B);
        assert_eq!(written[1], 0x5C);
        assert_eq!(written[2], (unsigned & 0xFF) as u8);
        assert_eq!(written[3], ((unsigned >> 8) & 0xFF) as u8);
    }

    #[test]
    fn test_move_relative_x_negative() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.move_relative_x(-60).unwrap();

        let written = printer.writer().written();
        assert_eq!(written[0], 0x1B);
        assert_eq!(written[1], 0x5C);
        // Check that negative is encoded as two's complement
        let expected = (-60i16) as u16;
        assert_eq!(written[2], (expected & 0xFF) as u8);
        assert_eq!(written[3], ((expected >> 8) & 0xFF) as u8);
    }
}

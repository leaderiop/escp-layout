//! Text formatting commands

use crate::errors::PrinterError;
use crate::printer::Printer;
use crate::types::Font;
use std::io::{Read, Write};

impl<W: Write, R: Read> Printer<W, R> {
    /// Enable bold text mode
    ///
    /// Sends the ESC E command to enable bold printing.
    pub fn bold_on(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x45])
    }

    /// Disable bold text mode
    ///
    /// Sends the ESC F command to disable bold printing.
    pub fn bold_off(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x46])
    }

    /// Enable underline mode
    ///
    /// Sends the ESC - 1 command to enable text underlining.
    pub fn underline_on(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x2D, 0x01])
    }

    /// Disable underline mode
    ///
    /// Sends the ESC - 0 command to disable text underlining.
    pub fn underline_off(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x2D, 0x00])
    }

    /// Enable double-strike mode
    ///
    /// Sends the ESC G command to enable double-strike printing for darker text.
    pub fn double_strike_on(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x47])
    }

    /// Disable double-strike mode
    ///
    /// Sends the ESC H command to disable double-strike printing.
    pub fn double_strike_off(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x48])
    }

    /// Select 10 characters per inch (Pica) pitch
    ///
    /// Sends the ESC P command.
    pub fn select_10cpi(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x50])
    }

    /// Select 12 characters per inch (Elite) pitch
    ///
    /// Sends the ESC M command.
    pub fn select_12cpi(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x4D])
    }

    /// Select 15 characters per inch (Condensed) pitch
    ///
    /// Sends the ESC g command.
    pub fn select_15cpi(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x67])
    }

    /// Select a specific font typeface
    ///
    /// Sends the ESC k n command where n is the font number.
    pub fn select_font(&mut self, font: Font) -> Result<(), PrinterError> {
        self.send(&font.as_command())
    }

    /// Write text to the printer
    ///
    /// Sends ASCII text to the printer. Non-ASCII characters are replaced with '?'.
    pub fn write_text(&mut self, text: &str) -> Result<(), PrinterError> {
        // Convert to ASCII, replacing non-ASCII with '?'
        let ascii_text: Vec<u8> = text
            .chars()
            .map(|c| if c.is_ascii() { c as u8 } else { b'?' })
            .collect();

        self.send(&ascii_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::mock::{MockReader, MockWriter};

    #[test]
    fn test_bold_on() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.bold_on().unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x45]);
    }

    #[test]
    fn test_bold_off() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.bold_off().unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x46]);
    }

    #[test]
    fn test_underline_on() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.underline_on().unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x2D, 0x01]);
    }

    #[test]
    fn test_underline_off() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.underline_off().unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x2D, 0x00]);
    }

    #[test]
    fn test_double_strike() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.double_strike_on().unwrap();
        assert_eq!(&printer.writer().written()[..2], &[0x1B, 0x47]);
    }

    #[test]
    fn test_pitch_selection() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.select_10cpi().unwrap();
        printer.select_12cpi().unwrap();
        printer.select_15cpi().unwrap();

        let written = printer.writer().written();
        assert_eq!(&written[0..2], &[0x1B, 0x50]); // 10 CPI
        assert_eq!(&written[2..4], &[0x1B, 0x4D]); // 12 CPI
        assert_eq!(&written[4..6], &[0x1B, 0x67]); // 15 CPI
    }

    #[test]
    fn test_select_font() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.select_font(Font::Courier).unwrap();

        assert_eq!(printer.writer().written(), &[0x1B, 0x6B, 2]);
    }

    #[test]
    fn test_write_text_ascii() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.write_text("Hello, World!").unwrap();

        assert_eq!(printer.writer().written(), b"Hello, World!");
    }

    #[test]
    fn test_write_text_non_ascii() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1440);

        printer.write_text("Hello 世界").unwrap();

        assert_eq!(printer.writer().written(), b"Hello ??");
    }
}

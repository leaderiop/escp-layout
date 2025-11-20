//! Printer status types

/// Represents the current operational status of the printer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrinterStatus {
    /// True if printer is online and ready
    pub online: bool,
    /// True if printer has no paper loaded
    pub paper_out: bool,
    /// True if printer has encountered an error condition
    pub error: bool,
}

impl PrinterStatus {
    /// Parse printer status from a status byte
    ///
    /// Bit layout:
    /// - Bit 3: 1 = offline, 0 = online
    /// - Bit 5: 1 = paper out
    /// - Bit 6: 1 = error occurred
    pub fn from_byte(byte: u8) -> Self {
        Self {
            online: (byte & 0b0000_1000) == 0,
            paper_out: (byte & 0b0010_0000) != 0,
            error: (byte & 0b0100_0000) != 0,
        }
    }

    /// Check if printer is ready to print
    pub fn is_ready(&self) -> bool {
        self.online && !self.paper_out && !self.error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_from_byte_ready() {
        let status = PrinterStatus::from_byte(0b0000_0000);
        assert!(status.online);
        assert!(!status.paper_out);
        assert!(!status.error);
        assert!(status.is_ready());
    }

    #[test]
    fn test_status_from_byte_offline() {
        let status = PrinterStatus::from_byte(0b0000_1000);
        assert!(!status.online);
        assert!(!status.is_ready());
    }

    #[test]
    fn test_status_from_byte_paper_out() {
        let status = PrinterStatus::from_byte(0b0010_0000);
        assert!(status.paper_out);
        assert!(!status.is_ready());
    }

    #[test]
    fn test_status_from_byte_error() {
        let status = PrinterStatus::from_byte(0b0100_0000);
        assert!(status.error);
        assert!(!status.is_ready());
    }
}

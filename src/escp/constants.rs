//! ESC/P command byte sequences for EPSON LQ-2090II.

/// ESC @ - Printer reset and initialization
pub const ESC_RESET: &[u8] = &[0x1B, 0x40];

/// SI (Shift In) - Condensed mode (12 CPI)
pub const SI_CONDENSED: &[u8] = &[0x0F];

/// ESC E - Bold on
pub const ESC_BOLD_ON: &[u8] = &[0x1B, 0x45];

/// ESC F - Bold off
pub const ESC_BOLD_OFF: &[u8] = &[0x1B, 0x46];

/// ESC - 1 - Underline on
pub const ESC_UNDERLINE_ON: &[u8] = &[0x1B, 0x2D, 0x01];

/// ESC - 0 - Underline off
pub const ESC_UNDERLINE_OFF: &[u8] = &[0x1B, 0x2D, 0x00];

/// CR - Carriage return
pub const CR: u8 = 0x0D;

/// LF - Line feed
pub const LF: u8 = 0x0A;

/// FF - Form feed (page separator)
pub const FF: u8 = 0x0C;

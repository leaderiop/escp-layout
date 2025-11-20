//! Font type definitions

/// Available font typefaces supported by ESC/P2 printers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Font {
    /// Roman typeface
    Roman = 0,
    /// Sans Serif typeface
    SansSerif = 1,
    /// Courier typeface
    Courier = 2,
    /// Script typeface
    Script = 3,
    /// Prestige typeface
    Prestige = 4,
}

impl Font {
    /// Convert font to ESC/P2 parameter byte
    pub fn as_byte(self) -> u8 {
        self as u8
    }

    /// Generate ESC/P2 command bytes for font selection (ESC k n)
    pub fn as_command(self) -> [u8; 3] {
        [0x1B, 0x6B, self.as_byte()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_values() {
        assert_eq!(Font::Roman.as_byte(), 0);
        assert_eq!(Font::SansSerif.as_byte(), 1);
        assert_eq!(Font::Courier.as_byte(), 2);
        assert_eq!(Font::Script.as_byte(), 3);
        assert_eq!(Font::Prestige.as_byte(), 4);
    }

    #[test]
    fn test_font_command() {
        assert_eq!(Font::Roman.as_command(), [0x1B, 0x6B, 0]);
        assert_eq!(Font::Courier.as_command(), [0x1B, 0x6B, 2]);
    }
}

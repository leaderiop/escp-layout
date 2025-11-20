//! Graphics mode type definitions

/// Graphics density modes for bitmap printing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsMode {
    /// 60 DPI (Single density)
    SingleDensity,
    /// 120 DPI (Double density)
    DoubleDensity,
    /// 180 DPI (High density)
    HighDensity,
}

impl GraphicsMode {
    /// Convert graphics mode to ESC/P2 command byte
    pub fn as_command_byte(self) -> u8 {
        match self {
            GraphicsMode::SingleDensity => 0x4B, // ESC K
            GraphicsMode::DoubleDensity => 0x4C, // ESC L
            GraphicsMode::HighDensity => 0x59,   // ESC Y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphics_mode_bytes() {
        assert_eq!(GraphicsMode::SingleDensity.as_command_byte(), 0x4B);
        assert_eq!(GraphicsMode::DoubleDensity.as_command_byte(), 0x4C);
        assert_eq!(GraphicsMode::HighDensity.as_command_byte(), 0x59);
    }
}

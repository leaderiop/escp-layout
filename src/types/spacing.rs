//! Line spacing type definitions

/// Line spacing configuration in 1/180-inch units
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineSpacing {
    /// Default 1/6-inch spacing (30/180)
    Default,
    /// Custom spacing in 1/180-inch units
    Custom(u8),
}

impl LineSpacing {
    /// Generate ESC/P2 command bytes for line spacing
    pub fn as_command(self) -> Vec<u8> {
        match self {
            LineSpacing::Default => vec![0x1B, 0x32], // ESC 2
            LineSpacing::Custom(dots) => vec![0x1B, 0x33, dots], // ESC 3 n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_spacing_default() {
        assert_eq!(LineSpacing::Default.as_command(), vec![0x1B, 0x32]);
    }

    #[test]
    fn test_line_spacing_custom() {
        assert_eq!(LineSpacing::Custom(60).as_command(), vec![0x1B, 0x33, 60]);
        assert_eq!(LineSpacing::Custom(180).as_command(), vec![0x1B, 0x33, 180]);
    }
}

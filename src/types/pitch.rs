//! Character pitch type definitions

/// Character pitch settings (characters per inch)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pitch {
    /// 10 characters per inch (Pica)
    Pica,
    /// 12 characters per inch (Elite)
    Elite,
    /// 15 characters per inch (Condensed)
    Condensed,
}

impl Pitch {
    /// Generate ESC/P2 command bytes for pitch selection
    pub fn as_command(self) -> &'static [u8] {
        match self {
            Pitch::Pica => &[0x1B, 0x50],      // ESC P
            Pitch::Elite => &[0x1B, 0x4D],     // ESC M
            Pitch::Condensed => &[0x1B, 0x67], // ESC g
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_commands() {
        assert_eq!(Pitch::Pica.as_command(), &[0x1B, 0x50]);
        assert_eq!(Pitch::Elite.as_command(), &[0x1B, 0x4D]);
        assert_eq!(Pitch::Condensed.as_command(), &[0x1B, 0x67]);
    }
}

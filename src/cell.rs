//! Cell and StyleFlags types for representing individual character positions.

/// Style flags for text formatting using bit packing.
///
/// Represents bold and underline styles that can be combined.
/// Uses bit manipulation for compact representation (1 byte).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StyleFlags(u8);

impl StyleFlags {
    /// No styling applied
    pub const NONE: StyleFlags = StyleFlags(0);

    /// Bold text style
    pub const BOLD: StyleFlags = StyleFlags(1 << 0);

    /// Underline text style
    pub const UNDERLINE: StyleFlags = StyleFlags(1 << 1);

    /// Returns true if bold style is active
    #[inline]
    pub fn bold(self) -> bool {
        self.0 & Self::BOLD.0 != 0
    }

    /// Returns true if underline style is active
    #[inline]
    pub fn underline(self) -> bool {
        self.0 & Self::UNDERLINE.0 != 0
    }

    /// Returns a new StyleFlags with bold enabled
    #[inline]
    pub fn with_bold(self) -> Self {
        StyleFlags(self.0 | Self::BOLD.0)
    }

    /// Returns a new StyleFlags with underline enabled
    #[inline]
    pub fn with_underline(self) -> Self {
        StyleFlags(self.0 | Self::UNDERLINE.0)
    }
}

/// Represents a single character cell in the page grid.
///
/// Each cell contains one ASCII character and associated style flags.
/// Non-ASCII characters are automatically converted to '?' during construction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    /// ASCII character (values 32-126, or 63 for '?')
    pub(crate) character: u8,
    /// Style flags (bold, underline)
    pub(crate) style: StyleFlags,
}

impl Cell {
    /// Empty cell containing a space character with no styling
    pub const EMPTY: Cell = Cell {
        character: b' ',
        style: StyleFlags::NONE,
    };

    /// Creates a new cell with the specified character and style.
    ///
    /// Non-ASCII characters (char code > 127) are replaced with '?' (0x3F).
    /// Control characters (char code < 32) are replaced with '?' (0x3F).
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{Cell, StyleFlags};
    ///
    /// let cell = Cell::new('A', StyleFlags::BOLD);
    /// assert_eq!(cell.character(), 'A');
    ///
    /// // Non-ASCII converted to '?'
    /// let cell = Cell::new('é', StyleFlags::NONE);
    /// assert_eq!(cell.character(), '?');
    /// ```
    pub fn new(ch: char, style: StyleFlags) -> Self {
        let character = if ch.is_ascii() && (ch as u8) >= 32 && (ch as u8) <= 126 {
            ch as u8
        } else {
            // Non-ASCII or control character → '?'
            b'?'
        };

        Cell { character, style }
    }

    /// Returns the character as a char
    #[inline]
    pub fn character(&self) -> char {
        self.character as char
    }

    /// Returns the style flags
    #[inline]
    pub fn style(&self) -> StyleFlags {
        self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_flags_constants() {
        assert_eq!(StyleFlags::NONE.0, 0);
        assert_eq!(StyleFlags::BOLD.0, 1);
        assert_eq!(StyleFlags::UNDERLINE.0, 2);
    }

    #[test]
    fn test_style_flags_bold() {
        assert!(!StyleFlags::NONE.bold());
        assert!(StyleFlags::BOLD.bold());
        assert!(!StyleFlags::UNDERLINE.bold());
    }

    #[test]
    fn test_style_flags_underline() {
        assert!(!StyleFlags::NONE.underline());
        assert!(!StyleFlags::BOLD.underline());
        assert!(StyleFlags::UNDERLINE.underline());
    }

    #[test]
    fn test_style_flags_with_bold() {
        let style = StyleFlags::NONE.with_bold();
        assert!(style.bold());
        assert!(!style.underline());
    }

    #[test]
    fn test_style_flags_with_underline() {
        let style = StyleFlags::NONE.with_underline();
        assert!(!style.bold());
        assert!(style.underline());
    }

    #[test]
    fn test_style_flags_combined() {
        let style = StyleFlags::BOLD.with_underline();
        assert!(style.bold());
        assert!(style.underline());
    }

    #[test]
    fn test_cell_empty() {
        assert_eq!(Cell::EMPTY.character(), ' ');
        assert_eq!(Cell::EMPTY.style(), StyleFlags::NONE);
    }

    #[test]
    fn test_cell_new_ascii() {
        let cell = Cell::new('A', StyleFlags::BOLD);
        assert_eq!(cell.character(), 'A');
        assert_eq!(cell.style(), StyleFlags::BOLD);
    }

    #[test]
    fn test_cell_new_non_ascii() {
        let cell = Cell::new('é', StyleFlags::NONE);
        assert_eq!(cell.character(), '?');
    }

    #[test]
    fn test_cell_new_control_char() {
        let cell = Cell::new('\n', StyleFlags::NONE);
        assert_eq!(cell.character(), '?');

        let cell = Cell::new('\t', StyleFlags::NONE);
        assert_eq!(cell.character(), '?');
    }

    #[test]
    fn test_cell_equality() {
        let cell1 = Cell::new('A', StyleFlags::BOLD);
        let cell2 = Cell::new('A', StyleFlags::BOLD);
        let cell3 = Cell::new('B', StyleFlags::BOLD);

        assert_eq!(cell1, cell2);
        assert_ne!(cell1, cell3);
    }
}

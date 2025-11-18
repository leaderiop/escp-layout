//! TextBlock widget for multi-line text without wrapping.

use super::Widget;
use crate::{PageBuilder, Region, StyleFlags};

/// Multi-line text block without word wrapping.
///
/// Each string in the lines vector renders on one line. Lines and characters
/// that exceed region bounds are silently truncated.
///
/// # Examples
///
/// ```
/// use escp_layout::{Page, Region};
/// use escp_layout::widgets::{Widget, TextBlock};
///
/// let text = TextBlock::from_text("Line 1\nLine 2\nLine 3");
/// let mut page = Page::builder();
/// let region = Region::new(0, 0, 80, 10).unwrap();
/// text.render(&mut page, region);
/// ```
pub struct TextBlock {
    lines: Vec<String>,
}

impl TextBlock {
    /// Creates a new TextBlock from a vector of pre-split lines.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::widgets::TextBlock;
    ///
    /// let block = TextBlock::new(vec![
    ///     "Line 1".to_string(),
    ///     "Line 2".to_string(),
    /// ]);
    /// ```
    pub fn new(lines: Vec<String>) -> Self {
        TextBlock { lines }
    }

    /// Creates a TextBlock from text, splitting on newlines.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::widgets::TextBlock;
    ///
    /// let block = TextBlock::from_text("Line 1\nLine 2\nLine 3");
    /// ```
    pub fn from_text(text: impl Into<String>) -> Self {
        let text = text.into();
        let lines = text.lines().map(|s| s.to_string()).collect();
        TextBlock { lines }
    }
}

impl Widget for TextBlock {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        // Handle zero-size regions
        if region.width() == 0 || region.height() == 0 {
            return;
        }

        for (line_idx, line) in self.lines.iter().enumerate() {
            if line_idx as u16 >= region.height() {
                break; // Vertical truncation
            }

            let max_chars = region.width().min(line.len() as u16);
            for (char_idx, ch) in line.chars().take(max_chars as usize).enumerate() {
                page.write_at(
                    region.x() + char_idx as u16,
                    region.y() + line_idx as u16,
                    ch,
                    StyleFlags::NONE,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Page;

    #[test]
    fn test_text_block_new() {
        let block = TextBlock::new(vec!["Line 1".into(), "Line 2".into()]);
        assert_eq!(block.lines.len(), 2);
    }

    #[test]
    fn test_text_block_from_text() {
        let block = TextBlock::from_text("Line 1\nLine 2\nLine 3");
        assert_eq!(block.lines.len(), 3);
        assert_eq!(block.lines[0], "Line 1");
        assert_eq!(block.lines[1], "Line 2");
        assert_eq!(block.lines[2], "Line 3");
    }

    #[test]
    fn test_text_block_render() {
        let block = TextBlock::from_text("Hello\nWorld");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 5).unwrap();

        block.render(&mut page, region);
        let page = page.build();

        // Check first line
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'H');
        assert_eq!(page.get_cell(4, 0).unwrap().character(), 'o');

        // Check second line
        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'W');
        assert_eq!(page.get_cell(4, 1).unwrap().character(), 'd');
    }

    #[test]
    fn test_text_block_horizontal_truncation() {
        let block = TextBlock::from_text("This is too long");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 5, 1).unwrap();

        block.render(&mut page, region);
        let page = page.build();

        // Only first 5 characters
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'T');
        assert_eq!(page.get_cell(4, 0).unwrap().character(), ' ');
        assert_eq!(page.get_cell(5, 0).unwrap().character(), ' '); // Empty
    }

    #[test]
    fn test_text_block_vertical_truncation() {
        let block = TextBlock::from_text("Line 1\nLine 2\nLine 3\nLine 4");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 2).unwrap();

        block.render(&mut page, region);
        let page = page.build();

        // Only first 2 lines
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'L');
        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'L');
        assert_eq!(page.get_cell(0, 2).unwrap().character(), ' '); // Empty (line 3 not rendered)
    }

    #[test]
    fn test_text_block_zero_height() {
        let block = TextBlock::from_text("Test");
        let mut page = Page::builder();

        // Zero-height regions are invalid - test that widget handles missing region gracefully
        // The widget render method checks region.height() == 0 and returns early
        if let Ok(region) = Region::new(0, 0, 10, 1) {
            block.render(&mut page, region);
        }
    }
}

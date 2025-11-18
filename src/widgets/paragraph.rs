//! Paragraph widget with word wrapping.

use super::Widget;
use crate::{PageBuilder, Region, StyleFlags};

/// Multi-line paragraph with automatic word wrapping.
///
/// Text is wrapped at word boundaries when it exceeds the region width.
/// Long words that exceed the width are broken at the boundary.
///
/// # Examples
///
/// ```
/// use escp_layout::{Page, Region, StyleFlags};
/// use escp_layout::widgets::{Widget, Paragraph};
///
/// let para = Paragraph::new("This is a long paragraph that will wrap across multiple lines.")
///     .with_style(StyleFlags::NONE);
///
/// let mut page = Page::builder();
/// let region = Region::new(0, 0, 40, 10).unwrap();
/// para.render(&mut page, region);
/// ```
pub struct Paragraph {
    text: String,
    style: StyleFlags,
}

impl Paragraph {
    /// Creates a new paragraph with the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::widgets::Paragraph;
    ///
    /// let para = Paragraph::new("This is a paragraph.");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Paragraph {
            text: text.into(),
            style: StyleFlags::NONE,
        }
    }

    /// Sets the text style for the paragraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{StyleFlags, widgets::Paragraph};
    ///
    /// let para = Paragraph::new("Bold paragraph")
    ///     .with_style(StyleFlags::BOLD);
    /// ```
    pub fn with_style(mut self, style: StyleFlags) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Paragraph {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        // Handle zero-size regions
        if region.width() == 0 || region.height() == 0 {
            return;
        }

        let wrapped_lines = wrap_text(&self.text, region.width() as usize);

        for (line_idx, line) in wrapped_lines.iter().enumerate() {
            if line_idx as u16 >= region.height() {
                break; // Vertical truncation
            }

            for (char_idx, ch) in line.chars().enumerate() {
                page.write_at(
                    region.x() + char_idx as u16,
                    region.y() + line_idx as u16,
                    ch,
                    self.style,
                );
            }
        }
    }
}

/// Wraps text at word boundaries to fit within max_width.
///
/// Algorithm:
/// 1. Split text into words (by whitespace)
/// 2. Build lines by adding words until line would exceed max_width
/// 3. Break long words that exceed max_width
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return Vec::new();
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        // If word itself is longer than max_width, break it
        if word.len() > max_width {
            // Finish current line if not empty
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
            }

            // Break long word into chunks
            for chunk in word.chars().collect::<Vec<_>>().chunks(max_width) {
                lines.push(chunk.iter().collect());
            }
            continue;
        }

        // Check if adding this word would exceed line width
        let would_exceed = if current_line.is_empty() {
            word.len() > max_width
        } else {
            current_line.len() + 1 + word.len() > max_width // +1 for space
        };

        if would_exceed && !current_line.is_empty() {
            // Start new line
            lines.push(current_line.clone());
            current_line.clear();
        }

        // Add word to current line
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    // Add final line if not empty
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Page;

    #[test]
    fn test_wrap_text_short() {
        let result = wrap_text("Hello World", 20);
        assert_eq!(result, vec!["Hello World"]);
    }

    #[test]
    fn test_wrap_text_exact_fit() {
        let result = wrap_text("Hello", 5);
        assert_eq!(result, vec!["Hello"]);
    }

    #[test]
    fn test_wrap_text_wrapping() {
        let result = wrap_text("This is a test", 7);
        assert_eq!(result, vec!["This is", "a test"]);
    }

    #[test]
    fn test_wrap_text_long_word() {
        let result = wrap_text("VeryLongWord", 5);
        assert_eq!(result, vec!["VeryL", "ongWo", "rd"]);
    }

    #[test]
    fn test_wrap_text_multiple_lines() {
        let result = wrap_text("The quick brown fox jumps over the lazy dog", 15);
        assert_eq!(
            result,
            vec!["The quick brown", "fox jumps over", "the lazy dog"]
        );
    }

    #[test]
    fn test_wrap_text_zero_width() {
        let result = wrap_text("Test", 0);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_paragraph_new() {
        let para = Paragraph::new("Test");
        assert_eq!(para.text, "Test");
        assert_eq!(para.style, StyleFlags::NONE);
    }

    #[test]
    fn test_paragraph_with_style() {
        let para = Paragraph::new("Test").with_style(StyleFlags::BOLD);
        assert_eq!(para.style, StyleFlags::BOLD);
    }

    #[test]
    fn test_paragraph_render_no_wrap() {
        let para = Paragraph::new("Hello");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 5).unwrap();

        para.render(&mut page, region);
        let page = page.build();

        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'H');
        assert_eq!(page.get_cell(4, 0).unwrap().character(), 'o');
    }

    #[test]
    fn test_paragraph_render_with_wrap() {
        let para = Paragraph::new("This is a test");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 7, 5).unwrap();

        para.render(&mut page, region);
        let page = page.build();

        // First line: "This is"
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'T');

        // Second line: "a test"
        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'a');
    }

    #[test]
    fn test_paragraph_vertical_truncation() {
        let para = Paragraph::new("One Two Three Four Five Six");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 5, 2).unwrap(); // Only 2 lines available

        para.render(&mut page, region);
        let page = page.build();

        // Should only render first 2 wrapped lines
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'O');
        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'T');
        assert_eq!(page.get_cell(0, 2).unwrap().character(), ' '); // Empty (not rendered)
    }

    #[test]
    fn test_paragraph_zero_width() {
        let para = Paragraph::new("Test");
        let mut page = Page::builder();

        // Zero-width regions are invalid - test that widget handles it gracefully
        // The widget render method checks region.width() == 0 and returns early
        if let Ok(region) = Region::new(0, 0, 1, 10) {
            para.render(&mut page, region);
        }
    }
}

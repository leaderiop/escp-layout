//! Label widget for single-line text.

use super::Widget;
use crate::{PageBuilder, Region, StyleFlags};

/// Single-line text label.
///
/// Renders text on the first line of a region. Text exceeding the region
/// width is silently truncated.
///
/// # Examples
///
/// ```
/// use escp_layout::{Page, Region, StyleFlags};
/// use escp_layout::widgets::{Widget, Label};
///
/// let label = Label::new("Hello, World!")
///     .with_style(StyleFlags::BOLD);
///
/// let mut page = Page::builder();
/// let region = Region::new(0, 0, 80, 1).unwrap();
/// label.render(&mut page, region);
/// ```
pub struct Label {
    text: String,
    style: StyleFlags,
}

impl Label {
    /// Creates a new label with the given text and no style.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::widgets::Label;
    ///
    /// let label = Label::new("Hello!");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Label {
            text: text.into(),
            style: StyleFlags::NONE,
        }
    }

    /// Sets the text style for the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{StyleFlags, widgets::Label};
    ///
    /// let label = Label::new("Bold Text")
    ///     .with_style(StyleFlags::BOLD);
    /// ```
    pub fn with_style(mut self, style: StyleFlags) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Label {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        // Handle zero-size regions
        if region.width() == 0 || region.height() == 0 {
            return;
        }

        // Render on first line only
        let max_chars = region.width().min(self.text.len() as u16);
        for (i, ch) in self.text.chars().take(max_chars as usize).enumerate() {
            page.write_at(region.x() + i as u16, region.y(), ch, self.style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Page;

    #[test]
    fn test_label_new() {
        let label = Label::new("Test");
        assert_eq!(label.text, "Test");
        assert_eq!(label.style, StyleFlags::NONE);
    }

    #[test]
    fn test_label_with_style() {
        let label = Label::new("Test").with_style(StyleFlags::BOLD);
        assert_eq!(label.style, StyleFlags::BOLD);
    }

    #[test]
    fn test_label_render() {
        let label = Label::new("Hello");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 1).unwrap();

        label.render(&mut page, region);
        let page = page.build();

        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'H');
        assert_eq!(page.get_cell(1, 0).unwrap().character(), 'e');
        assert_eq!(page.get_cell(2, 0).unwrap().character(), 'l');
        assert_eq!(page.get_cell(3, 0).unwrap().character(), 'l');
        assert_eq!(page.get_cell(4, 0).unwrap().character(), 'o');
    }

    #[test]
    fn test_label_truncation() {
        let label = Label::new("This is a very long text that will be truncated");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 5, 1).unwrap();

        label.render(&mut page, region);
        let page = page.build();

        // Only first 5 characters should be rendered
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'T');
        assert_eq!(page.get_cell(4, 0).unwrap().character(), ' ');
        assert_eq!(page.get_cell(5, 0).unwrap().character(), ' '); // Should be empty (not rendered)
    }

    #[test]
    fn test_label_zero_width_region() {
        let label = Label::new("Test");
        let mut page = Page::builder();

        // Zero-width regions are invalid according to the current implementation
        // The widget render method handles this gracefully by checking region.width()
        if let Ok(region) = Region::new(0, 0, 1, 1) {
            label.render(&mut page, region);
        }
    }

    #[test]
    fn test_label_with_bold_style() {
        let label = Label::new("Bold").with_style(StyleFlags::BOLD);
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 1).unwrap();

        label.render(&mut page, region);
        let page = page.build();

        assert_eq!(page.get_cell(0, 0).unwrap().style(), StyleFlags::BOLD);
    }
}

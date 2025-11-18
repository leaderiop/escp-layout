//! ASCIIBox widget for bordered boxes.

use super::Widget;
use crate::{PageBuilder, Region, StyleFlags};

/// Bordered box with optional title and inner content widget.
///
/// Draws an ASCII border using `+`, `-`, and `|` characters.
/// The inner content is rendered with 1-cell padding on all sides.
///
/// # Examples
///
/// ```
/// use escp_layout::{Page, Region};
/// use escp_layout::widgets::{Widget, ASCIIBox, Label};
///
/// let content = Label::new("Inside the box");
/// let boxed = ASCIIBox::new(Box::new(content))
///     .with_title("Section");
///
/// let mut page = Page::builder();
/// let region = Region::new(0, 0, 25, 5).unwrap();
/// boxed.render(&mut page, region);
/// ```
pub struct ASCIIBox {
    title: Option<String>,
    content: Box<dyn Widget>,
}

impl ASCIIBox {
    /// Creates a new ASCIIBox without a title.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::widgets::{ASCIIBox, Label};
    ///
    /// let content = Label::new("Content");
    /// let boxed = ASCIIBox::new(Box::new(content));
    /// ```
    pub fn new(content: Box<dyn Widget>) -> Self {
        ASCIIBox {
            title: None,
            content,
        }
    }

    /// Sets the title for the box (rendered in the top border).
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::widgets::{ASCIIBox, Label};
    ///
    /// let content = Label::new("Content");
    /// let boxed = ASCIIBox::new(Box::new(content))
    ///     .with_title("My Title");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Widget for ASCIIBox {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        // Need at least 3×3 to draw a visible box
        if region.width() < 3 || region.height() < 3 {
            return;
        }

        let x = region.x();
        let y = region.y();
        let width = region.width();
        let height = region.height();

        // Draw corners
        page.write_at(x, y, '+', StyleFlags::NONE);
        page.write_at(x + width - 1, y, '+', StyleFlags::NONE);
        page.write_at(x, y + height - 1, '+', StyleFlags::NONE);
        page.write_at(x + width - 1, y + height - 1, '+', StyleFlags::NONE);

        // Draw top and bottom borders
        for dx in 1..width - 1 {
            page.write_at(x + dx, y, '-', StyleFlags::NONE);
            page.write_at(x + dx, y + height - 1, '-', StyleFlags::NONE);
        }

        // Draw left and right borders
        for dy in 1..height - 1 {
            page.write_at(x, y + dy, '|', StyleFlags::NONE);
            page.write_at(x + width - 1, y + dy, '|', StyleFlags::NONE);
        }

        // Draw title if present (in top border, starting at x+2)
        if let Some(title) = &self.title {
            let title_start = x + 2;
            let max_title_len = width.saturating_sub(4); // Leave space for corners and padding
            for (i, ch) in title.chars().take(max_title_len as usize).enumerate() {
                page.write_at(title_start + i as u16, y, ch, StyleFlags::NONE);
            }
        }

        // Render inner content with 1-cell padding
        if let Ok(inner_region) = region.with_padding(1, 1, 1, 1) {
            self.content.render(page, inner_region);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::Label;
    use crate::Page;

    #[test]
    fn test_ascii_box_no_title() {
        let content = Label::new("Test");
        let boxed = ASCIIBox::new(Box::new(content));
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 3).unwrap();

        boxed.render(&mut page, region);
        let page = page.build();

        // Check corners
        assert_eq!(page.get_cell(0, 0).unwrap().character(), '+');
        assert_eq!(page.get_cell(9, 0).unwrap().character(), '+');
        assert_eq!(page.get_cell(0, 2).unwrap().character(), '+');
        assert_eq!(page.get_cell(9, 2).unwrap().character(), '+');

        // Check borders
        assert_eq!(page.get_cell(1, 0).unwrap().character(), '-');
        assert_eq!(page.get_cell(0, 1).unwrap().character(), '|');
    }

    #[test]
    fn test_ascii_box_with_title() {
        let content = Label::new("Content");
        let boxed = ASCIIBox::new(Box::new(content)).with_title("Title");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 15, 3).unwrap();

        boxed.render(&mut page, region);
        let page = page.build();

        // Title should appear at x=2 on top border
        assert_eq!(page.get_cell(2, 0).unwrap().character(), 'T');
        assert_eq!(page.get_cell(3, 0).unwrap().character(), 'i');
        assert_eq!(page.get_cell(4, 0).unwrap().character(), 't');
    }

    #[test]
    fn test_ascii_box_too_small() {
        let content = Label::new("Test");
        let boxed = ASCIIBox::new(Box::new(content));
        let mut page = Page::builder();
        let region = Region::new(0, 0, 2, 2).unwrap();

        // Should not panic, but also won't render
        boxed.render(&mut page, region);
    }

    #[test]
    fn test_ascii_box_content_inside() {
        let content = Label::new("Hi");
        let boxed = ASCIIBox::new(Box::new(content));
        let mut page = Page::builder();
        let region = Region::new(0, 0, 6, 3).unwrap();

        boxed.render(&mut page, region);
        let page = page.build();

        // Content should be at (1, 1) with 1-cell padding
        assert_eq!(page.get_cell(1, 1).unwrap().character(), 'H');
        assert_eq!(page.get_cell(2, 1).unwrap().character(), 'i');
    }

    #[test]
    fn test_ascii_box_title_truncation() {
        let content = Label::new("X");
        let boxed =
            ASCIIBox::new(Box::new(content)).with_title("Very Long Title That Will Be Truncated");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 3).unwrap();

        boxed.render(&mut page, region);
        let page = page.build();

        // Title length should be limited to width - 4
        // With width=10, max title is 6 chars (from index 2 to 7)
        // Title: "Very L" (6 chars)
        assert_eq!(page.get_cell(2, 0).unwrap().character(), 'V');
        assert_eq!(page.get_cell(3, 0).unwrap().character(), 'e');
        assert_eq!(page.get_cell(4, 0).unwrap().character(), 'r');
        assert_eq!(page.get_cell(5, 0).unwrap().character(), 'y');
        assert_eq!(page.get_cell(6, 0).unwrap().character(), ' ');
        assert_eq!(page.get_cell(7, 0).unwrap().character(), 'L');
        assert_eq!(page.get_cell(8, 0).unwrap().character(), '-'); // Border, not title
    }
}

//! Page and PageBuilder types for representing 160×51 character grids.

use crate::cell::{Cell, StyleFlags};

/// Represents a single 160×51 character grid page.
///
/// Pages are immutable after construction using the builder pattern.
/// Each page contains a fixed grid of cells storing characters and styles.
#[derive(Clone, Debug)]
pub struct Page {
    /// Fixed 160×51 cell grid (row-major order for cache efficiency)
    cells: Box<[[Cell; 160]; 51]>,
}

impl Page {
    /// Creates a new PageBuilder for constructing a page.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::Page;
    ///
    /// let mut builder = Page::builder();
    /// let page = builder.build();
    /// ```
    pub fn builder() -> PageBuilder {
        PageBuilder::new()
    }

    /// Returns the cell at the specified position, if within bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{Page, Cell, StyleFlags};
    ///
    /// let mut builder = Page::builder();
    /// builder.write_at(10, 5, 'A', StyleFlags::BOLD);
    /// let page = builder.build();
    ///
    /// let cell = page.get_cell(10, 5).unwrap();
    /// assert_eq!(cell.character(), 'A');
    /// ```
    pub fn get_cell(&self, x: u16, y: u16) -> Option<Cell> {
        if x < 160 && y < 51 {
            Some(self.cells[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Returns a reference to the entire cell grid.
    pub fn cells(&self) -> &[[Cell; 160]; 51] {
        &self.cells
    }
}

/// Builder for constructing Pages with mutable operations.
///
/// Consumes itself when building to enforce immutability.
pub struct PageBuilder {
    cells: Box<[[Cell; 160]; 51]>,
}

impl PageBuilder {
    /// Creates a new PageBuilder with all cells initialized to EMPTY.
    fn new() -> Self {
        PageBuilder {
            cells: Box::new([[Cell::EMPTY; 160]; 51]),
        }
    }

    /// Writes a single character at the specified position.
    ///
    /// Out-of-bounds writes are silently ignored (no panic, no error).
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{Page, StyleFlags};
    ///
    /// let mut builder = Page::builder();
    /// builder.write_at(0, 0, 'H', StyleFlags::BOLD);
    /// builder.write_at(1, 0, 'i', StyleFlags::NONE);
    ///
    /// // Out of bounds - silently ignored
    /// builder.write_at(200, 0, 'X', StyleFlags::NONE);
    /// ```
    pub fn write_at(&mut self, x: u16, y: u16, ch: char, style: StyleFlags) -> &mut Self {
        if x < 160 && y < 51 {
            self.cells[y as usize][x as usize] = Cell::new(ch, style);
        }
        // Silent truncation - no panic, no error
        self
    }

    /// Writes a string starting at the specified position.
    ///
    /// Characters exceeding line width are silently truncated.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{Page, StyleFlags};
    ///
    /// let mut builder = Page::builder();
    /// builder.write_str(0, 0, "Hello, World!", StyleFlags::NONE);
    /// ```
    pub fn write_str(&mut self, x: u16, y: u16, text: &str, style: StyleFlags) -> &mut Self {
        let mut current_x = x;
        for ch in text.chars() {
            if current_x >= 160 {
                break; // Truncate at line boundary
            }
            self.write_at(current_x, y, ch, style);
            current_x += 1;
        }
        self
    }

    /// Fills a rectangular region with the specified character.
    ///
    /// Useful for drawing borders or background patterns.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{Page, Region, StyleFlags};
    ///
    /// let mut builder = Page::builder();
    /// let region = Region::new(0, 0, 80, 1).unwrap();
    /// builder.fill_region(region, '-', StyleFlags::NONE);
    /// ```
    pub fn fill_region(
        &mut self,
        region: crate::region::Region,
        ch: char,
        style: StyleFlags,
    ) -> &mut Self {
        for y in region.y()..region.y() + region.height() {
            for x in region.x()..region.x() + region.width() {
                self.write_at(x, y, ch, style);
            }
        }
        self
    }

    /// Renders a widget into the specified region.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::{Page, Region};
    /// use escp_layout::widgets::{Widget, Label};
    ///
    /// let mut builder = Page::builder();
    /// let region = Region::new(0, 0, 80, 1).unwrap();
    /// let label = Label::new("Hello, World!");
    /// builder.render_widget(region, &label);
    /// ```
    pub fn render_widget(
        &mut self,
        region: crate::region::Region,
        widget: &dyn crate::widgets::Widget,
    ) -> &mut Self {
        widget.render(self, region);
        self
    }

    /// Render a widget tree to this page (new widget composability system).
    ///
    /// The widget tree is traversed depth-first, with each widget rendering
    /// at its cumulative absolute position. The root widget is always positioned
    /// at (0, 0).
    ///
    /// Widgets are borrowed immutably and can be rendered multiple times.
    ///
    /// # Errors
    ///
    /// Returns `RenderError::OutOfBounds` if any widget attempts to render
    /// outside page bounds (160 × 51).
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::Page;
    /// use escp_layout::widget::{box_new, label_new};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut root = box_new!(80, 30);
    /// let label = label_new!(20).add_text("Hello")?;
    /// root.add_child(label, (0, 0))?;
    ///
    /// let mut page_builder = Page::builder();
    /// page_builder.render(&root)?;  // Can render multiple times
    /// # Ok(())
    /// # }
    /// ```
    pub fn render(
        &mut self,
        widget: &impl crate::widget::Widget,
    ) -> Result<(), crate::widget::RenderError> {
        let mut context = crate::widget::RenderContext::new(self);
        widget.render_to(&mut context, (0, 0))
    }

    /// Consumes the builder and returns an immutable Page.
    ///
    /// After calling this, the builder cannot be reused.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::Page;
    ///
    /// let mut builder = Page::builder();
    /// builder.write_str(0, 0, "Test", escp_layout::StyleFlags::NONE);
    /// let page = builder.build();
    /// ```
    pub fn build(self) -> Page {
        Page { cells: self.cells }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_builder_new() {
        let builder = PageBuilder::new();
        // Verify all cells are EMPTY
        assert_eq!(builder.cells[0][0], Cell::EMPTY);
        assert_eq!(builder.cells[50][159], Cell::EMPTY);
    }

    #[test]
    fn test_page_builder_write_at() {
        let mut builder = PageBuilder::new();
        builder.write_at(10, 5, 'A', StyleFlags::BOLD);

        let cell = builder.cells[5][10];
        assert_eq!(cell.character(), 'A');
        assert_eq!(cell.style(), StyleFlags::BOLD);
    }

    #[test]
    fn test_page_builder_write_at_out_of_bounds() {
        let mut builder = PageBuilder::new();

        // Should not panic - silent truncation
        builder.write_at(200, 0, 'X', StyleFlags::NONE);
        builder.write_at(0, 100, 'Y', StyleFlags::NONE);
    }

    #[test]
    fn test_page_builder_write_str() {
        let mut builder = PageBuilder::new();
        builder.write_str(0, 0, "Hello", StyleFlags::NONE);

        assert_eq!(builder.cells[0][0].character(), 'H');
        assert_eq!(builder.cells[0][1].character(), 'e');
        assert_eq!(builder.cells[0][2].character(), 'l');
        assert_eq!(builder.cells[0][3].character(), 'l');
        assert_eq!(builder.cells[0][4].character(), 'o');
    }

    #[test]
    fn test_page_builder_write_str_truncation() {
        let mut builder = PageBuilder::new();

        // Write a very long string - should truncate at column 160
        let long_string = "A".repeat(200);
        builder.write_str(0, 0, &long_string, StyleFlags::NONE);

        assert_eq!(builder.cells[0][159].character(), 'A');
        // No panic occurred
    }

    #[test]
    fn test_page_builder_fill_region() {
        let mut builder = PageBuilder::new();
        let region = crate::region::Region::new(0, 0, 5, 3).unwrap();

        builder.fill_region(region, '-', StyleFlags::NONE);

        // Check filled area
        for y in 0..3 {
            for x in 0..5 {
                assert_eq!(builder.cells[y][x].character(), '-');
            }
        }

        // Check area outside region is still empty
        assert_eq!(builder.cells[0][5].character(), ' ');
        assert_eq!(builder.cells[3][0].character(), ' ');
    }

    #[test]
    fn test_page_builder_build() {
        let mut builder = PageBuilder::new();
        builder.write_str(0, 0, "Test", StyleFlags::NONE);

        let page = builder.build();

        let cell = page.get_cell(0, 0).unwrap();
        assert_eq!(cell.character(), 'T');
    }

    #[test]
    fn test_page_get_cell_valid() {
        let mut builder = PageBuilder::new();
        builder.write_at(50, 25, 'X', StyleFlags::UNDERLINE);
        let page = builder.build();

        let cell = page.get_cell(50, 25).unwrap();
        assert_eq!(cell.character(), 'X');
        assert_eq!(cell.style(), StyleFlags::UNDERLINE);
    }

    #[test]
    fn test_page_get_cell_out_of_bounds() {
        let page = Page::builder().build();

        assert!(page.get_cell(200, 0).is_none());
        assert!(page.get_cell(0, 100).is_none());
    }

    #[test]
    fn test_page_builder_write_at_exact_boundaries() {
        let mut builder = PageBuilder::new();

        // Test exact boundaries - should work without panic
        builder.write_at(159, 50, 'X', StyleFlags::NONE);
        builder.write_at(159, 0, 'Y', StyleFlags::NONE);
        builder.write_at(0, 50, 'Z', StyleFlags::NONE);

        let page = builder.build();

        // Verify writes succeeded
        assert_eq!(page.get_cell(159, 50).unwrap().character(), 'X');
        assert_eq!(page.get_cell(159, 0).unwrap().character(), 'Y');
        assert_eq!(page.get_cell(0, 50).unwrap().character(), 'Z');

        // Test just beyond boundaries - should be silent (no panic)
        let mut builder = PageBuilder::new();
        builder.write_at(160, 0, 'A', StyleFlags::NONE);
        builder.write_at(0, 51, 'B', StyleFlags::NONE);
        // No assertions needed - test passes if no panic
    }

    #[test]
    fn test_page_cells() {
        let mut builder = PageBuilder::new();
        builder.write_at(0, 0, 'A', StyleFlags::NONE);
        let page = builder.build();

        let cells = page.cells();
        assert_eq!(cells[0][0].character(), 'A');
    }

    #[test]
    fn test_page_immutability() {
        let mut builder = Page::builder();
        builder.write_str(0, 0, "Immutable", StyleFlags::NONE);
        let page = builder.build();

        // This test verifies that Page has no public mutable methods
        // If we could mutate the page, this would fail to compile
        let _cell = page.get_cell(0, 0);
    }
}

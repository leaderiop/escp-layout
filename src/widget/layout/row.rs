//! Row layout component for horizontal division.

use crate::widget::{Rect, RenderError};

/// Layout component that divides parent Rect horizontally.
///
/// Returns nested Rect widgets for each column with compile-time dimensions
/// specified via turbofish syntax.
///
/// # Examples
///
/// ```rust
/// use escp_layout::widget::layout::{Row, row_new, row_area};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut row = row_new!(80, 30);
/// let (col1, pos1) = row_area!(row, 20)?;  // Rect<20, 30>
/// let (col2, pos2) = row_area!(row, 60)?;  // Rect<60, 30>
/// # Ok(())
/// # }
/// ```
pub struct Row<const WIDTH: u16, const HEIGHT: u16> {
    current_x: u16,
}

impl<const WIDTH: u16, const HEIGHT: u16> Default for Row<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> Row<WIDTH, HEIGHT> {
    /// Create a Row layout with const generic parent dimensions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::layout::Row;
    ///
    /// let row = Row::<80, 30>::new();
    /// ```
    pub fn new() -> Self {
        Self { current_x: 0 }
    }

    /// Allocate a vertical area (column) with specified width via const generic.
    ///
    /// Returns a Rect<W, HEIGHT> positioned at the next available X offset.
    ///
    /// # Errors
    ///
    /// Returns `RenderError::InsufficientSpace` if requested width exceeds
    /// remaining space.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::layout::Row;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut row = Row::<80, 30>::new();
    /// let (rect1, pos1) = row.area::<20>()?; // Returns Rect<20, 30>
    /// # Ok(())
    /// # }
    /// ```
    pub fn area<const W: u16>(&mut self) -> Result<(Rect<W, HEIGHT>, (u16, u16)), RenderError> {
        if self.current_x + W > WIDTH {
            return Err(RenderError::InsufficientSpace {
                available: WIDTH - self.current_x,
                required: W,
                layout_type: "Row",
            });
        }

        let position = (self.current_x, 0);
        let rect_widget = Rect::<W, HEIGHT>::new();

        self.current_x += W;

        Ok((rect_widget, position))
    }
}

/// Ergonomic macro for creating Row layouts.
#[macro_export]
macro_rules! row_new {
    ($w:expr, $h:expr) => {
        $crate::widget::layout::Row::<$w, $h>::new()
    };
}

/// Ergonomic macro for allocating row areas.
#[macro_export]
macro_rules! row_area {
    ($layout:expr, $w:expr) => {
        $layout.area::<$w>()
    };
}

pub use row_area;
pub use row_new;

// Implementation will be added in Phase 5

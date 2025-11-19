//! Column layout component for vertical division.

use crate::widget::{Box, RenderError};

/// Layout component that divides parent Box vertically.
///
/// Returns nested Box widgets for each row with compile-time dimensions
/// specified via turbofish syntax.
///
/// # Examples
///
/// ```rust
/// use escp_layout::widget::layout::{Column, column_new, column_area};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut column = column_new!(80, 30);
/// let (row1, pos1) = column_area!(column, 10)?;  // Box<80, 10>
/// let (row2, pos2) = column_area!(column, 20)?;  // Box<80, 20>
/// # Ok(())
/// # }
/// ```
pub struct Column<const WIDTH: u16, const HEIGHT: u16> {
    current_y: u16,
}

impl<const WIDTH: u16, const HEIGHT: u16> Column<WIDTH, HEIGHT> {
    /// Create a Column layout with const generic parent dimensions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::layout::Column;
    ///
    /// let column = Column::<80, 30>::new();
    /// ```
    pub fn new() -> Self {
        Self { current_y: 0 }
    }

    /// Allocate a horizontal area (row) with specified height via const generic.
    ///
    /// Returns a Box<WIDTH, H> positioned at the next available Y offset.
    ///
    /// # Errors
    ///
    /// Returns `RenderError::InsufficientSpace` if requested height exceeds
    /// remaining space.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::layout::Column;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut column = Column::<80, 30>::new();
    /// let (box1, pos1) = column.area::<10>()?; // Returns Box<80, 10>
    /// # Ok(())
    /// # }
    /// ```
    pub fn area<const H: u16>(&mut self) -> Result<(Box<WIDTH, H>, (u16, u16)), RenderError> {
        if self.current_y + H > HEIGHT {
            return Err(RenderError::InsufficientSpace {
                available: HEIGHT - self.current_y,
                required: H,
                layout_type: "Column",
            });
        }

        let position = (0, self.current_y);
        let box_widget = Box::<WIDTH, H>::new();

        self.current_y += H;

        Ok((box_widget, position))
    }
}

/// Ergonomic macro for creating Column layouts.
#[macro_export]
macro_rules! column_new {
    ($w:expr, $h:expr) => {
        $crate::widget::layout::Column::<$w, $h>::new()
    };
}

/// Ergonomic macro for allocating column areas.
#[macro_export]
macro_rules! column_area {
    ($layout:expr, $h:expr) => {
        $layout.area::<$h>()
    };
}

pub use column_area;
pub use column_new;

// Implementation will be added in Phase 5

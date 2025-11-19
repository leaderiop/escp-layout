//! Stack layout component for overlapping layers.

use crate::widget::Box;

/// Layout component for overlapping layers.
///
/// Returns overlapping nested Box widgets at the same position (for layering)
/// with compile-time dimensions.
///
/// # Examples
///
/// ```rust
/// use escp_layout::widget::layout::{Stack, stack_new};
///
/// let stack = stack_new!(80, 30);
/// let (bg, pos) = stack.area();  // Box<80, 30> at (0, 0)
/// let (fg, pos) = stack.area();  // Box<80, 30> at (0, 0)
/// ```
pub struct Stack<const WIDTH: u16, const HEIGHT: u16>;

impl<const WIDTH: u16, const HEIGHT: u16> Stack<WIDTH, HEIGHT> {
    /// Create a Stack layout with const generic parent dimensions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::layout::Stack;
    ///
    /// let stack = Stack::<80, 30>::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Allocate an overlapping area (same position for all calls).
    ///
    /// Returns a Box<WIDTH, HEIGHT> positioned at (0, 0) for layering.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::layout::Stack;
    ///
    /// let stack = Stack::<80, 30>::new();
    /// let (box1, pos1) = stack.area(); // Returns Box<80, 30> at (0, 0)
    /// let (box2, pos2) = stack.area(); // Returns Box<80, 30> at (0, 0)
    /// ```
    pub fn area(&self) -> (Box<WIDTH, HEIGHT>, (u16, u16)) {
        let position = (0, 0);
        let box_widget = Box::<WIDTH, HEIGHT>::new();
        (box_widget, position)
    }
}

/// Ergonomic macro for creating Stack layouts.
#[macro_export]
macro_rules! stack_new {
    ($w:expr, $h:expr) => {
        $crate::widget::layout::Stack::<$w, $h>::new()
    };
}

pub use stack_new;

// Implementation will be added in Phase 5

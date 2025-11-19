//! Error types for widget composition and rendering.

use std::fmt;

/// Comprehensive error type for all widget rendering violations.
///
/// Per Constitution Principle III Widget Exception, boundary violations
/// during widget composition and rendering return `Result` errors instead
/// of panicking.
///
/// # Non-Exhaustive
///
/// This enum is marked `#[non_exhaustive]` to allow future error variants
/// without breaking changes (V2+).
///
/// # Examples
///
/// ```rust
/// use escp_layout::widget::{Box, Label, RenderError, box_new, label_new};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut parent = box_new!(20, 20);
/// let child = box_new!(30, 10);
///
/// match parent.add_child(child, (0, 0)) {
///     Err(RenderError::ChildExceedsParent { .. }) => {
///         println!("Child is too large for parent");
///     }
///     Ok(()) => println!("Success"),
///     Err(e) => println!("Other error: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// Child widget exceeds parent widget's bounds.
    ///
    /// Returned by `Box::add_child()` when the child's dimensions extend
    /// beyond the parent's WIDTH or HEIGHT.
    ChildExceedsParent {
        /// Parent widget width
        parent_width: u16,
        /// Parent widget height
        parent_height: u16,
        /// Child widget width
        child_width: u16,
        /// Child widget height
        child_height: u16,
        /// Child position within parent
        position: (u16, u16),
    },

    /// Widget positioned outside valid bounds.
    ///
    /// Returned by `RenderContext` when a write operation starts outside
    /// the current clip bounds.
    OutOfBounds {
        /// Write position (x, y)
        position: (u16, u16),
        /// Clip bounds (x, y, width, height)
        bounds: (u16, u16, u16, u16),
    },

    /// Two or more children overlap within parent widget.
    ///
    /// Returned by `Box::add_child()` when AABB collision detection finds
    /// an intersection between the new child and an existing child.
    ///
    /// Note: Touching edges (shared boundary) does NOT count as overlap.
    OverlappingChildren {
        /// First child bounds (x, y, width, height)
        child1_bounds: (u16, u16, u16, u16),
        /// Second child bounds (x, y, width, height)
        child2_bounds: (u16, u16, u16, u16),
    },

    /// Layout component cannot fit all children in available space.
    ///
    /// Returned by layout component `area()` methods (Column, Row) when
    /// the requested area exceeds remaining space.
    InsufficientSpace {
        /// Available space in the layout dimension
        available: u16,
        /// Required space for this area
        required: u16,
        /// Layout type ("Column", "Row", etc.)
        layout_type: &'static str,
    },

    /// Integer overflow in coordinate or size calculation.
    ///
    /// Returned when checked arithmetic detects overflow (e.g., position + size
    /// exceeds `u16::MAX`).
    IntegerOverflow {
        /// Description of the operation that overflowed
        operation: String,
    },

    /// Text content exceeds widget width or contains newlines.
    ///
    /// Returned by `Label::add_text()` when text validation fails:
    /// - Text length exceeds widget WIDTH
    /// - Text contains newline characters (`\n`, `\r\n`)
    TextExceedsWidth {
        /// Length of the provided text
        text_length: u16,
        /// Widget width constraint
        widget_width: u16,
    },
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::ChildExceedsParent {
                parent_width,
                parent_height,
                child_width,
                child_height,
                position,
            } => write!(
                f,
                "Child widget ({}×{}) at position ({}, {}) exceeds parent bounds ({}×{})",
                child_width, child_height, position.0, position.1, parent_width, parent_height
            ),
            RenderError::OutOfBounds { position, bounds } => write!(
                f,
                "Position ({}, {}) exceeds bounds ({}×{} at {}, {})",
                position.0, position.1, bounds.2, bounds.3, bounds.0, bounds.1
            ),
            RenderError::OverlappingChildren {
                child1_bounds,
                child2_bounds,
            } => write!(
                f,
                "Child widgets overlap: child1 (x:{}, y:{}, w:{}, h:{}) intersects child2 (x:{}, y:{}, w:{}, h:{})",
                child1_bounds.0, child1_bounds.1, child1_bounds.2, child1_bounds.3,
                child2_bounds.0, child2_bounds.1, child2_bounds.2, child2_bounds.3
            ),
            RenderError::InsufficientSpace {
                available,
                required,
                layout_type,
            } => write!(
                f,
                "{} layout requires {} units but only {} available",
                layout_type, required, available
            ),
            RenderError::IntegerOverflow { operation } => {
                write!(f, "Integer overflow in {}", operation)
            }
            RenderError::TextExceedsWidth {
                text_length,
                widget_width,
            } => write!(
                f,
                "Text length ({}) exceeds widget width ({})",
                text_length, widget_width
            ),
        }
    }
}

impl std::error::Error for RenderError {}

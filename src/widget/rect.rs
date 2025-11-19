//! Rect container widget for widget composition.

use super::tree::WidgetNode;
use super::{RenderContext, RenderError, Widget};

/// Primary container widget that stores children with explicit positions.
///
/// Uses const generic parameters for compile-time size specification.
///
/// # Validation
///
/// Per Constitution Principle VI validation hierarchy:
/// - **Compile-time**: Const generic dimensions (WIDTH, HEIGHT)
/// - **Debug-time**: `debug_assert!(WIDTH > 0 && HEIGHT > 0)` in `new()`
/// - **Runtime**: Child boundary and overlap validation in `add_child()`
///
/// # Examples
///
/// ```rust
/// use escp_layout::widget::{rect_new, label_new};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a container
/// let mut container = rect_new!(80, 30);
///
/// // Add a child label
/// let label = label_new!(20).add_text("Hello")?;
/// container.add_child(label, (10, 5))?;
/// # Ok(())
/// # }
/// ```
pub struct Rect<const WIDTH: u16, const HEIGHT: u16> {
    /// Children widgets with relative positions
    children: Vec<WidgetNode>,
}

impl<const WIDTH: u16, const HEIGHT: u16> Rect<WIDTH, HEIGHT> {
    /// Create a new Rect widget with const generic dimensions.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if WIDTH or HEIGHT is zero.
    /// In release builds with zero-size const generics, behavior is undefined
    /// per Constitution Principle IX.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::Rect;
    ///
    /// let container = Rect::<80, 30>::new();
    /// ```
    pub fn new() -> Self {
        debug_assert!(WIDTH > 0 && HEIGHT > 0, "Rect dimensions must be non-zero");

        Self {
            children: Vec::new(),
        }
    }

    /// Add a child widget at the specified relative position (composition phase).
    ///
    /// # Validation
    ///
    /// This method performs comprehensive validation:
    /// - Child size must fit within parent bounds (ChildExceedsParent)
    /// - Position must not cause integer overflow (IntegerOverflow)
    /// - Child must not overlap existing children per AABB (OverlappingChildren)
    ///
    /// Note: Touching edges (shared boundary) does NOT count as overlap.
    ///
    /// # Errors
    ///
    /// - `RenderError::ChildExceedsParent`: Child's size extends beyond parent bounds
    /// - `RenderError::OutOfBounds`: Position places child outside parent bounds
    /// - `RenderError::OverlappingChildren`: Child overlaps with existing child
    /// - `RenderError::IntegerOverflow`: Coordinate calculation overflows
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::{rect_new, label_new};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut container = rect_new!(80, 30);
    /// let label = label_new!(20).add_text("Hello")?;
    /// container.add_child(label, (10, 5))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_child<W: Widget + 'static>(
        &mut self,
        widget: W,
        position: (u16, u16),
    ) -> Result<(), RenderError> {
        let child_width = W::WIDTH;
        let child_height = W::HEIGHT;

        // Validate child fits within parent bounds (with checked arithmetic)
        let child_right =
            position
                .0
                .checked_add(child_width)
                .ok_or(RenderError::IntegerOverflow {
                    operation: format!(
                        "child position.x ({}) + width ({})",
                        position.0, child_width
                    ),
                })?;
        let child_bottom =
            position
                .1
                .checked_add(child_height)
                .ok_or(RenderError::IntegerOverflow {
                    operation: format!(
                        "child position.y ({}) + height ({})",
                        position.1, child_height
                    ),
                })?;

        if child_right > WIDTH || child_bottom > HEIGHT {
            return Err(RenderError::ChildExceedsParent {
                parent_width: WIDTH,
                parent_height: HEIGHT,
                child_width,
                child_height,
                position,
            });
        }

        // Check for overlaps with existing children using AABB collision detection
        // Per FR-005A: touching edges (shared boundary) does NOT count as overlap
        for existing in &self.children {
            let existing_right = existing.position.0.checked_add(existing.width).ok_or(
                RenderError::IntegerOverflow {
                    operation: "existing child bounds calculation".to_string(),
                },
            )?;
            let existing_bottom = existing.position.1.checked_add(existing.height).ok_or(
                RenderError::IntegerOverflow {
                    operation: "existing child bounds calculation".to_string(),
                },
            )?;

            // AABB intersection check with strict inequality (touching edges allowed)
            let overlaps = child_right > existing.position.0
                && position.0 < existing_right
                && child_bottom > existing.position.1
                && position.1 < existing_bottom;

            if overlaps {
                return Err(RenderError::OverlappingChildren {
                    child1_bounds: (
                        existing.position.0,
                        existing.position.1,
                        existing.width,
                        existing.height,
                    ),
                    child2_bounds: (position.0, position.1, child_width, child_height),
                });
            }
        }

        // Add child to tree
        self.children.push(WidgetNode::new(widget, position));

        Ok(())
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> Widget for Rect<WIDTH, HEIGHT> {
    const WIDTH: u16 = WIDTH;
    const HEIGHT: u16 = HEIGHT;

    fn render_to(
        &self,
        context: &mut RenderContext,
        position: (u16, u16),
    ) -> Result<(), RenderError> {
        // Render all children with cumulative offset
        for child in &self.children {
            let child_pos = (position.0 + child.position.0, position.1 + child.position.1);
            child.widget.render_to_dyn(context, child_pos)?;
        }
        Ok(())
    }
}

/// Ergonomic macro for creating Rect widgets.
///
/// Expands `rect_new!(W, H)` to `Rect::<W, H>::new()`.
///
/// # Examples
///
/// ```rust
/// use escp_layout::widget::rect_new;
///
/// let container = rect_new!(80, 30);
/// ```
#[macro_export]
macro_rules! rect_new {
    ($w:expr, $h:expr) => {
        $crate::widget::Rect::<$w, $h>::new()
    };
}

pub use rect_new;

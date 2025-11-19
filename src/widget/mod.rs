//! Widget composability system for declarative UI composition.
//!
//! This module provides a React-like widget composition system with:
//! - Compile-time size validation via const generics
//! - Automatic coordinate calculation through parent-child relationships
//! - Type-safe boundary enforcement with three-tier validation
//! - Zero runtime dependencies
//!
//! # Quick Start
//!
//! ```rust
//! use escp_layout::widget::{Rect, Label, rect_new, label_new};
//! use escp_layout::Page;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create root container (80×30)
//! let mut root = rect_new!(80, 30);
//!
//! // Add a label
//! let label = label_new!(20).add_text("Hello World")?;
//! root.add_child(label, (0, 0))?;
//!
//! // Render to page
//! let mut page = Page::builder();
//! page.render(&root)?;
//! # Ok(())
//! # }
//! ```

mod rect;
mod context;
mod label;
pub mod layout;
mod tree;

// Re-export core types
pub use context::RenderContext;

// WidgetNode is internal only, not re-exported
// (it contains implementation details of type erasure)

// Re-export error types
pub use error::RenderError;

// Error module
mod error;

// Re-export widgets
pub use rect::Rect;
pub use label::Label;

// Re-export macros
pub use rect::rect_new;
pub use label::label_new;

// Re-export layout components (will be added in Phase 5)
pub use layout::{column_area, column_new, row_area, row_new, stack_new, Column, Row, Stack};

/// Common trait for all renderable widgets.
///
/// All widgets must implement this trait to participate in the composition system.
/// The trait uses associated constants for compile-time dimensions and provides
/// an immutable rendering interface.
///
/// # Type Safety
///
/// Dimensions are specified at the type level using const generics. For example,
/// `Rect<80, 30>` is a different type from `Rect<40, 15>`, enabling compile-time
/// validation.
///
/// # Validation Hierarchy
///
/// Per Constitution Principle VI, validation follows a three-tier hierarchy:
///
/// 1. **Compile-time** (preferred): Const generics enforce dimensions at type level
/// 2. **Debug-time**: `debug_assert!` catches violations in debug builds only
/// 3. **Runtime**: `Result<(), RenderError>` for user-provided data validation
///
/// # Examples
///
/// ```rust
/// use escp_layout::widget::{Widget, Rect, Label, RenderContext, rect_new, label_new};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Turbofish syntax
/// let container = Rect::<80, 30>::new();
/// let label = Label::<20, 1>::new().add_text("Hello")?;
///
/// // Macro syntax (ergonomic)
/// let container = rect_new!(80, 30);
/// let label = label_new!(20).add_text("Hello")?;
/// # Ok(())
/// # }
/// ```
pub trait Widget {
    /// Width of the widget in character columns (compile-time constant).
    const WIDTH: u16;

    /// Height of the widget in character rows (compile-time constant).
    const HEIGHT: u16;

    /// Render this widget to the provided context at the given absolute position.
    ///
    /// The context handles coordinate translation and boundary checking.
    /// Widgets must not panic under any input conditions.
    ///
    /// # Errors
    ///
    /// Returns `RenderError::OutOfBounds` if the widget attempts to render outside
    /// the context's clip bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::{Widget, RenderContext, Label, label_new};
    ///
    /// # fn example(context: &mut RenderContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let label = label_new!(20).add_text("Hello")?;
    /// label.render_to(context, (10, 5))?;
    /// # Ok(())
    /// # }
    /// ```
    fn render_to(
        &self,
        context: &mut RenderContext,
        position: (u16, u16),
    ) -> Result<(), RenderError>;
}

//! RenderContext for widget rendering with boundary validation.

use super::RenderError;
use crate::cell::StyleFlags;
use crate::PageBuilder;

/// RenderContext wraps PageBuilder during render phase, tracking cumulative
/// coordinates and enforcing boundary validation.
///
/// Created by `Page::render()` and passed to widget tree traversal.
///
/// # Three-Layer Validation Architecture (FR-004)
///
/// - **Layer 1 (Widget Construction)**: `Label::add_text()` validates content
/// - **Layer 2 (RenderContext)**: Validates write start position within clip_bounds
/// - **Layer 3 (PageBuilder)**: Silently truncates content extending beyond bounds
///
/// # Examples
///
/// ```rust
/// use escp_layout::{Page, PageBuilder};
/// use escp_layout::widget::RenderContext;
///
/// # fn example() {
/// let mut page_builder = Page::builder();
/// let mut context = RenderContext::new(&mut page_builder);
///
/// // Write text at position
/// context.write_text("Hello", (10, 5)).expect("Valid position");
/// # }
/// ```
pub struct RenderContext<'a> {
    /// Reference to the underlying PageBuilder
    page_builder: &'a mut PageBuilder,

    /// Clip bounds (x, y, width, height) for boundary enforcement
    clip_bounds: (u16, u16, u16, u16),
}

impl<'a> RenderContext<'a> {
    /// Create a new RenderContext wrapping a PageBuilder.
    ///
    /// Called by `Page::render()` to create context for widget tree traversal.
    /// Initializes clip_bounds to full page (160×51 per EPSON LQ-2090II spec).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::{Page, PageBuilder};
    /// use escp_layout::widget::RenderContext;
    ///
    /// # fn example() {
    /// let mut page_builder = Page::builder();
    /// let context = RenderContext::new(&mut page_builder);
    /// # }
    /// ```
    pub(crate) fn new(page_builder: &'a mut PageBuilder) -> Self {
        Self {
            page_builder,
            clip_bounds: (0, 0, 160, 51), // EPSON LQ-2090II page bounds
        }
    }

    /// Write text to the page at the specified absolute position.
    ///
    /// This is the primary public API for widgets to render text content.
    ///
    /// # Validation
    ///
    /// - Validates write start position is within clip bounds (Layer 2)
    /// - Delegates to PageBuilder for rendering and content truncation (Layer 3)
    /// - PageBuilder handles text extending beyond bounds via silent clipping
    ///
    /// # Errors
    ///
    /// Returns `RenderError::OutOfBounds` if position exceeds clip bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::{Page, PageBuilder};
    /// use escp_layout::widget::RenderContext;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut page_builder = Page::builder();
    /// let mut context = RenderContext::new(&mut page_builder);
    ///
    /// context.write_text("Hello", (10, 5))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_text(&mut self, text: &str, position: (u16, u16)) -> Result<(), RenderError> {
        // Validate position is within bounds (RenderContext validates start position only)
        if position.0 >= self.clip_bounds.2 || position.1 >= self.clip_bounds.3 {
            return Err(RenderError::OutOfBounds {
                position,
                bounds: self.clip_bounds,
            });
        }

        // Delegate to PageBuilder for rendering and horizontal truncation
        // PageBuilder handles text that extends beyond bounds via silent character-level clipping
        self.page_builder
            .write_str(position.0, position.1, text, StyleFlags::NONE);
        Ok(())
    }

    /// Write styled text to the page at the specified absolute position.
    ///
    /// This is the public API for widgets to render styled text (bold, underline).
    ///
    /// # Validation
    ///
    /// Same validation as `write_text()` - validates start position only.
    ///
    /// # Errors
    ///
    /// Returns `RenderError::OutOfBounds` if position exceeds clip bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::{Page, PageBuilder, StyleFlags};
    /// use escp_layout::widget::RenderContext;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut page_builder = Page::builder();
    /// let mut context = RenderContext::new(&mut page_builder);
    ///
    /// context.write_styled("Bold Text", (10, 5), StyleFlags::BOLD)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_styled(
        &mut self,
        text: &str,
        position: (u16, u16),
        style: StyleFlags,
    ) -> Result<(), RenderError> {
        // Validate position is within bounds (RenderContext validates start position only)
        if position.0 >= self.clip_bounds.2 || position.1 >= self.clip_bounds.3 {
            return Err(RenderError::OutOfBounds {
                position,
                bounds: self.clip_bounds,
            });
        }

        // Delegate to PageBuilder for rendering and horizontal truncation
        // PageBuilder handles text that extends beyond bounds via silent character-level clipping
        self.page_builder
            .write_str(position.0, position.1, text, style);
        Ok(())
    }

    /// Get current clip bounds for advanced boundary checking.
    ///
    /// Returns (x, y, width, height) tuple representing the current clip region.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::{Page, PageBuilder};
    /// use escp_layout::widget::RenderContext;
    ///
    /// # fn example() {
    /// let mut page_builder = Page::builder();
    /// let context = RenderContext::new(&mut page_builder);
    ///
    /// let (x, y, width, height) = context.clip_bounds();
    /// assert_eq!((x, y, width, height), (0, 0, 160, 51));
    /// # }
    /// ```
    pub fn clip_bounds(&self) -> (u16, u16, u16, u16) {
        self.clip_bounds
    }
}

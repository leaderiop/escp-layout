// API Contract: RenderContext
// Feature: 002-widget-composability
// Status: Phase 1 Design

/// Internal abstraction that wraps PageBuilder during render phase.
///
/// RenderContext tracks cumulative coordinates during widget tree traversal
/// and enforces boundary clipping by pre-validating positions before delegating
/// to PageBuilder.
///
/// # Visibility
/// This type is `pub(crate)` (internal implementation detail). Widget implementors
/// receive `&mut RenderContext` but do not construct instances themselves.
///
/// # Lifetime
/// RenderContext borrows PageBuilder mutably for the duration of widget rendering.
/// The lifetime `'a` ensures RenderContext cannot outlive the PageBuilder.
pub(crate) struct RenderContext<'a> {
    /// Reference to the underlying PageBuilder (existing rendering backend)
    page_builder: &'a mut PageBuilder,

    /// Clip bounds (x, y, width, height) for boundary enforcement
    /// V1: Always (0, 0, 160, 51) for EPSON LQ-2090II page bounds
    clip_bounds: (u16, u16, u16, u16),
}

impl<'a> RenderContext<'a> {
    /// Create a new RenderContext wrapping a PageBuilder.
    ///
    /// # Parameters
    /// - `page_builder`: Mutable reference to the underlying PageBuilder
    ///
    /// # Clip Bounds
    /// Initialized to page bounds (0, 0, 160, 51) for V1. Future versions
    /// may support nested contexts with tighter clip bounds.
    pub(crate) fn new(page_builder: &'a mut PageBuilder) -> Self;

    /// Write text to the page at the specified absolute position.
    ///
    /// Pre-validates bounds before delegating to PageBuilder. If validation
    /// passes, delegates to `PageBuilder::write()` which applies existing
    /// truncation rules (constitutional requirement III).
    ///
    /// # Parameters
    /// - `text`: Text to render (ASCII characters 32-126)
    /// - `position`: Absolute position (column, row) on the page
    ///
    /// # Errors
    /// - `RenderError::OutOfBounds`: If position exceeds clip bounds
    ///
    /// # Validation
    /// - `position.0 < clip_bounds.2` (column within width)
    /// - `position.1 < clip_bounds.3` (row within height)
    ///
    /// # Truncation
    /// Horizontal overflow is handled by PageBuilder (silent truncation per
    /// constitutional requirement III). This method only validates the starting
    /// position.
    ///
    /// # Example
    /// ```rust,ignore
    /// impl Widget for Label {
    ///     fn render_to(&self, context: &mut RenderContext, position: (u16, u16)) -> Result<(), RenderError> {
    ///         context.write_text(&self.text, position)
    ///     }
    /// }
    /// ```
    pub fn write_text(
        &mut self,
        text: &str,
        position: (u16, u16),
    ) -> Result<(), RenderError>;

    /// Write styled text to the page at the specified absolute position.
    ///
    /// Applies text styling (bold, underline) via ESC/P commands. Delegates
    /// to `PageBuilder::write_styled()` after bounds validation.
    ///
    /// # Parameters
    /// - `text`: Text to render
    /// - `position`: Absolute position (column, row)
    /// - `style`: Text style (bold, underline flags)
    ///
    /// # Errors
    /// - `RenderError::OutOfBounds`: If position exceeds clip bounds
    ///
    /// # ESC/P Compliance
    /// Style flags map to ESC/P commands:
    /// - Bold: `ESC E` (0x1B 0x45) / `ESC F` (0x1B 0x46)
    /// - Underline: `ESC - 1` (0x1B 0x2D 0x01) / `ESC - 0` (0x1B 0x2D 0x00)
    pub fn write_styled(
        &mut self,
        text: &str,
        position: (u16, u16),
        style: Style,
    ) -> Result<(), RenderError>;

    /// Get a reference to the underlying PageBuilder (internal use only).
    ///
    /// Allows adapter widgets to directly access PageBuilder for legacy
    /// compatibility. Should NOT be used by new widget implementations.
    pub(crate) fn page_builder(&mut self) -> &mut PageBuilder;
}

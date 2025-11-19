// API Contract: Widget Trait
// Feature: 002-widget-composability
// Status: Phase 1 Design

/// Core trait for all renderable components in the widget system.
///
/// All widgets must implement this trait to participate in the composition
/// and rendering system. Widgets declare their dimensions and provide a
/// render implementation that outputs to a RenderContext.
///
/// # Object Safety
/// This trait is object-safe, allowing heterogeneous widget collections via
/// `Box<dyn Widget>`.
///
/// # Determinism
/// Implementations MUST produce identical output for identical inputs across
/// all invocations (constitutional requirement I).
///
/// # Immutability
/// Rendering operates on `&self` (immutable reference), ensuring thread safety
/// and supporting the immutability guarantee (constitutional requirement IV).
pub trait Widget {
    /// Width of the widget in character columns.
    ///
    /// # Requirements
    /// - MUST return a non-zero value
    /// - MUST return a consistent value across all calls
    /// - MUST NOT panic
    fn width(&self) -> u16;

    /// Height of the widget in character rows.
    ///
    /// # Requirements
    /// - MUST return a non-zero value
    /// - MUST return a consistent value across all calls
    /// - MUST NOT panic
    fn height(&self) -> u16;

    /// Render this widget to the provided context at the given absolute position.
    ///
    /// The context handles coordinate translation and boundary checking. Widgets
    /// delegate all actual rendering to context methods, which in turn delegate
    /// to the underlying PageBuilder.
    ///
    /// # Parameters
    /// - `context`: Mutable reference to RenderContext (wraps PageBuilder)
    /// - `position`: Absolute position (column, row) where widget should render
    ///
    /// # Errors
    /// - `RenderError::OutOfBounds`: Position or content exceeds context bounds
    ///
    /// # Requirements
    /// - MUST NOT panic under any input
    /// - MUST produce deterministic output (same input → same output)
    /// - MUST delegate all rendering to context methods (no direct PageBuilder access)
    /// - MUST respect immutability (`&self` only)
    ///
    /// # Example
    /// ```rust,ignore
    /// impl Widget for Label {
    ///     fn render_to(
    ///         &self,
    ///         context: &mut RenderContext,
    ///         position: (u16, u16),
    ///     ) -> Result<(), RenderError> {
    ///         context.write_text(&self.text, position)
    ///     }
    /// }
    /// ```
    fn render_to(
        &self,
        context: &mut RenderContext,
        position: (u16, u16),
    ) -> Result<(), RenderError>;
}

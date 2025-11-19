// API Contract: Page Enhancement (render method)
// Feature: 002-widget-composability
// Status: Phase 1 Design

/// Enhancement to the existing Page struct to support widget tree rendering.
///
/// This contract defines the new `render()` method added to Page. The method
/// traverses a widget tree depth-first, calculating cumulative positions and
/// delegating to PageBuilder for output.
impl Page {
    /// Render a widget tree to this page.
    ///
    /// The widget tree is traversed depth-first, with each widget rendering
    /// at its cumulative absolute position. The root widget is always positioned
    /// at (0, 0).
    ///
    /// # Rendering Algorithm
    /// 1. Create RenderContext wrapping PageBuilder
    /// 2. Call `widget.render_to(context, (0, 0))`
    /// 3. Widget traverses its children (if any), passing cumulative positions
    /// 4. Leaf widgets delegate to `context.write_text()` or `context.write_styled()`
    /// 5. Context pre-validates bounds, then delegates to PageBuilder
    ///
    /// # Parameters
    /// - `widget`: Reference to any type implementing Widget
    ///
    /// # Errors
    /// - `RenderError::OutOfBounds`: If any widget attempts to render outside
    ///   page bounds (160 × 51 in V1)
    /// - Propagates errors from widget `render_to()` implementations
    ///
    /// # Determinism
    /// - Same widget tree → same rendering order → identical ESC/P output
    /// - Widget children rendered in insertion order (Vec iteration)
    /// - No HashMap iteration, no random values, no timestamps
    ///
    /// # Immutability
    /// - Widget tree is immutable during rendering (`&impl Widget`)
    /// - Page state is modified (PageBuilder writes), but widget state is not
    ///
    /// # Example
    /// ```rust,ignore
    /// // Build widget tree (composition phase)
    /// let mut root = DynamicRect::new(80, 30)?;
    /// let mut column = Column::new(80, 30);
    ///
    /// let (mut row1, pos1) = column.area(10)?;
    /// row1.add_child(Label::new("Row 1"), (0, 0))?;
    /// root.add_child(row1, pos1)?;
    ///
    /// let (mut row2, pos2) = column.area(10)?;
    /// row2.add_child(Label::new("Row 2"), (0, 0))?;
    /// root.add_child(row2, pos2)?;
    ///
    /// // Render phase (immutable widget tree)
    /// let mut page = Page::new();
    /// page.render(&root)?;
    ///
    /// // Generate ESC/P output
    /// let escp_bytes = page.to_bytes();
    /// ```
    ///
    /// # Constitutional Compliance
    /// - Deterministic behavior (Principle I): Depth-first traversal with stable ordering
    /// - Immutability guarantee (Principle IV): Widget tree is `&impl Widget` (immutable)
    /// - Zero-panic guarantee (Principle IX): All errors returned as Result
    /// - ESC/P compliance (Principle V): Delegates to PageBuilder (unchanged)
    /// - Performance targets (Principle XI): Iterative traversal, < 100 μs p99
    pub fn render(&mut self, widget: &impl Widget) -> Result<(), RenderError>;
}

// Internal rendering implementation (not part of public API)
//
// This pseudocode illustrates the internal traversal algorithm.
// Actual implementation may vary, but MUST maintain deterministic ordering.
//
// ```rust,ignore
// fn render_widget_tree_internal(
//     widget: &dyn Widget,
//     context: &mut RenderContext,
//     position: (u16, u16),
// ) -> Result<(), RenderError> {
//     // Delegate to widget's render implementation
//     widget.render_to(context, position)?;
//
//     // For container widgets (e.g., DynamicRect), render_to() will:
//     // 1. Iterate children in insertion order (deterministic)
//     // 2. Calculate cumulative position for each child
//     // 3. Recursively call child.render_to(context, cumulative_pos)
//     //
//     // For leaf widgets (e.g., Label), render_to() will:
//     // 1. Call context.write_text(text, position)
//     // 2. Context validates bounds, delegates to PageBuilder
//
//     Ok(())
// }
// ```

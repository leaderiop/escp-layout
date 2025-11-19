// API Contract: DynamicBox Widget
// Feature: 002-widget-composability
// Status: Phase 1 Design

/// Primary container widget that stores children with explicit positions.
///
/// DynamicBox supports runtime-determined dimensions (unlike const-generic
/// alternatives) to enable layout components (Column, Row, Stack) to return
/// boxes with calculated sizes.
///
/// # Lifecycle
/// 1. **Creation**: `DynamicBox::new(width, height)` validates non-zero dimensions
/// 2. **Composition**: `add_child(widget, position)` builds the widget tree
/// 3. **Rendering**: `render_to(context, position)` traverses tree immutably
///
/// # Memory Overhead
/// - Base struct: 32 bytes (width, height, Vec metadata)
/// - Per child: ~20 bytes (WidgetNode) + child's own size
/// - Typical 3-column layout: ~140 bytes overhead (7 widgets)
pub struct DynamicBox {
    /// Width in character columns (validated non-zero at construction)
    width: u16,

    /// Height in character rows (validated non-zero at construction)
    height: u16,

    /// Children widgets with relative positions (insertion order preserved)
    children: Vec<WidgetNode>,
}

impl DynamicBox {
    /// Create a new Box widget with specified dimensions.
    ///
    /// # Parameters
    /// - `width`: Width in character columns (must be > 0)
    /// - `height`: Height in character rows (must be > 0)
    ///
    /// # Errors
    /// - `RenderError::ZeroSizeParent`: If width or height is zero
    ///
    /// # Example
    /// ```rust,ignore
    /// let box_widget = DynamicBox::new(80, 30)?;
    /// ```
    pub fn new(width: u16, height: u16) -> Result<Self, RenderError>;

    /// Add a child widget at the specified relative position (composition phase).
    ///
    /// Validates that the child fits within the parent's bounds before adding.
    /// Children are stored in insertion order (deterministic rendering).
    ///
    /// # Parameters
    /// - `widget`: Any type implementing Widget + 'static
    /// - `position`: Relative position (column, row) within this Box
    ///
    /// # Errors
    /// - `RenderError::ChildExceedsParent`: Child's size extends beyond parent bounds
    /// - `RenderError::OutOfBounds`: Position places child outside parent bounds
    /// - `RenderError::IntegerOverflow`: Position + size calculation overflows u16
    ///
    /// # Validation Rules
    /// - `position.0 + widget.width() <= self.width`
    /// - `position.1 + widget.height() <= self.height`
    /// - All arithmetic checked for overflow
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut box_widget = DynamicBox::new(80, 30)?;
    /// let label = Label::new("Hello");
    /// box_widget.add_child(label, (10, 5))?;
    /// ```
    pub fn add_child(
        &mut self,
        widget: impl Widget + 'static,
        position: (u16, u16),
    ) -> Result<(), RenderError>;

    /// Get the width of this Box (implements Widget::width).
    pub fn width(&self) -> u16;

    /// Get the height of this Box (implements Widget::height).
    pub fn height(&self) -> u16;
}

impl Widget for DynamicBox {
    /// Render this Box and all its children to the context.
    ///
    /// Traverses children in insertion order (deterministic), calculating
    /// cumulative positions for each child. Each child is rendered at
    /// `parent_position + child.position`.
    ///
    /// # Rendering Order
    /// Depth-first traversal: parent renders, then children in insertion order.
    /// This ensures earlier children are visually "behind" later children if
    /// they overlap.
    ///
    /// # Errors
    /// - `RenderError::OutOfBounds`: If any child renders outside context bounds
    /// - Propagates errors from child widgets' `render_to()` implementations
    ///
    /// # Determinism
    /// - Same widget tree → same rendering order → identical output
    /// - Children rendered in Vec insertion order (stable)
    fn render_to(
        &self,
        context: &mut RenderContext,
        position: (u16, u16),
    ) -> Result<(), RenderError>;
}

// Internal structure (not exposed in public API)
pub(crate) struct WidgetNode {
    /// Relative position within parent Box
    position: (u16, u16),

    /// The widget instance (trait object for heterogeneous types)
    widget: Box<dyn Widget>,
}

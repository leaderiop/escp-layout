// API Contract: Layout Components (Column, Row, Stack)
// Feature: 002-widget-composability
// Status: Phase 1 Design

//
// Column Layout
//

/// Divides parent Rect space vertically, returning nested DynamicRect widgets
/// for each row.
///
/// Column layout allocates horizontal areas (rows) with specified heights,
/// positioned sequentially from top to bottom. Each call to `area()` returns
/// a DynamicRect and its relative position within the parent.
///
/// # Lifecycle
/// 1. Create Column with parent dimensions
/// 2. Call `area(height)` repeatedly to allocate rows
/// 3. Add returned Rectes to parent with returned positions
///
/// # Space Tracking
/// Column tracks `current_y` offset to ensure sequential positioning and
/// prevent space over-allocation.
pub struct Column {
    /// Parent width (all returned Rectes have this width)
    width: u16,

    /// Parent height (total available vertical space)
    height: u16,

    /// Current Y offset (next available row position)
    current_y: u16,
}

impl Column {
    /// Create a Column layout for the given parent dimensions.
    ///
    /// # Parameters
    /// - `width`: Parent Rect width (all rows will have this width)
    /// - `height`: Parent Rect height (total vertical space to divide)
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut column = Column::new(80, 30);
    /// ```
    pub fn new(width: u16, height: u16) -> Self;

    /// Allocate a horizontal area (row) with specified height.
    ///
    /// Returns a DynamicRect with dimensions (width, height) and its relative
    /// position within the parent Rect. Position is automatically calculated
    /// based on previous area() calls.
    ///
    /// # Parameters
    /// - `height`: Height of the row in character lines
    ///
    /// # Returns
    /// - `(DynamicRect, (u16, u16))`: Rect widget and its (x, y) position
    ///   - Position x is always 0 (rows span full width)
    ///   - Position y is cumulative (sum of all previous row heights)
    ///
    /// # Errors
    /// - `RenderError::InsufficientSpace`: Requested height exceeds remaining space
    ///
    /// # State Changes
    /// - Advances `current_y` by `height` (sequential allocation)
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut root = DynamicRect::new(80, 30)?;
    /// let mut column = Column::new(80, 30);
    ///
    /// let (mut row1, pos1) = column.area(10)?; // Returns Rect<80, 10> at (0, 0)
    /// row1.add_child(Label::new("Row 1"), (0, 0))?;
    /// root.add_child(row1, pos1)?;
    ///
    /// let (mut row2, pos2) = column.area(10)?; // Returns Rect<80, 10> at (0, 10)
    /// row2.add_child(Label::new("Row 2"), (0, 0))?;
    /// root.add_child(row2, pos2)?;
    /// ```
    pub fn area(&mut self, height: u16) -> Result<(DynamicRect, (u16, u16)), RenderError>;
}

//
// Row Layout
//

/// Divides parent Rect space horizontally, returning nested DynamicRect widgets
/// for each column.
///
/// Row layout allocates vertical areas (columns) with specified widths,
/// positioned sequentially from left to right.
pub struct Row {
    /// Parent width (total available horizontal space)
    width: u16,

    /// Parent height (all returned Rectes have this height)
    height: u16,

    /// Current X offset (next available column position)
    current_x: u16,
}

impl Row {
    /// Create a Row layout for the given parent dimensions.
    ///
    /// # Parameters
    /// - `width`: Parent Rect width (total horizontal space to divide)
    /// - `height`: Parent Rect height (all columns will have this height)
    pub fn new(width: u16, height: u16) -> Self;

    /// Allocate a vertical area (column) with specified width.
    ///
    /// Returns a DynamicRect with dimensions (width, height) and its relative
    /// position within the parent Rect.
    ///
    /// # Parameters
    /// - `width`: Width of the column in character columns
    ///
    /// # Returns
    /// - `(DynamicRect, (u16, u16))`: Rect widget and its (x, y) position
    ///   - Position x is cumulative (sum of all previous column widths)
    ///   - Position y is always 0 (columns span full height)
    ///
    /// # Errors
    /// - `RenderError::InsufficientSpace`: Requested width exceeds remaining space
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut root = DynamicRect::new(80, 30)?;
    /// let mut row = Row::new(80, 30);
    ///
    /// let (mut col1, pos1) = row.area(25)?; // Returns Rect<25, 30> at (0, 0)
    /// col1.add_child(Label::new("Column 1"), (0, 0))?;
    /// root.add_child(col1, pos1)?;
    ///
    /// let (mut col2, pos2) = row.area(25)?; // Returns Rect<25, 30> at (25, 0)
    /// col2.add_child(Label::new("Column 2"), (0, 0))?;
    /// root.add_child(col2, pos2)?;
    /// ```
    pub fn area(&mut self, width: u16) -> Result<(DynamicRect, (u16, u16)), RenderError>;
}

//
// Stack Layout
//

/// Returns overlapping nested DynamicRect widgets at the same position.
///
/// Stack layout is used for layering widgets (e.g., background + foreground).
/// All calls to `area()` return Rectes with the same dimensions at position (0, 0).
///
/// # Rendering Order
/// Later children obscure earlier children when overlapping (depth-first
/// rendering order in DynamicRect).
pub struct Stack {
    /// Parent width (all returned Rectes have this width)
    width: u16,

    /// Parent height (all returned Rectes have this height)
    height: u16,
}

impl Stack {
    /// Create a Stack layout for the given parent dimensions.
    ///
    /// # Parameters
    /// - `width`: Parent Rect width (all layers will have this width)
    /// - `height`: Parent Rect height (all layers will have this height)
    pub fn new(width: u16, height: u16) -> Self;

    /// Allocate an overlapping area (same position for all calls).
    ///
    /// Returns a DynamicRect with full parent dimensions at position (0, 0).
    /// Multiple calls return identical positions (overlapping layers).
    ///
    /// # Returns
    /// - `(DynamicRect, (u16, u16))`: Rect widget and its (x, y) position
    ///   - Position is always (0, 0) for all layers
    ///
    /// # Errors
    /// - `RenderError::ZeroSizeParent`: If parent width or height is zero
    ///   (propagated from DynamicRect::new)
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut root = DynamicRect::new(80, 30)?;
    /// let stack = Stack::new(80, 30);
    ///
    /// // Background layer
    /// let (mut bg, pos) = stack.area()?; // Returns Rect<80, 30> at (0, 0)
    /// bg.add_child(Background::new(), (0, 0))?;
    /// root.add_child(bg, pos)?;
    ///
    /// // Foreground layer (overlays background)
    /// let (mut fg, pos) = stack.area()?; // Returns Rect<80, 30> at (0, 0)
    /// fg.add_child(Label::new("Overlay"), (10, 10))?;
    /// root.add_child(fg, pos)?;
    /// ```
    pub fn area(&self) -> Result<(DynamicRect, (u16, u16)), RenderError>;
}

// API Contract: RenderError
// Feature: Widget Composability System
// Branch: 002-widget-composability
// Status: Phase 1 Design (Updated)
// Date: 2025-11-19

/// Comprehensive error type for all widget rendering violations.
///
/// RenderError provides specific variants for each invalid rendering condition,
/// with contextual information to aid debugging. All errors implement the
/// standard Error trait for compatibility with Rust error handling patterns.
///
/// Marked `#[non_exhaustive]` to allow future error variants without breaking
/// changes (V2+ can add new variants without major version bump).
///
/// # Error Categories
/// - **Composition Errors**: Returned during tree building (add_child, layout area)
/// - **Render Errors**: Returned during tree traversal (render_to)
/// - **Construction Errors**: Returned during widget construction (Label::add_text)
///
/// # Constitutional Compliance
/// - All errors MUST be Result-based (no panics) per zero-panic guarantee (IX)
/// - Errors provide context for debugging per documentation requirements (XIV)
/// - Widget Exception (Principle III) allows boundary errors instead of silent truncation
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// Child widget exceeds parent widget's bounds.
    ///
    /// Returned when `add_child()` is called with a child whose size
    /// (width × height) extends beyond the parent's available space.
    ///
    /// # Context
    /// - `parent_width`, `parent_height`: Parent Rect dimensions
    /// - `child_width`, `child_height`: Child widget dimensions
    /// - `position`: Attempted child position within parent
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut parent = Rect::<10, 10>::new();
    /// let child = Label::<26, 1>::new().add_text("This is a very long label")?;
    /// parent.add_child(child, (0, 0))?; // ERROR: 26 > 10
    /// ```
    ChildExceedsParent {
        parent_width: u16,
        parent_height: u16,
        child_width: u16,
        child_height: u16,
        position: (u16, u16),
    },

    /// Widget positioned outside valid bounds.
    ///
    /// Returned when a widget attempts to render at a position outside the
    /// RenderContext's clip bounds (page boundaries: 160×51 in V1).
    ///
    /// # Context
    /// - `position`: Attempted absolute position (column, row)
    /// - `bounds`: Clip bounds (x, y, width, height)
    ///
    /// # Example
    /// ```rust,ignore
    /// // Page bounds: (0, 0, 160, 51)
    /// context.write_text("Text", (170, 10))?; // ERROR: 170 >= 160
    /// ```
    OutOfBounds {
        position: (u16, u16),
        bounds: (u16, u16, u16, u16), // (x, y, width, height)
    },

    /// Two or more children overlap within parent widget.
    ///
    /// Returned when `add_child()` detects that the new child's bounding rect
    /// intersects with an existing child's bounding rect. Uses AABB (Axis-Aligned
    /// Bounding Rect) collision detection with strict inequality per FR-005A
    /// (touching edges without intersection does NOT count as overlap).
    ///
    /// # Context
    /// - `child1_bounds`: First child bounds (x, y, width, height)
    /// - `child2_bounds`: Second child bounds (x, y, width, height)
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut parent = Rect::<80, 30>::new();
    /// let label1 = Label::<20, 1>::new().add_text("Label 1")?;
    /// parent.add_child(label1, (0, 0))?; // OK
    ///
    /// let label2 = Label::<20, 1>::new().add_text("Label 2")?;
    /// parent.add_child(label2, (10, 0))?; // ERROR: overlaps label1 (0-20 intersects 10-30)
    ///
    /// let label3 = Label::<20, 1>::new().add_text("Label 3")?;
    /// parent.add_child(label3, (20, 0))?; // OK: touching edges (20 == 20) allowed
    /// ```
    OverlappingChildren {
        child1_bounds: (u16, u16, u16, u16),
        child2_bounds: (u16, u16, u16, u16),
    },

    /// Layout component cannot fit all children in available space.
    ///
    /// Returned when a layout's `area()` method is called with a dimension
    /// that exceeds remaining space (e.g., Column with insufficient vertical
    /// space).
    ///
    /// # Context
    /// - `available`: Remaining space in the layout
    /// - `required`: Requested space
    /// - `layout_type`: Layout component name ("Column", "Row", "Stack")
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut column = Column::<80, 30>::new();
    /// let (row1, pos1) = column.area::<20>()?; // OK: 30 - 20 = 10 remaining
    /// let (row2, pos2) = column.area::<15>()?; // ERROR: 15 > 10 remaining
    /// ```
    InsufficientSpace {
        available: u16,
        required: u16,
        layout_type: &'static str,
    },

    /// Integer overflow in coordinate or size calculation.
    ///
    /// Returned when position + size calculations would exceed u16::MAX.
    /// All arithmetic in the widget system uses checked operations to detect
    /// overflow per constitutional requirement IX (zero-panic guarantee).
    ///
    /// # Context
    /// - `operation`: Description of the operation that caused overflow
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut parent = Rect::<100, 100>::new();
    /// let child = Label::<4, 1>::new().add_text("Text")?;
    /// parent.add_child(child, (65535, 0))?; // ERROR: 65535 + 4 overflows u16
    /// ```
    IntegerOverflow {
        operation: String,
    },

    /// Text content exceeds widget width or contains newline characters.
    ///
    /// Returned by `Label::add_text()` when:
    /// - Text length (in bytes) exceeds the widget's WIDTH
    /// - Text contains newline characters (\n or \r\n)
    ///
    /// # Context
    /// - `text_length`: Length of the text in bytes
    /// - `widget_width`: Width of the widget
    ///
    /// # Example
    /// ```rust,ignore
    /// let label = Label::<10, 1>::new()
    ///     .add_text("This is too long")?; // ERROR: 16 > 10
    ///
    /// let label2 = Label::<20, 1>::new()
    ///     .add_text("Line 1\nLine 2")?; // ERROR: contains newline
    /// ```
    TextExceedsWidth {
        text_length: u16,
        widget_width: u16,
    },
}

// Note: ZeroSizeParent variant removed per FR-007 and Constitution Principle VI
//
// Zero-size prevention uses validation hierarchy:
// 1. Compile-time (future): const generic value constraints (requires unstable
//    Rust feature `generic_const_exprs`, not available in Rust 1.91.1+ stable)
// 2. Development-time (PRIMARY for Rust 1.91.1): debug_assert!(WIDTH > 0 && HEIGHT > 0)
//    in Rect::new() and Label::new() - panics in debug builds, zero cost in release
// 3. Project Policy: Zero-size widget instantiation in release builds is undefined
//    behavior; developers MUST NOT create zero-size widgets per API documentation

impl std::fmt::Display for RenderError {
    /// Format error message for user-friendly display.
    ///
    /// All error messages provide actionable context (sizes, positions, bounds)
    /// to aid debugging.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

// Usage Context:
// - Composition Phase: ChildExceedsParent, OverlappingChildren, InsufficientSpace, IntegerOverflow
// - Render Phase: OutOfBounds, IntegerOverflow
// - Widget Construction: TextExceedsWidth

# Data Model: Widget Composability System

**Feature**: Widget Composability System
**Branch**: `002-widget-composability`
**Date**: 2025-11-19
**Status**: Phase 1 Complete (Updated)

## Overview

This document defines the core data structures, traits, and types for the widget composability system. All entities are designed to maintain deterministic behavior, immutability guarantees, and zero-panic operation per constitutional requirements.

---

## Core Traits

### Widget Trait

**Purpose**: Defines the contract for all renderable components in the system.

**Definition**:
```rust
pub trait Widget {
    /// Width of the widget in character columns (compile-time constant)
    const WIDTH: u16;

    /// Height of the widget in character rows (compile-time constant)
    const HEIGHT: u16;

    /// Render this widget to the provided context at the given absolute position.
    /// Context handles coordinate translation and boundary checking.
    ///
    /// # Errors
    /// Returns `RenderError::OutOfBounds` if the widget attempts to render outside
    /// the context's clip bounds.
    fn render_to(
        &self,
        context: &mut RenderContext,
        position: (u16, u16),
    ) -> Result<(), RenderError>;
}
```

**Design Notes**:
- Const generic parameters (WIDTH, HEIGHT) enable compile-time size specification for Box<WIDTH, HEIGHT>
- Associated constants provide fixed dimensions at type level
- `&self` (immutable) for rendering aligns with immutability guarantee
- Position parameter is absolute coordinates (RenderContext handles coordinate translation)

**Validation Rules**:
- `WIDTH` and `HEIGHT` MUST be non-zero const values (enforced at type level)
- `render_to()` MUST NOT panic under any input (constitutional zero-panic guarantee)
- `render_to()` MUST produce identical output for identical inputs (deterministic behavior)
- `render_to()` MUST return Result errors for boundary violations per Constitution Principle III Widget Exception

---

## Core Structures

### WidgetNode

**Purpose**: Internal tree node representing a widget and its position within a parent.

**Definition**:
```rust
pub(crate) struct WidgetNode {
    /// Relative position within parent (column, row)
    position: (u16, u16),

    /// The widget instance (trait object for heterogeneous types)
    widget: Box<dyn Widget>,
}
```

**Relationships**:
- Owned by parent widget (e.g., `Box` widget contains `Vec<WidgetNode>`)
- `position` is relative to parent's origin (0, 0)
- `widget` is heap-allocated trait object (enables polymorphism)

**Memory Layout**:
- `position`: 4 bytes (2 × u16)
- `widget`: 16 bytes (fat pointer: data ptr + vtable ptr)
- **Total**: 20 bytes per node (excluding widget's own data)

**Validation Rules**:
- `position` MUST be validated against parent bounds before node creation
- `widget` MUST implement Widget trait (enforced by type system)

#### Type Erasure Strategy

**Challenge**: `WidgetNode` must store widgets with different const generic parameters (e.g., `Box<80, 30>`, `Label<20, 1>`) in a homogeneous collection.

**Solution**: Trait object storage via `Box<dyn Widget>`

The `Widget` trait uses **associated constants** (not const generic parameters):

```rust
pub trait Widget {
    const WIDTH: u16;
    const HEIGHT: u16;
    fn render_to(&self, context: &mut RenderContext, position: (u16, u16)) -> Result<(), RenderError>;
}
```

Widget implementations (e.g., `Box<const WIDTH: u16, const HEIGHT: u16>`) use const generics to provide these associated constant values:

```rust
impl<const WIDTH: u16, const HEIGHT: u16> Widget for Box<WIDTH, HEIGHT> {
    const WIDTH: u16 = WIDTH;
    const HEIGHT: u16 = HEIGHT;
    // ...
}
```

**Why this works**:
- Associated constants are **monomorphized** at compile-time when concrete types instantiate the trait
- Once `Box::<80, 30>::new()` is created, its `Widget::WIDTH` and `Widget::HEIGHT` are baked into the vtable
- `Box<dyn Widget>` stores a trait object with a vtable pointer that includes the concrete type's WIDTH/HEIGHT values
- Calling `widget.WIDTH` on a trait object accesses the monomorphized constant from the vtable

**Alternative Considered**: Enum-based storage (`enum AnyWidget { Box80x30(Box<80, 30>), Label20x1(Label<20, 1>) }`) - rejected due to non-extensibility and code generation burden.

**Validation**: T007 task implements WidgetNode with `Box<dyn Widget>` storage.

---

### Box<WIDTH, HEIGHT> Widget

**Purpose**: Primary container widget that stores children with explicit positions. Uses const generic parameters for compile-time size specification.

**Definition**:
```rust
pub struct Box<const WIDTH: u16, const HEIGHT: u16> {
    /// Children widgets with relative positions
    children: Vec<WidgetNode>,
}

impl<const WIDTH: u16, const HEIGHT: u16> Box<WIDTH, HEIGHT> {
    /// Create a new Box widget with const generic dimensions.
    ///
    /// # Panics
    /// Panics in debug builds if WIDTH or HEIGHT is zero.
    /// In release builds with zero-size const generics, behavior is undefined
    /// (const generic validation should prevent instantiation).
    pub fn new() -> Self {
        // Runtime assertion in debug builds (const_assert! not stable yet)
        debug_assert!(WIDTH > 0 && HEIGHT > 0, "Box dimensions must be non-zero");

        Self {
            children: Vec::new(),
        }
    }

    /// Add a child widget at the specified relative position (composition phase).
    ///
    /// # Errors
    /// - `ChildExceedsParent`: Child's size extends beyond parent bounds
    /// - `OutOfBounds`: Position places child outside parent bounds
    /// - `OverlappingChildren`: Child overlaps with existing child
    /// - `IntegerOverflow`: Coordinate calculation overflows
    pub fn add_child<W: Widget + 'static>(
        &mut self,
        widget: W,
        position: (u16, u16),
    ) -> Result<(), RenderError> {
        // Validate child fits within parent bounds (with checked arithmetic)
        let child_right = position.0.checked_add(W::WIDTH)
            .ok_or(RenderError::IntegerOverflow {
                operation: "child position.x + width".to_string(),
            })?;
        let child_bottom = position.1.checked_add(W::HEIGHT)
            .ok_or(RenderError::IntegerOverflow {
                operation: "child position.y + height".to_string(),
            })?;

        if child_right > WIDTH || child_bottom > HEIGHT {
            return Err(RenderError::ChildExceedsParent {
                parent_width: WIDTH,
                parent_height: HEIGHT,
                child_width: W::WIDTH,
                child_height: W::HEIGHT,
                position,
            });
        }

        // Check for overlaps with existing children using AABB collision detection
        // Per FR-005A: touching edges (shared boundary) does NOT count as overlap
        let new_bounds = (position.0, position.1, W::WIDTH, W::HEIGHT);
        for existing in &self.children {
            let existing_right = existing.position.0.checked_add(existing.widget.WIDTH)
                .ok_or(RenderError::IntegerOverflow {
                    operation: "existing child bounds calculation".to_string(),
                })?;
            let existing_bottom = existing.position.1.checked_add(existing.widget.HEIGHT)
                .ok_or(RenderError::IntegerOverflow {
                    operation: "existing child bounds calculation".to_string(),
                })?;

            // AABB intersection check with strict inequality (touching edges allowed)
            let overlaps = child_right > existing.position.0
                && position.0 < existing_right
                && child_bottom > existing.position.1
                && position.1 < existing_bottom;

            if overlaps {
                return Err(RenderError::OverlappingChildren {
                    child1_bounds: (existing.position.0, existing.position.1,
                                    existing.widget.WIDTH, existing.widget.HEIGHT),
                    child2_bounds: new_bounds,
                });
            }
        }

        // Add child to tree
        self.children.push(WidgetNode {
            position,
            widget: std::boxed::Box::new(widget),
        });

        Ok(())
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> Widget for Box<WIDTH, HEIGHT> {
    const WIDTH: u16 = WIDTH;
    const HEIGHT: u16 = HEIGHT;

    fn render_to(
        &self,
        context: &mut RenderContext,
        position: (u16, u16),
    ) -> Result<(), RenderError> {
        // Render all children with cumulative offset
        for child in &self.children {
            let child_pos = (
                position.0 + child.position.0,
                position.1 + child.position.1,
            );
            child.widget.render_to(context, child_pos)?;
        }
        Ok(())
    }
}
```

**State Transitions**:
```
[Created] --> add_child() --> [Building]
[Building] --> add_child() --> [Building]
[Building] --> &self for render --> [Rendering]
[Rendering] --> (immutable, no state changes)
```

**Validation Rules**:
- Width and height MUST be > 0 (enforced at construction)
- Children MUST fit within parent bounds (validated in `add_child()`)
- Children positions MUST NOT cause integer overflow (checked arithmetic)
- Rendering MUST NOT modify state (`&self` borrow)

---

### RenderContext

**Purpose**: Public API abstraction that wraps PageBuilder during render phase, tracking cumulative coordinates and enforcing boundary validation with error returns.

**Definition**:
```rust
pub struct RenderContext<'a> {
    /// Reference to the underlying PageBuilder
    page_builder: &'a mut PageBuilder,

    /// Clip bounds (x, y, width, height) for boundary enforcement
    clip_bounds: (u16, u16, u16, u16),
}
```

**Public API Methods**:

```rust
impl<'a> RenderContext<'a> {
    /// Create a new RenderContext wrapping a PageBuilder (internal use).
    ///
    /// Called by Page::render() to create context for widget tree traversal.
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
    /// # Errors
    /// Returns `RenderError::OutOfBounds` if position exceeds clip bounds.
    ///
    /// # Example
    /// ```rust
    /// impl Widget for MyWidget {
    ///     fn render_to(&self, context: &mut RenderContext, position: (u16, u16)) -> Result<(), RenderError> {
    ///         context.write_text("Hello", position)?;
    ///         Ok(())
    ///     }
    /// }
    /// ```
    pub fn write_text(
        &mut self,
        text: &str,
        position: (u16, u16),
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
        self.page_builder.write(position.0, position.1, text);
        Ok(())
    }

    /// Write styled text to the page at the specified absolute position.
    ///
    /// This is the public API for widgets to render styled text (bold, underline).
    ///
    /// # Errors
    /// Returns `RenderError::OutOfBounds` if position exceeds clip bounds.
    ///
    /// # Example
    /// ```rust
    /// impl Widget for Label {
    ///     fn render_to(&self, context: &mut RenderContext, position: (u16, u16)) -> Result<(), RenderError> {
    ///         context.write_styled(&self.text, position, self.style)?;
    ///         Ok(())
    ///     }
    /// }
    /// ```
    pub fn write_styled(
        &mut self,
        text: &str,
        position: (u16, u16),
        style: Style,
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
        self.page_builder.write_styled(position.0, position.1, text, style);
        Ok(())
    }

    /// Get current clip bounds for advanced boundary checking.
    ///
    /// Returns (x, y, width, height) tuple representing the current clip region.
    ///
    /// # Example
    /// ```rust
    /// let (_x, _y, width, height) = context.clip_bounds();
    /// if position.0 + text.len() as u16 > width {
    ///     // Handle text truncation
    /// }
    /// ```
    pub fn clip_bounds(&self) -> (u16, u16, u16, u16) {
        self.clip_bounds
    }
}
```

**Relationships**:
- Wraps `PageBuilder` during render phase (scoped lifetime `'a`)
- Created by `Page::render()` and passed to widget tree traversal
- **Public API** - widgets implement `render_to()` using RenderContext methods

**Validation Rules**:
- All write operations MUST validate bounds and return errors (per Constitution Widget Exception)
- Clip bounds MUST match page dimensions (160 × 51 for V1)
- MUST delegate to PageBuilder for actual rendering (maintains ESC/P compliance)
- Boundary violations return `Result::Err` (not silent truncation)

---

### Label<WIDTH, HEIGHT> Widget

**Purpose**: Leaf widget for rendering styled text content with compile-time const generic dimensions.

**Definition**:
```rust
pub struct Label<const WIDTH: u16, const HEIGHT: u16> {
    /// Text content to render (None until add_text() called)
    text: Option<String>,

    /// Text style (bold, underline, etc.)
    style: Style,
}

impl<const WIDTH: u16, const HEIGHT: u16> Label<WIDTH, HEIGHT> {
    /// Create a new Label with specified const generic dimensions.
    ///
    /// # Panics
    /// Panics in debug builds if HEIGHT ≠ 1 (Label must be single-line).
    ///
    /// # Examples
    /// ```
    /// let label = Label::<20, 1>::new()
    ///     .add_text("Hello World")?
    ///     .bold();
    /// ```
    pub fn new() -> Self {
        debug_assert!(HEIGHT == 1, "Label HEIGHT must be 1");

        Self {
            text: None,
            style: Style::default(),
        }
    }

    /// Add text content to the label (builder pattern).
    ///
    /// # Errors
    /// Returns `RenderError::TextExceedsWidth` if:
    /// - Text length exceeds WIDTH
    /// - Text contains newline characters (`\n`, `\r\n`)
    ///
    /// # Examples
    /// ```
    /// let label = Label::<20, 1>::new().add_text("Hello")?;
    /// ```
    pub fn add_text(mut self, text: impl Into<String>) -> Result<Self, RenderError> {
        let text = text.into();

        // Validate text length
        if text.len() as u16 > WIDTH {
            return Err(RenderError::TextExceedsWidth {
                text_length: text.len() as u16,
                widget_width: WIDTH,
            });
        }

        // Validate single-line constraint
        if text.contains('\n') || text.contains("\r\n") {
            return Err(RenderError::TextExceedsWidth {
                text_length: text.len() as u16,
                widget_width: WIDTH,
            });
        }

        self.text = Some(text);
        Ok(self)
    }

    /// Apply bold styling (builder pattern).
    pub fn bold(mut self) -> Self {
        self.style = self.style.bold();
        self
    }

    /// Apply underline styling (builder pattern).
    pub fn underline(mut self) -> Self {
        self.style = self.style.underline();
        self
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> Widget for Label<WIDTH, HEIGHT> {
    const WIDTH: u16 = WIDTH;
    const HEIGHT: u16 = HEIGHT;

    fn render_to(
        &self,
        context: &mut RenderContext,
        position: (u16, u16),
    ) -> Result<(), RenderError> {
        // Render text with style at given position
        // Text was validated at construction time (Label::new checks text.len() <= WIDTH)
        context.write_styled(&self.text, position, self.style)
    }
}
```

**Usage Examples**:
```rust
// Label with explicit const generic dimensions (builder pattern)
let label1 = Label::<15, 1>::new()
    .add_text("Item 1: $10.00")?;
root.add_child(label1, (0, 0))?;

// Text that exceeds WIDTH returns error
let label2 = Label::<10, 1>::new()
    .add_text("Long text will be truncated");
// Returns Err(RenderError::TextExceedsWidth { text_length: 28, widget_width: 10 })

// Styled label with const dimensions (builder pattern chaining)
let label3 = Label::<15, 1>::new()
    .add_text("Total: $25.00")?
    .bold()
    .underline();
root.add_child(label3, (0, 4))?;

// Using macro wrapper syntax
let label4 = label_new!(15)
    .add_text("Subtotal")?
    .bold();
root.add_child(label4, (0, 2))?;
```

**Validation Rules**:
- Text MUST be valid UTF-8 (enforced by String type)
- WIDTH and HEIGHT MUST be > 0 (enforced via debug_assert!)
- HEIGHT MUST be 1 (enforced via debug_assert! in new(), panics in debug builds if HEIGHT ≠ 1)
- Text length MUST NOT exceed WIDTH (validated in add_text(), returns TextExceedsWidth error)
- Text MUST NOT contain newlines (`\n`, `\r\n`) - validated in add_text(), returns TextExceedsWidth error
- Non-ASCII characters replaced with '?' per Constitution Principle V

---

## Error Types

### RenderError

**Purpose**: Comprehensive error type for all widget rendering violations.

**Definition**:
```rust
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// Child widget exceeds parent widget's bounds
    ChildExceedsParent {
        parent_width: u16,
        parent_height: u16,
        child_width: u16,
        child_height: u16,
        position: (u16, u16),
    },

    /// Widget positioned outside valid bounds
    OutOfBounds {
        position: (u16, u16),
        bounds: (u16, u16, u16, u16), // (x, y, width, height)
    },

    /// Two or more children overlap within parent widget
    OverlappingChildren {
        child1_bounds: (u16, u16, u16, u16), // (x, y, width, height)
        child2_bounds: (u16, u16, u16, u16),
    },

    /// Layout component cannot fit all children in available space
    InsufficientSpace {
        available: u16,
        required: u16,
        layout_type: &'static str,
    },

    /// Integer overflow in coordinate or size calculation
    IntegerOverflow {
        operation: String,
    },

    /// Text content exceeds widget width or contains newlines
    TextExceedsWidth {
        text_length: u16,
        widget_width: u16,
    },
}

// Note: ZeroSizeParent error variant removed per FR-007 and Constitution Principle VI.
// Zero-size prevention uses validation hierarchy:
// 1. Compile-time (future): const generic value constraints (not yet stable in Rust 1.91.1)
// 2. Development-time (PRIMARY): debug_assert!(WIDTH > 0 && HEIGHT > 0) in Box::new()
// 3. Project Policy: Zero-size Box instantiation in release builds is undefined behavior

impl std::fmt::Display for RenderError {
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
            RenderError::OverlappingChildren { child1_bounds, child2_bounds } => write!(
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
            RenderError::TextExceedsWidth { text_length, widget_width } => write!(
                f,
                "Text length ({}) exceeds widget width ({})",
                text_length, widget_width
            ),
        }
    }
}

impl std::error::Error for RenderError {}
```

**Usage Context**:
- **Composition Phase**: `ChildExceedsParent`, `OverlappingChildren`, `InsufficientSpace`, `IntegerOverflow` (returned by `add_child()` or layout methods)
- **Render Phase**: `OutOfBounds`, `IntegerOverflow` (returned by `RenderContext` or widget traversal)
- **Widget Construction**: `TextExceedsWidth` (returned by `Label::add_text()` when text validation fails)

**Validation Rules**:
- All errors MUST provide contextual information (sizes, positions, bounds)
- Error messages MUST be human-readable for debugging
- Errors MUST NOT be ignored (Result type forces handling)

---

## Layout Components

**Mutability Model**: Layout components (Column, Row, Stack) use **mutable state** (`&mut self`) in their `area()` methods to track position state (current_x, current_y). This is necessary because:
- Each call to `area()` advances the internal position counter
- The component must remember how much space has been allocated
- Multiple calls to `area()` build up the layout incrementally

**Const Generic Area Methods**: Layout components return Box<const WIDTH, const HEIGHT> widgets using const generic area() methods. The dimensions are specified via turbofish syntax at the call site (e.g., `column.area::<10>()`), preserving compile-time size specification.

### Column Layout

**Purpose**: Divides parent Box vertically, returning nested Box widgets for each row with compile-time dimensions specified via turbofish.

**Definition**:
```rust
pub struct Column<const WIDTH: u16, const HEIGHT: u16> {
    current_y: u16,
}

impl<const WIDTH: u16, const HEIGHT: u16> Column<WIDTH, HEIGHT> {
    /// Create a Column layout with const generic parent dimensions.
    pub fn new() -> Self {
        Self {
            current_y: 0,
        }
    }

    /// Allocate a horizontal area (row) with specified height via const generic.
    ///
    /// Returns a Box<WIDTH, H> positioned at the next available Y offset.
    ///
    /// # Errors
    /// Returns `InsufficientSpace` if requested height exceeds remaining space.
    ///
    /// # Example
    /// ```
    /// let mut column = Column::<80, 30>::new();
    /// let (box1, pos1) = column.area::<10>()?; // Returns Box<80, 10>
    /// ```
    pub fn area<const H: u16>(&mut self) -> Result<(Box<WIDTH, H>, (u16, u16)), RenderError> {
        if self.current_y + H > HEIGHT {
            return Err(RenderError::InsufficientSpace {
                available: HEIGHT - self.current_y,
                required: H,
                layout_type: "Column",
            });
        }

        let position = (0, self.current_y);
        let box_widget = Box::<WIDTH, H>::new();

        self.current_y += H;

        Ok((box_widget, position))
    }
}
```

**State Transitions**:
```
[Created] --> area::<H>() --> [Allocating]
[Allocating] --> area::<H>() --> [Allocating]
[Allocating] --> (space exhausted) --> [Complete]
```

**Usage Example**:
```rust
let mut root = Box::<80, 30>::new();
let mut column = Column::<80, 30>::new();

let (mut row1, pos1) = column.area::<10>()?;  // Box<80, 10>
let label1 = Label::<20, 1>::new("Row 1")?;
row1.add_child(label1, (0, 0))?;
root.add_child(row1, pos1)?;

let (mut row2, pos2) = column.area::<10>()?;  // Box<80, 10>
let label2 = Label::<20, 1>::new("Row 2")?;
row2.add_child(label2, (0, 0))?;
root.add_child(row2, pos2)?;
```

---

### Row Layout

**Purpose**: Divides parent Box horizontally, returning nested Box widgets for each column with compile-time dimensions specified via turbofish.

**Definition**:
```rust
pub struct Row<const WIDTH: u16, const HEIGHT: u16> {
    current_x: u16,
}

impl<const WIDTH: u16, const HEIGHT: u16> Row<WIDTH, HEIGHT> {
    /// Create a Row layout with const generic parent dimensions.
    pub fn new() -> Self {
        Self {
            current_x: 0,
        }
    }

    /// Allocate a vertical area (column) with specified width via const generic.
    ///
    /// Returns a Box<W, HEIGHT> positioned at the next available X offset.
    ///
    /// # Errors
    /// Returns `InsufficientSpace` if requested width exceeds remaining space.
    ///
    /// # Example
    /// ```
    /// let mut row = Row::<80, 30>::new();
    /// let (box1, pos1) = row.area::<20>()?; // Returns Box<20, 30>
    /// ```
    pub fn area<const W: u16>(&mut self) -> Result<(Box<W, HEIGHT>, (u16, u16)), RenderError> {
        if self.current_x + W > WIDTH {
            return Err(RenderError::InsufficientSpace {
                available: WIDTH - self.current_x,
                required: W,
                layout_type: "Row",
            });
        }

        let position = (self.current_x, 0);
        let box_widget = Box::<W, HEIGHT>::new();

        self.current_x += W;

        Ok((box_widget, position))
    }
}
```

---

### Stack Layout

**Purpose**: Returns overlapping nested Box widgets at the same position (for layering) with compile-time dimensions.

**Definition**:
```rust
pub struct Stack<const WIDTH: u16, const HEIGHT: u16>;

impl<const WIDTH: u16, const HEIGHT: u16> Stack<WIDTH, HEIGHT> {
    /// Create a Stack layout with const generic parent dimensions.
    pub fn new() -> Self {
        Self
    }

    /// Allocate an overlapping area (same position for all calls).
    ///
    /// Returns a Box<WIDTH, HEIGHT> positioned at (0, 0) for layering.
    ///
    /// # Example
    /// ```
    /// let stack = Stack::<80, 30>::new();
    /// let (box1, pos1) = stack.area(); // Returns Box<80, 30> at (0, 0)
    /// let (box2, pos2) = stack.area(); // Returns Box<80, 30> at (0, 0)
    /// ```
    pub fn area(&self) -> (Box<WIDTH, HEIGHT>, (u16, u16)) {
        let position = (0, 0);
        let box_widget = Box::<WIDTH, HEIGHT>::new();
        (box_widget, position)
    }
}
```

**Usage Example**:
```rust
let mut root = Box::<80, 30>::new();
let stack = Stack::<80, 30>::new();

let (mut bg, pos) = stack.area();  // Box<80, 30> at (0, 0)
// bg.add_child(Background::new(), (0, 0))?;
root.add_child(bg, pos)?;

let (mut fg, pos) = stack.area();  // Box<80, 30> at (0, 0)
let label = Label::<10, 1>::new("Overlay")?;
fg.add_child(label, (10, 10))?;
root.add_child(fg, pos)?;
```

---

## Page Enhancement

### Page::render (New Method)

**Purpose**: Render a widget tree to the page, traversing the tree and outputting to PageBuilder.

**Definition**:
```rust
impl Page {
    /// Render a widget tree to this page.
    ///
    /// The widget tree is traversed depth-first, with each widget rendering
    /// at its cumulative absolute position. The root widget is always positioned
    /// at (0, 0).
    ///
    /// Widgets are borrowed immutably and can be rendered multiple times.
    ///
    /// # Errors
    /// Returns `RenderError::OutOfBounds` if any widget attempts to render
    /// outside page bounds (160 × 51).
    ///
    /// # Example
    /// ```
    /// let mut page = Page::new();
    /// let root = Box::<80, 30>::new();
    /// page.render(&root)?;  // Can render multiple times
    /// page.render(&root)?;  // Same widget, multiple renders
    /// ```
    pub fn render(&mut self, widget: &impl Widget) -> Result<(), RenderError> {
        let mut context = RenderContext::new(&mut self.page_builder);
        widget.render_to(&mut context, (0, 0))
    }
}
```

---

## Entity Relationships

```
┌─────────────────────────────────────────────────────────────┐
│                        Page                                  │
│  ┌────────────────────────────────────────────────────┐     │
│  │ render(widget: &impl Widget)                       │     │
│  │  ├─> Creates RenderContext(PageBuilder)            │     │
│  │  └─> Calls widget.render_to(context, (0,0))        │     │
│  └────────────────────────────────────────────────────┘     │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐
          │      RenderContext              │
          │  ┌───────────────────────────┐  │
          │  │ PageBuilder (wrapped)     │  │
          │  │ clip_bounds: (u16,u16,    │  │
          │  │               u16,u16)    │  │
          │  └───────────────────────────┘  │
          └─────────────────────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐
          │   Box<WIDTH, HEIGHT> (Widget)   │
          │  ┌───────────────────────────┐  │
          │  │ children: Vec<WidgetNode> │  │
          │  │ (const WIDTH, HEIGHT)     │  │
          │  └───────────────────────────┘  │
          └─────────────────────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐
          │      WidgetNode                 │
          │  ┌───────────────────────────┐  │
          │  │ position: (u16, u16)      │  │
          │  │ widget: Box<dyn Widget>   │  │
          │  └───────────────────────────┘  │
          └─────────────────────────────────┘
                           │
                           ▼
          ┌─────────────────────────────────┐
          │      Any Widget Impl            │
          │  (Box, Label, etc.)             │
          └─────────────────────────────────┘
```

---

## Memory Layout Summary

**Per-Page Budget**: < 128 KB total (per Constitution Principle X and spec clarification 2025-11-19)

**Per-Page Allocation Breakdown**:
- Character grid: 160 × 51 = 8,160 cells × 1 byte (char) = ~8 KB
- Style data: 160 × 51 = 8,160 cells × 1 byte (style flags) = ~8 KB
- Widget tree overhead (estimated 100 nodes): 100 × 40 bytes = ~4 KB
- Page metadata and other allocations: ~4 KB
- **Total Estimated**: ~24 KB (well within 128 KB budget, < 19% utilization)

**Typical Widget Tree** (3-column layout with labels):
```
Root Box<80, 30>
  ├─ Column1 Box<25, 30> at (0, 0)
  │   └─ Label<25, 1> at (0, 0)
  ├─ Column2 Box<25, 30> at (27, 0)
  │   └─ Label<25, 1> at (0, 0)
  └─ Column3 Box<25, 30> at (54, 0)
      └─ Label<25, 1> at (0, 0)

Node count: 7 widgets × 20 bytes = 140 bytes overhead
```

---

## Validation Summary

| Entity | Validation Rules | Error Type |
|--------|------------------|------------|
| Box::new | WIDTH > 0, HEIGHT > 0 | debug_assert! panic (debug builds only) |
| Box::add_child | child fits in parent bounds | ChildExceedsParent |
| Box::add_child | position + size <= parent size | OutOfBounds |
| Box::add_child | no overlap with existing children (AABB) | OverlappingChildren |
| Label::new | HEIGHT == 1 | debug_assert! panic (debug builds only) |
| Label::add_text | text.len() <= WIDTH | TextExceedsWidth |
| Label::add_text | no newline characters in text | TextExceedsWidth |
| RenderContext::write_text | position within clip bounds | OutOfBounds |
| Column::area | current_y + H <= HEIGHT | InsufficientSpace |
| Row::area | current_x + W <= WIDTH | InsufficientSpace |
| All coordinate arithmetic | no integer overflow (checked_add) | IntegerOverflow |

---

## Next Steps

Phase 1 continues with:
1. **contracts/**: API contracts (Rust trait definitions, type signatures)
2. **quickstart.md**: Developer onboarding guide with examples
3. **Agent context update**: Update CLAUDE.md with new technologies

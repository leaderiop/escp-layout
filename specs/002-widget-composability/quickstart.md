# Quickstart: Widget Composability System

**Feature**: Widget Composability System
**Branch**: `002-widget-composability`
**Audience**: Developers integrating widget composition into applications
**Time to First Working Example**: < 5 minutes
**Date**: 2025-11-19

---

## Overview

The widget composability system enables React-like composition of UI elements for ESC/P thermal printing. Build complex layouts from simple, reusable components with compile-time size validation, automatic coordinate calculation, and boundary enforcement.

**Key Benefits**:

- **Declarative composition**: Build widget trees instead of calculating coordinates manually
- **Compile-time safety**: Const generic dimensions catch sizing errors at compile time
- **Automatic positioning**: Parent-child relationships handle offset calculation
- **Type-safe boundaries**: Three-tier validation (compile-time > debug > runtime) prevents layout bugs
- **Dual syntax support**: Choose between explicit turbofish or ergonomic macros
- **Zero dependencies**: Pure Rust with no external runtime dependencies

---

## Prerequisites

- **Rust**: 1.91.1+ (stable channel, 2021 edition)
- **Cargo**: Comes with Rust installation
- **Dependencies**: None (only Rust `std`)

---

## Quick Example (Turbofish Syntax)

```rust
use escp_layout::widget::{Rect, Column, Label};
use escp_layout::Page;

fn main() -> Result<(), Rect<dyn std::error::Error>> {
    // Step 1: Create root container with compile-time dimensions (80 cols × 30 rows)
    let mut root = Rect::<80, 30>::new();

    // Step 2: Use Column layout to divide vertical space
    let mut column = Column::<80, 30>::new();

    // Step 3: Allocate rows with compile-time heights and add content
    let (mut header, header_pos) = column.area::<5>()?;
    let header_label = Label::<15, 1>::new()
        .add_text("=== Receipt ===")?;
    header.add_child(header_label, (0, 0))?;
    root.add_child(header, header_pos)?;

    let (mut body, body_pos) = column.area::<20>()?;
    let item1 = Label::<20, 1>::new().add_text("Item 1: $10.00")?;
    let item2 = Label::<20, 1>::new().add_text("Item 2: $15.00")?;
    body.add_child(item1, (0, 0))?;
    body.add_child(item2, (0, 2))?;
    root.add_child(body, body_pos)?;

    let (mut footer, footer_pos) = column.area::<5>()?;
    let total = Label::<20, 1>::new()
        .add_text("Total: $25.00")?
        .bold();
    footer.add_child(total, (0, 0))?;
    root.add_child(footer, footer_pos)?;

    // Step 4: Render widget tree to page (immutable borrow)
    let mut page = Page::new();
    page.render(&root)?;

    // Step 5: Generate ESC/P output
    let escp_bytes = page.to_bytes();
    std::fs::write("receipt.escp", escp_bytes)?;

    Ok(())
}
```

---

## Quick Example (Macro Syntax)

```rust
use escp_layout::widget::{rect_new, column_new, column_area, label_new};
use escp_layout::Page;

fn main() -> Result<(), Rect<dyn std::error::Error>> {
    // Step 1: Create root container (80 cols × 30 rows)
    let mut root = rect_new!(80, 30);

    // Step 2: Use Column layout to divide vertical space
    let mut column = column_new!(80, 30);

    // Step 3: Allocate rows and add content (HEIGHT=1 automatic for label_new!)
    let (mut header, header_pos) = column_area!(column, 5)?;
    let header_label = label_new!(15).add_text("=== Receipt ===")?;
    header.add_child(header_label, (0, 0))?;
    root.add_child(header, header_pos)?;

    let (mut body, body_pos) = column_area!(column, 20)?;
    let item1 = label_new!(20).add_text("Item 1: $10.00")?;
    let item2 = label_new!(20).add_text("Item 2: $15.00")?;
    body.add_child(item1, (0, 0))?;
    body.add_child(item2, (0, 2))?;
    root.add_child(body, body_pos)?;

    let (mut footer, footer_pos) = column_area!(column, 5)?;
    let total = label_new!(20)
        .add_text("Total: $25.00")?
        .bold();
    footer.add_child(total, (0, 0))?;
    root.add_child(footer, footer_pos)?;

    // Step 4: Render widget tree to page
    let mut page = Page::new();
    page.render(&root)?;

    // Step 5: Generate ESC/P output
    let escp_bytes = page.to_bytes();
    std::fs::write("receipt.escp", escp_bytes)?;

    Ok(())
}
```

**Output** (thermal printer):

```
=== Receipt ===

Item 1: $10.00

Item 2: $15.00


Total: $25.00
```

---

## Core Concepts

### Two-Phase Rendering

Widget composition uses a two-phase model:

1. **Composition Phase** (mutable): Build the widget tree with validation

   ```rust
   let mut root = Rect::<80, 30>::new();
   root.add_child(child_widget, (10, 5))?; // Validates bounds, overlap, overflow
   ```

2. **Render Phase** (immutable): Traverse tree and output to PageBuilder
   ```rust
   let mut page = Page::new();
   page.render(&root)?; // Immutable borrow, can render multiple times
   ```

### Widget Trait

All widgets implement the `Widget` trait with associated constants:

```rust
pub trait Widget {
    const WIDTH: u16;   // Compile-time dimension
    const HEIGHT: u16;  // Compile-time dimension

    fn render_to(
        &self,
        context: &mut RenderContext,
        position: (u16, u16),
    ) -> Result<(), RenderError>;
}
```

**Implementation Example** (Rect widget):

```rust
impl<const WIDTH: u16, const HEIGHT: u16> Widget for Rect<WIDTH, HEIGHT> {
    const WIDTH: u16 = WIDTH;   // Const generic provides value
    const HEIGHT: u16 = HEIGHT;

    fn render_to(&self, context: &mut RenderContext, position: (u16, u16)) -> Result<(), RenderError> {
        // Render all children with cumulative offset
        for child in &self.children {
            let child_pos = (position.0 + child.position.0, position.1 + child.position.1);
            child.widget.render_to(context, child_pos)?;
        }
        Ok(())
    }
}
```

### Const Generic Dimensions

Widgets use const generics for compile-time size specification:

```rust
// Turbofish syntax (explicit)
let container = Rect::<80, 30>::new();
let label = Label::<20, 1>::new().add_text("Hello")?;

// Macro syntax (ergonomic)
let container = rect_new!(80, 30);
let label = label_new!(20).add_text("Hello")?; // HEIGHT=1 automatic
```

**Why const generics?**

- Dimensions known at compile time (zero runtime overhead)
- Type-safe: `Rect<80, 30>` is different from `Rect<40, 15>`
- Enables compile-time validation (future Rust versions will prevent `Rect<0, 10>`)

### Validation Hierarchy

The system uses a three-tier validation strategy per Constitution Principle VI:

**Tier 1 - Compile-time** (preferred, zero runtime cost):

```rust
let rect1 = Rect::<80, 30>::new();  // Dimensions baked into type
let rect2 = Rect::<40, 15>::new();  // Different type from rect1
```

**Tier 2 - Development-time** (debug builds only, zero release cost):

```rust
let rect_zero = Rect::<0, 10>::new();    // Panics in debug: "WIDTH must be > 0"
let label_multi = Label::<20, 2>::new(); // Panics in debug: "HEIGHT must be 1"
```

**Tier 3 - Runtime** (only when compile-time impossible):

```rust
let label = Label::<10, 1>::new()
    .add_text("This text is too long")?;  // Err(TextExceedsWidth)

let mut parent = Rect::<20, 20>::new();
let child1 = Label::<10, 1>::new().add_text("Child 1")?;
let child2 = Label::<10, 1>::new().add_text("Child 2")?;
parent.add_child(child1, (0, 0))?;       // OK
parent.add_child(child2, (5, 0))?;       // Err(OverlappingChildren)
```

---

## Widget Types

### Rect<WIDTH, HEIGHT> (Container)

Primary container widget with compile-time dimensions:

```rust
// Turbofish syntax
let mut container = Rect::<80, 30>::new();

// Macro syntax
let mut container = rect_new!(80, 30);

// Add children with explicit positions
container.add_child(child_widget, (10, 5))?;
```

**Validation**:

- Child must fit within parent bounds (ChildExceedsParent error)
- Children cannot overlap (OverlappingChildren error per AABB detection)
- All arithmetic checked (IntegerOverflow error)

### Label<WIDTH, 1> (Leaf)

Single-line text widget with builder pattern:

```rust
// Turbofish syntax
let label = Label::<20, 1>::new()
    .add_text("Hello World")?
    .bold()
    .underline();

// Macro syntax (HEIGHT=1 automatic)
let label = label_new!(20)
    .add_text("Hello World")?
    .bold();
```

**Constraints**:

- HEIGHT must always be 1 (enforced via debug_assert!)
- Text length must be ≤ WIDTH (returns TextExceedsWidth error)
- No newline characters allowed (returns TextExceedsWidth error)

---

## Layout Components

Layout components divide parent Rect space and return nested Rect widgets with compile-time dimensions.

### Column (Vertical Division)

```rust
// Turbofish syntax
let mut column = Column::<80, 30>::new();
let (row1, pos1) = column.area::<10>()?;  // Rect<80, 10>
let (row2, pos2) = column.area::<20>()?;  // Rect<80, 20>

// Macro syntax
let mut column = column_new!(80, 30);
let (row1, pos1) = column_area!(column, 10)?;
let (row2, pos2) = column_area!(column, 20)?;
```

### Row (Horizontal Division)

```rust
// Turbofish syntax
let mut row = Row::<80, 30>::new();
let (col1, pos1) = row.area::<20>()?;  // Rect<20, 30>
let (col2, pos2) = row.area::<60>()?;  // Rect<60, 30>

// Macro syntax
let mut row = row_new!(80, 30);
let (col1, pos1) = row_area!(row, 20)?;
let (col2, pos2) = row_area!(row, 60)?;
```

### Stack (Overlapping Layers)

```rust
// Turbofish syntax
let stack = Stack::<80, 30>::new();
let (bg, pos) = stack.area();   // Rect<80, 30> at (0, 0)
let (fg, pos) = stack.area();   // Rect<80, 30> at (0, 0)

// Macro syntax
let stack = stack_new!(80, 30);
let (bg, pos) = stack.area();
let (fg, pos) = stack.area();
```

---

## Error Handling

All composition and rendering operations return `Result<T, RenderError>`:

```rust
#[non_exhaustive]
pub enum RenderError {
    ChildExceedsParent { .. },     // Child too large for parent
    OutOfBounds { .. },             // Position outside bounds
    OverlappingChildren { .. },     // AABB collision detected
    InsufficientSpace { .. },       // Layout space exhausted
    IntegerOverflow { .. },         // Checked arithmetic overflow
    TextExceedsWidth { .. },        // Text validation failure
}
```

**Example Error Handling**:

```rust
match parent.add_child(child, (10, 5)) {
    Ok(()) => println!("Child added successfully"),
    Err(RenderError::ChildExceedsParent { child_width, parent_width, .. }) => {
        eprintln!("Child width {} exceeds parent width {}", child_width, parent_width);
    }
    Err(RenderError::OverlappingChildren { child1_bounds, child2_bounds }) => {
        eprintln!("Children overlap at {:?} and {:?}", child1_bounds, child2_bounds);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

---

## Common Patterns

### 3-Column Layout

```rust
let mut root = rect_new!(80, 30);
let mut row = row_new!(80, 30);

let (mut col1, pos1) = row_area!(row, 25)?;
col1.add_child(label_new!(25).add_text("Column 1")?, (0, 0))?;
root.add_child(col1, pos1)?;

let (mut col2, pos2) = row_area!(row, 30)?;
col2.add_child(label_new!(30).add_text("Column 2")?, (0, 0))?;
root.add_child(col2, pos2)?;

let (mut col3, pos3) = row_area!(row, 25)?;
col3.add_child(label_new!(25).add_text("Column 3")?, (0, 0))?;
root.add_child(col3, pos3)?;
```

### Nested Layouts

```rust
let mut root = rect_new!(80, 30);
let mut column = column_new!(80, 30);

// Header row
let (mut header, header_pos) = column_area!(column, 5)?;
header.add_child(label_new!(80).add_text("Header")?.bold(), (0, 0))?;
root.add_child(header, header_pos)?;

// Body with 3 columns
let (mut body, body_pos) = column_area!(column, 20)?;
let mut body_row = row_new!(80, 20);

let (mut body_col1, pos1) = row_area!(body_row, 25)?;
body_col1.add_child(label_new!(25).add_text("Left Panel")?, (0, 0))?;
body.add_child(body_col1, pos1)?;

let (mut body_col2, pos2) = row_area!(body_row, 30)?;
body_col2.add_child(label_new!(30).add_text("Center Panel")?, (0, 0))?;
body.add_child(body_col2, pos2)?;

let (mut body_col3, pos3) = row_area!(body_row, 25)?;
body_col3.add_child(label_new!(25).add_text("Right Panel")?, (0, 0))?;
body.add_child(body_col3, pos3)?;

root.add_child(body, body_pos)?;

// Footer row
let (mut footer, footer_pos) = column_area!(column, 5)?;
footer.add_child(label_new!(80).add_text("Footer")?, (0, 0))?;
root.add_child(footer, footer_pos)?;
```

---

## Best Practices

### 1. Choose Syntax Based on Preference

Both syntaxes are equally valid - choose based on team preference:

**Turbofish** (explicit type information):

```rust
let widget = Rect::<80, 30>::new();
let label = Label::<20, 1>::new();
```

**Macros** (concise, ergonomic):

```rust
let widget = rect_new!(80, 30);
let label = label_new!(20);  // HEIGHT=1 automatic
```

### 2. Validate in Debug Builds

Run tests in debug mode to catch constraint violations:

```bash
cargo test  # Runs in debug mode, debug_assert! active
```

### 3. Use Builder Pattern for Labels

```rust
let label = label_new!(30)
    .add_text("Formatted Text")?
    .bold()
    .underline();
```

### 4. Handle Errors Gracefully

```rust
// ❌ Avoid: Unwrapping can panic
parent.add_child(child, (10, 5)).unwrap();

// ✅ Prefer: Propagate errors
parent.add_child(child, (10, 5))?;

// ✅ Or: Handle explicitly
if let Err(e) = parent.add_child(child, (10, 5)) {
    eprintln!("Failed to add child: {}", e);
    // Fallback strategy
}
```

### 5. Reuse Widgets via Immutable Rendering

```rust
let root = rect_new!(80, 30);
// ... build widget tree ...

let mut page1 = Page::new();
page1.render(&root)?;  // First render

let mut page2 = Page::new();
page2.render(&root)?;  // Reuse same widget tree
```

---

## Troubleshooting

### "Child exceeds parent bounds"

**Problem**: `ChildExceedsParent` error during `add_child()`

**Solution**: Ensure child dimensions fit within parent:

```rust
// ❌ Wrong: Child (30×10) doesn't fit in parent (20×20)
let mut parent = rect_new!(20, 20);
let child = rect_new!(30, 10);
parent.add_child(child, (0, 0))?;  // ERROR

// ✅ Correct: Child (15×10) fits in parent (20×20)
let mut parent = rect_new!(20, 20);
let child = rect_new!(15, 10);
parent.add_child(child, (0, 0))?;  // OK
```

### "Children overlap"

**Problem**: `OverlappingChildren` error during `add_child()`

**Solution**: Ensure children don't intersect (touching edges OK):

```rust
let mut parent = rect_new!(80, 30);
let child1 = label_new!(20).add_text("Child 1")?;
let child2 = label_new!(20).add_text("Child 2")?;

parent.add_child(child1, (0, 0))?;    // OK
parent.add_child(child2, (10, 0))?;   // ERROR: overlaps (0-20 intersects 10-30)
parent.add_child(child2, (20, 0))?;   // OK: touching edges (20 == 20) allowed
```

### "Text exceeds width"

**Problem**: `TextExceedsWidth` error during `add_text()`

**Solution**: Ensure text length ≤ widget WIDTH:

```rust
// ❌ Wrong: Text (16 chars) exceeds width (10)
let label = label_new!(10).add_text("Too long text!")?;  // ERROR

// ✅ Correct: Text (10 chars) fits width (10)
let label = label_new!(10).add_text("Short text")?;      // OK
```

### "Insufficient space in layout"

**Problem**: `InsufficientSpace` error during layout `area()` call

**Solution**: Ensure total allocated space ≤ layout dimensions:

```rust
// ❌ Wrong: 10 + 15 + 10 = 35 > 30
let mut column = column_new!(80, 30);
let (row1, pos1) = column_area!(column, 10)?;  // OK: 30 - 10 = 20 remaining
let (row2, pos2) = column_area!(column, 15)?;  // OK: 20 - 15 = 5 remaining
let (row3, pos3) = column_area!(column, 10)?;  // ERROR: 10 > 5

// ✅ Correct: 10 + 15 + 5 = 30
let mut column = column_new!(80, 30);
let (row1, pos1) = column_area!(column, 10)?;  // 20 remaining
let (row2, pos2) = column_area!(column, 15)?;  // 5 remaining
let (row3, pos3) = column_area!(column, 5)?;   // 0 remaining
```

---

## Next Steps

- **See Examples**: `/examples/widget_composition.rs` - Multi-level nesting example
- **Read API Docs**: `cargo doc --open` - Full API documentation
- **View Spec**: `/specs/002-widget-composability/spec.md` - Complete requirements
- **Study Data Model**: `/specs/002-widget-composability/data-model.md` - Entity relationships

---

## FAQ

**Q: Can I use runtime-sized widgets?**
A: No, all widget dimensions must be known at compile time via const generics. This enables compile-time validation and zero runtime overhead.

**Q: What happens if I violate constraints in release builds?**
A: Zero-size widgets or multi-line Labels have undefined behavior in release builds. Always test in debug mode to catch violations via `debug_assert!` panics.

**Q: Can children exceed page bounds (160×51)?**
A: No, RenderContext validates all write positions against clip bounds and returns `OutOfBounds` error.

**Q: Are Layout components required?**
A: No, you can manually position children with `add_child(widget, (x, y))`. Layouts are helpers for common patterns.

**Q: Can I create custom widgets?**
A: Yes, implement the `Widget` trait with associated constants and `render_to()` method. See `/specs/002-widget-composability/contracts/widget_trait.rs`.

**Q: How do I debug layout issues?**
A: Run tests in debug mode (`cargo test`). All errors include contextual information (sizes, positions, bounds) for debugging.

---

**Document Status**: ✅ READY FOR DEVELOPERS
**Version**: 1.0 (aligned with spec.md 2025-11-19)

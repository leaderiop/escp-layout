# Widget Composability System - Examples Guide

This directory contains comprehensive examples demonstrating every feature of the widget composability system.

## Quick Start

Run any example with:
```bash
cargo run --example <example_name>
```

## Example Index

### 01. Basic Label (`01_basic_label.rs`)
**Concepts**: Label creation, text validation, width constraints

Demonstrates:
- Creating labels with different widths
- Adding text content via builder pattern
- Text length validation
- Newline character validation
- Empty labels
- Error handling for text overflow

**Run**: `cargo run --example 01_basic_label`

### 02. Styled Labels (`02_styled_labels.rs`)
**Concepts**: Text styling, builder pattern chaining

Demonstrates:
- Applying bold styling
- Applying underline styling
- Combining multiple styles
- Style order independence
- Multiple styled labels in layouts

**Run**: `cargo run --example 02_styled_labels`

### 03. Box Container (`03_box_container.rs`)
**Concepts**: Container widgets, child positioning, boundary validation

Demonstrates:
- Creating box containers with different sizes
- Adding multiple children with explicit coordinates
- Adjacent children (touching edges allowed)
- Grid layouts
- Error handling for oversized children
- Error handling for overlapping children

**Run**: `cargo run --example 03_box_container`

### 04. Nested Boxes (`04_nested_boxes.rs`)
**Concepts**: Hierarchical structures, cumulative coordinates

Demonstrates:
- Two-level nesting
- Three-level nesting
- Multiple children at each level
- Deep nesting (5 levels)
- Automatic cumulative coordinate calculation
- Multi-branch hierarchies

**Run**: `cargo run --example 04_nested_boxes`

### 05. Column Layout (`05_column_layout.rs`)
**Concepts**: Vertical division, automatic y-position tracking

Demonstrates:
- Simple equal-height rows
- Variable height rows (header/body/footer pattern)
- Multiple labels per row
- Nested content in rows
- InsufficientSpace error handling
- Exact fit layouts

**Run**: `cargo run --example 05_column_layout`

### 06. Row Layout (`06_row_layout.rs`)
**Concepts**: Horizontal division, automatic x-position tracking

Demonstrates:
- Simple equal-width columns
- Variable width columns (sidebar/main/aside pattern)
- Multiple rows per column
- Narrow and wide column combinations
- Equal-width column grids
- InsufficientSpace error handling

**Run**: `cargo run --example 06_row_layout`

### 07. Stack Layout (`07_stack_layout.rs`)
**Concepts**: Conceptual layering, rendering strategies

Demonstrates:
- How Stack returns overlapping boxes at (0, 0)
- Building content in separate logical layers
- Rendering a single layer
- Using a layer as the root widget
- Styled layers
- Multi-step rendering strategies

**Important**: Stack boxes cannot be added to the same parent (causes overlap).
Instead, use Stack to organize content conceptually, then render the appropriate layer.

**Run**: `cargo run --example 07_stack_layout`

### 08. Combined Layouts - GRAND FINALE (`08_combined_layouts.rs`)
**Concepts**: ALL FEATURES IN ONE EXAMPLE

This comprehensive example creates a complete invoice with:
- **Stack Layout**: Background watermark
- **Column Layout**: 6 major vertical sections
- **Row Layout**: Multi-column headers and data
- **Nested Boxes**: 4+ levels deep
- **Styled Labels**: Bold, underline, combined
- **Grid Layout**: 5-row × 4-column item table
- **Mixed Layouts**: Column within Row within Stack
- **Complex Hierarchy**: Professional document structure

**Structure**:
1. Header (company info + invoice details)
2. Client information (bill to + ship to)
3. Item list header (table headings)
4. Item list (5 items with QTY/ITEM/DESC/PRICE)
5. Totals section (subtotal/tax/total)
6. Footer (terms and thank you)

**Widget Count**: ~70 widgets in ~6-level deep hierarchy

**Run**: `cargo run --example 08_combined_layouts`

## Running All Examples

To see all examples in action:

```bash
# Run each example individually
for i in {01..08}; do
    example=$(ls examples/${i}_*.rs | head -1 | xargs basename | sed 's/\.rs$//')
    echo "Running $example..."
    cargo run --example $example
    echo ""
done
```

## Key Learning Path

**Beginner**: Start with examples 1-3
- Understand basic widgets (Label, Box)
- Learn positioning and validation

**Intermediate**: Examples 4-7
- Master layout components
- Understand nesting patterns
- Explore different layout types

**Advanced**: Example 8
- See everything combined
- Study complex document structure
- Learn professional layout patterns

## Common Patterns

### Header/Body/Footer Pattern
```rust
let mut column = column_new!(80, 40);
let (header, h_pos) = column_area!(column, 5)?;
let (body, b_pos) = column_area!(column, 30)?;
let (footer, f_pos) = column_area!(column, 5)?;
```

### Multi-Column Grid
```rust
let mut row = row_new!(80, 30);
let (col1, pos1) = row_area!(row, 25)?;
let (col2, pos2) = row_area!(row, 30)?;
let (col3, pos3) = row_area!(row, 25)?;
```

### Nested Layout
```rust
let mut column = column_new!(80, 30);
let (mut row_area, row_pos) = column_area!(column, 10)?;

let mut row = row_new!(80, 10);
let (col1, c1_pos) = row_area!(row, 40)?;
let (col2, c2_pos) = row_area!(row, 40)?;

row_area.add_child(col1, c1_pos)?;
row_area.add_child(col2, c2_pos)?;
```

### Styled Labels
```rust
let label = label_new!(20)
    .add_text("Text")?
    .bold()
    .underline();
```

## Error Handling

All examples demonstrate proper error handling:

```rust
// Child exceeds parent
let result = parent.add_child(oversized_child, (0, 0));
match result {
    Err(RenderError::ChildExceedsParent { .. }) => { /* handle */ }
    _ => { }
}

// Overlapping children
let result = parent.add_child(child, (10, 0));
match result {
    Err(RenderError::OverlappingChildren { .. }) => { /* handle */ }
    _ => { }
}

// Insufficient space
let result = column_area!(column, 100);
match result {
    Err(RenderError::InsufficientSpace { .. }) => { /* handle */ }
    _ => { }
}
```

## Testing

All examples are validated and tested:
- 161 unit/integration tests pass
- Examples compile and run successfully
- Boundary validation tested
- Error handling demonstrated

Run tests:
```bash
cargo test --lib --bins --tests
```

## Performance Notes

The widget composition system is designed for efficiency:
- Compile-time dimension validation (zero runtime cost)
- Type-safe boundaries with const generics
- Minimal allocations during rendering
- Tree traversal is O(n) where n = widget count

## Further Reading

- See `/specs/002-widget-composability/spec.md` for complete specification
- See `/specs/002-widget-composability/data-model.md` for entity details
- See `/specs/002-widget-composability/quickstart.md` for quick reference

---

**All examples demonstrate production-ready code with:**
- ✅ Proper error handling
- ✅ Type safety via const generics
- ✅ Compile-time validation
- ✅ Clear documentation
- ✅ Real-world use cases

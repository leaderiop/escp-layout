# Quickstart Guide: Rust ESC/P Layout Engine

**Branch**: `001-rust-escap-layout-engine` | **Date**: 2025-11-18

## Goal

Get from zero to printing your first document in **under 5 minutes**.

---

## Prerequisites

- Rust 1.75+ installed (`rustc --version` to check)
- Basic familiarity with Rust syntax
- (Optional) EPSON LQ-2090II or compatible printer for hardware testing

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
escp-layout = "1.0"
```

Or use `cargo add`:

```bash
cargo add escp-layout
```

---

## Hello World

**Goal**: Create a single-page document with "Hello, World!" and render to bytes.

```rust
use escp_layout::*;

fn main() {
    // 1. Create a page
    let page = Page::builder()
        .write_str(0, 0, "Hello, World!", StyleFlags::BOLD)
        .build();

    // 2. Create a document
    let doc = Document::builder()
        .add_page(page)
        .build();

    // 3. Render to ESC/P bytes
    let bytes = doc.render();

    // 4. Save to file or send to printer
    std::fs::write("output.prn", &bytes).unwrap();

    println!("Rendered {} bytes to output.prn", bytes.len());
}
```

**Run**:
```bash
cargo run
```

**Output**:
```
Rendered 8266 bytes to output.prn
```

**Send to printer** (Linux/macOS):
```bash
cat output.prn > /dev/usb/lp0  # Or your printer device
```

---

## Understanding the API

### The Builder Pattern

All objects use builders for construction:

```rust
Page::builder()          // Create mutable PageBuilder
    .write_str(...)      // Add content (chainable)
    .build()             // Finalize to immutable Page

Document::builder()      // Create mutable DocumentBuilder
    .add_page(...)       // Add pages (chainable)
    .build()             // Finalize to immutable Document
```

**Why**: Ensures pages and documents are immutable after creation, preventing accidental modifications.

---

### Coordinates and Regions

**Page Dimensions**: 160 columns × 51 lines (0-indexed)

```rust
// Full page region
let full_page = Region::full_page();  // x=0, y=0, width=160, height=51

// Custom region
let header = Region::new(0, 0, 160, 5)?;  // Top 5 lines
let body = Region::new(0, 5, 160, 46)?;   // Remaining 46 lines

// Split regions
let (left, right) = body.split_horizontal(80)?;  // 80 chars left, 80 right
let (top, bottom) = body.split_vertical(20)?;    // 20 lines top, 26 bottom
```

**Validation**: `Region::new()` returns `Result` - validates coordinates fit in 160×51 bounds.

---

### Writing Text

**Direct writes**:
```rust
page.write_at(x, y, 'A', StyleFlags::BOLD);      // Single character
page.write_str(x, y, "Hello", StyleFlags::NONE); // String
```

**Silent truncation**: Coordinates outside bounds are ignored (no panic).

---

### Text Styles

```rust
StyleFlags::NONE                                  // No styles
StyleFlags::BOLD                                  // Bold only
StyleFlags::UNDERLINE                             // Underline only
StyleFlags::BOLD.with_underline(true)            // Both bold and underline
```

---

## Common Patterns

### Pattern 1: Simple Invoice Header

```rust
let page = Page::builder()
    .write_str(0, 0, "INVOICE #12345", StyleFlags::BOLD)
    .write_str(0, 1, "Date: 2025-11-18", StyleFlags::NONE)
    .fill_region(Region::new(0, 2, 160, 1)?, '-', StyleFlags::NONE)
    .build();
```

---

### Pattern 2: Using Widgets

Widgets render complex content into regions:

```rust
// Table widget
let table = Table::new(
    vec![
        ColumnDef { name: "Item".into(), width: 80 },
        ColumnDef { name: "Qty".into(), width: 20 },
        ColumnDef { name: "Price".into(), width: 30 },
    ],
    vec![
        vec!["Widget A".into(), "5".into(), "$10.00".into()],
        vec!["Widget B".into(), "3".into(), "$15.00".into()],
    ],
);

let page = Page::builder()
    .render_widget(Region::new(0, 5, 130, 40)?, &table)
    .build();
```

**Available widgets**:
- `Label`: Single-line text
- `TextBlock`: Multi-line text (no wrapping)
- `Paragraph`: Multi-line text with word wrapping
- `ASCIIBox`: Bordered box with optional title
- `KeyValueList`: Aligned key-value pairs
- `Table`: Fixed-column tables

---

### Pattern 3: Multi-Page Document

```rust
let mut builder = DocumentBuilder::new();

for i in 0..10 {
    let page = Page::builder()
        .write_str(0, 0, &format!("Page {}", i + 1), StyleFlags::BOLD)
        .build();

    builder.add_page(page);
}

let doc = builder.build();
```

---

### Pattern 4: Complex Layout (Nested Regions)

```rust
let full_page = Region::full_page();

// Split into header/body/footer
let (header, rest) = full_page.split_vertical(5)?;
let (body, footer) = rest.split_vertical(41)?;

// Split body into sidebar and main
let (sidebar, main) = body.split_horizontal(40)?;

let page = Page::builder()
    // Header
    .render_widget(header, &Label::new("REPORT TITLE")
        .with_style(StyleFlags::BOLD))

    // Sidebar
    .render_widget(sidebar, &KeyValueList::new(vec![
        ("Date".into(), "2025-11-18".into()),
        ("Author".into(), "System".into()),
    ]))

    // Main content
    .render_widget(main, &Paragraph::new(
        "This is the main content area..."
    ))

    // Footer
    .render_widget(footer, &Label::new("Page 1"))

    .build();
```

---

## Widget Examples

### Label

```rust
let label = Label::new("Hello")
    .with_style(StyleFlags::BOLD);

page.render_widget(region, &label);
```

---

### TextBlock

```rust
let text = TextBlock::from_text("Line 1\nLine 2\nLine 3");
page.render_widget(region, &text);
```

---

### Paragraph (with wrapping)

```rust
let para = Paragraph::new(
    "This is a long paragraph that will wrap across multiple lines \
     when rendered into a region that is narrower than the text."
).with_style(StyleFlags::NONE);

page.render_widget(region, &para);
```

---

### ASCIIBox

```rust
let content = Label::new("Inside the box");

let boxed = ASCIIBox::new(Box::new(content))
    .with_title("Section Title");

page.render_widget(region, &boxed);
```

**Output**:
```
+--Section Title-------+
|Inside the box       |
|                     |
+---------------------+
```

---

### KeyValueList

```rust
let kv_list = KeyValueList::new(vec![
    ("Name".into(), "John Doe".into()),
    ("ID".into(), "12345".into()),
    ("Status".into(), "Active".into()),
]).with_separator(": ");

page.render_widget(region, &kv_list);
```

**Output**:
```
Name: John Doe
ID: 12345
Status: Active
```

---

### Table

```rust
let table = Table::new(
    vec![
        ColumnDef { name: "Product".into(), width: 40 },
        ColumnDef { name: "Quantity".into(), width: 15 },
        ColumnDef { name: "Price".into(), width: 20 },
    ],
    vec![
        vec!["Widget A".into(), "5".into(), "$50.00".into()],
        vec!["Widget B".into(), "3".into(), "$45.00".into()],
    ],
);

page.render_widget(region, &table);
```

**Output**:
```
Product                                  Quantity       Price
Widget A                                 5              $50.00
Widget B                                 3              $45.00
```

---

## Error Handling

Operations that can fail return `Result<T, LayoutError>`:

```rust
match Region::new(200, 0, 10, 10) {
    Ok(region) => { /* use region */ },
    Err(LayoutError::RegionOutOfBounds) => {
        eprintln!("Region exceeds page bounds");
    },
    Err(e) => {
        eprintln!("Error: {}", e);
    },
}
```

**Common errors**:
- `RegionOutOfBounds`: Coordinates exceed 160×51
- `InvalidDimensions`: Width/height calculation underflow
- `InvalidSplit`: Split dimensions exceed parent

**Note**: Content overflow (truncation) is **not** an error - it's handled silently.

---

## Complete Example: Invoice

```rust
use escp_layout::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let full_page = Region::full_page();

    // Layout regions
    let (header, rest) = full_page.split_vertical(5)?;
    let (items_area, rest) = rest.split_vertical(40)?;
    let (_, footer) = rest.split_vertical(5)?;

    // Create invoice table
    let items = Table::new(
        vec![
            ColumnDef { name: "Description".into(), width: 80 },
            ColumnDef { name: "Qty".into(), width: 15 },
            ColumnDef { name: "Unit Price".into(), width: 25 },
            ColumnDef { name: "Total".into(), width: 25 },
        ],
        vec![
            vec!["Premium Widget".into(), "10".into(), "$25.00".into(), "$250.00".into()],
            vec!["Standard Widget".into(), "5".into(), "$15.00".into(), "$75.00".into()],
            vec!["Basic Widget".into(), "20".into(), "$5.00".into(), "$100.00".into()],
        ],
    );

    // Build page
    let page = Page::builder()
        // Header
        .write_str(0, 0, "ACME CORPORATION", StyleFlags::BOLD)
        .write_str(0, 1, "Invoice #INV-2025-001", StyleFlags::NONE)
        .write_str(0, 2, "Date: 2025-11-18", StyleFlags::NONE)
        .fill_region(Region::new(0, 3, 160, 1)?, '=', StyleFlags::NONE)

        // Items table
        .render_widget(items_area, &items)

        // Footer totals
        .write_str(0, 47, "Subtotal:  $425.00", StyleFlags::NONE)
        .write_str(0, 48, "Tax (10%): $42.50", StyleFlags::NONE)
        .write_str(0, 49, "TOTAL:     $467.50", StyleFlags::BOLD)

        .build();

    // Create and render document
    let doc = Document::builder()
        .add_page(page)
        .build();

    let bytes = doc.render();

    // Save to file
    std::fs::write("invoice.prn", &bytes)?;

    println!("✓ Invoice rendered ({} bytes)", bytes.len());
    println!("✓ Saved to invoice.prn");
    println!("\nTo print: cat invoice.prn > /dev/usb/lp0");

    Ok(())
}
```

---

## Output to File vs Printer

### Save to File

```rust
let bytes = doc.render();
std::fs::write("output.prn", &bytes)?;
```

---

### Send to Printer (Unix)

```bash
# Find printer device
ls /dev/usb/lp*

# Send file to printer
cat output.prn > /dev/usb/lp0
```

---

### Send to Printer (Rust)

```rust
use std::fs::OpenOptions;
use std::io::Write;

let bytes = doc.render();

let mut printer = OpenOptions::new()
    .write(true)
    .open("/dev/usb/lp0")?;

printer.write_all(&bytes)?;
```

---

## Next Steps

1. **Read contracts**: See `contracts/public-api.md` for complete API reference
2. **Explore widgets**: Try all 6 widget types
3. **Test layouts**: Experiment with region splits and nesting
4. **Multi-page documents**: Generate reports with 10+ pages
5. **Hardware testing**: Print on physical EPSON LQ-2090II

---

## Common Pitfalls

### Pitfall 1: Forgetting `?` on Region creation

**❌ Wrong**:
```rust
let region = Region::new(0, 0, 200, 60);  // Compile error: Result not handled
```

**✅ Correct**:
```rust
let region = Region::new(0, 0, 200, 60)?;  // Returns error if out of bounds
```

---

### Pitfall 2: Expecting errors on overflow

**❌ Wrong assumption**:
```rust
// This does NOT return an error - it silently truncates
page.write_str(150, 0, "This text is too long...", StyleFlags::NONE);
```

**✅ Correct understanding**:
- Overflow is **not an error** (by design)
- Text exceeding region boundaries is silently truncated
- This is intentional for predictable behavior

---

### Pitfall 3: Modifying after finalization

**❌ Wrong (won't compile)**:
```rust
let page = Page::builder().build();
page.write_at(0, 0, 'A', StyleFlags::NONE);  // Error: page is immutable
```

**✅ Correct**:
```rust
let page = Page::builder()
    .write_at(0, 0, 'A', StyleFlags::NONE)  // Write before build()
    .build();
```

---

## Performance Tips

1. **Pre-allocate documents**: Use `DocumentBuilder` capacity hint if available (future enhancement)
2. **Reuse regions**: Region is `Copy`, so reuse freely without allocation
3. **Batch writes**: Use `write_str()` instead of multiple `write_at()` calls
4. **Minimize style changes**: Adjacent cells with same style reduce ESC/P overhead

---

## Troubleshooting

### Problem: "Region out of bounds" error

**Cause**: Coordinates exceed 160×51 page size

**Solution**: Check region calculations:
```rust
// Debug print region
println!("Region: x={}, y={}, width={}, height={}", r.x, r.y, r.width, r.height);
println!("End: x={}, y={}", r.x + r.width, r.y + r.height);

assert!(r.x + r.width <= 160);
assert!(r.y + r.height <= 51);
```

---

### Problem: Text not appearing

**Possible causes**:
1. Text written outside region bounds (silently truncated)
2. Text color same as background (use styles to debug)
3. Empty cells being rendered as spaces

**Debug**:
```rust
// Check if cell was written
let cell = page.get_cell(x, y).unwrap();
println!("Cell at ({}, {}): char={}, bold={}",
    x, y, cell.character as char, cell.style.bold());
```

---

### Problem: Printer not responding

**Checklist**:
1. Verify printer is on and online
2. Check cable connection
3. Verify condensed mode is supported (LQ-2090II required)
4. Try printing a test page from printer's self-test menu
5. Verify byte stream format: `hexdump -C output.prn | head`

---

## Resources

- **API Reference**: `contracts/public-api.md`
- **ESC/P Format**: `contracts/escp-output-spec.md`
- **Data Model**: `data-model.md`
- **Research**: `research.md`
- **Project Constitution**: `.specify/memory/constitution.md`

---

## Support

For issues, questions, or feature requests:
- **GitHub**: (Repository URL placeholder)
- **Email**: (Contact placeholder)

---

**You're ready to start building! 🚀**

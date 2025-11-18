# Public API Contract: Rust ESC/P Layout Engine

**Branch**: `001-rust-escap-layout-engine` | **Date**: 2025-11-18

## Overview

This document defines the complete public API surface of the `escp-layout` Rust library. All functions, types, and traits listed here are part of the stable V1 API and follow Semantic Versioning 2.0.0.

---

## Module: `escp_layout`

Root module exposing all public types.

### Re-exports

```rust
pub use cell::{Cell, StyleFlags};
pub use page::{Page, PageBuilder};
pub use document::{Document, DocumentBuilder};
pub use region::Region;
pub use widgets::{
    Widget,
    Label, TextBlock, Paragraph,
    ASCIIBox, KeyValueList, Table, ColumnDef
};
pub use error::LayoutError;
```

---

## Module: `escp_layout::cell`

Types for representing individual character cells.

### Type: `Cell`

Represents a single character with style information.

**Signature**:
```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub character: u8,
    pub style: StyleFlags,
}
```

**Constructor**:
```rust
impl Cell {
    pub const EMPTY: Cell;
    pub fn new(ch: char, style: StyleFlags) -> Cell;
}
```

**Contract**:
- `new()` converts non-ASCII characters to `'?'`
- `new()` converts control characters (< 32) to `EMPTY` (character = 0)
- `EMPTY` represents an unoccupied cell

**Example**:
```rust
let cell = Cell::new('A', StyleFlags::BOLD);
assert_eq!(cell.character, b'A');
assert!(cell.style.bold());
```

---

### Type: `StyleFlags`

Bit-packed text style flags.

**Signature**:
```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StyleFlags(u8);
```

**Constants**:
```rust
impl StyleFlags {
    pub const NONE: StyleFlags;
    pub const BOLD: StyleFlags;
    pub const UNDERLINE: StyleFlags;
}
```

**Methods**:
```rust
impl StyleFlags {
    pub fn bold(self) -> bool;
    pub fn underline(self) -> bool;
    pub fn with_bold(self, enabled: bool) -> StyleFlags;
    pub fn with_underline(self, enabled: bool) -> StyleFlags;
}
```

**Contract**:
- All flag combinations are valid
- Methods are pure (no side effects)

**Example**:
```rust
let style = StyleFlags::NONE
    .with_bold(true)
    .with_underline(true);

assert!(style.bold() && style.underline());
```

---

## Module: `escp_layout::region`

Types for defining rectangular page regions.

### Type: `Region`

Rectangular area within a page.

**Signature**:
```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Region {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
```

**Constants**:
```rust
impl Region {
    pub const PAGE_WIDTH: u16 = 160;
    pub const PAGE_HEIGHT: u16 = 51;
}
```

**Constructor**:
```rust
impl Region {
    pub fn new(x: u16, y: u16, width: u16, height: u16)
        -> Result<Self, LayoutError>;
    pub fn full_page() -> Self;
}
```

**Operations**:
```rust
impl Region {
    pub fn split_vertical(&self, top_height: u16)
        -> Result<(Region, Region), LayoutError>;

    pub fn split_horizontal(&self, left_width: u16)
        -> Result<(Region, Region), LayoutError>;

    pub fn with_padding(&self, top: u16, right: u16, bottom: u16, left: u16)
        -> Result<Region, LayoutError>;
}
```

**Contract**:
- `new()` validates `x + width <= 160` and `y + height <= 51`
- `split_*()` methods validate split dimensions fit within parent
- `with_padding()` validates padding doesn't exceed region size
- Zero-width or zero-height regions are valid

**Errors**:
- Returns `LayoutError::RegionOutOfBounds` if coordinates exceed page bounds
- Returns `LayoutError::InvalidSplit` if split dimensions invalid
- Returns `LayoutError::InvalidDimensions` if padding calculation underflows

**Example**:
```rust
let page_region = Region::full_page();
let (header, body) = page_region.split_vertical(10)?;
let (left, right) = body.split_horizontal(80)?;

assert_eq!(header.height, 10);
assert_eq!(left.width, 80);
```

---

## Module: `escp_layout::page`

Types for constructing and representing pages.

### Type: `Page`

Immutable 160×51 character grid.

**Signature**:
```rust
pub struct Page { /* private */ }
```

**Constructor**:
```rust
impl Page {
    pub fn builder() -> PageBuilder;
}
```

**Accessors**:
```rust
impl Page {
    pub fn get_cell(&self, x: u16, y: u16) -> Option<Cell>;
    pub fn cells(&self) -> &[[Cell; 160]; 51];
}
```

**Contract**:
- Immutable after construction (no mutable methods)
- `get_cell()` returns `None` for out-of-bounds coordinates
- `cells()` provides read-only access to entire grid

**Traits**:
- `Debug`, `Clone`

**Example**:
```rust
let page = Page::builder()
    .write_str(0, 0, "Hello", StyleFlags::BOLD)
    .build();

assert_eq!(page.get_cell(0, 0).unwrap().character, b'H');
```

---

### Type: `PageBuilder`

Mutable builder for constructing pages.

**Signature**:
```rust
pub struct PageBuilder { /* private */ }
```

**Constructor**:
```rust
impl PageBuilder {
    pub fn new() -> Self;
}
```

**Write Operations** (chainable):
```rust
impl PageBuilder {
    pub fn write_at(&mut self, x: u16, y: u16, ch: char, style: StyleFlags)
        -> &mut Self;

    pub fn write_str(&mut self, x: u16, y: u16, text: &str, style: StyleFlags)
        -> &mut Self;

    pub fn fill_region(&mut self, region: Region, ch: char, style: StyleFlags)
        -> &mut Self;

    pub fn render_widget(&mut self, region: Region, widget: &dyn Widget)
        -> &mut Self;
}
```

**Finalization**:
```rust
impl PageBuilder {
    pub fn build(self) -> Page;
}
```

**Contract**:
- All write operations silently truncate out-of-bounds coordinates (no panic)
- Multiple writes to same cell: last write wins
- `build()` consumes builder, returns immutable `Page`
- Builder cannot be reused after `build()`

**Example**:
```rust
let page = PageBuilder::new()
    .write_str(0, 0, "Invoice #12345", StyleFlags::BOLD)
    .fill_region(Region::new(0, 1, 160, 1)?, '-', StyleFlags::NONE)
    .render_widget(Region::new(0, 2, 160, 48)?, &my_table)
    .build();
```

---

## Module: `escp_layout::document`

Types for multi-page documents.

### Type: `Document`

Immutable collection of pages.

**Signature**:
```rust
pub struct Document { /* private */ }
```

**Constructor**:
```rust
impl Document {
    pub fn builder() -> DocumentBuilder;
}
```

**Accessors**:
```rust
impl Document {
    pub fn pages(&self) -> &[Page];
    pub fn page_count(&self) -> usize;
}
```

**Rendering**:
```rust
impl Document {
    pub fn render(&self) -> Vec<u8>;
}
```

**Contract**:
- Immutable after construction
- `render()` produces ESC/P byte stream
- `render()` is deterministic (same document → same bytes)
- Empty documents (0 pages) are valid

**Traits**:
- `Debug`, `Clone`

**Example**:
```rust
let doc = Document::builder()
    .add_page(page1)
    .add_page(page2)
    .build();

let escp_bytes = doc.render();
// Send to printer or save to file
```

---

### Type: `DocumentBuilder`

Mutable builder for constructing documents.

**Signature**:
```rust
pub struct DocumentBuilder { /* private */ }
```

**Constructor**:
```rust
impl DocumentBuilder {
    pub fn new() -> Self;
}
```

**Operations** (chainable):
```rust
impl DocumentBuilder {
    pub fn add_page(&mut self, page: Page) -> &mut Self;
}
```

**Finalization**:
```rust
impl DocumentBuilder {
    pub fn build(self) -> Document;
}
```

**Contract**:
- Pages must be finalized before adding (type-safe)
- Pages added in order they will be rendered
- `build()` consumes builder, returns immutable `Document`

**Example**:
```rust
let mut builder = DocumentBuilder::new();

for i in 0..10 {
    let page = create_report_page(i);
    builder.add_page(page);
}

let doc = builder.build();
```

---

## Module: `escp_layout::widgets`

Built-in widget types and trait.

### Trait: `Widget`

Common interface for renderable content.

**Signature**:
```rust
pub trait Widget {
    fn render(&self, page: &mut PageBuilder, region: Region);
}
```

**Contract**:
- MUST NOT write outside region boundaries
- MUST handle zero-size regions gracefully (render nothing)
- MUST truncate content exceeding region size
- MUST NOT panic for any inputs

---

### Type: `Label`

Single-line text widget.

**Signature**:
```rust
pub struct Label { /* private */ }
```

**Constructor**:
```rust
impl Label {
    pub fn new(text: impl Into<String>) -> Self;
    pub fn with_style(self, style: StyleFlags) -> Self;
}
```

**Behavior**:
- Renders on first line of region
- Truncates if text exceeds region width
- Ignores region height (only uses line 0)

**Example**:
```rust
let label = Label::new("Product Name")
    .with_style(StyleFlags::BOLD);

page.render_widget(region, &label);
```

---

### Type: `TextBlock`

Multi-line text without word wrapping.

**Signature**:
```rust
pub struct TextBlock { /* private */ }
```

**Constructor**:
```rust
impl TextBlock {
    pub fn new(lines: Vec<String>) -> Self;
    pub fn from_text(text: impl Into<String>) -> Self;
}
```

**Behavior**:
- One line per string in `lines`
- `from_text()` splits on `\n`
- Truncates lines exceeding width
- Truncates lines exceeding height
- No word wrapping

**Example**:
```rust
let text = TextBlock::from_text("Line 1\nLine 2\nLine 3");
page.render_widget(region, &text);
```

---

### Type: `Paragraph`

Multi-line text with word wrapping.

**Signature**:
```rust
pub struct Paragraph { /* private */ }
```

**Constructor**:
```rust
impl Paragraph {
    pub fn new(text: impl Into<String>) -> Self;
    pub fn with_style(self, style: StyleFlags) -> Self;
}
```

**Behavior**:
- Wraps at word boundaries
- Breaks words if single word exceeds width
- Truncates lines exceeding region height
- Preserves spaces between words

**Example**:
```rust
let para = Paragraph::new("This is a long paragraph that will wrap across multiple lines.")
    .with_style(StyleFlags::NONE);

page.render_widget(region, &para);
```

---

### Type: `ASCIIBox`

Bordered box with optional title.

**Signature**:
```rust
pub struct ASCIIBox { /* private */ }
```

**Constructor**:
```rust
impl ASCIIBox {
    pub fn new(content: Box<dyn Widget>) -> Self;
    pub fn with_title(self, title: impl Into<String>) -> Self;
}
```

**Behavior**:
- Draws border using `+`, `-`, `|` characters
- Title rendered in top border if present
- Content rendered in inset region (1-cell padding)
- Requires minimum 3×3 region

**Example**:
```rust
let boxed = ASCIIBox::new(Box::new(Label::new("Content")))
    .with_title("Section Title");

page.render_widget(region, &boxed);
```

---

### Type: `KeyValueList`

Vertically aligned key-value pairs.

**Signature**:
```rust
pub struct KeyValueList { /* private */ }
```

**Constructor**:
```rust
impl KeyValueList {
    pub fn new(entries: Vec<(String, String)>) -> Self;
    pub fn with_separator(self, separator: impl Into<String>) -> Self;
}
```

**Behavior**:
- One entry per line
- Default separator: `": "`
- Truncates entry if exceeds width
- Truncates entries if exceed height

**Example**:
```rust
let kv_list = KeyValueList::new(vec![
    ("Name".into(), "John Doe".into()),
    ("ID".into(), "12345".into()),
    ("Status".into(), "Active".into()),
]);

page.render_widget(region, &kv_list);
```

---

### Type: `Table`

Fixed-column tabular data.

**Signature**:
```rust
pub struct Table { /* private */ }
```

**Constructor**:
```rust
impl Table {
    pub fn new(columns: Vec<ColumnDef>, rows: Vec<Vec<String>>) -> Self;
}
```

**Behavior**:
- First line renders column headers (bold)
- Subsequent lines render rows
- Cells truncated to column width
- Rows truncated if exceed height
- Left-aligned by default

**Example**:
```rust
let table = Table::new(
    vec![
        ColumnDef { name: "Item".into(), width: 40 },
        ColumnDef { name: "Qty".into(), width: 10 },
        ColumnDef { name: "Price".into(), width: 15 },
    ],
    vec![
        vec!["Widget A".into(), "5".into(), "$10.00".into()],
        vec!["Widget B".into(), "3".into(), "$15.00".into()],
    ],
);

page.render_widget(region, &table);
```

---

### Type: `ColumnDef`

Column definition for tables.

**Signature**:
```rust
#[derive(Clone, Debug)]
pub struct ColumnDef {
    pub name: String,
    pub width: u16,
}
```

**Usage**: See `Table` example above.

---

## Module: `escp_layout::error`

Error types.

### Type: `LayoutError`

Recoverable layout construction errors.

**Signature**:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    RegionOutOfBounds,
    InvalidDimensions,
    InvalidSplit,
}
```

**Traits**:
- `Display`, `Error`, `Debug`, `Clone`, `PartialEq`, `Eq`

**Variants**:
- `RegionOutOfBounds`: Region coordinates exceed page bounds (160×51)
- `InvalidDimensions`: Width, height, or padding calculation invalid
- `InvalidSplit`: Split dimensions exceed parent region

**Example**:
```rust
match Region::new(200, 0, 10, 10) {
    Ok(region) => { /* use region */ },
    Err(LayoutError::RegionOutOfBounds) => {
        eprintln!("Region exceeds page bounds");
    },
    Err(e) => { /* other errors */ },
}
```

---

## API Stability Guarantees

### V1.x.x Series

**Guaranteed**:
- No breaking changes to public API
- ESC/P output format remains byte-compatible
- Deterministic rendering guarantees maintained
- Error types and semantics unchanged

**Allowed**:
- New widget types added
- New optional methods on existing types
- Performance optimizations (no behavior changes)
- Bug fixes that don't change output

**Deprecated APIs**:
- Will emit warnings for ≥ 1 minor version before removal
- Removal requires MAJOR version bump

---

## Thread Safety

**Send + Sync Types**:
- `Cell`, `StyleFlags`, `Region` (Copy types, inherently thread-safe)
- `Page`, `Document` (immutable, safe to share across threads)

**Not Send/Sync**:
- `PageBuilder`, `DocumentBuilder` (mutable, not intended for concurrent use)

**Usage**:
```rust
let doc: Document = /* ... */;

// Safe: Document is Send + Sync
std::thread::spawn(move || {
    let bytes = doc.render();
    // Send bytes to printer
});
```

---

## Performance Characteristics

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| `Cell::new()` | O(1) | Constant time |
| `Region::new()` | O(1) | Bounds checking only |
| `PageBuilder::write_at()` | O(1) | Direct array indexing |
| `PageBuilder::write_str()` | O(n) | n = string length |
| `PageBuilder::build()` | O(1) | Moves data, no copy |
| `DocumentBuilder::add_page()` | O(1) amortized | Vec push |
| `Document::render()` | O(p × 160 × 51) | p = page count, linear in total cells |
| Widget rendering | O(region area) | Varies by widget type |

---

## Memory Usage

| Type | Approximate Size |
|------|------------------|
| `Cell` | 2 bytes |
| `StyleFlags` | 1 byte |
| `Region` | 8 bytes |
| `Page` | ~16 KB (8,160 cells × 2 bytes) |
| `Document` | 8 bytes + page size × page count |

---

## Examples

### Minimal Example (< 10 lines)

```rust
use escp_layout::*;

let page = Page::builder()
    .write_str(0, 0, "Hello, Printer!", StyleFlags::BOLD)
    .build();

let doc = Document::builder()
    .add_page(page)
    .build();

let bytes = doc.render(); // Ready to send to printer
```

### Invoice Example

```rust
use escp_layout::*;

fn create_invoice() -> Document {
    let page = Page::builder()
        // Header
        .write_str(0, 0, "INVOICE #12345", StyleFlags::BOLD)
        .write_str(0, 1, "Date: 2025-11-18", StyleFlags::NONE)
        .fill_region(Region::new(0, 2, 160, 1).unwrap(), '-', StyleFlags::NONE)

        // Items table
        .render_widget(
            Region::new(0, 3, 160, 40).unwrap(),
            &Table::new(
                vec![
                    ColumnDef { name: "Item".into(), width: 80 },
                    ColumnDef { name: "Qty".into(), width: 20 },
                    ColumnDef { name: "Price".into(), width: 30 },
                    ColumnDef { name: "Total".into(), width: 30 },
                ],
                vec![
                    vec!["Widget A".into(), "5".into(), "$10.00".into(), "$50.00".into()],
                    vec!["Widget B".into(), "3".into(), "$15.00".into(), "$45.00".into()],
                ],
            ),
        )

        // Footer
        .write_str(0, 50, "Total: $95.00", StyleFlags::BOLD | StyleFlags::UNDERLINE)
        .build();

    Document::builder().add_page(page).build()
}
```

### Complex Layout Example

```rust
use escp_layout::*;

fn create_report_page() -> Page {
    let full_page = Region::full_page();

    // Split into header/body/footer
    let (header, rest) = full_page.split_vertical(5).unwrap();
    let (body, footer) = rest.split_vertical(41).unwrap();

    // Split body into sidebar and main
    let (sidebar, main) = body.split_horizontal(40).unwrap();

    let mut page = PageBuilder::new();

    // Render header
    page.render_widget(&header, &ASCIIBox::new(
        Box::new(Label::new("Monthly Report"))
    ).with_title("Header"));

    // Render sidebar
    page.render_widget(&sidebar, &KeyValueList::new(vec![
        ("Date".into(), "2025-11-18".into()),
        ("Author".into(), "System".into()),
    ]));

    // Render main content
    page.render_widget(&main, &Paragraph::new(
        "This is the main content area with wrapped text..."
    ));

    // Render footer
    page.render_widget(&footer, &Label::new("Page 1 of 1")
        .with_style(StyleFlags::NONE));

    page.build()
}
```

---

## Version History

- **V1.0.0** (Initial release): Complete V1 feature set as specified
- **V1.x.x** (Future): Bug fixes, performance improvements, new widgets (backward compatible)
- **V2.0.0** (Future): Dynamic layout, auto-pagination, breaking changes

---

**API Contract Document Complete**

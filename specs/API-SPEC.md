📚 API SPECIFICATION

# EPSON LQ-2090II Rust Layout Engine — V1

**Public API Reference**

---

## Document Control

| Field | Value |
|-------|-------|
| **Document Version** | 1.0 |
| **Product Version** | V1.0 |
| **Author** | Mohammad AlMechkor |
| **Status** | Draft |
| **Classification** | Public - API Documentation |
| **Date Created** | 2025-01-18 |
| **Last Updated** | 2025-01-18 |

### Related Documents

- **Product Requirements Document (PRD)**: `PRD.md` (v1.1)
- **Technical Design Document (TDD)**: `TDD.md` (v1.0)
- **User Guide**: [To be created]

### API Stability

- **Version**: 1.0.0
- **Stability Level**: Stable (SemVer 2.0.0)
- **MSRV**: Rust 1.75.0
- **Breaking Changes**: Require major version bump

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Installation](#2-installation)
3. [Quick Start](#3-quick-start)
4. [Core Types](#4-core-types)
5. [Builder APIs](#5-builder-apis)
6. [Widget APIs](#6-widget-apis)
7. [Error Handling](#7-error-handling)
8. [Complete Examples](#8-complete-examples)
9. [API Contracts & Guarantees](#9-api-contracts--guarantees)
10. [Performance Characteristics](#10-performance-characteristics)
11. [Migration Guide](#11-migration-guide)
12. [Appendices](#12-appendices)

---

## 1. Introduction

### 1.1 Overview

The EPSON LQ-2090II Rust Layout Engine provides a type-safe, deterministic API for generating ESC/P documents for dot-matrix printers. This API specification defines all public types, methods, and contracts.

### 1.2 Design Principles

- **Type Safety**: Lifetimes prevent dangling references at compile time
- **Ergonomics**: Fluent builder API with method chaining
- **Determinism**: Same input always produces identical ESC/P output
- **No Panics**: All errors return `Result<T, LayoutError>`
- **Zero Cost**: Builder pattern compiles to optimal code

### 1.3 Audience

This document is for:
- Application developers integrating the library
- API users writing invoices, forms, and documents
- Library maintainers documenting changes

---

## 2. Installation

### 2.1 Adding the Dependency

**Cargo.toml**:
```toml
[dependencies]
epson-lq2090-layout = "1.0"
```

### 2.2 Optional Features

```toml
[dependencies]
epson-lq2090-layout = { version = "1.0", features = ["serde"] }
```

**Available Features**:
- `serde`: Enable serialization/deserialization support

### 2.3 MSRV

Minimum Supported Rust Version: **1.75.0**

---

## 3. Quick Start

### 3.1 Hello World

```rust
use epson_lq2090_layout::*;

fn main() {
    // Create a document builder
    let mut doc_builder = DocumentBuilder::new();

    // Add a page
    let mut page_builder = doc_builder.add_page();

    // Get the root region (160×51)
    let mut root = page_builder.root_region();

    // Write text
    root.write_text(0, 0, "Hello, EPSON LQ-2090II!", Style::BOLD);

    // Finalize and build
    page_builder.finalize().unwrap();
    let document = doc_builder.build();

    // Render to ESC/P bytes
    let escp_bytes = document.render();

    // Send to printer (example using std::fs)
    std::fs::write("/dev/usb/lp0", escp_bytes).unwrap();
}
```

### 3.2 Simple Invoice

```rust
use epson_lq2090_layout::*;

fn main() {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();
    let mut root = page.root_region();

    // Split into header, body, footer
    let mut regions = root.split_vertical(&[10, 35, 6]).unwrap();

    // Header
    regions[0].label("INVOICE", Alignment::Center)
               .with_style(Style::BOLD);

    // Body
    let mut table = Table::new(vec![40, 60, 30, 30]);
    table.with_headers(vec![
        "Item".to_string(),
        "Description".to_string(),
        "Qty".to_string(),
        "Price".to_string(),
    ]);
    table.add_row(vec!["A001".into(), "Widget".into(), "5".into(), "$10.00".into()]);
    regions[1].add_widget(table).unwrap();

    // Footer
    regions[2].label("Thank you for your business!", Alignment::Center);

    page.finalize().unwrap();
    let document = doc.build();

    // Render
    let bytes = document.render();
}
```

---

## 4. Core Types

### 4.1 Style

#### Definition

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub bold: bool,
    pub underline: bool,
}
```

#### Constants

```rust
impl Style {
    /// No formatting
    pub const NORMAL: Style;

    /// Bold only
    pub const BOLD: Style;

    /// Underline only
    pub const UNDERLINE: Style;

    /// Bold + Underline
    pub const BOLD_UNDERLINE: Style;
}
```

#### Example

```rust
// Using constants
let style = Style::BOLD;

// Custom style
let style = Style {
    bold: true,
    underline: false,
};
```

#### ESC/P Mapping

| Style | ESC/P Commands |
|-------|----------------|
| `NORMAL` | `ESC F`, `ESC - 0` (disable all) |
| `BOLD` | `ESC E` |
| `UNDERLINE` | `ESC - 1` |
| `BOLD_UNDERLINE` | `ESC E`, `ESC - 1` |

---

### 4.2 Alignment

#### Definition

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}
```

#### Example

```rust
use epson_lq2090_layout::Alignment;

let align = Alignment::Center;
```

---

### 4.3 LayoutError

#### Definition

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    RegionOutOfBounds { x: u16, y: u16, max_x: u16, max_y: u16 },
    InvalidDimensions { width: u16, height: u16 },
    BuilderNotFinalized,
    InvalidSplitRatios { provided: usize, expected: usize },
    WidgetRenderError { widget_name: &'static str, reason: String },
}
```

#### Methods

```rust
impl std::fmt::Display for LayoutError { /* ... */ }
impl std::error::Error for LayoutError {}
```

#### Example

```rust
use epson_lq2090_layout::LayoutError;

match region.split_horizontal(&[]) {
    Ok(children) => { /* ... */ },
    Err(LayoutError::InvalidSplitRatios { provided, expected }) => {
        eprintln!("Error: provided {}, expected {}", provided, expected);
    },
    Err(e) => {
        eprintln!("Layout error: {}", e);
    }
}
```

---

## 5. Builder APIs

### 5.1 DocumentBuilder

#### Definition

```rust
pub struct DocumentBuilder { /* private */ }
```

#### Methods

##### `new() -> Self`

Creates a new document builder.

**Example**:
```rust
let doc = DocumentBuilder::new();
```

**Complexity**: O(1)

---

##### `add_page(&mut self) -> PageBuilder<'_>`

Adds a new page and returns a builder for configuring it.

**Lifetime**: The returned `PageBuilder` borrows from this `DocumentBuilder`.

**Example**:
```rust
let mut doc = DocumentBuilder::new();
let page1 = doc.add_page();
let page2 = doc.add_page();
```

**Complexity**: O(1)

---

##### `build(self) -> Document`

Finalizes and returns an immutable `Document`.

**Consumes**: This method consumes the builder.

**Example**:
```rust
let mut doc = DocumentBuilder::new();
doc.add_page();
let document = doc.build(); // DocumentBuilder is consumed
```

**Complexity**: O(1)

---

#### Complete Example

```rust
use epson_lq2090_layout::DocumentBuilder;

fn create_document() -> Document {
    let mut builder = DocumentBuilder::new();

    // Page 1
    let mut page1 = builder.add_page();
    page1.root_region().write_text(0, 0, "Page 1", Style::BOLD);
    page1.finalize().unwrap();

    // Page 2
    let mut page2 = builder.add_page();
    page2.root_region().write_text(0, 0, "Page 2", Style::NORMAL);
    page2.finalize().unwrap();

    builder.build()
}
```

---

### 5.2 PageBuilder

#### Definition

```rust
pub struct PageBuilder<'doc> { /* private */ }
```

#### Lifetime Parameter

- `'doc`: Ties this builder to the parent `DocumentBuilder`

#### Methods

##### `root_region(&mut self) -> RegionHandle<'_>`

Returns a handle to the root region (160 columns × 51 rows).

**Example**:
```rust
let mut page = doc.add_page();
let mut root = page.root_region();
root.write_text(0, 0, "Hello", Style::NORMAL);
```

**Complexity**: O(1)

---

##### `finalize(self) -> Result<(), LayoutError>`

Finalizes the page. Currently always returns `Ok(())`.

**Consumes**: This method consumes the page builder.

**Example**:
```rust
let page = doc.add_page();
page.finalize()?;
```

**Complexity**: O(1)

---

#### Complete Example

```rust
fn configure_page(doc: &mut DocumentBuilder) {
    let mut page = doc.add_page();

    let mut root = page.root_region();
    root.with_padding(2, 2, 2, 2);

    let mut sections = root.split_vertical(&[1, 3, 1]).unwrap();
    sections[0].label("HEADER", Alignment::Center);
    sections[1].write_text(0, 0, "Body content", Style::NORMAL);
    sections[2].label("FOOTER", Alignment::Right);

    page.finalize().unwrap();
}
```

---

### 5.3 RegionHandle

#### Definition

```rust
pub struct RegionHandle<'page> { /* private */ }
```

#### Lifetime Parameter

- `'page`: Ties this handle to the parent `PageBuilder`

---

#### Methods

##### `write_text(&mut self, x: u16, y: u16, text: &str, style: Style) -> &mut Self`

Writes text at local coordinates (relative to region origin).

**Parameters**:
- `x`: Column offset (0-based, relative to region)
- `y`: Row offset (0-based, relative to region)
- `text`: Text to write (UTF-8, non-ASCII replaced with '?')
- `style`: Text styling

**Returns**: `&mut Self` for method chaining

**Behavior**:
- Truncates horizontally at region width
- Silently ignores if `y >= region.height`
- Non-ASCII characters replaced with `'?'`

**Example**:
```rust
region.write_text(0, 0, "Line 1", Style::NORMAL)
      .write_text(0, 1, "Line 2", Style::BOLD)
      .write_text(0, 2, "Line 3", Style::UNDERLINE);
```

**Complexity**: O(text.len())

---

##### `label(&mut self, text: &str, alignment: Alignment) -> &mut Self`

Convenience method to add a single-line label with alignment.

**Parameters**:
- `text`: Label text
- `alignment`: Left, Center, or Right

**Example**:
```rust
region.label("Centered Title", Alignment::Center);
```

**Equivalent to**:
```rust
use epson_lq2090_layout::widgets::Label;

let label = Label::new(text, alignment);
region.add_widget(label).unwrap();
```

**Complexity**: O(text.len())

---

##### `split_horizontal(&mut self, ratios: &[u16]) -> Result<Vec<RegionHandle<'_>>, LayoutError>`

Splits the region horizontally into N subregions with given ratios.

**Parameters**:
- `ratios`: Slice of relative widths (e.g., `&[1, 2, 1]` = 25%, 50%, 25%)

**Returns**: `Vec<RegionHandle<'_>>` with N handles

**Errors**:
- `InvalidSplitRatios` if `ratios.is_empty()`

**Example**:
```rust
let mut cols = region.split_horizontal(&[1, 2, 1]).unwrap();
// cols[0] = 25% width
// cols[1] = 50% width
// cols[2] = 25% width

cols[0].label("Left", Alignment::Left);
cols[1].label("Center", Alignment::Center);
cols[2].label("Right", Alignment::Right);
```

**Complexity**: O(ratios.len())

---

##### `split_vertical(&mut self, ratios: &[u16]) -> Result<Vec<RegionHandle<'_>>, LayoutError>`

Splits the region vertically into N subregions with given ratios.

**Parameters**:
- `ratios`: Slice of relative heights

**Returns**: `Vec<RegionHandle<'_>>` with N handles

**Errors**:
- `InvalidSplitRatios` if `ratios.is_empty()`

**Example**:
```rust
let mut rows = region.split_vertical(&[10, 35, 6]).unwrap();
// rows[0] = header (10/51 = ~20%)
// rows[1] = body (35/51 = ~69%)
// rows[2] = footer (6/51 = ~11%)
```

**Complexity**: O(ratios.len())

---

##### `grid(&mut self, rows: u16, cols: u16) -> Result<Vec<Vec<RegionHandle<'_>>>, LayoutError>`

Creates an M×N grid of equal-sized subregions.

**Parameters**:
- `rows`: Number of rows
- `cols`: Number of columns

**Returns**: `Vec<Vec<RegionHandle<'_>>>` (2D grid)

**Errors**:
- `InvalidDimensions` if `rows == 0 || cols == 0`

**Example**:
```rust
let grid = region.grid(3, 4).unwrap(); // 3 rows × 4 columns

for (i, row) in grid.iter_mut().enumerate() {
    for (j, cell) in row.iter_mut().enumerate() {
        cell.write_text(0, 0, &format!("({},{})", i, j), Style::NORMAL);
    }
}
```

**Complexity**: O(rows × cols)

---

##### `with_padding(&mut self, top: u16, right: u16, bottom: u16, left: u16) -> &mut Self`

Applies padding to the region (reduces inner usable area).

**Parameters**:
- `top`, `right`, `bottom`, `left`: Padding in cells

**Returns**: `&mut Self` for chaining

**Example**:
```rust
region.with_padding(1, 2, 1, 2) // Top/bottom=1, left/right=2
      .write_text(0, 0, "Padded content", Style::NORMAL);
```

**Complexity**: O(1)

---

##### `set_default_style(&mut self, style: Style) -> &mut Self`

Sets the default style for all text written to this region.

**Parameters**:
- `style`: Default style

**Returns**: `&mut Self` for chaining

**Example**:
```rust
region.set_default_style(Style::BOLD)
      .write_text(0, 0, "This is bold", Style::NORMAL); // Still uses provided style
```

**Note**: Explicit style in `write_text()` overrides default.

**Complexity**: O(1)

---

##### `child_region(&mut self, x: u16, y: u16, width: u16, height: u16) -> Result<RegionHandle<'_>, LayoutError>`

Creates a child region at specific coordinates.

**Parameters**:
- `x`, `y`: Origin (relative to parent region)
- `width`, `height`: Dimensions

**Returns**: `RegionHandle<'_>` to the child region

**Errors**:
- `InvalidDimensions` if `width == 0 || height == 0`
- `RegionOutOfBounds` if child exceeds parent bounds

**Example**:
```rust
let mut child = region.child_region(10, 5, 50, 20)?;
child.label("Nested region", Alignment::Left);
```

**Complexity**: O(1)

---

##### `add_widget<W: Widget>(&mut self, widget: W) -> Result<(), LayoutError>`

Adds a widget to this region.

**Type Parameter**:
- `W`: Any type implementing the `Widget` trait

**Parameters**:
- `widget`: Widget instance

**Returns**: `Result<(), LayoutError>`

**Errors**:
- `WidgetRenderError` if widget fails to render

**Example**:
```rust
use epson_lq2090_layout::widgets::{Table, TableStyle};

let mut table = Table::new(vec![30, 30, 30]);
table.with_headers(vec!["Col1".into(), "Col2".into(), "Col3".into()]);
table.add_row(vec!["A".into(), "B".into(), "C".into()]);

region.add_widget(table)?;
```

**Complexity**: O(widget-specific)

---

#### Complete Example

```rust
fn complex_layout(page: &mut PageBuilder) {
    let mut root = page.root_region();

    // Apply padding
    root.with_padding(2, 2, 2, 2);

    // Split into header, body, footer
    let mut sections = root.split_vertical(&[8, 37, 4]).unwrap();

    // Header with 3 columns
    let mut header_cols = sections[0].split_horizontal(&[1, 2, 1]).unwrap();
    header_cols[0].label("Left", Alignment::Left);
    header_cols[1].label("CENTER", Alignment::Center)
                  .set_default_style(Style::BOLD);
    header_cols[2].label("Right", Alignment::Right);

    // Body content
    sections[1].write_text(0, 0, "Body line 1", Style::NORMAL)
               .write_text(0, 1, "Body line 2", Style::BOLD);

    // Footer
    sections[2].label("Page 1 of 1", Alignment::Center);
}
```

---

### 5.4 Document

#### Definition

```rust
pub struct Document { /* private */ }
```

#### Thread Safety

```rust
unsafe impl Send for Document {}
unsafe impl Sync for Document {}
```

**`Document` is immutable and thread-safe.**

---

#### Methods

##### `render(&self) -> Vec<u8>`

Renders the document to ESC/P byte stream.

**Returns**: `Vec<u8>` containing ESC/P commands

**Determinism**: Calling `render()` multiple times produces identical output.

**Example**:
```rust
let document = builder.build();
let bytes = document.render();

// Send to printer
std::fs::write("/dev/usb/lp0", bytes)?;

// Or via network
// tcp_stream.write_all(&bytes)?;
```

**Complexity**: O(pages × 160 × 51)

---

##### `page_count(&self) -> usize`

Returns the number of pages in the document.

**Example**:
```rust
let count = document.page_count();
println!("Document has {} page(s)", count);
```

**Complexity**: O(1)

---

#### Complete Example

```rust
use std::fs::File;
use std::io::Write;

fn render_to_file(document: &Document, path: &str) -> std::io::Result<()> {
    let bytes = document.render();

    let mut file = File::create(path)?;
    file.write_all(&bytes)?;

    println!("Rendered {} pages to {}", document.page_count(), path);
    Ok(())
}
```

---

## 6. Widget APIs

### 6.1 Widget Trait

#### Definition

```rust
pub trait Widget {
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError>;
}
```

#### Usage

Implement this trait to create custom widgets.

**Example**:
```rust
struct MyWidget {
    title: String,
}

impl Widget for MyWidget {
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError> {
        region.write_text(0, 0, &self.title, Style::BOLD);
        Ok(())
    }
}

// Usage
let widget = MyWidget { title: "Custom Widget".into() };
region.add_widget(widget)?;
```

---

### 6.2 Label

#### Definition

```rust
pub struct Label {
    // private fields
}
```

#### Methods

##### `new(text: impl Into<String>, alignment: Alignment) -> Self`

Creates a new label.

**Parameters**:
- `text`: Label text
- `alignment`: Text alignment (Left, Center, Right)

**Example**:
```rust
use epson_lq2090_layout::widgets::Label;

let label = Label::new("Invoice", Alignment::Center);
```

---

##### `with_style(self, style: Style) -> Self`

Sets the label style.

**Example**:
```rust
let label = Label::new("Title", Alignment::Center)
    .with_style(Style::BOLD_UNDERLINE);
```

---

#### Widget Implementation

```rust
impl Widget for Label {
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError>;
}
```

#### Complete Example

```rust
use epson_lq2090_layout::widgets::Label;

let label = Label::new("INVOICE #12345", Alignment::Center)
    .with_style(Style::BOLD);

region.add_widget(label)?;
```

---

### 6.3 TextBlock

#### Definition

```rust
pub struct TextBlock {
    // private fields
}
```

#### Methods

##### `new(text: impl Into<String>) -> Self`

Creates a new text block with preformatted text.

**Parameters**:
- `text`: Multi-line text (preserves whitespace, no wrapping)

**Example**:
```rust
use epson_lq2090_layout::widgets::TextBlock;

let text = r#"
    Line 1
        Indented line 2
    Line 3
"#;

let block = TextBlock::new(text);
region.add_widget(block)?;
```

---

##### `with_style(self, style: Style) -> Self`

Sets the text style.

**Example**:
```rust
let block = TextBlock::new("Code:\n  fn main() {}")
    .with_style(Style::NORMAL);
```

---

### 6.4 Paragraph

#### Definition

```rust
pub struct Paragraph {
    // private fields
}
```

#### Methods

##### `new(text: impl Into<String>) -> Self`

Creates a paragraph with word-wrapping.

**Parameters**:
- `text`: Text to wrap at region width

**Behavior**:
- Wraps words at spaces
- No hyphenation
- Vertical truncation if exceeds region height

**Example**:
```rust
use epson_lq2090_layout::widgets::Paragraph;

let para = Paragraph::new(
    "This is a long paragraph that will be wrapped automatically \
     at the region width. No hyphenation is performed."
);

region.add_widget(para)?;
```

---

##### `with_style(self, style: Style) -> Self`

Sets the paragraph style.

---

### 6.5 Box

#### Definition

```rust
pub struct Box {
    // private fields
}
```

#### Methods

##### `new() -> Self`

Creates a new box with ASCII borders.

**Border Characters**: `+`, `-`, `|`

**Example**:
```rust
use epson_lq2090_layout::widgets::Box;

let mut box_widget = Box::new();
region.add_widget(box_widget)?;

// Renders:
// +------------------+
// |                  |
// |                  |
// +------------------+
```

---

##### `with_title(self, title: impl Into<String>) -> Self`

Adds a title to the top border.

**Example**:
```rust
let box_widget = Box::new()
    .with_title("Section 1");

// Renders:
// +--- Section 1 ---+
// |                 |
// +------------------+
```

---

##### `with_style(self, style: Style) -> Self`

Sets the border style.

---

#### Inner Region

The `Box` widget provides an inner region with dimensions `(width - 2, height - 2)`.

**Example**:
```rust
// Get inner region handle
let inner = box_widget.inner_region();
inner.write_text(0, 0, "Content inside box", Style::NORMAL);
```

---

### 6.6 KeyValue

#### Definition

```rust
pub struct KeyValue {
    // private fields
}
```

#### Methods

##### `new(key_width: u16) -> Self`

Creates a key-value list with fixed key column width.

**Parameters**:
- `key_width`: Width of key column

**Example**:
```rust
use epson_lq2090_layout::widgets::KeyValue;

let mut kv = KeyValue::new(20);
```

---

##### `add_pair(&mut self, key: impl Into<String>, value: impl Into<String>)`

Adds a key-value pair.

**Example**:
```rust
let mut kv = KeyValue::new(20);
kv.add_pair("Invoice Number", "INV-2025-001");
kv.add_pair("Date", "2025-01-18");
kv.add_pair("Customer", "Acme Corp");

region.add_widget(kv)?;

// Renders:
// Invoice Number    : INV-2025-001
// Date              : 2025-01-18
// Customer          : Acme Corp
```

---

##### `with_separator(self, separator: impl Into<String>) -> Self`

Sets the separator between key and value (default: `": "`).

**Example**:
```rust
let kv = KeyValue::new(15)
    .with_separator(" = ");

// Renders:
// Key1           = Value1
```

---

### 6.7 Table

#### Definition

```rust
pub struct Table {
    // private fields
}
```

#### Methods

##### `new(column_widths: Vec<u16>) -> Self`

Creates a table with fixed column widths.

**Parameters**:
- `column_widths`: Width of each column in characters

**Panics**: If `column_widths.is_empty()`

**Example**:
```rust
use epson_lq2090_layout::widgets::Table;

let table = Table::new(vec![30, 40, 20, 30]);
// 4 columns: 30, 40, 20, 30 chars wide
```

---

##### `with_headers(self, headers: Vec<String>) -> Self`

Sets the header row.

**Example**:
```rust
let table = Table::new(vec![30, 30, 30])
    .with_headers(vec![
        "Product".to_string(),
        "Quantity".to_string(),
        "Price".to_string(),
    ]);
```

---

##### `add_row(&mut self, row: Vec<String>)`

Adds a data row.

**Example**:
```rust
let mut table = Table::new(vec![30, 30, 30]);
table.add_row(vec!["Widget".into(), "5".into(), "$50.00".into()]);
table.add_row(vec!["Gadget".into(), "3".into(), "$30.00".into()]);
```

---

##### `with_style(self, style: TableStyle) -> Self`

Sets the table style.

**Example**:
```rust
use epson_lq2090_layout::widgets::TableStyle;

let style = TableStyle {
    header_style: Style::BOLD_UNDERLINE,
    row_style: Style::NORMAL,
    draw_separators: true,
};

let table = Table::new(vec![30, 30])
    .with_style(style);
```

---

#### TableStyle

```rust
#[derive(Debug, Clone)]
pub struct TableStyle {
    pub header_style: Style,
    pub row_style: Style,
    pub draw_separators: bool,
}

impl Default for TableStyle {
    fn default() -> Self {
        Self {
            header_style: Style::BOLD,
            row_style: Style::NORMAL,
            draw_separators: true,
        }
    }
}
```

---

#### Complete Example

```rust
use epson_lq2090_layout::widgets::{Table, TableStyle};

let mut table = Table::new(vec![40, 60, 20, 30]);

table.with_headers(vec![
    "Item Code".to_string(),
    "Description".to_string(),
    "Qty".to_string(),
    "Price".to_string(),
]);

table.add_row(vec!["A001".into(), "Industrial Widget".into(), "10".into(), "$125.50".into()]);
table.add_row(vec!["B002".into(), "Premium Gadget".into(), "5".into(), "$87.25".into()]);
table.add_row(vec!["C003".into(), "Standard Component".into(), "20".into(), "$15.00".into()]);

let style = TableStyle {
    header_style: Style::BOLD,
    row_style: Style::NORMAL,
    draw_separators: true,
};
table.with_style(style);

region.add_widget(table)?;
```

**Output**:
```
Item Code                         Description                                   Qty    Price
-------------------------------------------------------------------------------------------------
A001                              Industrial Widget                             10     $125.50
B002                              Premium Gadget                                5      $87.25
C003                              Standard Component                            20     $15.00
```

---

## 7. Error Handling

### 7.1 Error Type

```rust
pub enum LayoutError {
    RegionOutOfBounds { x: u16, y: u16, max_x: u16, max_y: u16 },
    InvalidDimensions { width: u16, height: u16 },
    BuilderNotFinalized,
    InvalidSplitRatios { provided: usize, expected: usize },
    WidgetRenderError { widget_name: &'static str, reason: String },
}
```

### 7.2 Error Handling Patterns

#### Pattern 1: Propagate with `?`

```rust
fn create_layout(region: &mut RegionHandle) -> Result<(), LayoutError> {
    let sections = region.split_vertical(&[1, 3, 1])?;

    let table = Table::new(vec![30, 30]);
    sections[1].add_widget(table)?;

    Ok(())
}
```

#### Pattern 2: Handle Specific Errors

```rust
match region.split_horizontal(&[]) {
    Ok(children) => {
        // Success
    },
    Err(LayoutError::InvalidSplitRatios { provided, .. }) => {
        eprintln!("Must provide at least 1 ratio, got {}", provided);
    },
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

#### Pattern 3: Unwrap for Infallible Operations

```rust
// Safe: We know the ratios are valid
let sections = region.split_vertical(&[10, 35, 6]).unwrap();
```

### 7.3 Silent Failures (By Design)

These operations **do not** return errors:

- **Out-of-bounds writes**: Silently clipped
- **Text truncation**: Silently truncated
- **Overflow content**: Silently discarded

**Rationale**: Per PRD requirements FR-T1, FR-T2, FR-T3, FR-T4.

**Example**:
```rust
// Writing beyond region bounds does not error
region.write_text(200, 100, "Out of bounds", Style::NORMAL);
// ✅ No panic, no error, content silently discarded
```

---

## 8. Complete Examples

### 8.1 Single-Page Invoice

```rust
use epson_lq2090_layout::*;
use epson_lq2090_layout::widgets::*;

fn main() -> Result<(), LayoutError> {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();
    let mut root = page.root_region();

    // Apply page margins
    root.with_padding(2, 2, 2, 2);

    // Split: Header (10), Body (35), Footer (4)
    let mut sections = root.split_vertical(&[10, 35, 4])?;

    // === HEADER ===
    let mut header = &mut sections[0];
    header.label("INVOICE", Alignment::Center)
          .write_text(0, 2, "Invoice #: INV-2025-001", Style::NORMAL)
          .write_text(0, 3, "Date: January 18, 2025", Style::NORMAL);

    // Company info (right-aligned)
    header.write_text(100, 2, "Acme Corporation", Style::BOLD);
    header.write_text(100, 3, "123 Business St", Style::NORMAL);
    header.write_text(100, 4, "City, State 12345", Style::NORMAL);

    // === BODY ===
    let mut body = &mut sections[1];

    // Items table
    let mut table = Table::new(vec![40, 60, 20, 30]);
    table.with_headers(vec![
        "Item Code".into(),
        "Description".into(),
        "Qty".into(),
        "Price".into(),
    ]);

    table.add_row(vec!["A001".into(), "Premium Widget".into(), "10".into(), "$125.50".into()]);
    table.add_row(vec!["B002".into(), "Standard Gadget".into(), "5".into(), "$87.25".into()]);
    table.add_row(vec!["C003".into(), "Basic Component".into(), "20".into(), "$15.00".into()]);

    body.add_widget(table)?;

    // Total
    body.write_text(100, 30, "TOTAL: $1,542.50", Style::BOLD);

    // === FOOTER ===
    sections[2].label("Thank you for your business!", Alignment::Center);

    // Finalize
    page.finalize()?;
    let document = doc.build();

    // Render and save
    let bytes = document.render();
    std::fs::write("invoice.escp", bytes)?;

    println!("Invoice generated successfully!");
    Ok(())
}
```

---

### 8.2 Multi-Page Document

```rust
use epson_lq2090_layout::*;

fn main() -> Result<(), LayoutError> {
    let mut doc = DocumentBuilder::new();

    // Generate 10 pages
    for page_num in 1..=10 {
        let mut page = doc.add_page();
        let mut root = page.root_region();

        // Header
        root.write_text(0, 0, &format!("Page {}/10", page_num), Style::BOLD);

        // Content
        for line in 0..40 {
            root.write_text(
                0,
                line + 5,
                &format!("This is line {} on page {}", line, page_num),
                Style::NORMAL
            );
        }

        // Footer
        root.write_text(0, 50, "Confidential", Style::UNDERLINE);

        page.finalize()?;
    }

    let document = doc.build();
    assert_eq!(document.page_count(), 10);

    let bytes = document.render();
    std::fs::write("report.escp", bytes)?;

    Ok(())
}
```

---

### 8.3 Complex Nested Layout

```rust
use epson_lq2090_layout::*;
use epson_lq2090_layout::widgets::*;

fn main() -> Result<(), LayoutError> {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();
    let mut root = page.root_region();

    // Main split: Sidebar (40) + Content (120)
    let mut main_cols = root.split_horizontal(&[1, 3])?;

    // === SIDEBAR ===
    let mut sidebar = &mut main_cols[0];
    sidebar.with_padding(1, 1, 1, 1);

    let mut sidebar_sections = sidebar.split_vertical(&[10, 30])?;

    // Sidebar header
    sidebar_sections[0].label("MENU", Alignment::Center)
                       .set_default_style(Style::BOLD);

    // Sidebar items
    let mut menu_items = sidebar_sections[1].split_vertical(&[1, 1, 1, 1])?;
    menu_items[0].label("1. Dashboard", Alignment::Left);
    menu_items[1].label("2. Reports", Alignment::Left);
    menu_items[2].label("3. Settings", Alignment::Left);
    menu_items[3].label("4. Help", Alignment::Left);

    // === CONTENT AREA ===
    let mut content = &mut main_cols[1];
    content.with_padding(2, 2, 2, 2);

    // Content: Header + Grid
    let mut content_sections = content.split_vertical(&[5, 40])?;

    // Content header
    content_sections[0].label("DASHBOARD", Alignment::Left)
                       .set_default_style(Style::BOLD);

    // 2×2 Grid of widgets
    let mut grid = content_sections[1].grid(2, 2)?;

    // Top-left: Stats
    let mut kv = KeyValue::new(15);
    kv.add_pair("Users", "1,234");
    kv.add_pair("Active", "987");
    grid[0][0].add_widget(kv)?;

    // Top-right: Recent activity
    grid[0][1].label("Recent Activity", Alignment::Center);

    // Bottom-left: Table
    let mut table = Table::new(vec![20, 20]);
    table.with_headers(vec!["Name".into(), "Status".into()]);
    table.add_row(vec!["Task 1".into(), "Done".into()]);
    grid[1][0].add_widget(table)?;

    // Bottom-right: Box with content
    let box_widget = Box::new().with_title("Notes");
    grid[1][1].add_widget(box_widget)?;

    // Finalize
    page.finalize()?;
    let document = doc.build();

    let bytes = document.render();
    std::fs::write("dashboard.escp", bytes)?;

    Ok(())
}
```

---

### 8.4 Custom Widget

```rust
use epson_lq2090_layout::*;
use epson_lq2090_layout::widgets::Widget;

/// Custom widget that draws a horizontal line
struct HorizontalLine {
    character: char,
    style: Style,
}

impl HorizontalLine {
    fn new(character: char) -> Self {
        Self {
            character,
            style: Style::NORMAL,
        }
    }
}

impl Widget for HorizontalLine {
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError> {
        let (_, _, width, height) = region.region.inner_bounds();

        if height == 0 {
            return Ok(());
        }

        let line = self.character.to_string().repeat(width as usize);
        region.write_text(0, 0, &line, self.style);

        Ok(())
    }
}

fn main() -> Result<(), LayoutError> {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();
    let mut root = page.root_region();

    root.write_text(0, 0, "Section 1", Style::BOLD);
    root.add_widget(HorizontalLine::new('='))?;
    root.write_text(0, 2, "Content here", Style::NORMAL);

    root.write_text(0, 5, "Section 2", Style::BOLD);
    root.add_widget(HorizontalLine::new('-'))?;

    page.finalize()?;
    let document = doc.build();

    let bytes = document.render();
    std::fs::write("custom_widget.escp", bytes)?;

    Ok(())
}
```

---

## 9. API Contracts & Guarantees

### 9.1 Determinism

**Contract**: `Document::render()` is deterministic.

```rust
let doc = create_document();

let render1 = doc.render();
let render2 = doc.render();
let render3 = doc.render();

assert_eq!(render1, render2);
assert_eq!(render2, render3);

// SHA-256 hashes will match
use sha2::{Sha256, Digest};
let hash1 = Sha256::digest(&render1);
let hash2 = Sha256::digest(&render2);
assert_eq!(hash1, hash2);
```

**Guaranteed**: Same input always produces byte-identical ESC/P output.

---

### 9.2 No Panics

**Contract**: No panics under documented usage.

```rust
// ✅ These never panic:
region.write_text(1000, 1000, "Out of bounds", Style::NORMAL);
region.split_horizontal(&[]); // Returns Err, doesn't panic
page.write_text(200, 100, "🦀", Style::NORMAL); // Non-ASCII → '?'
```

**Exception**: `debug_assert!` may panic in debug builds.

---

### 9.3 Thread Safety

**Contract**: `Document` is `Send + Sync`.

```rust
use std::thread;

let document = create_document();
let doc_ref = &document;

let handle = thread::spawn(move || {
    let bytes = doc_ref.render();
    bytes.len()
});

let length = handle.join().unwrap();
println!("Rendered {} bytes", length);
```

**Guaranteed**: Can share `&Document` across threads.

---

### 9.4 Immutability

**Contract**: Finalized `Document` is immutable.

```rust
let document = builder.build();

// ❌ Cannot modify document after build()
// document.add_page(); // Compile error: no such method

// ✅ Can only read
let count = document.page_count();
let bytes = document.render();
```

---

### 9.5 Lifetime Safety

**Contract**: Borrow checker prevents dangling references.

```rust
// ❌ This will not compile:
let mut doc = DocumentBuilder::new();
let page = doc.add_page();
let document = doc.build(); // Error: doc borrowed by page
drop(page); // Must drop page first
let document = doc.build(); // ✅ Now OK
```

---

### 9.6 ESC/P Compliance

**Contract**: Rendered output follows ESC/P specification.

**Guaranteed commands**:
- `ESC @` (0x1B 0x40): Reset printer
- `SI` (0x0F): Condensed mode (12 CPI)
- `ESC E` / `ESC F`: Bold on/off
- `ESC - 1` / `ESC - 0`: Underline on/off
- `CR` (0x0D), `LF` (0x0A): Line termination
- `FF` (0x0C): Form feed (page separator)

**Validation**: Hardware-tested on EPSON LQ-2090II.

---

## 10. Performance Characteristics

### 10.1 Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| `DocumentBuilder::new()` | O(1) | |
| `add_page()` | O(1) | Allocates 16 KB |
| `Page::write_cell()` | O(1) | Inline, very fast |
| `Page::write_text()` | O(n) | n = text length |
| `Region::split_horizontal()` | O(m) | m = number of splits |
| `Region::split_vertical()` | O(m) | m = number of splits |
| `Region::grid()` | O(r × c) | r = rows, c = cols |
| `Document::render()` | O(p × 8,160) | p = page count |
| `Widget::render()` | Varies | Widget-dependent |

### 10.2 Space Complexity

| Structure | Size | Notes |
|-----------|------|-------|
| `Cell` | 2 bytes | Character + style |
| `Page` | ~16 KB | 160 × 51 × 2 bytes |
| `Document` (100 pages) | ~1.6 MB | 100 × 16 KB |
| `Region` (tree node) | ~48 bytes | Vec overhead + fields |

### 10.3 Benchmark Targets

| Operation | Target (p99) | Measurement |
|-----------|--------------|-------------|
| `Page::new()` | < 10 μs | `criterion` |
| Single page render | < 100 μs | `criterion` |
| 100-page render | < 10 ms | `criterion` |

### 10.4 Optimization Tips

#### Tip 1: Reuse Documents

```rust
// ❌ Slow: Create new document each time
for i in 0..1000 {
    let doc = create_invoice(i);
    send_to_printer(doc.render());
}

// ✅ Fast: Batch into single document
let mut doc = DocumentBuilder::new();
for i in 0..1000 {
    add_invoice_page(&mut doc, i);
}
send_to_printer(doc.build().render());
```

#### Tip 2: Pre-allocate When Possible

```rust
// ✅ Pre-allocate for known size
let mut rows = Vec::with_capacity(100);
for i in 0..100 {
    rows.push(vec![format!("Row {}", i), "Data".into()]);
}
```

---

## 11. Migration Guide

### 11.1 Semantic Versioning

This library follows [SemVer 2.0.0](https://semver.org/).

**Version format**: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking API changes
- **MINOR**: Backward-compatible features
- **PATCH**: Backward-compatible bug fixes

### 11.2 Deprecation Policy

Deprecated APIs will:
1. Emit warnings for ≥ 1 minor version before removal
2. Be documented in CHANGELOG.md
3. Provide migration path in deprecation message

**Example**:
```rust
#[deprecated(since = "1.2.0", note = "Use `split_horizontal_equal()` instead")]
pub fn split_equal(&mut self, count: u16) -> Result<...> {
    // Old implementation
}
```

### 11.3 API Stability Guarantees

**V1.x.x Series**:
- Public API frozen except for additions
- ESC/P output format remains stable (byte-compatible)
- No breaking changes within 1.x.x

**V2.0.0 (Future)**:
- May introduce breaking changes
- Migration guide provided
- Automated migration tools where possible

---

## 12. Appendices

### 12.1 Quick Reference

#### Imports

```rust
use epson_lq2090_layout::*;
use epson_lq2090_layout::widgets::*;
```

#### Common Patterns

**Create Document**:
```rust
let mut doc = DocumentBuilder::new();
```

**Add Page**:
```rust
let mut page = doc.add_page();
let mut root = page.root_region();
```

**Write Text**:
```rust
root.write_text(0, 0, "Text", Style::BOLD);
```

**Split Region**:
```rust
let mut sections = root.split_vertical(&[1, 3, 1])?;
```

**Add Widget**:
```rust
let table = Table::new(vec![30, 30]);
region.add_widget(table)?;
```

**Render**:
```rust
page.finalize()?;
let doc = doc.build();
let bytes = doc.render();
```

---

### 12.2 Common Errors

#### Error: "Cannot borrow as mutable more than once"

```rust
// ❌ Error
let region1 = page.root_region();
let region2 = page.root_region(); // Error!

// ✅ Solution: Use split or child regions
let mut root = page.root_region();
let sections = root.split_vertical(&[1, 1])?;
```

#### Error: "Value moved"

```rust
// ❌ Error
let page = doc.add_page();
page.finalize()?;
page.root_region(); // Error: page moved

// ✅ Solution: Don't use after finalize
let mut page = doc.add_page();
let mut root = page.root_region();
// ... use root ...
page.finalize()?;
```

---

### 12.3 ESC/P Byte Sequences

| Operation | Sequence | Hex |
|-----------|----------|-----|
| Reset | ESC @ | 1B 40 |
| Condensed mode | SI | 0F |
| Bold ON | ESC E | 1B 45 |
| Bold OFF | ESC F | 1B 46 |
| Underline ON | ESC - 1 | 1B 2D 01 |
| Underline OFF | ESC - 0 | 1B 2D 00 |
| Carriage return | CR | 0D |
| Line feed | LF | 0A |
| Form feed | FF | 0C |

---

### 12.4 Frequently Asked Questions

**Q: Can I change page dimensions?**
A: No, pages are fixed at 160×51 per EPSON LQ-2090II condensed mode.

**Q: What happens to non-ASCII characters?**
A: Replaced with `'?'` per FR-E3.

**Q: Can I use this with other printers?**
A: Compatible printers that support ESC/P condensed mode should work. Test carefully.

**Q: How do I handle errors?**
A: Use `Result<T, LayoutError>` and the `?` operator.

**Q: Is output deterministic?**
A: Yes, 100% deterministic (same input → identical bytes).

**Q: Can I serialize `Document`?**
A: Enable the `serde` feature.

---

## 🔒 End of API Specification

**Document Status**: ✅ Ready for Developer Use

**Next Steps**:
1. Review API with developers
2. Implement public API surface
3. Generate rustdoc documentation
4. Create interactive examples

---

**For API questions, contact:**
**Author**: Mohammad AlMechkor
**Document Location**: `/Users/mohammadalmechkor/Projects/matrix/specs/API-SPEC.md`
**Related**: `PRD.md`, `TDD.md`

---

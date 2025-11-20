# Data Model: Rust ESC/P Layout Engine

**Branch**: `001-rust-escap-layout-engine` | **Date**: 2025-11-18

## Overview

This document defines the complete data model for the Rust ESC/P layout engine library, including all types, their relationships, validation rules, and state transitions.

---

## Core Types

### Cell

**Purpose**: Represents a single character position within a page grid.

**Attributes**:

- `character: u8` - ASCII character value (32-126), 0 represents empty cell
- `style: StyleFlags` - Bit-packed style flags (bold, underline)

**Validation Rules**:

- Character MUST be ASCII (0-127)
- Non-ASCII characters (128-255) MUST be replaced with '?' (63) before storage
- Control characters (0-31 except 0) MUST be rejected or replaced

**State**: Immutable value type (Copy + Clone)

**Relationships**: Owned by Page in 160×51 grid

**Implementation**:

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub character: u8,
    pub style: StyleFlags,
}

impl Cell {
    pub const EMPTY: Cell = Cell { character: 0, style: StyleFlags::NONE };

    pub fn new(ch: char, style: StyleFlags) -> Cell {
        let character = if ch.is_ascii() && ch as u8 >= 32 {
            ch as u8
        } else if ch as u32 > 127 {
            b'?' // Non-ASCII replacement
        } else {
            0 // Empty
        };
        Cell { character, style }
    }
}
```

---

### StyleFlags

**Purpose**: Bit-packed text style information.

**Attributes**:

- `flags: u8` - Bit field storing style flags

**Flags**:

- Bit 0: Bold (0x01)
- Bit 1: Underline (0x02)
- Bits 2-7: Reserved for future styles

**Validation Rules**:

- Only defined bits may be set
- Invalid flag combinations are not possible (all combinations of bold/underline are valid)

**State**: Immutable value type (Copy + Clone)

**Operations**:

- `bold() -> bool` - Test if bold is set
- `underline() -> bool` - Test if underline is set
- `with_bold(bool) -> StyleFlags` - Create new flags with bold set/unset
- `with_underline(bool) -> StyleFlags` - Create new flags with underline set/unset

**Implementation**:

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StyleFlags(u8);

impl StyleFlags {
    pub const NONE: StyleFlags = StyleFlags(0);
    pub const BOLD: StyleFlags = StyleFlags(0b0000_0001);
    pub const UNDERLINE: StyleFlags = StyleFlags(0b0000_0010);

    pub fn bold(self) -> bool { self.0 & Self::BOLD.0 != 0 }
    pub fn underline(self) -> bool { self.0 & Self::UNDERLINE.0 != 0 }

    pub fn with_bold(self, enabled: bool) -> Self {
        if enabled {
            StyleFlags(self.0 | Self::BOLD.0)
        } else {
            StyleFlags(self.0 & !Self::BOLD.0)
        }
    }

    pub fn with_underline(self, enabled: bool) -> Self {
        if enabled {
            StyleFlags(self.0 | Self::UNDERLINE.0)
        } else {
            StyleFlags(self.0 & !Self::UNDERLINE.0)
        }
    }
}
```

---

### Region

**Purpose**: Represents a rectangular view into a page grid.

**Attributes**:

- `x: u16` - Column start position (0-159)
- `y: u16` - Row start position (0-50)
- `width: u16` - Column count
- `height: u16` - Row count

**Validation Rules**:

- `x + width <= 160` (PAGE_WIDTH)
- `y + height <= 51` (PAGE_HEIGHT)
- Width and height MAY be zero (creates empty region)
- All coordinates are inclusive of start, exclusive of end

**State**: Immutable value type (Copy + Clone)

**Operations**:

- `new(x, y, width, height) -> Result<Region, LayoutError>` - Create with validation
- `full_page() -> Region` - Create region covering entire page
- `split_vertical(top_height) -> Result<(Region, Region), LayoutError>` - Split into top/bottom
- `split_horizontal(left_width) -> Result<(Region, Region), LayoutError>` - Split into left/right
- `with_padding(top, right, bottom, left) -> Result<Region, LayoutError>` - Create inset region

**State Transitions**: None (immutable value type)

**Relationships**: Used by PageBuilder and Widgets, does not own data

**Implementation**:

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Region {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Region {
    pub const PAGE_WIDTH: u16 = 160;
    pub const PAGE_HEIGHT: u16 = 51;

    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Result<Self, LayoutError> {
        if x.checked_add(width).map_or(true, |end| end > Self::PAGE_WIDTH) {
            return Err(LayoutError::RegionOutOfBounds);
        }
        if y.checked_add(height).map_or(true, |end| end > Self::PAGE_HEIGHT) {
            return Err(LayoutError::RegionOutOfBounds);
        }
        Ok(Region { x, y, width, height })
    }

    pub fn full_page() -> Self {
        Region { x: 0, y: 0, width: Self::PAGE_WIDTH, height: Self::PAGE_HEIGHT }
    }

    pub fn split_vertical(&self, top_height: u16) -> Result<(Region, Region), LayoutError> {
        if top_height > self.height {
            return Err(LayoutError::InvalidSplit);
        }
        let top = Region::new(self.x, self.y, self.width, top_height)?;
        let bottom = Region::new(self.x, self.y + top_height, self.width, self.height - top_height)?;
        Ok((top, bottom))
    }

    pub fn split_horizontal(&self, left_width: u16) -> Result<(Region, Region), LayoutError> {
        if left_width > self.width {
            return Err(LayoutError::InvalidSplit);
        }
        let left = Region::new(self.x, self.y, left_width, self.height)?;
        let right = Region::new(self.x + left_width, self.y, self.width - left_width, self.height)?;
        Ok((left, right))
    }

    pub fn with_padding(&self, top: u16, right: u16, bottom: u16, left: u16) -> Result<Region, LayoutError> {
        let new_width = self.width.checked_sub(left + right).ok_or(LayoutError::InvalidDimensions)?;
        let new_height = self.height.checked_sub(top + bottom).ok_or(LayoutError::InvalidDimensions)?;
        Region::new(self.x + left, self.y + top, new_width, new_height)
    }
}
```

---

### Page

**Purpose**: Immutable 160×51 character grid representing a single printed page.

**Attributes**:

- `cells: Rect<[[Cell; 160]; 51]>` - Row-major cell grid
- `finalized: bool` - Finalization state (internal use)

**Validation Rules**:

- Always exactly 160×51 cells (compile-time guarantee via array type)
- Cannot be modified after creation (no public mutable methods)

**State**: Immutable after construction

**State Transitions**:

```
PageBuilder (Building)
    -> .build()
    -> Page (Finalized, immutable)
```

**Operations**:

- `builder() -> PageBuilder` - Create new page builder
- `get_cell(x, y) -> Option<Cell>` - Read cell at position (returns None if out of bounds)
- `cells() -> &[[Cell; 160]; 51]` - Read-only access to entire grid

**Relationships**:

- Owned by Document
- Created by PageBuilder

**Implementation**:

```rust
pub struct Page {
    cells: Rect<[[Cell; PAGE_WIDTH]; PAGE_HEIGHT]>,
}

impl Page {
    pub fn builder() -> PageBuilder {
        PageBuilder::new()
    }

    pub fn get_cell(&self, x: u16, y: u16) -> Option<Cell> {
        if x < PAGE_WIDTH as u16 && y < PAGE_HEIGHT as u16 {
            Some(self.cells[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn cells(&self) -> &[[Cell; PAGE_WIDTH]; PAGE_HEIGHT] {
        &self.cells
    }
}
```

---

### PageBuilder

**Purpose**: Mutable builder for constructing Page.

**Attributes**:

- `cells: Rect<[[Cell; 160]; 51]>` - Mutable cell grid under construction
- `_state: PhantomData<State>` - Type state marker (Building vs Finalized)

**Validation Rules**:

- Write operations silently truncate out-of-bounds coordinates (no panic)
- Multiple writes to same cell are allowed (last write wins)

**State**: Mutable during construction, consumed on build

**State Transitions**:

```
new() -> PageBuilder<Building>
    -> write operations
    -> build() -> Page (consumes builder)
```

**Operations**:

- `new() -> PageBuilder` - Create builder with empty cells
- `write_at(x, y, ch, style) -> &mut Self` - Write single cell (truncates if out of bounds)
- `write_str(x, y, text, style) -> &mut Self` - Write string starting at position
- `fill_region(region, ch, style) -> &mut Self` - Fill region with character
- `render_widget(region, widget) -> &mut Self` - Render widget into region
- `build(self) -> Page` - Finalize and create immutable Page

**Relationships**:

- Creates Page
- Used by Widget implementations

**Implementation**:

```rust
pub struct PageBuilder {
    cells: Rect<[[Cell; PAGE_WIDTH]; PAGE_HEIGHT]>,
}

impl PageBuilder {
    pub fn new() -> Self {
        PageBuilder {
            cells: Rect::new([[Cell::EMPTY; PAGE_WIDTH]; PAGE_HEIGHT]),
        }
    }

    pub fn write_at(&mut self, x: u16, y: u16, ch: char, style: StyleFlags) -> &mut Self {
        // Silent truncation (Constitution Principle III)
        if x < PAGE_WIDTH as u16 && y < PAGE_HEIGHT as u16 {
            self.cells[y as usize][x as usize] = Cell::new(ch, style);
        }
        self
    }

    pub fn write_str(&mut self, x: u16, y: u16, text: &str, style: StyleFlags) -> &mut Self {
        for (i, ch) in text.chars().enumerate() {
            self.write_at(x + i as u16, y, ch, style);
        }
        self
    }

    pub fn fill_region(&mut self, region: Region, ch: char, style: StyleFlags) -> &mut Self {
        for dy in 0..region.height {
            for dx in 0..region.width {
                self.write_at(region.x + dx, region.y + dy, ch, style);
            }
        }
        self
    }

    pub fn render_widget(&mut self, region: Region, widget: &dyn Widget) -> &mut Self {
        widget.render(self, region);
        self
    }

    pub fn build(self) -> Page {
        Page { cells: self.cells }
    }
}
```

---

### Document

**Purpose**: Immutable collection of Pages representing a multi-page output.

**Attributes**:

- `pages: Vec<Page>` - Ordered list of pages

**Validation Rules**:

- May contain zero or more pages (zero pages is valid)
- Pages are ordered (order matters for rendering)
- Cannot be modified after creation

**State**: Immutable after construction

**State Transitions**:

```
DocumentBuilder (Building)
    -> .add_page() (multiple times)
    -> .build()
    -> Document (Finalized, immutable)
```

**Operations**:

- `builder() -> DocumentBuilder` - Create new document builder
- `pages() -> &[Page]` - Read-only access to pages
- `page_count() -> usize` - Get number of pages
- `render() -> Vec<u8>` - Render document to ESC/P byte stream

**Relationships**:

- Owns multiple Page instances
- Created by DocumentBuilder

**Implementation**:

```rust
pub struct Document {
    pages: Vec<Page>,
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::new()
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn render(&self) -> Vec<u8> {
        escp::render_document(self)
    }
}
```

---

### DocumentBuilder

**Purpose**: Mutable builder for constructing Document.

**Attributes**:

- `pages: Vec<Page>` - List of pages being accumulated
- `_state: PhantomData<State>` - Type state marker

**Validation Rules**:

- Pages must be finalized before adding (enforced by type system)
- Duplicate pages are allowed
- Pages may be added in any order

**State**: Mutable during construction, consumed on build

**State Transitions**:

```
new() -> DocumentBuilder<Building>
    -> add_page() (multiple times)
    -> build() -> Document (consumes builder)
```

**Operations**:

- `new() -> DocumentBuilder` - Create empty builder
- `add_page(page: Page) -> &mut Self` - Add finalized page
- `build(self) -> Document` - Finalize and create immutable Document

**Relationships**:

- Creates Document
- Accepts Page instances

**Implementation**:

```rust
pub struct DocumentBuilder {
    pages: Vec<Page>,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        DocumentBuilder { pages: Vec::new() }
    }

    pub fn add_page(&mut self, page: Page) -> &mut Self {
        self.pages.push(page);
        self
    }

    pub fn build(self) -> Document {
        Document { pages: self.pages }
    }
}
```

---

## Widget Types

### Widget Trait

**Purpose**: Common interface for renderable content types.

**Operations**:

- `render(&self, page: &mut PageBuilder, region: Region)` - Render widget into region

**Implementations**: Label, TextBlock, Paragraph, ASCIIRect, KeyValueList, Table

**Contract**:

- MUST respect region boundaries (no writes outside region)
- MUST handle zero-width/zero-height regions gracefully (render nothing)
- MUST truncate content that exceeds region size
- MUST NOT panic under any inputs

**Implementation**:

```rust
pub trait Widget {
    fn render(&self, page: &mut PageBuilder, region: Region);
}
```

---

### Label

**Purpose**: Single-line text widget.

**Attributes**:

- `text: String` - Text content
- `style: StyleFlags` - Text style

**Rendering Rules**:

- Renders on first line of region only
- Truncates if text exceeds region width
- Ignores region height (only uses first line)

**Implementation**:

```rust
pub struct Label {
    text: String,
    style: StyleFlags,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label { text: text.into(), style: StyleFlags::NONE }
    }

    pub fn with_style(mut self, style: StyleFlags) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Label {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        if region.height == 0 || region.width == 0 {
            return; // Empty region
        }

        let max_chars = region.width.min(self.text.len() as u16);
        for (i, ch) in self.text.chars().take(max_chars as usize).enumerate() {
            page.write_at(region.x + i as u16, region.y, ch, self.style);
        }
    }
}
```

---

### TextBlock

**Purpose**: Multi-line text widget without word wrapping.

**Attributes**:

- `lines: Vec<String>` - Pre-split text lines

**Rendering Rules**:

- Each string renders on one line
- Lines that exceed width are truncated
- Lines that exceed height are dropped
- No word wrapping performed

**Implementation**:

```rust
pub struct TextBlock {
    lines: Vec<String>,
}

impl TextBlock {
    pub fn new(lines: Vec<String>) -> Self {
        TextBlock { lines }
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        let text = text.into();
        let lines = text.lines().map(|s| s.to_string()).collect();
        TextBlock { lines }
    }
}

impl Widget for TextBlock {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        for (line_idx, line) in self.lines.iter().enumerate() {
            if line_idx as u16 >= region.height {
                break; // Vertical truncation
            }

            let max_chars = region.width.min(line.len() as u16);
            for (char_idx, ch) in line.chars().take(max_chars as usize).enumerate() {
                page.write_at(
                    region.x + char_idx as u16,
                    region.y + line_idx as u16,
                    ch,
                    StyleFlags::NONE
                );
            }
        }
    }
}
```

---

### Paragraph

**Purpose**: Multi-line text with word wrapping.

**Attributes**:

- `text: String` - Original text
- `style: StyleFlags` - Text style

**Rendering Rules**:

- Wraps at word boundaries when line width exceeded
- Breaks words if single word exceeds width
- Truncates lines that exceed region height
- Preserves spaces between words

**Implementation**:

```rust
pub struct Paragraph {
    text: String,
    style: StyleFlags,
}

impl Paragraph {
    pub fn new(text: impl Into<String>) -> Self {
        Paragraph { text: text.into(), style: StyleFlags::NONE }
    }

    pub fn with_style(mut self, style: StyleFlags) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Paragraph {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        let wrapped_lines = wrap_text(&self.text, region.width as usize);

        for (line_idx, line) in wrapped_lines.iter().enumerate() {
            if line_idx as u16 >= region.height {
                break; // Vertical truncation
            }

            for (char_idx, ch) in line.chars().enumerate() {
                page.write_at(
                    region.x + char_idx as u16,
                    region.y + line_idx as u16,
                    ch,
                    self.style
                );
            }
        }
    }
}

// Helper function (internal)
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    // Word-wrapping algorithm implementation
    // Details in widget implementation
}
```

---

### ASCIIRect

**Purpose**: Bordered rect with optional title and content.

**Attributes**:

- `title: Option<String>` - Optional title text
- `content: Rect<dyn Widget>` - Widget rendered inside rect

**Rendering Rules**:

- Border uses ASCII rect-drawing characters (+-|)
- Title rendered in top border if present
- Content rendered in inset region (1 cell padding)
- Requires minimum 3×3 region to be visible

**Border Characters**:

- Corners: `+`
- Horizontal: `-`
- Vertical: `|`

**Implementation**:

```rust
pub struct ASCIIRect {
    title: Option<String>,
    content: Rect<dyn Widget>,
}

impl ASCIIRect {
    pub fn new(content: Rect<dyn Widget>) -> Self {
        ASCIIRect { title: None, content }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Widget for ASCIIRect {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        if region.width < 3 || region.height < 3 {
            return; // Too small for rect
        }

        // Draw corners
        page.write_at(region.x, region.y, '+', StyleFlags::NONE);
        page.write_at(region.x + region.width - 1, region.y, '+', StyleFlags::NONE);
        page.write_at(region.x, region.y + region.height - 1, '+', StyleFlags::NONE);
        page.write_at(region.x + region.width - 1, region.y + region.height - 1, '+', StyleFlags::NONE);

        // Draw top and bottom borders
        for x in 1..region.width - 1 {
            page.write_at(region.x + x, region.y, '-', StyleFlags::NONE);
            page.write_at(region.x + x, region.y + region.height - 1, '-', StyleFlags::NONE);
        }

        // Draw left and right borders
        for y in 1..region.height - 1 {
            page.write_at(region.x, region.y + y, '|', StyleFlags::NONE);
            page.write_at(region.x + region.width - 1, region.y + y, '|', StyleFlags::NONE);
        }

        // Draw title if present
        if let Some(title) = &self.title {
            let title_start = region.x + 2;
            let max_title_len = region.width.saturating_sub(4);
            for (i, ch) in title.chars().take(max_title_len as usize).enumerate() {
                page.write_at(title_start + i as u16, region.y, ch, StyleFlags::NONE);
            }
        }

        // Render content in inset region
        if let Ok(inner) = region.with_padding(1, 1, 1, 1) {
            self.content.render(page, inner);
        }
    }
}
```

---

### KeyValueList

**Purpose**: Vertically aligned key-value pairs.

**Attributes**:

- `entries: Vec<(String, String)>` - Key-value pairs
- `separator: String` - Separator between key and value (default ": ")

**Rendering Rules**:

- One entry per line
- Keys and values separated by separator
- Truncates entry if exceeds region width
- Truncates entries if exceed region height

**Implementation**:

```rust
pub struct KeyValueList {
    entries: Vec<(String, String)>,
    separator: String,
}

impl KeyValueList {
    pub fn new(entries: Vec<(String, String)>) -> Self {
        KeyValueList {
            entries,
            separator: ": ".to_string(),
        }
    }

    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }
}

impl Widget for KeyValueList {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        for (line_idx, (key, value)) in self.entries.iter().enumerate() {
            if line_idx as u16 >= region.height {
                break; // Vertical truncation
            }

            let line = format!("{}{}{}", key, self.separator, value);
            let max_chars = region.width.min(line.len() as u16);

            for (char_idx, ch) in line.chars().take(max_chars as usize).enumerate() {
                page.write_at(
                    region.x + char_idx as u16,
                    region.y + line_idx as u16,
                    ch,
                    StyleFlags::NONE
                );
            }
        }
    }
}
```

---

### Table

**Purpose**: Fixed-column tabular data.

**Attributes**:

- `columns: Vec<ColumnDef>` - Column definitions (name, width)
- `rows: Vec<Vec<String>>` - Row data (outer vec = rows, inner vec = cells)

**Column Definition**:

```rust
pub struct ColumnDef {
    pub name: String,
    pub width: u16,
}
```

**Rendering Rules**:

- First line renders column headers
- Subsequent lines render row data
- Cells truncated to column width
- Rows truncated if exceed region height
- Columns aligned left by default

**Implementation**:

```rust
pub struct Table {
    columns: Vec<ColumnDef>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(columns: Vec<ColumnDef>, rows: Vec<Vec<String>>) -> Self {
        Table { columns, rows }
    }
}

impl Widget for Table {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        if region.height == 0 {
            return;
        }

        // Render header
        let mut col_x = region.x;
        for col in &self.columns {
            let max_chars = col.width.min((region.x + region.width).saturating_sub(col_x));
            for (i, ch) in col.name.chars().take(max_chars as usize).enumerate() {
                page.write_at(col_x + i as u16, region.y, ch, StyleFlags::BOLD);
            }
            col_x += col.width;
            if col_x >= region.x + region.width {
                break;
            }
        }

        // Render rows
        for (row_idx, row) in self.rows.iter().enumerate() {
            let y = region.y + 1 + row_idx as u16;
            if y >= region.y + region.height {
                break; // Vertical truncation
            }

            let mut col_x = region.x;
            for (col_idx, col_def) in self.columns.iter().enumerate() {
                let cell_text = row.get(col_idx).map(|s| s.as_str()).unwrap_or("");
                let max_chars = col_def.width.min((region.x + region.width).saturating_sub(col_x));

                for (i, ch) in cell_text.chars().take(max_chars as usize).enumerate() {
                    page.write_at(col_x + i as u16, y, ch, StyleFlags::NONE);
                }

                col_x += col_def.width;
                if col_x >= region.x + region.width {
                    break;
                }
            }
        }
    }
}
```

---

## ESC/P Rendering Types

### RenderState (Internal)

**Purpose**: Tracks current style state during rendering to minimize ESC/P code emission.

**Attributes**:

- `bold: bool` - Current bold state
- `underline: bool` - Current underline state

**State Transitions**:

```
Initial: { bold: false, underline: false }
    -> transition_to(StyleFlags) -> emits ESC/P codes for changes
    -> reset() -> emits codes to clear all styles
```

**Operations**:

- `new() -> RenderState` - Create initial state
- `transition_to(&mut self, target: StyleFlags, output: &mut Vec<u8>)` - Emit codes for style changes
- `reset(&mut self, output: &mut Vec<u8>)` - Emit codes to clear all styles

**Implementation** (internal, not public API):

```rust
struct RenderState {
    bold: bool,
    underline: bool,
}

impl RenderState {
    fn new() -> Self {
        RenderState { bold: false, underline: false }
    }

    fn transition_to(&mut self, target: StyleFlags, output: &mut Vec<u8>) {
        if target.bold() != self.bold {
            output.extend_from_slice(if target.bold() {
                ESC_BOLD_ON
            } else {
                ESC_BOLD_OFF
            });
            self.bold = target.bold();
        }

        if target.underline() != self.underline {
            output.extend_from_slice(if target.underline() {
                ESC_UNDERLINE_ON
            } else {
                ESC_UNDERLINE_OFF
            });
            self.underline = target.underline();
        }
    }

    fn reset(&mut self, output: &mut Vec<u8>) {
        self.transition_to(StyleFlags::NONE, output);
    }
}
```

---

## Error Types

### LayoutError

**Purpose**: Represents recoverable errors during layout construction.

**Variants**:

- `RegionOutOfBounds` - Region coordinates exceed page bounds (160×51)
- `InvalidDimensions` - Width or height is invalid (e.g., overflow in calculations)
- `InvalidSplit` - Split dimensions exceed parent region

**Usage**: Returned from Region and builder validation methods

**Implementation**:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    RegionOutOfBounds,
    InvalidDimensions,
    InvalidSplit,
}

impl std::fmt::Display for LayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutError::RegionOutOfBounds => {
                write!(f, "Region exceeds page bounds (160×51)")
            }
            LayoutError::InvalidDimensions => {
                write!(f, "Invalid region dimensions")
            }
            LayoutError::InvalidSplit => {
                write!(f, "Split dimensions exceed parent region")
            }
        }
    }
}

impl std::error::Error for LayoutError {}
```

---

## Type Relationships Diagram

```
Document
  ├── pages: Vec<Page>
  └── created by: DocumentBuilder

Page
  ├── cells: Rect<[[Cell; 160]; 51]>
  └── created by: PageBuilder

Cell
  ├── character: u8
  └── style: StyleFlags

Region (value type)
  ├── x, y, width, height: u16
  └── used by: PageBuilder, Widget

Widget (trait)
  ├── Label
  ├── TextBlock
  ├── Paragraph
  ├── ASCIIRect
  ├── KeyValueList
  └── Table

RenderState (internal)
  └── used by: escp::render_document()
```

---

## Entity Lifecycle Summary

1. **Cell**: Immutable value, created per-write, copied freely
2. **Region**: Immutable value, validated at creation, copied freely
3. **PageBuilder**: Mutable → build() → Page (immutable)
4. **DocumentBuilder**: Mutable → add_page() → build() → Document (immutable)
5. **Document**: Immutable → render() → Vec<u8> (ESC/P bytes)

---

## Validation Summary

| Type     | Validation Point | Invalid Input Handling                 |
| -------- | ---------------- | -------------------------------------- |
| Cell     | Constructor      | Non-ASCII → '?', control chars → empty |
| Region   | Constructor      | Out of bounds → LayoutError            |
| Page     | Build            | None (grid size fixed at compile time) |
| Document | Build            | None (empty document valid)            |
| Widget   | Render           | Out of bounds → silent truncation      |

---

## Memory Layout Summary

| Type       | Size (approx)   | Allocation      |
| ---------- | --------------- | --------------- |
| Cell       | 2 bytes         | Inline in array |
| StyleFlags | 1 byte          | Inline          |
| Region     | 8 bytes         | Stack           |
| Page       | ~16 KB          | Heap (Rect)     |
| Document   | 8 bytes + pages | Heap (Vec)      |

---

**Data model complete and validated against constitution principles.**

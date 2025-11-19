🔧 TECHNICAL DESIGN DOCUMENT (TDD)

# EPSON LQ-2090II Rust Layout Engine — V1

**Detailed Implementation Specification**

---

## Document Control

| Field                | Value                  |
| -------------------- | ---------------------- |
| **Document Version** | 1.0                    |
| **Product Version**  | V1.0                   |
| **Author**           | Mohammad AlMechkor     |
| **Status**           | Draft                  |
| **Classification**   | Internal - Engineering |
| **Date Created**     | 2025-01-18             |
| **Last Updated**     | 2025-01-18             |

### Related Documents

- **Product Requirements Document (PRD)**: `PRD.md` (v1.1)
- **API Specification**: [To be created]
- **Test Plan**: [To be created]

### Reviewers

| Role                     | Name  | Status      |
| ------------------------ | ----- | ----------- |
| **Lead Architect**       | [TBD] | [ ] Pending |
| **Senior Rust Engineer** | [TBD] | [ ] Pending |
| **Performance Engineer** | [TBD] | [ ] Pending |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Architecture Overview](#2-architecture-overview)
3. [Core Data Structures](#3-core-data-structures)
4. [Memory Layout & Optimization](#4-memory-layout--optimization)
5. [Builder Pattern Implementation](#5-builder-pattern-implementation)
6. [Region Tree System](#6-region-tree-system)
7. [Widget System](#7-widget-system)
8. [ESC/P Rendering Engine](#8-escp-rendering-engine)
9. [Error Handling Strategy](#9-error-handling-strategy)
10. [Performance Optimization](#10-performance-optimization)
11. [Testing Strategy](#11-testing-strategy)
12. [Implementation Phases](#12-implementation-phases)
13. [Appendices](#13-appendices)

---

## 1. Introduction

### 1.1 Purpose

This Technical Design Document (TDD) provides detailed implementation specifications for the EPSON LQ-2090II Rust Layout Engine V1. It translates product requirements from the PRD into concrete data structures, algorithms, and architectural decisions.

### 1.2 Scope

**In Scope**:

- Detailed Rust struct/enum/trait definitions
- Memory layout analysis and optimization
- Algorithm pseudocode and complexity analysis
- State machine implementations
- API lifetime design
- Performance optimization strategies

**Out of Scope**:

- Business requirements (see PRD)
- User documentation (see separate docs)
- Deployment procedures (see DevOps docs)

### 1.3 Audience

- Rust developers implementing the library
- Code reviewers
- Performance engineers
- QA engineers writing integration tests

### 1.4 Design Philosophy

1. **Zero-cost abstractions**: Builder pattern should compile to optimal code
2. **Memory efficiency**: Minimize allocations, predictable memory usage
3. **Type safety**: Use Rust's type system to prevent misuse
4. **Determinism**: Pure functional rendering (same input → same output)
5. **No panics**: All errors return `Result<T, LayoutError>`

---

## 2. Architecture Overview

### 2.1 Layered Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    PUBLIC API LAYER                          │
│  DocumentBuilder, PageBuilder, RegionHandle                 │
│  (Zero runtime cost, compile-time safety via lifetimes)     │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Builds & validates
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    CORE MODEL LAYER                          │
│  Document → Page → Cell[160×51]                             │
│  Region tree (hierarchical geometry)                         │
│  (Immutable after finalization)                              │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Renders to
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    WIDGET LAYER                              │
│  Label, Table, Paragraph, Rect, etc.                         │
│  (Implements Widget trait)                                   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Uses
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  RENDERING LAYER                             │
│  ESC/P byte stream generation                                │
│  Style state machine                                         │
│  (Pure function: Document → Vec<u8>)                        │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Module Structure

```
src/
├── lib.rs                  // Public API exports, module declarations
├── builder/
│   ├── mod.rs             // Re-exports
│   ├── document.rs        // DocumentBuilder
│   ├── page.rs            // PageBuilder
│   └── region.rs          // RegionHandle
├── core/
│   ├── mod.rs
│   ├── document.rs        // Document (immutable)
│   ├── page.rs            // Page (160×51 cells)
│   ├── cell.rs            // Cell (char + style)
│   ├── style.rs           // Style, StyleBits
│   └── region.rs          // Region tree node
├── widgets/
│   ├── mod.rs
│   ├── trait.rs           // Widget trait
│   ├── label.rs
│   ├── text_block.rs
│   ├── paragraph.rs
│   ├── rect_widget.rs
│   ├── key_value.rs
│   └── table.rs
├── renderer/
│   ├── mod.rs
│   ├── escp.rs            // EscpRenderer
│   └── state_machine.rs   // StyleStateMachine
├── error.rs               // LayoutError
└── utils/
    ├── mod.rs
    └── alignment.rs       // text_align(), word_wrap()
```

### 2.3 Dependency Graph

```
builder/*  ──┐
             ├──> core/*  ──┐
widgets/*  ──┘              ├──> renderer/*
                            │
error.rs  ──────────────────┘

utils/*  ───> (used by widgets/*, renderer/*)
```

**Key Principle**: No circular dependencies. Dependency flow is strictly one-directional.

---

## 3. Core Data Structures

### 3.1 Cell

**Purpose**: Represents a single character position on the page.

**Requirements**:

- Store ASCII character (32-126)
- Store style (bold, underline)
- Minimize memory footprint

#### Implementation

```rust
/// Represents a single cell in the 160×51 page grid.
///
/// Memory layout: 2 bytes per cell
/// - 1 byte: ASCII character (or space)
/// - 1 byte: Style bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// ASCII character (32-126), or 32 (space) for empty cells
    character: u8,

    /// Packed style bits
    /// Bit 0: bold
    /// Bit 1: underline
    /// Bits 2-7: reserved for future use
    style: StyleBits,
}

impl Cell {
    /// Creates an empty cell (space with no styling)
    #[inline]
    pub const fn empty() -> Self {
        Self {
            character: b' ',
            style: StyleBits::NORMAL,
        }
    }

    /// Creates a cell with the given character and style
    ///
    /// # Panics
    ///
    /// Panics in debug mode if character is non-ASCII printable
    #[inline]
    pub fn new(character: char, style: Style) -> Self {
        let char_byte = if character.is_ascii() && character >= ' ' {
            character as u8
        } else {
            b'?' // Non-ASCII replacement per FR-E3
        };

        debug_assert!(char_byte >= 32 && char_byte <= 126,
            "Invalid ASCII character: {}", char_byte);

        Self {
            character: char_byte,
            style: style.into(),
        }
    }

    #[inline]
    pub fn character(&self) -> char {
        self.character as char
    }

    #[inline]
    pub fn style(&self) -> Style {
        self.style.into()
    }
}

impl Default for Cell {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
```

**Memory Analysis**:

- Size: `2 bytes` per cell
- Total page memory: `2 bytes × 8,160 cells = 16,320 bytes ≈ 16 KB`

---

### 3.2 StyleBits

**Purpose**: Compact bit-packed representation of text styles.

````rust
/// Bit-packed style representation (1 byte)
///
/// Layout:
/// ```
/// Bit 0: Bold
/// Bit 1: Underline
/// Bits 2-7: Reserved (future: italic, strikethrough, etc.)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct StyleBits(u8);

impl StyleBits {
    pub const NORMAL: Self = Self(0b0000_0000);
    pub const BOLD: Self = Self(0b0000_0001);
    pub const UNDERLINE: Self = Self(0b0000_0010);
    pub const BOLD_UNDERLINE: Self = Self(0b0000_0011);

    const BOLD_BIT: u8 = 0b0000_0001;
    const UNDERLINE_BIT: u8 = 0b0000_0010;

    #[inline]
    pub const fn new(bold: bool, underline: bool) -> Self {
        let mut bits = 0u8;
        if bold {
            bits |= Self::BOLD_BIT;
        }
        if underline {
            bits |= Self::UNDERLINE_BIT;
        }
        Self(bits)
    }

    #[inline]
    pub const fn is_bold(self) -> bool {
        (self.0 & Self::BOLD_BIT) != 0
    }

    #[inline]
    pub const fn is_underline(self) -> bool {
        (self.0 & Self::UNDERLINE_BIT) != 0
    }

    #[inline]
    pub const fn raw(self) -> u8 {
        self.0
    }
}
````

---

### 3.3 Style

**Purpose**: User-facing style API (ergonomic).

```rust
/// Text styling options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub bold: bool,
    pub underline: bool,
}

impl Style {
    /// No styling
    pub const NORMAL: Self = Self { bold: false, underline: false };

    /// Bold only
    pub const BOLD: Self = Self { bold: true, underline: false };

    /// Underline only
    pub const UNDERLINE: Self = Self { bold: false, underline: true };

    /// Bold + Underline
    pub const BOLD_UNDERLINE: Self = Self { bold: true, underline: true };
}

impl Default for Style {
    fn default() -> Self {
        Self::NORMAL
    }
}

// Conversion to compact representation
impl From<Style> for StyleBits {
    #[inline]
    fn from(style: Style) -> Self {
        StyleBits::new(style.bold, style.underline)
    }
}

impl From<StyleBits> for Style {
    #[inline]
    fn from(bits: StyleBits) -> Self {
        Self {
            bold: bits.is_bold(),
            underline: bits.is_underline(),
        }
    }
}
```

---

### 3.4 Page

**Purpose**: Represents the 160×51 cell grid.

**Design Decision**: Use fixed-size array for maximum performance (stack allocation possible).

```rust
/// A fixed-size page of 160 columns × 51 rows
///
/// Memory layout: ~16 KB on stack or heap
pub struct Page {
    /// Cell storage: 2D array for cache-friendly access
    /// Row-major order: cells[y][x]
    cells: Rect<[[Cell; Self::WIDTH]; Self::HEIGHT]>,

    /// Optional metadata (page number, etc.)
    metadata: PageMetadata,
}

impl Page {
    pub const WIDTH: usize = 160;
    pub const HEIGHT: usize = 51;

    /// Creates a new empty page
    pub fn new() -> Self {
        Self {
            // Rect to avoid stack overflow (16KB)
            cells: Rect::new([[Cell::empty(); Self::WIDTH]; Self::HEIGHT]),
            metadata: PageMetadata::default(),
        }
    }

    /// Writes a character at (x, y) with given style
    ///
    /// Out-of-bounds writes are silently ignored (per FR-P3)
    #[inline]
    pub fn write_cell(&mut self, x: u16, y: u16, character: char, style: Style) {
        if let Some(cell) = self.get_cell_mut(x, y) {
            *cell = Cell::new(character, style);
        }
    }

    /// Gets a cell reference (read-only)
    #[inline]
    pub fn get_cell(&self, x: u16, y: u16) -> Option<&Cell> {
        if x < Self::WIDTH as u16 && y < Self::HEIGHT as u16 {
            Some(&self.cells[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Gets a mutable cell reference
    #[inline]
    fn get_cell_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x < Self::WIDTH as u16 && y < Self::HEIGHT as u16 {
            Some(&mut self.cells[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Writes a string starting at (x, y)
    ///
    /// Truncates at page/region boundaries
    pub fn write_text(&mut self, x: u16, y: u16, text: &str, style: Style) {
        let mut col = x;
        for ch in text.chars() {
            if col >= Self::WIDTH as u16 {
                break; // Horizontal truncation
            }
            self.write_cell(col, y, ch, style);
            col += 1;
        }
    }

    /// Iterates over all cells (for rendering)
    pub fn iter_cells(&self) -> impl Iterator<Item = (u16, u16, &Cell)> + '_ {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| {
                (x as u16, y as u16, cell)
            })
        })
    }
}

#[derive(Debug, Default)]
struct PageMetadata {
    page_number: Option<usize>,
}

impl Default for Page {
    fn default() -> Self {
        Self::new()
    }
}
```

**Performance Notes**:

- `Rect<[[Cell; 160]; 51]>` prevents stack overflow
- Row-major layout: `cells[y][x]` for cache-friendly iteration
- Inline hints for hot path (write_cell, get_cell)

---

### 3.5 Region

**Purpose**: Hierarchical geometry tree for layout composition.

**Design Decision**: Use tree structure with parent/child relationships.

```rust
/// A rectangular region within a page
///
/// Supports hierarchical nesting for complex layouts
#[derive(Debug)]
pub struct Region {
    /// Absolute coordinates on page
    x: u16,
    y: u16,

    /// Dimensions
    width: u16,
    height: u16,

    /// Optional padding (reduces inner usable area)
    padding: Padding,

    /// Default style for this region
    default_style: Style,

    /// Child regions (for nesting)
    children: Vec<Region>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Padding {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

impl Region {
    /// Creates a new region at (x, y) with given dimensions
    ///
    /// # Errors
    ///
    /// Returns `LayoutError::InvalidDimensions` if width or height is 0
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Result<Self, LayoutError> {
        if width == 0 || height == 0 {
            return Err(LayoutError::InvalidDimensions { width, height });
        }

        Ok(Self {
            x,
            y,
            width,
            height,
            padding: Padding::default(),
            default_style: Style::NORMAL,
            children: Vec::new(),
        })
    }

    /// Returns the inner usable area after padding
    pub fn inner_bounds(&self) -> (u16, u16, u16, u16) {
        let inner_x = self.x + self.padding.left;
        let inner_y = self.y + self.padding.top;
        let inner_width = self.width.saturating_sub(self.padding.left + self.padding.right);
        let inner_height = self.height.saturating_sub(self.padding.top + self.padding.bottom);

        (inner_x, inner_y, inner_width, inner_height)
    }

    /// Splits region horizontally with given ratios
    ///
    /// Example: ratios = [1, 2, 1] creates 3 regions (25%, 50%, 25%)
    pub fn split_horizontal(&mut self, ratios: &[u16]) -> Result<Vec<&mut Region>, LayoutError> {
        if ratios.is_empty() {
            return Err(LayoutError::InvalidSplitRatios {
                provided: 0,
                expected: 1
            });
        }

        let (inner_x, inner_y, inner_width, inner_height) = self.inner_bounds();
        let total_weight: u32 = ratios.iter().map(|&r| r as u32).sum();

        let mut x_offset = inner_x;
        let mut remaining_width = inner_width;

        for (i, &ratio) in ratios.iter().enumerate() {
            let is_last = i == ratios.len() - 1;

            let width = if is_last {
                remaining_width // Avoid rounding errors
            } else {
                ((inner_width as u32 * ratio as u32) / total_weight) as u16
            };

            let child = Region::new(x_offset, inner_y, width, inner_height)?;
            self.children.push(child);

            x_offset += width;
            remaining_width = remaining_width.saturating_sub(width);
        }

        // Return mutable references to children
        Ok(self.children.iter_mut().collect())
    }

    /// Splits region vertically with given ratios
    pub fn split_vertical(&mut self, ratios: &[u16]) -> Result<Vec<&mut Region>, LayoutError> {
        if ratios.is_empty() {
            return Err(LayoutError::InvalidSplitRatios {
                provided: 0,
                expected: 1
            });
        }

        let (inner_x, inner_y, inner_width, inner_height) = self.inner_bounds();
        let total_weight: u32 = ratios.iter().map(|&r| r as u32).sum();

        let mut y_offset = inner_y;
        let mut remaining_height = inner_height;

        for (i, &ratio) in ratios.iter().enumerate() {
            let is_last = i == ratios.len() - 1;

            let height = if is_last {
                remaining_height
            } else {
                ((inner_height as u32 * ratio as u32) / total_weight) as u16
            };

            let child = Region::new(inner_x, y_offset, inner_width, height)?;
            self.children.push(child);

            y_offset += height;
            remaining_height = remaining_height.saturating_sub(height);
        }

        Ok(self.children.iter_mut().collect())
    }

    /// Applies padding to this region
    pub fn with_padding(&mut self, top: u16, right: u16, bottom: u16, left: u16) {
        self.padding = Padding { top, right, bottom, left };
    }

    /// Sets the default style for this region
    pub fn set_default_style(&mut self, style: Style) {
        self.default_style = style;
    }
}
```

---

### 3.6 Document

**Purpose**: Immutable container for finalized pages.

```rust
/// An immutable document containing one or more pages
///
/// Thread-safe (Send + Sync)
pub struct Document {
    pages: Vec<Page>,
}

impl Document {
    /// Creates a document from a list of pages
    pub(crate) fn new(pages: Vec<Page>) -> Self {
        Self { pages }
    }

    /// Returns the number of pages
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Renders the document to ESC/P byte stream
    ///
    /// This is the primary output method
    pub fn render(&self) -> Vec<u8> {
        let renderer = EscpRenderer::new();
        renderer.render_document(self)
    }

    /// Internal: Iterate over pages
    pub(crate) fn pages(&self) -> &[Page] {
        &self.pages
    }
}

// Thread safety: Document is immutable, so safe to share
unsafe impl Send for Document {}
unsafe impl Sync for Document {}
```

---

## 4. Memory Layout & Optimization

### 4.1 Memory Budget Analysis

**Per Page**:

```
Cell storage: 160 × 51 × 2 bytes = 16,320 bytes
Metadata:                          ~8 bytes
─────────────────────────────────────────────
Total per page:                   ~16 KB
```

**Per Document (100 pages)**:

```
100 pages × 16 KB = 1.6 MB
Document overhead:  ~24 bytes (Vec header)
─────────────────────────────────────────────
Total:              ~1.6 MB
```

**Target**: < 1 MB per document → **EXCEEDED for 100 pages**

**Optimization**: Acceptable for V1 (PRD allows < 1MB for "reasonable" documents). For future optimization, consider page streaming or compression.

### 4.2 Cache-Friendly Layout

**Row-Major Order**:

```rust
cells: [[Cell; 160]; 51]  // cells[y][x]
```

**Rationale**:

- Rendering iterates line-by-line (y, then x)
- Row-major layout keeps consecutive x-values in the same cache line
- Cache line (64 bytes) holds 32 cells (64 / 2 bytes per cell)

**Cache Miss Analysis**:

```
Total cells: 8,160
Cache lines (64 bytes): 8,160 / 32 = 255 cache lines
Cold miss rate: 255 / 8,160 = 3.1%
```

### 4.3 Allocation Strategy

**Stack vs Heap**:

```rust
// ❌ Stack allocation would overflow (16 KB):
// cells: [[Cell; 160]; 51]

// ✅ Heap allocation via Rect:
cells: Rect<[[Cell; 160]; 51]>
```

**Benchmark Target**: Page allocation should be < 10 μs.

---

## 5. Builder Pattern Implementation

### 5.1 Lifetime Design

**Challenge**: Prevent dangling references between builder layers.

**Solution**: Use hierarchical lifetimes:

```
DocumentBuilder (owns data)
    ↓
PageBuilder<'doc> (borrows from DocumentBuilder)
    ↓
RegionHandle<'page> (borrows from PageBuilder)
```

### 5.2 DocumentBuilder

```rust
/// Builder for creating multi-page documents
pub struct DocumentBuilder {
    pages: Vec<Page>,
}

impl DocumentBuilder {
    /// Creates a new document builder
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
        }
    }

    /// Adds a new page and returns a builder for it
    pub fn add_page(&mut self) -> PageBuilder<'_> {
        let page = Page::new();
        self.pages.push(page);

        // Return builder that borrows the last page
        PageBuilder {
            page: self.pages.last_mut().unwrap(),
            root_region: None,
        }
    }

    /// Finalizes and returns the immutable document
    ///
    /// Consumes the builder
    pub fn build(self) -> Document {
        Document::new(self.pages)
    }
}

impl Default for DocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
```

### 5.3 PageBuilder

```rust
/// Builder for configuring a single page
///
/// Lifetime 'doc ties this builder to the parent DocumentBuilder
pub struct PageBuilder<'doc> {
    page: &'doc mut Page,
    root_region: Option<Region>,
}

impl<'doc> PageBuilder<'doc> {
    /// Returns a handle to the root region (160×51)
    pub fn root_region(&mut self) -> RegionHandle<'_> {
        // Initialize root region if not already done
        if self.root_region.is_none() {
            self.root_region = Some(
                Region::new(0, 0, Page::WIDTH as u16, Page::HEIGHT as u16)
                    .expect("Root region should always be valid")
            );
        }

        RegionHandle {
            page: self.page,
            region: self.root_region.as_mut().unwrap(),
        }
    }

    /// Finalizes the page (validates all regions)
    pub fn finalize(self) -> Result<(), LayoutError> {
        // Validation happens during region creation
        // This method exists for API symmetry and future use
        Ok(())
    }
}
```

### 5.4 RegionHandle

```rust
/// Handle for writing to a region
///
/// Lifetime 'page ties this handle to the parent PageBuilder
pub struct RegionHandle<'page> {
    page: &'page mut Page,
    region: &'page mut Region,
}

impl<'page> RegionHandle<'page> {
    /// Writes text at local coordinates (relative to region origin)
    pub fn write_text(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        style: Style
    ) -> &mut Self {
        let (region_x, region_y, width, height) = self.region.inner_bounds();

        // Validate local coordinates
        if x >= width || y >= height {
            return self; // Silent clipping per FR-T1, FR-T2
        }

        // Translate to absolute page coordinates
        let abs_x = region_x + x;
        let abs_y = region_y + y;

        // Write with truncation
        let max_len = (width - x) as usize;
        let truncated_text: String = text.chars().take(max_len).collect();

        self.page.write_text(abs_x, abs_y, &truncated_text, style);
        self
    }

    /// Convenience: Write a label with alignment
    pub fn label(&mut self, text: &str, alignment: Alignment) -> &mut Self {
        use crate::widgets::Label;

        let label = Label::new(text, alignment);
        let _ = label.render(self); // Ignore errors (truncation is not an error)
        self
    }

    /// Splits horizontally and returns handles to child regions
    pub fn split_horizontal(
        &mut self,
        ratios: &[u16]
    ) -> Result<Vec<RegionHandle<'_>>, LayoutError> {
        let child_regions = self.region.split_horizontal(ratios)?;

        // Create handles for each child
        Ok(child_regions.into_iter().map(|child_region| {
            RegionHandle {
                page: self.page,
                region: child_region,
            }
        }).collect())
    }

    /// Splits vertically and returns handles to child regions
    pub fn split_vertical(
        &mut self,
        ratios: &[u16]
    ) -> Result<Vec<RegionHandle<'_>>, LayoutError> {
        let child_regions = self.region.split_vertical(ratios)?;

        Ok(child_regions.into_iter().map(|child_region| {
            RegionHandle {
                page: self.page,
                region: child_region,
            }
        }).collect())
    }

    /// Applies padding
    pub fn with_padding(
        &mut self,
        top: u16,
        right: u16,
        bottom: u16,
        left: u16
    ) -> &mut Self {
        self.region.with_padding(top, right, bottom, left);
        self
    }

    /// Adds a widget to this region
    pub fn add_widget<W: Widget>(&mut self, widget: W) -> Result<(), LayoutError> {
        widget.render(self)
    }
}
```

**Lifetime Safety**:

```rust
// ✅ Valid: region handle borrows from page builder
let mut doc = DocumentBuilder::new();
let mut page = doc.add_page();
let mut region = page.root_region();
region.write_text(0, 0, "Hello", Style::NORMAL);

// ❌ Invalid: would cause compile error
// let region2 = page.root_region(); // Error: page already borrowed mutably
```

---

## 6. Region Tree System

### 6.1 Tree Structure

**Representation**:

```
Root Region (160×51)
├── Header (160×10)
│   ├── Logo Area (40×10)
│   └── Title Area (120×10)
├── Body (160×35)
│   ├── Left Sidebar (40×35)
│   └── Main Content (120×35)
└── Footer (160×6)
```

**In-Memory**:

```rust
Region {
    x: 0, y: 0, width: 160, height: 51,
    children: [
        Region { x: 0, y: 0, width: 160, height: 10, children: [...] },
        Region { x: 0, y: 10, width: 160, height: 35, children: [...] },
        Region { x: 0, y: 45, width: 160, height: 6, children: [] },
    ]
}
```

### 6.2 Coordinate Translation

**Algorithm**:

```
Absolute coordinates = Region origin + Local offset + Padding offset

Example:
  Region at (10, 5) with padding_left=2, padding_top=1
  Local write at (3, 2)
  → Absolute: (10 + 2 + 3, 5 + 1 + 2) = (15, 8)
```

**Implementation** (already shown in `RegionHandle::write_text`).

---

## 7. Widget System

### 7.1 Widget Trait

```rust
/// Trait for renderable content widgets
pub trait Widget {
    /// Renders the widget into the given region
    ///
    /// # Errors
    ///
    /// Returns `LayoutError` if widget cannot be rendered
    /// (e.g., region too small for minimum content)
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError>;
}
```

### 7.2 Label Widget

```rust
/// Single-line text with alignment
pub struct Label {
    text: String,
    alignment: Alignment,
    style: Style,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Label {
    pub fn new(text: impl Into<String>, alignment: Alignment) -> Self {
        Self {
            text: text.into(),
            alignment,
            style: Style::NORMAL,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Label {
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError> {
        let (_, _, width, height) = region.region.inner_bounds();

        if height == 0 {
            return Ok(()); // Silent truncation
        }

        let text = crate::utils::text_align(&self.text, width as usize, self.alignment);
        region.write_text(0, 0, &text, self.style);

        Ok(())
    }
}
```

### 7.3 Table Widget

```rust
/// Fixed-width column table
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    column_widths: Vec<u16>,
    style: TableStyle,
}

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

impl Table {
    /// Creates a new table with specified column widths
    ///
    /// # Panics
    ///
    /// Panics if column_widths is empty
    pub fn new(column_widths: Vec<u16>) -> Self {
        assert!(!column_widths.is_empty(), "Table must have at least one column");

        Self {
            headers: Vec::new(),
            rows: Vec::new(),
            column_widths,
            style: TableStyle::default(),
        }
    }

    pub fn with_headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }
}

impl Widget for Table {
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError> {
        let (_, _, width, height) = region.region.inner_bounds();
        let mut current_y = 0u16;

        // Render header if present
        if !self.headers.is_empty() && current_y < height {
            self.render_row(region, current_y, &self.headers, self.style.header_style);
            current_y += 1;

            // Separator line
            if self.style.draw_separators && current_y < height {
                let separator = "-".repeat(width as usize);
                region.write_text(0, current_y, &separator, Style::NORMAL);
                current_y += 1;
            }
        }

        // Render rows
        for row in &self.rows {
            if current_y >= height {
                break; // Vertical truncation
            }

            self.render_row(region, current_y, row, self.style.row_style);
            current_y += 1;
        }

        Ok(())
    }
}

impl Table {
    fn render_row(&self, region: &mut RegionHandle, y: u16, row: &[String], style: Style) {
        let mut x_offset = 0u16;

        for (i, cell_text) in row.iter().enumerate() {
            if i >= self.column_widths.len() {
                break;
            }

            let col_width = self.column_widths[i];
            let truncated: String = cell_text.chars().take(col_width as usize).collect();
            let padded = format!("{:<width$}", truncated, width = col_width as usize);

            region.write_text(x_offset, y, &padded, style);
            x_offset += col_width;
        }
    }
}
```

---

## 8. ESC/P Rendering Engine

### 8.1 Style State Machine

**Purpose**: Track current style state to minimize ESC/P command output.

**State Diagram**:

```
      ┌──────────────────────────────────────┐
      │         NORMAL (00)                  │
      └───┬────────────────────────────┬─────┘
          │                            │
    ESC E │                            │ ESC - 1
          ▼                            ▼
      ┌─────────┐                  ┌──────────┐
      │ BOLD(01)│◄─────────────────┤UNDER(10) │
      └────┬────┘     ESC E         └────┬─────┘
           │                             │
  ESC - 1  │                             │ ESC F
           ▼                             ▼
      ┌──────────────────────────────────────┐
      │      BOLD+UNDERLINE (11)             │
      └──────────────────────────────────────┘

Transitions emit ESC codes ONLY when style changes
```

**Implementation**:

```rust
/// Tracks current style state during rendering
#[derive(Debug)]
pub struct StyleStateMachine {
    current: StyleBits,
}

impl StyleStateMachine {
    pub fn new() -> Self {
        Self {
            current: StyleBits::NORMAL,
        }
    }

    /// Transitions to new style, returning necessary ESC/P commands
    ///
    /// Returns empty Vec if no transition needed
    pub fn transition_to(&mut self, target: StyleBits) -> Vec<u8> {
        if self.current == target {
            return Vec::new(); // No change
        }

        let mut commands = Vec::new();

        // Handle bold transition
        if self.current.is_bold() != target.is_bold() {
            if target.is_bold() {
                commands.extend_from_slice(&[0x1B, 0x45]); // ESC E
            } else {
                commands.extend_from_slice(&[0x1B, 0x46]); // ESC F
            }
        }

        // Handle underline transition
        if self.current.is_underline() != target.is_underline() {
            if target.is_underline() {
                commands.extend_from_slice(&[0x1B, 0x2D, 0x01]); // ESC - 1
            } else {
                commands.extend_from_slice(&[0x1B, 0x2D, 0x00]); // ESC - 0
            }
        }

        self.current = target;
        commands
    }

    /// Resets to normal style
    pub fn reset(&mut self) -> Vec<u8> {
        self.transition_to(StyleBits::NORMAL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_transition_when_same_style() {
        let mut sm = StyleStateMachine::new();
        let cmds = sm.transition_to(StyleBits::NORMAL);
        assert!(cmds.is_empty());
    }

    #[test]
    fn test_bold_on() {
        let mut sm = StyleStateMachine::new();
        let cmds = sm.transition_to(StyleBits::BOLD);
        assert_eq!(cmds, vec![0x1B, 0x45]); // ESC E
    }

    #[test]
    fn test_bold_off() {
        let mut sm = StyleStateMachine::new();
        sm.transition_to(StyleBits::BOLD);
        let cmds = sm.transition_to(StyleBits::NORMAL);
        assert_eq!(cmds, vec![0x1B, 0x46]); // ESC F
    }
}
```

### 8.2 ESC/P Renderer

```rust
/// Renders documents to ESC/P byte streams
pub struct EscpRenderer {
    // Stateless renderer (all state in render methods)
}

impl EscpRenderer {
    pub fn new() -> Self {
        Self {}
    }

    /// Renders a complete document to ESC/P bytes
    pub fn render_document(&self, document: &Document) -> Vec<u8> {
        let mut output = Vec::with_capacity(document.page_count() * 20_000);

        for page in document.pages() {
            self.render_page(page, &mut output);
        }

        output
    }

    /// Renders a single page
    fn render_page(&self, page: &Page, output: &mut Vec<u8>) {
        // 1. Reset printer
        output.extend_from_slice(&[0x1B, 0x40]); // ESC @

        // 2. Set condensed mode
        output.push(0x0F); // SI

        // 3. Render cells line by line
        let mut state_machine = StyleStateMachine::new();

        for y in 0..Page::HEIGHT as u16 {
            // Track if line is empty (optimization: skip empty lines)
            let mut has_content = false;

            for x in 0..Page::WIDTH as u16 {
                if let Some(cell) = page.get_cell(x, y) {
                    if cell.character() != ' ' || cell.style() != Style::NORMAL {
                        has_content = true;

                        // Emit style transitions
                        let style_cmds = state_machine.transition_to(cell.style().into());
                        output.extend_from_slice(&style_cmds);

                        // Emit character
                        output.push(cell.character() as u8);
                    } else {
                        // Emit space
                        output.push(b' ');
                    }
                }
            }

            // End of line
            output.push(0x0D); // CR
            output.push(0x0A); // LF
        }

        // 4. Form feed (page separator)
        output.push(0x0C); // FF
    }
}

impl Default for EscpRenderer {
    fn default() -> Self {
        Self::new()
    }
}
```

**Optimization Opportunities** (future):

- Skip trailing spaces on each line
- Compress runs of identical characters
- Use relative positioning (ESC $ for horizontal positioning)

---

## 9. Error Handling Strategy

### 9.1 Error Type

```rust
/// Errors that can occur during layout construction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    /// Region coordinates are out of bounds
    RegionOutOfBounds {
        x: u16,
        y: u16,
        max_x: u16,
        max_y: u16,
    },

    /// Invalid dimensions (width or height is 0)
    InvalidDimensions {
        width: u16,
        height: u16,
    },

    /// Builder was not finalized before use
    BuilderNotFinalized,

    /// Invalid split ratios provided
    InvalidSplitRatios {
        provided: usize,
        expected: usize,
    },

    /// Widget rendering failed (e.g., region too small)
    WidgetRenderError {
        widget_name: &'static str,
        reason: String,
    },
}

impl std::fmt::Display for LayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegionOutOfBounds { x, y, max_x, max_y } => {
                write!(f, "Region coordinates ({}, {}) exceed page bounds ({}, {})",
                    x, y, max_x, max_y)
            }
            Self::InvalidDimensions { width, height } => {
                write!(f, "Invalid dimensions: width={}, height={} (must be > 0)",
                    width, height)
            }
            Self::BuilderNotFinalized => {
                write!(f, "Builder must be finalized before use")
            }
            Self::InvalidSplitRatios { provided, expected } => {
                write!(f, "Invalid split ratios: provided {}, expected at least {}",
                    provided, expected)
            }
            Self::WidgetRenderError { widget_name, reason } => {
                write!(f, "Widget '{}' failed to render: {}", widget_name, reason)
            }
        }
    }
}

impl std::error::Error for LayoutError {}
```

### 9.2 Error Propagation Pattern

```rust
// Example: Validating region creation
pub fn create_subregion(
    parent: &Region,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) -> Result<Region, LayoutError> {
    // Validate dimensions
    if width == 0 || height == 0 {
        return Err(LayoutError::InvalidDimensions { width, height });
    }

    // Validate bounds
    if x + width > parent.width || y + height > parent.height {
        return Err(LayoutError::RegionOutOfBounds {
            x: x + width,
            y: y + height,
            max_x: parent.width,
            max_y: parent.height,
        });
    }

    Ok(Region::new(x, y, width, height)?)
}
```

---

## 10. Performance Optimization

### 10.1 Benchmark Targets

| Operation                      | Target (p99) | Measurement |
| ------------------------------ | ------------ | ----------- |
| Page::new()                    | < 10 μs      | `criterion` |
| Page::write_cell()             | < 50 ns      | `criterion` |
| Region::split_horizontal()     | < 1 μs       | `criterion` |
| Document::render() (1 page)    | < 100 μs     | `criterion` |
| Document::render() (100 pages) | < 10 ms      | `criterion` |

### 10.2 Hot Path Optimization

**Inline Hints**:

```rust
#[inline]
pub fn write_cell(&mut self, x: u16, y: u16, character: char, style: Style) {
    // ... implementation
}
```

**Branch Prediction**:

```rust
// ✅ Fast path first (common case)
if x < Self::WIDTH as u16 && y < Self::HEIGHT as u16 {
    // Write cell (hot path)
} else {
    // Out of bounds (cold path)
}
```

### 10.3 Memory Allocation Minimization

**Pre-allocation**:

```rust
// Estimate output size: ~200 bytes per line × 51 lines = ~10 KB
let mut output = Vec::with_capacity(document.page_count() * 10_240);
```

**Avoid Allocations in Loops**:

```rust
// ❌ Bad: Allocates on every iteration
for y in 0..51 {
    let line = format!("{}", y); // Allocation!
}

// ✅ Good: Reuse buffer
let mut buffer = String::with_capacity(160);
for y in 0..51 {
    buffer.clear();
    // ... use buffer
}
```

### 10.4 Profiling Strategy

**Tools**:

- `cargo-flamegraph` for CPU profiling
- `heaptrack` or `valgrind --tool=massif` for memory profiling
- `perf` on Linux

**Profiling Commands**:

```bash
# CPU profiling
cargo flamegraph --bench rendering

# Memory profiling (Linux)
valgrind --tool=massif --massif-out-file=massif.out \
    target/release/examples/large_document

# Benchmark with profiling
cargo bench --bench rendering -- --profile-time=10
```

---

## 11. Testing Strategy

### 11.1 Unit Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_creation() {
        let page = Page::new();
        assert_eq!(page.WIDTH, 160);
        assert_eq!(page.HEIGHT, 51);
    }

    #[test]
    fn test_write_cell_in_bounds() {
        let mut page = Page::new();
        page.write_cell(0, 0, 'A', Style::BOLD);

        let cell = page.get_cell(0, 0).unwrap();
        assert_eq!(cell.character(), 'A');
        assert_eq!(cell.style(), Style::BOLD);
    }

    #[test]
    fn test_write_cell_out_of_bounds() {
        let mut page = Page::new();
        page.write_cell(200, 100, 'X', Style::NORMAL);
        // Should not panic, just silently ignore
    }

    #[test]
    fn test_non_ascii_replacement() {
        let cell = Cell::new('🦀', Style::NORMAL);
        assert_eq!(cell.character(), '?');
    }
}
```

### 11.2 Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_write_never_panics(
        x in 0u16..200,
        y in 0u16..100,
        c in any::<char>(),
    ) {
        let mut page = Page::new();
        page.write_cell(x, y, c, Style::NORMAL);
        // Should never panic
    }

    #[test]
    fn test_region_split_sum_equals_total(
        ratios in prop::collection::vec(1u16..10, 1..10)
    ) {
        let mut region = Region::new(0, 0, 100, 50).unwrap();
        let children = region.split_horizontal(&ratios).unwrap();

        let total_width: u16 = children.iter()
            .map(|r| r.width)
            .sum();

        assert_eq!(total_width, 100);
    }
}
```

### 11.3 Golden Master Testing

```rust
#[test]
fn test_golden_master_simple_invoice() {
    let doc = create_simple_invoice(); // Helper function
    let output = doc.render();

    let expected = include_bytes!("../golden_masters/simple_invoice.escp");
    assert_eq!(output, expected);
}

#[test]
fn test_deterministic_rendering() {
    let doc = create_complex_document();

    let render1 = doc.render();
    let render2 = doc.render();
    let render3 = doc.render();

    assert_eq!(render1, render2);
    assert_eq!(render2, render3);
}
```

---

## 12. Implementation Phases

### Phase 1: Core Foundation (Week 1-2)

**Goal**: Basic page model and rendering

**Tasks**:

- [ ] Implement `Cell`, `StyleBits`, `Style`
- [ ] Implement `Page` with basic write operations
- [ ] Implement `EscpRenderer` (basic line-by-line rendering)
- [ ] Unit tests for core types (coverage > 90%)

**Deliverable**: Can create a page, write text, render to ESC/P

---

### Phase 2: Builder API (Week 3)

**Goal**: Ergonomic builder pattern

**Tasks**:

- [ ] Implement `DocumentBuilder`
- [ ] Implement `PageBuilder`
- [ ] Implement `RegionHandle` (basic write operations)
- [ ] Lifetime safety verification (compile-time tests)

**Deliverable**: Can build documents using builder API

---

### Phase 3: Region System (Week 4)

**Goal**: Hierarchical layout

**Tasks**:

- [ ] Implement `Region` tree structure
- [ ] Implement `split_horizontal()`, `split_vertical()`
- [ ] Implement padding
- [ ] Integration tests for nested regions

**Deliverable**: Can create complex layouts with nested regions

---

### Phase 4: Widget System (Week 5-6)

**Goal**: Reusable content components

**Tasks**:

- [ ] Define `Widget` trait
- [ ] Implement `Label`
- [ ] Implement `Table`
- [ ] Implement `Paragraph` (word-wrapping)
- [ ] Implement `Rect`, `TextBlock`, `KeyValue`
- [ ] Widget integration tests

**Deliverable**: All 6 widgets implemented and tested

---

### Phase 5: Style State Machine (Week 7)

**Goal**: Optimized ESC/P output

**Tasks**:

- [ ] Implement `StyleStateMachine`
- [ ] Integrate with `EscpRenderer`
- [ ] Unit tests for all state transitions
- [ ] Verify minimal ESC/P output

**Deliverable**: Renderer emits optimal ESC/P sequences

---

### Phase 6: Testing & Validation (Week 8)

**Goal**: Hardware validation and benchmarking

**Tasks**:

- [ ] Create 10 hardware test forms
- [ ] Print on EPSON LQ-2090II
- [ ] Visual inspection and alignment verification
- [ ] Create golden master test suite (20+ files)
- [ ] Performance benchmarks (criterion)
- [ ] Fuzzing (cargo-fuzz, 1M iterations)

**Deliverable**: All acceptance criteria met

---

### Phase 7: Documentation & Polish (Week 9)

**Goal**: Production-ready release

**Tasks**:

- [ ] 100% rustdoc coverage
- [ ] 5 runnable examples
- [ ] README with quickstart
- [ ] Troubleshooting guide
- [ ] Architecture decision records
- [ ] CHANGELOG.md

**Deliverable**: Ready for v1.0.0 release

---

## 13. Appendices

### 13.1 Benchmark Code Template

```rust
use criterion::{black_rect, criterion_group, criterion_main, Criterion};
use epson_lq2090_layout::*;

fn bench_page_creation(c: &mut Criterion) {
    c.bench_function("Page::new()", |b| {
        b.iter(|| {
            black_rect(Page::new())
        });
    });
}

fn bench_single_page_render(c: &mut Criterion) {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();
    page.root_region().write_text(0, 0, "Test", Style::NORMAL);
    let doc = doc.build();

    c.bench_function("Document::render() [1 page]", |b| {
        b.iter(|| {
            black_rect(doc.render())
        });
    });
}

criterion_group!(benches, bench_page_creation, bench_single_page_render);
criterion_main!(benches);
```

### 13.2 Fuzz Target Template

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;
use epson_lq2090_layout::*;

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }

    let x = u16::from_le_bytes([data[0], data[1]]);
    let y = u16::from_le_bytes([data[2], data[3]]);
    let text = String::from_utf8_lossy(&data[4..]);

    let mut page = Page::new();
    page.write_text(x, y, &text, Style::NORMAL);
    // Should never panic
});
```

### 13.3 CI Configuration Snippet

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, "1.75.0", nightly]

    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@${{ matrix.rust }}

      - name: Run tests
        run: cargo test --all-features

      - name: Run benchmarks
        run: cargo bench --no-run

      - name: Check formatting
        run: cargo fmt --check

      - name: Run clippy
        run: cargo clippy -- -D warnings
```

---

## 🔒 End of Technical Design Document

**Document Status**: ✅ Ready for Engineering Review

**Next Steps**:

1. Architecture review with senior engineers
2. Approve/revise data structure designs
3. Begin Phase 1 implementation
4. Set up CI/CD pipeline

---

**For technical questions, contact:**
**Author**: Mohammad AlMechkor
**Document Location**: `/Users/mohammadalmechkor/Projects/matrix/specs/TDD.md`

---

# Research Document: Rust ESC/P Layout Engine

**Branch**: `001-rust-escap-layout-engine` | **Date**: 2025-11-18

## Phase 0: Research & Technical Decisions

This document captures technical research and architectural decisions made during planning. Since the feature specification is comprehensive and all requirements are clearly defined, this phase focuses on best practices, implementation patterns, and technology choices.

## Key Technical Decisions

### Decision 1: Memory Layout for Cell Grid

**Decision**: Use row-major contiguous `Rect<[[Cell; 160]; 51]>` for page storage

**Rationale**:

- **Cache efficiency**: Row-major layout matches line-by-line rendering pattern (reading left-to-right, top-to-bottom)
- **Fixed size**: Array provides compile-time size guarantees (160 × 51 = 8,160 cells)
- **Zero dynamic allocation**: No Vec growth or reallocation during rendering
- **Bounds checking**: Rust array indexing provides automatic bounds checking
- **Rect allocation**: Prevents stack overflow (8KB+ structure) while maintaining contiguity

**Alternatives considered**:

- `Vec<Cell>` with manual indexing: Rejected due to dynamic allocation overhead
- Column-major layout: Rejected due to poor cache locality for line rendering
- Flat array with index calculation: Considered equivalent, but 2D array syntax is clearer

**Implementation notes**:

```rust
pub struct Page {
    cells: Rect<[[Cell; PAGE_WIDTH]; PAGE_HEIGHT]>,
    finalized: bool,
}

const PAGE_WIDTH: usize = 160;
const PAGE_HEIGHT: usize = 51;
```

---

### Decision 2: Cell Structure and Bit Packing

**Decision**: Use compact `Cell` struct with bit-packed style flags

**Rationale**:

- **Memory efficiency**: Target 2 bytes per cell (1 byte char + 1 byte flags)
- **Style storage**: Bold and underline fit in 2 bits, leaving room for future expansion
- **Alignment**: Struct naturally aligns to 2-byte boundary
- **Copy semantics**: Small size makes `Copy` trait viable (efficient pass-by-value)

**Implementation**:

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub character: u8,  // ASCII only (32-126), 0 = empty
    pub style: StyleFlags,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StyleFlags(u8);

impl StyleFlags {
    pub const BOLD: u8 = 0b0000_0001;
    pub const UNDERLINE: u8 = 0b0000_0010;
    // Bits 2-7 reserved for future styles
}
```

**Alternatives considered**:

- Separate bool fields: Rejected due to padding overhead (4+ bytes per cell)
- Single u16 with char + styles: Rejected due to endianness concerns and less clear API
- Enum for styles: Rejected due to inability to combine styles (bold + underline)

---

### Decision 3: Region Representation

**Decision**: Use lightweight `Region` struct with copy semantics

**Rationale**:

- **Zero-copy views**: Regions are coordinate ranges, not data copies
- **Stack allocation**: Small struct (4 × u16 = 8 bytes) stays on stack
- **No lifetimes needed**: Copy semantics eliminate borrow checker complexity
- **Validation**: Bounds checking at creation time ensures validity

**Implementation**:

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Region {
    pub x: u16,      // Column start (0-159)
    pub y: u16,      // Row start (0-50)
    pub width: u16,  // Column count
    pub height: u16, // Row count
}

impl Region {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Result<Self, LayoutError> {
        if x + width > PAGE_WIDTH as u16 || y + height > PAGE_HEIGHT as u16 {
            return Err(LayoutError::RegionOutOfBounds);
        }
        Ok(Region { x, y, width, height })
    }

    pub fn split_vertical(&self, top_height: u16) -> Result<(Region, Region), LayoutError> {
        // Validation and split logic
    }

    pub fn split_horizontal(&self, left_width: u16) -> Result<(Region, Region), LayoutError> {
        // Validation and split logic
    }
}
```

**Alternatives considered**:

- Reference-based regions with lifetimes: Rejected due to API complexity
- Regions storing cell slices: Rejected due to inability to represent non-contiguous sub-regions
- Separate RegionBuilder: Rejected as overkill for simple validation

---

### Decision 4: Builder API Architecture

**Decision**: Use consuming builder pattern with phantom types for state enforcement

**Rationale**:

- **Compile-time safety**: Type state pattern prevents invalid API usage
- **Ergonomics**: Method chaining provides fluent API
- **Zero-cost abstraction**: Phantom types compile away (zero runtime overhead)
- **Clear lifecycle**: Distinct Building → Finalized states

**Implementation**:

```rust
// Type state pattern
pub struct Building;
pub struct Finalized;

pub struct DocumentBuilder<State = Building> {
    pages: Vec<Page>,
    _state: PhantomData<State>,
}

impl DocumentBuilder<Building> {
    pub fn new() -> Self { /* ... */ }
    pub fn add_page(&mut self, page: Page) -> &mut Self { /* ... */ }
    pub fn build(self) -> Document { /* consumes builder */ }
}

pub struct PageBuilder<'doc, State = Building> {
    cells: Rect<[[Cell; PAGE_WIDTH]; PAGE_HEIGHT]>,
    _state: PhantomData<State>,
    _lifetime: PhantomData<&'doc ()>,
}

impl<'doc> PageBuilder<'doc, Building> {
    pub fn new() -> Self { /* ... */ }
    pub fn write_at(&mut self, x: u16, y: u16, ch: char) -> &mut Self { /* ... */ }
    pub fn render_widget(&mut self, region: Region, widget: &dyn Widget) -> &mut Self { /* ... */ }
    pub fn build(self) -> Page { /* ... */ }
}
```

**Alternatives considered**:

- Runtime state flags: Rejected due to missing compile-time guarantees
- Separate builder and finalized types: Adopted (type state pattern)
- Mutable Document: Rejected due to immutability requirements (Constitution IV)

---

### Decision 5: ESC/P Rendering State Machine

**Decision**: Implement explicit style state machine with minimal transitions

**Rationale**:

- **Deterministic output**: Explicit state tracking ensures byte-for-byte reproducibility
- **Optimization**: Track current style state to emit only necessary state changes
- **Correctness**: Guarantee style reset at line/page boundaries
- **Testability**: State machine logic isolated and unit testable

**Implementation**:

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct RenderState {
    bold: bool,
    underline: bool,
}

impl RenderState {
    fn new() -> Self {
        RenderState { bold: false, underline: false }
    }

    fn transition_to(&mut self, target: StyleFlags, output: &mut Vec<u8>) {
        // Emit ESC/P codes only for style changes
        if target.bold() != self.bold {
            output.extend_from_slice(if target.bold() { ESC_BOLD_ON } else { ESC_BOLD_OFF });
            self.bold = target.bold();
        }
        if target.underline() != self.underline {
            output.extend_from_slice(if target.underline() { ESC_UNDERLINE_ON } else { ESC_UNDERLINE_OFF });
            self.underline = target.underline();
        }
    }

    fn reset(&mut self, output: &mut Vec<u8>) {
        self.transition_to(StyleFlags::empty(), output);
    }
}
```

**Alternatives considered**:

- Emit all style codes every time: Rejected due to bloated output size
- Stateless rendering: Rejected due to inability to optimize redundant codes
- Global style state: Rejected due to non-determinism risks

---

### Decision 6: Widget Trait Design

**Decision**: Use trait objects with `&dyn Widget` for runtime polymorphism

**Rationale**:

- **Flexibility**: Users can create custom widgets by implementing trait
- **Type erasure**: Allows heterogeneous widget collections
- **No generics explosion**: Avoids monomorphization of every widget combination
- **Clear contract**: Single `render()` method defines widget behavior

**Implementation**:

```rust
pub trait Widget {
    fn render(&self, page: &mut PageBuilder, region: Region);
}

// Built-in widgets
pub struct Label { text: String, style: StyleFlags }
pub struct TextBlock { lines: Vec<String> }
pub struct Paragraph { text: String, style: StyleFlags }
pub struct ASCIIRect { title: Option<String>, content: Rect<dyn Widget> }
pub struct KeyValueList { entries: Vec<(String, String)> }
pub struct Table { columns: Vec<ColumnDef>, rows: Vec<Vec<String>> }

impl Widget for Label {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        // Render single-line text with truncation
    }
}

// Similar implementations for other widgets
```

**Alternatives considered**:

- Enum of widget types: Rejected due to inability to support custom user widgets
- Generic `impl Widget for T`: Rejected due to type parameter complexity
- Separate render functions: Rejected due to lack of extensibility

---

### Decision 7: Error Handling Strategy

**Decision**: Use `Result<T, LayoutError>` for recoverable errors, silent truncation for overflow

**Rationale**:

- **Geometry errors are recoverable**: Invalid dimensions should be caught at build time
- **Overflow is not an error**: Constitution Principle III mandates silent truncation
- **Clear API contract**: Errors documented via Result type
- **No panics**: Constitution Principle IX forbids panics in release builds

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
            LayoutError::RegionOutOfBounds => write!(f, "Region exceeds page bounds (160×51)"),
            LayoutError::InvalidDimensions => write!(f, "Width or height cannot be zero"),
            LayoutError::InvalidSplit => write!(f, "Split dimensions exceed parent region"),
        }
    }
}

impl std::error::Error for LayoutError {}
```

**Alternatives considered**:

- Panic on errors: Rejected due to Constitution requirement (zero panics)
- Return Option: Rejected due to loss of error context
- Custom error types per module: Rejected as overkill for small error surface

---

### Decision 8: Testing Strategy

**Decision**: Multi-layered testing with unit, integration, property-based, fuzzing, and golden master tests

**Rationale**:

- **Unit tests**: Fast feedback, high coverage of individual functions
- **Integration tests**: End-to-end validation of API contracts
- **Property-based tests**: Verify invariants hold for all inputs (determinism, no panics, truncation correctness)
- **Fuzzing**: Detect crashes and panics with arbitrary inputs
- **Golden master tests**: Byte-level ESC/P output verification

**Implementation**:

```toml
# Cargo.toml dev-dependencies
[dev-dependencies]
proptest = "1.4"
criterion = "0.5"

[profile.release]
lto = true
codegen-units = 1
```

**Test structure**:

```
tests/
├── unit/
│   ├── cell_tests.rs
│   ├── region_tests.rs
│   ├── page_tests.rs
│   └── escp_tests.rs
├── integration/
│   ├── single_page_tests.rs
│   ├── multi_page_tests.rs
│   ├── widget_tests.rs
│   └── nested_region_tests.rs
├── property/
│   ├── determinism_tests.rs
│   ├── truncation_tests.rs
│   └── no_panic_tests.rs
├── golden/
│   ├── invoice.bin
│   ├── report.bin
│   └── complex_layout.bin
└── benches/
    ├── render_bench.rs
    └── widget_bench.rs
```

**Alternatives considered**:

- Unit tests only: Rejected due to insufficient coverage of integration behavior
- Snapshot testing instead of golden files: Rejected due to binary ESC/P format requirements
- Manual hardware testing only: Rejected due to slow feedback loop

---

### Decision 9: Module Structure

**Decision**: Organize code into clear domain-focused modules

**Rationale**:

- **Separation of concerns**: Layout, widgets, and ESC/P rendering are distinct domains
- **Testability**: Isolated modules enable focused unit tests
- **Maintainability**: Clear boundaries reduce coupling
- **Future extensibility**: New widgets or ESC/P features can be added in existing modules

**Structure**:

```
src/
├── lib.rs              // Public API exports
├── cell.rs             // Cell and StyleFlags types
├── page.rs             // Page and PageBuilder
├── document.rs         // Document and DocumentBuilder
├── region.rs           // Region geometry and operations
├── widgets/
│   ├── mod.rs          // Widget trait and shared utilities
│   ├── label.rs        // Label widget
│   ├── text_block.rs   // TextBlock widget
│   ├── paragraph.rs    // Paragraph widget
│   ├── ascii_rect.rs    // ASCIIRect widget
│   ├── key_value.rs    // KeyValueList widget
│   └── table.rs        // Table widget
├── escp/
│   ├── mod.rs          // ESC/P public interface
│   ├── renderer.rs     // Main rendering logic
│   ├── state.rs        // RenderState state machine
│   └── constants.rs    // ESC/P command byte sequences
└── error.rs            // LayoutError type
```

**Alternatives considered**:

- Flat module structure: Rejected due to poor organization
- Feature-based modules (builders/, rendering/): Rejected due to cross-cutting concerns
- Separate crates for widgets: Rejected as premature optimization for V1

---

### Decision 10: ESC/P Command Reference

**Decision**: Use const byte arrays for ESC/P commands with inline documentation

**Rationale**:

- **Correctness**: Hard-coded byte sequences eliminate transcription errors
- **Readability**: Named constants self-document command purpose
- **Testability**: Easy to verify against ESC/P specification
- **Performance**: Compile-time constants have zero overhead

**Implementation**:

```rust
// src/escp/constants.rs

/// ESC @ - Printer reset (0x1B 0x40)
pub const ESC_RESET: &[u8] = &[0x1B, 0x40];

/// SI - Condensed mode on (0x0F)
pub const SI_CONDENSED: &[u8] = &[0x0F];

/// ESC E - Bold on (0x1B 0x45)
pub const ESC_BOLD_ON: &[u8] = &[0x1B, 0x45];

/// ESC F - Bold off (0x1B 0x46)
pub const ESC_BOLD_OFF: &[u8] = &[0x1B, 0x46];

/// ESC - 1 - Underline on (0x1B 0x2D 0x01)
pub const ESC_UNDERLINE_ON: &[u8] = &[0x1B, 0x2D, 0x01];

/// ESC - 0 - Underline off (0x1B 0x2D 0x00)
pub const ESC_UNDERLINE_OFF: &[u8] = &[0x1B, 0x2D, 0x00];

/// CR - Carriage return (0x0D)
pub const CR: u8 = 0x0D;

/// LF - Line feed (0x0A)
pub const LF: u8 = 0x0A;

/// FF - Form feed / Page separator (0x0C)
pub const FF: u8 = 0x0C;
```

**Reference**: EPSON LQ-2090II ESC/P2 Reference Manual, Text Mode Commands Section

**Alternatives considered**:

- Runtime string encoding: Rejected due to performance overhead
- Macro generation: Rejected as unnecessary complexity
- Hex literals in code: Rejected due to poor readability

---

## Best Practices Research

### Rust API Design Guidelines

**Source**: Rust API Guidelines (https://rust-lang.github.io/api-guidelines/)

**Applied principles**:

- **C-CONV**: Follow naming conventions (snake_case for functions, PascalCase for types)
- **C-EXAMPLE**: All public APIs have rustdoc examples
- **C-FAILURE**: Errors use Result type and implement std::error::Error
- **C-DEBUG**: All types implement Debug
- **C-CLONE**: Small types implement Copy, larger types implement Clone
- **C-SEND-SYNC**: Document thread safety (Document is Send + Sync)

### Zero-Copy Patterns

**Technique**: Use copy-on-write semantics and lightweight views

**Applied to**:

- **Regions**: Copy semantics avoid lifetime complexity
- **Cell access**: Direct array indexing (no intermediate buffers)
- **ESC/P rendering**: Write directly to output Vec<u8> (no intermediate strings)

**Performance impact**: Eliminates allocation overhead in rendering hot path

### Property-Based Testing for Determinism

**Framework**: proptest

**Properties to test**:

1. **Idempotence**: `render(doc) == render(doc)` for all doc
2. **Commutativity**: Page order matters, but identical pages produce identical output
3. **Truncation**: No panic for any (x, y, text) combination
4. **Bounds**: All output indices stay within [0, 160) × [0, 51)

**Example**:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn deterministic_rendering(text: String) {
        let doc = create_document_with_text(&text);
        let output1 = doc.render();
        let output2 = doc.render();
        prop_assert_eq!(output1, output2);
    }

    #[test]
    fn no_panic_on_any_region(x in 0u16..200, y in 0u16..60, w in 0u16..200, h in 0u16..60) {
        // Should either succeed or return Err, never panic
        let result = Region::new(x, y, w, h);
        // Test passes if we reach this line (no panic occurred)
    }
}
```

---

## Open Questions (None)

All technical requirements are fully specified in the feature spec. No unresolved questions remain.

---

## Research Validation

✅ All decisions documented with rationale
✅ Alternatives considered for major choices
✅ No NEEDS CLARIFICATION items remaining
✅ Best practices research completed
✅ Constitution compliance verified

**Phase 0 complete. Proceeding to Phase 1: Design & Contracts**

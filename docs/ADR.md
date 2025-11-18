📐 ARCHITECTURE DECISION RECORDS (ADR)

# EPSON LQ-2090II Rust Layout Engine — V1

**Key Architectural Decisions**

---

## Document Control

| Field | Value |
|-------|-------|
| **Document Version** | 1.0 |
| **Product Version** | V1.0 |
| **Author** | Mohammad AlMechkor |
| **Status** | Living Document |
| **Classification** | Internal - Architecture |
| **Date Created** | 2025-01-18 |
| **Last Updated** | 2025-01-18 |

### Related Documents

- **Product Requirements Document (PRD)**: `PRD.md` (v1.1)
- **Technical Design Document (TDD)**: `TDD.md` (v1.0)
- **API Specification**: `API-SPEC.md` (v1.0)

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [ADR Index](#2-adr-index)
3. [ADR-001: Static Layout Only (V1)](#adr-001-static-layout-only-v1)
4. [ADR-002: Manual Pagination](#adr-002-manual-pagination)
5. [ADR-003: Builder Pattern with Lifetimes](#adr-003-builder-pattern-with-lifetimes)
6. [ADR-004: Row-Major Memory Layout](#adr-004-row-major-memory-layout)
7. [ADR-005: Bit-Packed Style Storage](#adr-005-bit-packed-style-storage)
8. [ADR-006: Silent Truncation (No Errors)](#adr-006-silent-truncation-no-errors)
9. [ADR-007: Zero Runtime Dependencies](#adr-007-zero-runtime-dependencies)
10. [ADR-008: ESC/P Text Mode Only](#adr-008-escp-text-mode-only)
11. [ADR-009: Deterministic Rendering](#adr-009-deterministic-rendering)
12. [ADR-010: Fixed Page Dimensions](#adr-010-fixed-page-dimensions)
13. [ADR-011: Immutable Document Model](#adr-011-immutable-document-model)
14. [ADR-012: Widget Trait Design](#adr-012-widget-trait-design)

---

## 1. Introduction

### 1.1 Purpose

This document captures all significant architectural decisions made during the design and implementation of the EPSON LQ-2090II Rust Layout Engine V1. Each decision is documented with its context, rationale, consequences, and alternatives considered.

### 1.2 ADR Format

Each ADR follows this structure:

- **Status**: Proposed | Accepted | Deprecated | Superseded
- **Context**: What is the issue we're seeing that is motivating this decision?
- **Decision**: What is the change we're proposing and/or doing?
- **Consequences**: What becomes easier or more difficult to do because of this change?
- **Alternatives Considered**: What other options were evaluated?

### 1.3 When to Create an ADR

Create an ADR when making decisions that:
- Affect the public API
- Impact performance or memory usage
- Constrain future development
- Involve significant trade-offs
- Are difficult to reverse

---

## 2. ADR Index

| ADR | Title | Status | Date | Supersedes |
|-----|-------|--------|------|------------|
| [ADR-001](#adr-001-static-layout-only-v1) | Static Layout Only (V1) | ✅ Accepted | 2025-01-18 | - |
| [ADR-002](#adr-002-manual-pagination) | Manual Pagination | ✅ Accepted | 2025-01-18 | - |
| [ADR-003](#adr-003-builder-pattern-with-lifetimes) | Builder Pattern with Lifetimes | ✅ Accepted | 2025-01-18 | - |
| [ADR-004](#adr-004-row-major-memory-layout) | Row-Major Memory Layout | ✅ Accepted | 2025-01-18 | - |
| [ADR-005](#adr-005-bit-packed-style-storage) | Bit-Packed Style Storage | ✅ Accepted | 2025-01-18 | - |
| [ADR-006](#adr-006-silent-truncation-no-errors) | Silent Truncation (No Errors) | ✅ Accepted | 2025-01-18 | - |
| [ADR-007](#adr-007-zero-runtime-dependencies) | Zero Runtime Dependencies | ✅ Accepted | 2025-01-18 | - |
| [ADR-008](#adr-008-escp-text-mode-only) | ESC/P Text Mode Only | ✅ Accepted | 2025-01-18 | - |
| [ADR-009](#adr-009-deterministic-rendering) | Deterministic Rendering | ✅ Accepted | 2025-01-18 | - |
| [ADR-010](#adr-010-fixed-page-dimensions) | Fixed Page Dimensions | ✅ Accepted | 2025-01-18 | - |
| [ADR-011](#adr-011-immutable-document-model) | Immutable Document Model | ✅ Accepted | 2025-01-18 | - |
| [ADR-012](#adr-012-widget-trait-design) | Widget Trait Design | ✅ Accepted | 2025-01-18 | - |

---

## ADR-001: Static Layout Only (V1)

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Engineering Team
**Tags**: #layout #scope #v1

---

### Context

We need to decide whether V1 should support:
1. **Static layout**: Fixed dimensions specified by developer
2. **Dynamic layout**: Auto-sizing regions based on content
3. **Hybrid**: Both static and dynamic

**Business Context**:
- Target users need predictable output for regulatory forms
- Development timeline is constrained (9 weeks)
- ESC/P printers don't support reflowing or dynamic layout

**Technical Context**:
- Dynamic layout requires complex constraint solvers
- Auto-sizing needs content measurement before rendering
- Regulatory forms require exact positioning

---

### Decision

**V1 will support static layout only.** All region dimensions must be specified explicitly by the developer at build time.

**What this means**:
- Regions have fixed `(x, y, width, height)`
- No auto-sizing based on content
- Developer controls all layout dimensions
- Truncation occurs if content exceeds region size

---

### Consequences

#### ✅ Positive

1. **Predictability**: Layout behavior is 100% deterministic
2. **Simplicity**: No constraint solver needed
3. **Performance**: Zero layout computation at runtime
4. **Correctness**: Matches regulatory form requirements
5. **Testability**: Easy to verify exact positioning
6. **Implementation Speed**: Faster to implement and ship V1

#### ❌ Negative

1. **Developer Burden**: Developer must calculate dimensions manually
2. **Less Flexible**: Cannot adapt to varying content sizes
3. **More Code**: Developer writes more layout code

#### 🔄 Neutral

1. **V2 Path**: Can add dynamic layout in V2 without breaking V1 API
2. **Migration**: Static → Dynamic is easier than Dynamic → Static

---

### Alternatives Considered

#### Alternative 1: Dynamic Layout (Auto-Sizing)

**Description**: Regions automatically size to fit content.

**Pros**:
- Easier for developers (less layout code)
- Adapts to varying content sizes
- More flexible

**Cons**:
- Non-deterministic output (content changes → layout changes)
- Complex constraint solver required
- Much longer development time
- Doesn't match regulatory form requirements
- Poor fit for ESC/P printers

**Decision**: ❌ Rejected for V1

---

#### Alternative 2: Hybrid Approach

**Description**: Support both static and dynamic regions.

**Pros**:
- Best of both worlds
- Developer can choose per region

**Cons**:
- Twice the complexity
- Ambiguous interaction between static and dynamic regions
- Larger API surface
- Unclear semantics (what happens when dynamic content overflows static parent?)

**Decision**: ❌ Rejected for V1

---

### Validation

**Test**:
```rust
// This should NOT compile (no auto-sizing API exists):
region.auto_size_to_content(); // Compile error

// Developer must specify dimensions explicitly:
let region = Region::new(0, 0, 50, 20)?; // ✅ OK
```

**Success Criteria**:
- All regions require explicit dimensions
- No auto-layout code in codebase
- Documentation clearly states static-only

---

### Notes

**For V2 Consideration**:
- Add `Region::auto_height()` for dynamic height
- Constraint-based layout system
- Flexbox-like API

**Related Issues**: None

---

## ADR-002: Manual Pagination

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Engineering Team
**Tags**: #pagination #api #v1

---

### Context

We need to decide how multi-page documents are created:
1. **Manual**: Developer explicitly creates each page
2. **Automatic**: Library automatically breaks content across pages
3. **Hybrid**: Both manual and automatic options

**Business Context**:
- Regulatory forms have strict page boundaries
- Carbon forms require exact page breaks
- Users need full control over page composition

**Technical Context**:
- Automatic pagination requires lookahead and backtracking
- Page break decisions affect layout globally
- ESC/P uses form feed (FF) for explicit page breaks

---

### Decision

**V1 will require manual pagination.** Developers must explicitly create each page using `DocumentBuilder::add_page()`.

**What this means**:
- No automatic page breaks
- No content overflow to next page
- Developer controls when pages are added
- Vertical truncation if content exceeds page height

---

### Consequences

#### ✅ Positive

1. **Predictability**: Developer knows exactly what's on each page
2. **Control**: Full control over page boundaries
3. **Simplicity**: No complex pagination algorithm
4. **Correctness**: Matches form requirements (page N always has section X)
5. **Performance**: Zero pagination computation

#### ❌ Negative

1. **Developer Work**: More code to manage pagination
2. **No Auto-Flow**: Long content must be manually split
3. **Error-Prone**: Developer must track what fits on each page

#### 🔄 Neutral

1. **V2 Path**: Can add auto-pagination without breaking V1
2. **Helper Functions**: Can provide utilities to assist pagination

---

### Alternatives Considered

#### Alternative 1: Automatic Pagination

**Description**: Library automatically breaks content across pages when height exceeded.

**Example**:
```rust
// Hypothetical auto-pagination API
doc.auto_paginate()
   .add_content(long_table) // Automatically spans multiple pages
   .add_content(footer);    // On last page only
```

**Pros**:
- Less developer code
- Handles varying content sizes
- More "magical" user experience

**Cons**:
- Non-deterministic (content changes → different page count)
- Difficult to control page breaks (orphans, widows)
- Complex algorithm (when to break? what to carry over?)
- Poor fit for fixed forms (can't guarantee "invoice items always on page 2")
- Challenging to implement correctly

**Decision**: ❌ Rejected for V1

---

#### Alternative 2: Hybrid with Explicit Breaks

**Description**: Auto-paginate but allow manual breaks.

**Example**:
```rust
doc.add_section(header)
   .page_break()  // Explicit break
   .add_section(body);  // Auto-paginate within this section
```

**Pros**:
- Flexibility
- Semi-automatic

**Cons**:
- Confusing semantics (when does auto-pagination happen?)
- Still complex to implement
- Unclear what "auto-paginate within section" means for ESC/P

**Decision**: ❌ Rejected for V1

---

### Validation

**Test**:
```rust
// Manual pagination is required:
let mut doc = DocumentBuilder::new();
doc.add_page();  // Explicit
doc.add_page();  // Explicit
doc.add_page();  // Explicit
let document = doc.build();
assert_eq!(document.page_count(), 3);

// No automatic overflow to next page:
let mut page = doc.add_page();
let mut root = page.root_region();
// Writing 1000 lines to 51-line page:
for i in 0..1000 {
    root.write_text(0, i, "Line", Style::NORMAL);
}
// Lines 51+ are truncated, NOT moved to page 2
```

**Success Criteria**:
- Developer must call `add_page()` explicitly
- No auto-pagination code exists
- Vertical truncation enforced

---

### Notes

**Helper Pattern** (can be added as utility):
```rust
// Utility function (not in V1 core)
fn split_content_across_pages(
    content: &[String],
    lines_per_page: usize,
) -> Vec<Vec<String>> {
    content.chunks(lines_per_page)
           .map(|chunk| chunk.to_vec())
           .collect()
}
```

**For V2**: Consider `DocumentBuilder::auto_paginate_section(content)`.

---

## ADR-003: Builder Pattern with Lifetimes

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Engineering Team
**Tags**: #api #safety #lifetimes

---

### Context

We need to design the API for creating documents. Options:
1. **Builder pattern with lifetimes**: Hierarchical borrowing
2. **Direct construction**: Mutable references to owned objects
3. **Arena allocation**: All objects in single arena
4. **Handle-based**: Integer IDs instead of references

**Requirements**:
- Type-safe (prevent use-after-free)
- Ergonomic (method chaining)
- Zero runtime overhead
- Prevent building document while pages are being edited

---

### Decision

**Use builder pattern with hierarchical lifetimes:**

```rust
DocumentBuilder (owns data)
    ↓ lifetime 'doc
PageBuilder<'doc> (borrows from DocumentBuilder)
    ↓ lifetime 'page
RegionHandle<'page> (borrows from PageBuilder)
```

**API**:
```rust
let mut doc = DocumentBuilder::new();        // Owner
let mut page = doc.add_page();               // Borrows 'doc
let mut root = page.root_region();           // Borrows 'page
root.write_text(0, 0, "Text", Style::BOLD); // Uses 'page
```

---

### Consequences

#### ✅ Positive

1. **Compile-Time Safety**: Borrow checker prevents:
   - Using page after document is built
   - Creating multiple page builders simultaneously
   - Dangling region handles

2. **Zero Runtime Cost**: Lifetimes are compile-time only

3. **Ergonomic**: Method chaining works naturally

4. **Clear Ownership**: Obvious who owns what

5. **Forces Finalization**: Can't build document while editing pages

#### ❌ Negative

1. **Learning Curve**: Developers must understand lifetimes

2. **Lifetime Errors**: Confusing compiler errors for beginners

3. **Less Flexible**: Can't hold multiple page builders at once

#### 🔄 Neutral

1. **Rust Idiomatic**: Standard Rust pattern
2. **No Runtime Overhead**: Pure compile-time feature

---

### Alternatives Considered

#### Alternative 1: Direct Mutable References

**Description**: No builders, just mutable references.

**Example**:
```rust
let mut doc = Document::new();
let page = doc.create_page();
page.write_text(0, 0, "Text", Style::BOLD);
```

**Pros**:
- Simpler API (no builders)
- More direct

**Cons**:
- Harder to prevent invalid states (e.g., rendering half-built document)
- No clear finalization boundary
- Risk of accidentally mutating after "done"

**Decision**: ❌ Rejected

---

#### Alternative 2: Arena Allocation

**Description**: All objects allocated in single arena.

**Example**:
```rust
let arena = Arena::new();
let page = arena.alloc(Page::new());
let region = arena.alloc(Region::new(0, 0, 160, 51));
```

**Pros**:
- Flexible borrowing
- Can create many objects at once

**Cons**:
- Runtime overhead (allocation tracking)
- Memory not freed until arena dropped
- Less idiomatic Rust
- Harder to guarantee safety without lifetimes

**Decision**: ❌ Rejected

---

#### Alternative 3: Handle-Based (Integer IDs)

**Description**: Return opaque IDs instead of references.

**Example**:
```rust
let doc = DocumentBuilder::new();
let page_id = doc.add_page();
let region_id = doc.add_region(page_id, 0, 0, 50, 50);
doc.write_text(region_id, 0, 0, "Text");
```

**Pros**:
- No lifetime issues
- Can store handles in collections

**Cons**:
- Runtime overhead (ID → object lookup)
- Weaker safety (can't prevent using invalid ID)
- Less ergonomic (no method chaining)
- Not idiomatic Rust

**Decision**: ❌ Rejected

---

### Validation

**Compile-Time Safety Test**:
```rust
// ✅ Valid: Proper lifetime scoping
let mut doc = DocumentBuilder::new();
{
    let mut page = doc.add_page();
    page.root_region().write_text(0, 0, "Text", Style::BOLD);
    page.finalize().unwrap();
} // page dropped here
let document = doc.build(); // ✅ OK

// ❌ Invalid: Compiler error
let mut doc = DocumentBuilder::new();
let page = doc.add_page();
let document = doc.build(); // Error: doc borrowed by page
```

**Success Criteria**:
- Invalid usage caught at compile time
- No runtime checks needed
- Ergonomic method chaining works

---

### Notes

**Lifetime Error Guidance**:
Document common lifetime errors in API docs with solutions.

**Example Error Message**:
```
error[E0505]: cannot move out of `doc` because it is borrowed
  --> src/main.rs:5:17
   |
3  |     let page = doc.add_page();
   |                --- borrow of `doc` occurs here
4  |
5  |     let document = doc.build();
   |                    ^^^ move out of `doc` occurs here

help: consider finalizing the page before building:
   |
4  |     page.finalize()?;
   |
```

---

## ADR-004: Row-Major Memory Layout

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Performance Engineer
**Tags**: #performance #memory #cache

---

### Context

We need to decide memory layout for the 160×51 cell grid:
1. **Row-major**: `cells[y][x]` (iterate y, then x)
2. **Column-major**: `cells[x][y]` (iterate x, then y)
3. **Flat array**: `cells[y * WIDTH + x]`

**Technical Context**:
- ESC/P rendering is line-by-line (top to bottom)
- Cache line size is typically 64 bytes
- Each cell is 2 bytes
- Page rendering iterates: `for y { for x { render(cell[y][x]) } }`

---

### Decision

**Use row-major layout: `[[Cell; 160]; 51]` (array of rows).**

```rust
pub struct Page {
    cells: Box<[[Cell; 160]; 51]>,  // cells[y][x]
}
```

**Access pattern**:
```rust
for y in 0..51 {
    for x in 0..160 {
        let cell = page.cells[y][x];  // Cache-friendly
    }
}
```

---

### Consequences

#### ✅ Positive

1. **Cache Efficiency**: Consecutive x-values in same cache line
   - Cache line (64 bytes) holds 32 cells (64 / 2)
   - Iterating x is sequential memory access
   - Cold miss rate: ~3.1% (255 cache lines / 8,160 cells)

2. **Natural Iteration**: Matches ESC/P rendering (line-by-line)

3. **Type Safety**: Two-level array prevents out-of-bounds on first dimension

#### ❌ Negative

1. **Stack Size**: 16 KB (too large for stack) → requires `Box`

2. **Column Access**: Accessing column is cache-unfriendly (strided)

#### 🔄 Neutral

1. **Memory Footprint**: Same as other layouts (16 KB)

---

### Alternatives Considered

#### Alternative 1: Column-Major Layout

**Description**: `cells[x][y]`

**Pros**:
- Good for column-oriented operations

**Cons**:
- Poor cache locality for line-by-line rendering
- ESC/P renders rows, not columns
- Doesn't match use case

**Decision**: ❌ Rejected

---

#### Alternative 2: Flat Array with Manual Indexing

**Description**: `cells: [Cell; 8160]` with `cells[y * 160 + x]`

**Pros**:
- Contiguous memory (no indirection)
- Flexible access patterns

**Cons**:
- No bounds checking on first dimension
- Manual index calculation (error-prone)
- Less idiomatic
- Benchmark showed <1% performance difference

**Decision**: ❌ Rejected (type safety more valuable than tiny perf gain)

---

#### Alternative 3: Vec of Vecs

**Description**: `Vec<Vec<Cell>>`

**Pros**:
- Flexible (could grow dynamically)
- Each row is separate allocation

**Cons**:
- Non-contiguous (each row is separate heap allocation)
- Cache-unfriendly (pointer chasing)
- Memory overhead (Vec headers)
- We don't need dynamic sizing

**Decision**: ❌ Rejected

---

### Validation

**Cache Miss Analysis**:
```
Total cells: 8,160
Cell size: 2 bytes
Cache line: 64 bytes
Cells per cache line: 64 / 2 = 32

Rows: 51
Cache lines per row: ceil(160 / 32) = 5
Total cache lines: 51 × 5 = 255

Cold miss rate: 255 / 8,160 = 3.1%
```

**Benchmark**:
```rust
#[bench]
fn bench_row_major_iteration(b: &mut Bencher) {
    let page = Page::new();
    b.iter(|| {
        for y in 0..51 {
            for x in 0..160 {
                black_box(page.cells[y][x]);
            }
        }
    });
}
// Target: < 10 μs per full page iteration
```

**Success Criteria**:
- Page iteration performance meets target
- Cache miss rate < 5%

---

### Notes

**Optimization Opportunities**:
- Use SIMD for processing full cache lines (future)
- Prefetch next cache line (future)

**Related**: NFR-2 (Performance), NFR-3 (Memory Efficiency)

---

## ADR-005: Bit-Packed Style Storage

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Performance Engineer
**Tags**: #memory #optimization #performance

---

### Context

We need to store text styles (bold, underline) for each cell. Options:
1. **Struct with bools**: `struct Style { bold: bool, underline: bool }`
2. **Bit-packed**: Single `u8` with bit flags
3. **Enum**: `enum Style { Normal, Bold, Underline, BoldUnderline }`

**Requirements**:
- Minimize memory per cell
- Fast style comparisons (for state machine)
- Support combining styles (bold + underline)
- Room for future styles (italic, strikethrough)

**Memory Impact**:
- 8,160 cells per page
- Style storage × 8,160 = significant impact

---

### Decision

**Use bit-packed storage with a thin wrapper:**

```rust
// Internal representation (1 byte)
#[repr(transparent)]
pub struct StyleBits(u8);

// Bit layout:
// Bit 0: Bold
// Bit 1: Underline
// Bits 2-7: Reserved for future

impl StyleBits {
    pub const NORMAL: Self = Self(0b0000_0000);
    pub const BOLD: Self = Self(0b0000_0001);
    pub const UNDERLINE: Self = Self(0b0000_0010);
    pub const BOLD_UNDERLINE: Self = Self(0b0000_0011);
}

// User-facing API (ergonomic)
pub struct Style {
    pub bold: bool,
    pub underline: bool,
}

// Conversion (zero-cost)
impl From<Style> for StyleBits { ... }
impl From<StyleBits> for Style { ... }
```

---

### Consequences

#### ✅ Positive

1. **Minimal Memory**: 1 byte per cell vs 2 bytes for struct
   - Saves 8,160 bytes per page
   - Total cell: 2 bytes (1 char + 1 style) vs 3 bytes

2. **Fast Comparisons**: Single byte comparison vs struct field-by-field

3. **Future-Proof**: 6 reserved bits for future styles

4. **Efficient State Machine**: Bit operations for style transitions

5. **Cache-Friendly**: Smaller cells = more cells per cache line

#### ❌ Negative

1. **Complexity**: Two representations (StyleBits internal, Style public)

2. **Conversion Overhead**: (Mitigated: conversion is zero-cost inline)

#### 🔄 Neutral

1. **Implementation**: Straightforward bit operations

---

### Alternatives Considered

#### Alternative 1: Struct with Bools

**Description**:
```rust
#[derive(Copy)]
pub struct Style {
    pub bold: bool,      // 1 byte
    pub underline: bool, // 1 byte
    // Total: 2 bytes (with padding)
}
```

**Pros**:
- Simple, no conversion needed
- Ergonomic (fields directly accessible)

**Cons**:
- 2× memory usage vs bit-packing
- Slower comparisons (two fields)
- Memory waste (could have 6 more styles in same space)

**Decision**: ❌ Rejected

---

#### Alternative 2: Enum

**Description**:
```rust
pub enum Style {
    Normal = 0,
    Bold = 1,
    Underline = 2,
    BoldUnderline = 3,
}
```

**Pros**:
- Compact (1 byte)
- Fast comparisons

**Cons**:
- Can't combine styles easily (need enum variant for each combination)
- Doesn't scale (8 styles = 256 enum variants)
- Less ergonomic (no `style.bold` accessor)

**Decision**: ❌ Rejected

---

#### Alternative 3: Separate Bold/Underline Arrays

**Description**:
```rust
pub struct Page {
    characters: [[char; 160]; 51],
    bold: [[bool; 160]; 51],
    underline: [[bool; 160]; 51],
}
```

**Pros**:
- Can process styles separately
- Good for SIMD (future)

**Cons**:
- Poor cache locality (fields are far apart)
- More complex API
- 3× memory access per cell

**Decision**: ❌ Rejected

---

### Validation

**Memory Size Test**:
```rust
#[test]
fn test_style_bits_size() {
    assert_eq!(std::mem::size_of::<StyleBits>(), 1);
    assert_eq!(std::mem::size_of::<Cell>(), 2); // char + StyleBits
}
```

**Correctness Test**:
```rust
#[test]
fn test_style_bits_packing() {
    let bits = StyleBits::new(true, true);
    assert_eq!(bits.raw(), 0b0000_0011);
    assert!(bits.is_bold());
    assert!(bits.is_underline());
}
```

**Performance Benchmark**:
```rust
#[bench]
fn bench_style_comparison(b: &mut Bencher) {
    let s1 = StyleBits::BOLD;
    let s2 = StyleBits::NORMAL;
    b.iter(|| {
        black_box(s1 == s2) // Single byte comparison
    });
}
// Target: < 1 ns per comparison
```

---

### Notes

**Future Styles** (bits 2-7 available):
- Bit 2: Italic (ESC/P: ESC 4)
- Bit 3: Strikethrough
- Bit 4: Double-width (ESC/P: SO)
- Bit 5: Superscript/Subscript
- Bits 6-7: Reserved

**Migration Path**: Adding styles is backward-compatible (new bits).

---

## ADR-006: Silent Truncation (No Errors)

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Engineering Team
**Tags**: #errors #api #behavior

---

### Context

We need to decide what happens when content overflows region boundaries:
1. **Return errors**: Methods return `Result<(), LayoutError>`
2. **Silent truncation**: Content is clipped, no error
3. **Panic**: Crash on overflow
4. **Hybrid**: Errors for some cases, truncation for others

**Use Cases**:
- Developer miscalculates region sizes
- Variable-length data (customer names, descriptions)
- Regulatory forms must print even if content is long

---

### Decision

**Content overflow results in silent truncation, not errors.**

**Behavior**:
- Horizontal overflow: Characters beyond region width are dropped
- Vertical overflow: Lines beyond region height are dropped
- Page boundary overflow: Writes outside (160×51) are ignored
- **No errors**, **no panics**, **no warnings**

**Example**:
```rust
let mut region = /* 10×5 region */;

// Writing 100 characters to 10-column region:
region.write_text(0, 0, &"A".repeat(100), Style::NORMAL);
// Only first 10 characters written, remaining 90 silently dropped

// Writing to line 20 of 5-line region:
region.write_text(0, 20, "Text", Style::NORMAL);
// Silently ignored (y >= height)
```

---

### Consequences

#### ✅ Positive

1. **Simplicity**: No error handling in layout code

2. **Ergonomic**: No `?` or `.unwrap()` needed for writes

3. **Graceful Degradation**: Partial content better than no content

4. **Predictable**: Truncation rules are deterministic

5. **Matches Printer Behavior**: Printers clip content at margins

6. **Performance**: No error allocation/propagation overhead

#### ❌ Negative

1. **Silent Failures**: Developer may not notice content is truncated

2. **Debugging**: Harder to find layout bugs (no error message)

3. **Surprise Behavior**: Developers expect errors for "mistakes"

#### 🔄 Neutral

1. **Documentation**: Must clearly document truncation behavior

2. **Testing**: Truncation must be tested (property-based tests)

---

### Alternatives Considered

#### Alternative 1: Return Errors on Overflow

**Description**: `write_text()` returns `Result<(), LayoutError::ContentOverflow>`

**Example**:
```rust
region.write_text(0, 0, "Long text", Style::NORMAL)?;
//                                                    ^ Error if overflow
```

**Pros**:
- Developer aware of overflow
- Can handle error (e.g., resize region, split content)

**Cons**:
- Every write needs error handling
- Propagates `Result` through entire layout code
- What should developer do? (Usually can't resize at that point)
- Less ergonomic (method chaining breaks)
- Conflicts with regulatory requirement (must print even if truncated)

**Decision**: ❌ Rejected

---

#### Alternative 2: Panic on Overflow

**Description**: `assert!(content_fits)` before write

**Pros**:
- Developer immediately knows something is wrong
- Forces fixing layout bugs

**Cons**:
- Violates NFR-4 (no panics under normal use)
- Production crash for variable-length data
- Can't handle edge cases gracefully
- Poor fit for embedded/critical systems

**Decision**: ❌ Rejected (violates requirements)

---

#### Alternative 3: Hybrid (Errors for Geometry, Truncation for Content)

**Description**:
- Geometry errors (invalid dimensions, out-of-bounds regions): Return errors
- Content overflow: Silent truncation

**Example**:
```rust
// Geometry: Error
let region = Region::new(0, 0, 0, 0)?; // Error: invalid dimensions

// Content: Truncation
region.write_text(0, 0, "Long text", Style::NORMAL); // Truncates, no error
```

**Pros**:
- Catches serious mistakes (geometry)
- Allows graceful content handling

**Cons**:
- Inconsistent (some operations error, some don't)
- Hard to document/learn

**Decision**: ✅ Partially Accepted
- We DO return errors for geometry (InvalidDimensions, RegionOutOfBounds)
- We DO NOT return errors for content overflow (truncation)

---

### Validation

**Property-Based Test**:
```rust
proptest! {
    #[test]
    fn test_write_never_panics(
        x in 0u16..500,
        y in 0u16..200,
        text in ".*",
    ) {
        let mut page = Page::new();
        page.write_text(x, y, &text, Style::NORMAL);
        // Should never panic, even for out-of-bounds writes
    }
}
```

**Truncation Correctness Test**:
```rust
#[test]
fn test_horizontal_truncation() {
    let mut page = Page::new();
    let long_text = "A".repeat(200);
    page.write_text(0, 0, &long_text, Style::NORMAL);

    // Verify truncation at page width
    for x in 0..160 {
        assert_eq!(page.get_cell(x, 0).unwrap().character(), 'A');
    }
    // No writes beyond width
}
```

**Success Criteria**:
- Zero panics in fuzzing (1M+ iterations)
- Truncation is deterministic
- Documentation clearly states behavior

---

### Notes

**Documentation Requirements**:
```rust
/// Writes text at (x, y) coordinates.
///
/// # Truncation
///
/// - Content beyond region width is silently dropped.
/// - Lines beyond region height are ignored.
/// - This is NOT an error and does not panic.
///
/// # Examples
///
/// ```
/// // Writing 100 characters to 80-column region:
/// region.write_text(0, 0, &"A".repeat(100), Style::NORMAL);
/// // Only first 80 characters are written
/// ```
pub fn write_text(&mut self, x: u16, y: u16, text: &str, style: Style) -> &mut Self;
```

**Related**: FR-T1, FR-T2, FR-T3, FR-T4, NFR-4

---

## ADR-007: Zero Runtime Dependencies

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Security Team
**Tags**: #dependencies #security #portability

---

### Context

We need to decide dependency policy:
1. **Zero dependencies**: Only `std`, no external crates
2. **Minimal dependencies**: Allow well-vetted, small crates
3. **Full ecosystem**: Use crates for all common tasks

**Concerns**:
- Supply chain security
- Binary size
- Compilation time
- Portability (future `no_std`)
- Maintenance burden

---

### Decision

**V1 will have zero required runtime dependencies** (beyond Rust `std`).

**Allowed**:
- **Runtime**: None (only `std`)
- **Optional (feature-gated)**: `serde` for serialization
- **Dev dependencies**: `criterion`, `proptest`, `libfuzzer-sys`

**Forbidden**:
- No required external crates at runtime
- No `unsafe` crates
- No C/C++ dependencies (FFI)

---

### Consequences

#### ✅ Positive

1. **Security**: Minimal supply chain attack surface
2. **Binary Size**: Smaller binaries (no dependency bloat)
3. **Compile Time**: Faster compilation (no transitive deps)
4. **Portability**: Easier to port to embedded/`no_std`
5. **Stability**: No dependency breakage
6. **Trust**: Users can audit entire codebase easily

#### ❌ Negative

1. **Reinventing Wheels**: Must implement some utilities ourselves
2. **Missing Features**: No access to ecosystem crates
3. **More Code**: More implementation work

#### 🔄 Neutral

1. **Standard Rust**: `std` provides most needed functionality
2. **Optional Features**: Can add dependencies as opt-in features

---

### Alternatives Considered

#### Alternative 1: Allow Minimal Dependencies

**Description**: Allow specific, well-vetted crates (e.g., `thiserror`, `bitflags`)

**Pros**:
- Less reimplementation
- Use proven libraries
- Better error messages (`thiserror`)

**Cons**:
- Supply chain risk (dependencies can be compromised)
- Transitive dependencies (each crate brings its own deps)
- Compilation time increases
- Binary size increases

**Decision**: ❌ Rejected (security > convenience)

---

#### Alternative 2: Full Ecosystem Usage

**Description**: Use crates for everything (serialization, error handling, utilities)

**Pros**:
- Fastest development
- Battle-tested code
- Rich features

**Cons**:
- Large dependency tree (100+ transitive deps common)
- Supply chain risk multiplied
- Compilation time: minutes instead of seconds
- Binary bloat (MBs of dependencies)
- Hard to audit

**Decision**: ❌ Rejected (incompatible with requirements)

---

### Validation

**Dependency Audit**:
```bash
# Verify zero runtime dependencies
cargo tree --depth 1

# Expected output:
# epson-lq2090-layout v1.0.0
# (no dependencies)

# Dev dependencies allowed:
cargo tree --depth 1 --dev-dependencies
# epson-lq2090-layout v1.0.0
# ├── criterion v0.5
# ├── proptest v1.0
# └── libfuzzer-sys v0.4
```

**Binary Size Test**:
```bash
cargo build --release
ls -lh target/release/libepson_lq2090_layout.rlib
# Target: < 500 KB
```

**CI Check**:
```yaml
# .github/workflows/ci.yml
- name: Check dependencies
  run: |
    deps=$(cargo tree --depth 1 | wc -l)
    if [ $deps -gt 1 ]; then
      echo "Error: Runtime dependencies detected"
      exit 1
    fi
```

---

### Notes

**What We Implement Ourselves**:
- Error types (no `thiserror`)
- Bit flags (no `bitflags`)
- Text alignment (no `textwrap`)

**Optional Features** (feature-gated):
```toml
[features]
serde = ["dep:serde"]

[dependencies]
serde = { version = "1.0", optional = true }
```

**Future**: If `no_std` support added in V2, this decision pays off.

**Related**: FR-DEP1, SEC-005

---

## ADR-008: ESC/P Text Mode Only

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Engineering Team
**Tags**: #scope #escp #printing

---

### Context

EPSON printers support multiple modes:
1. **Text mode**: Characters with simple formatting (bold, underline)
2. **Bitmap mode**: Raster graphics, custom fonts
3. **Vector mode**: Line drawing commands

We need to decide scope for V1:
- Support text mode only?
- Support bitmap mode (for logos, barcodes)?
- Support all modes?

---

### Decision

**V1 will support ESC/P text mode only.** No bitmap, raster, or vector graphics.

**Supported ESC/P Commands**:
- `ESC @`: Reset
- `SI`: Condensed mode (12 CPI)
- `ESC E` / `ESC F`: Bold on/off
- `ESC - 1` / `ESC - 0`: Underline on/off
- `CR`, `LF`, `FF`: Basic control

**Not Supported**:
- Bitmap/raster graphics (`ESC *`, `ESC K`)
- Custom fonts
- Line drawing
- Barcodes
- QR codes
- Logos/images

---

### Consequences

#### ✅ Positive

1. **Simplicity**: Text-only rendering is straightforward
2. **Determinism**: No complex image rendering algorithms
3. **Performance**: Fast rendering (no image processing)
4. **Small Scope**: Achievable in 9-week timeline
5. **ASCII Focus**: Matches regulatory form requirements

#### ❌ Negative

1. **Limited Graphics**: No logos, barcodes, or images
2. **Branding**: Can't include company logo on forms
3. **Barcode Dependency**: Must use separate barcode printer or pre-printed forms

#### 🔄 Neutral

1. **V2 Path**: Can add bitmap support later without breaking V1 API
2. **Workarounds**: ASCII art for simple graphics

---

### Alternatives Considered

#### Alternative 1: Include Bitmap Mode

**Description**: Support ESC/P bitmap commands for images.

**Example**:
```rust
region.add_image(logo_bitmap, x, y)?;
```

**Pros**:
- Company logos on invoices
- Barcodes
- More professional-looking output

**Cons**:
- Complex implementation (image encoding, dithering)
- Non-deterministic (image processing algorithms)
- Large scope increase (2-3 weeks more)
- Binary dependencies (image libraries)
- Memory overhead (image storage)

**Decision**: ❌ Rejected for V1 (scope constraint)

---

#### Alternative 2: ASCII Art Helper

**Description**: Provide utility to convert simple graphics to ASCII.

**Example**:
```rust
let ascii_logo = ascii_art::from_text("ACME\nCORP");
region.add_widget(TextBlock::new(ascii_logo))?;
```

**Pros**:
- Simple implementation
- No bitmap mode needed
- Fits text-mode constraint

**Cons**:
- Limited aesthetic appeal
- Not suitable for complex logos

**Decision**: 🤔 Possible utility (not core V1)

---

### Validation

**Code Review**:
```rust
// ESC/P commands in codebase should be limited to:
const ESC_RESET: &[u8] = &[0x1B, 0x40];       // ✅ Allowed
const SI_CONDENSED: u8 = 0x0F;                // ✅ Allowed
const ESC_BOLD_ON: &[u8] = &[0x1B, 0x45];     // ✅ Allowed

// These should NOT exist in codebase:
const ESC_BITMAP: &[u8] = &[0x1B, 0x2A];      // ❌ Not in V1
const ESC_LINE_DRAW: &[u8] = &[0x1B, 0x5C];   // ❌ Not in V1
```

**Success Criteria**:
- No graphics ESC/P commands in renderer
- Documentation states "text mode only"

---

### Notes

**For V2**: Consider adding:
```rust
pub trait ImageWidget: Widget {
    fn render_bitmap(&self, region: &mut RegionHandle) -> Result<(), LayoutError>;
}
```

**Workarounds for V1 Users**:
- Pre-printed forms with logos
- Separate barcode printer
- ASCII art for simple graphics

**Related**: FR-E4

---

## ADR-009: Deterministic Rendering

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Engineering Team
**Tags**: #rendering #determinism #testing

---

### Context

We need to decide if rendering should be deterministic:
1. **Deterministic**: Same input always produces identical output (byte-for-byte)
2. **Non-deterministic**: Output may vary (timestamps, random IDs, etc.)

**Use Cases**:
- Regulatory compliance (must reproduce exact output)
- Golden master testing
- Caching rendered output
- Debugging (reproducible bugs)

**Challenges**:
- Timestamps in output
- Random number generation
- Hash map iteration order
- Floating point calculations

---

### Decision

**Rendering must be 100% deterministic.**

**Requirements**:
- Same `Document` input → Identical ESC/P output (byte-for-byte)
- No timestamps in output
- No randomness
- No HashMap iteration (use Vec/BTreeMap for stable ordering)
- No floating-point calculations (use integer arithmetic)

**Validation**:
```rust
let doc = create_document();

let render1 = doc.render();
let render2 = doc.render();
let render3 = doc.render();

assert_eq!(render1, render2);
assert_eq!(render2, render3);

// SHA-256 hash comparison
let hash1 = sha256(&render1);
let hash2 = sha256(&render2);
assert_eq!(hash1, hash2);
```

---

### Consequences

#### ✅ Positive

1. **Regulatory Compliance**: Can reproduce exact output for audits
2. **Golden Master Testing**: Byte-level comparison works
3. **Caching**: Can cache rendered output (hash as key)
4. **Debugging**: Reproducible bugs (no "works on my machine")
5. **Confidence**: Know exactly what will be printed

#### ❌ Negative

1. **No Timestamps**: Can't auto-generate timestamps in output
2. **No UUIDs**: Can't use random IDs
3. **More Constraints**: Developers must avoid non-deterministic operations

#### 🔄 Neutral

1. **User Responsibility**: User provides all data (including timestamps)

---

### Alternatives Considered

#### Alternative 1: Allow Non-Deterministic Output

**Description**: Allow timestamps, random IDs, etc.

**Pros**:
- More convenient (auto-timestamps)
- Can use UUIDs for tracking

**Cons**:
- Can't reproduce exact output
- Golden master testing impossible
- Can't cache rendered output
- Harder to debug

**Decision**: ❌ Rejected (violates requirements)

---

#### Alternative 2: Optional Determinism

**Description**: Provide flag: `Document::render_deterministic()` vs `render()`

**Pros**:
- Flexibility (deterministic for tests, non-deterministic for production)

**Cons**:
- Two code paths to maintain
- Confusing API (when to use which?)
- Regulatory requirement is for production output (not just tests)

**Decision**: ❌ Rejected

---

### Validation

**Property-Based Test**:
```rust
proptest! {
    #[test]
    fn test_rendering_is_deterministic(
        text in ".*",
        x in 0u16..160,
        y in 0u16..51,
    ) {
        let doc1 = create_document(x, y, &text);
        let doc2 = create_document(x, y, &text);

        let render1 = doc1.render();
        let render2 = doc2.render();

        assert_eq!(render1, render2);
    }
}
```

**SHA-256 Test**:
```rust
#[test]
fn test_sha256_determinism() {
    let doc = create_complex_document();

    let hashes: Vec<_> = (0..1000)
        .map(|_| {
            let bytes = doc.render();
            sha256(&bytes)
        })
        .collect();

    // All 1000 hashes should be identical
    assert!(hashes.windows(2).all(|w| w[0] == w[1]));
}
```

---

### Notes

**Non-Deterministic Operations to Avoid**:
```rust
// ❌ Avoid
let timestamp = SystemTime::now(); // Non-deterministic
let uuid = Uuid::new_v4();         // Random
let mut map = HashMap::new();      // Iteration order undefined

// ✅ Use instead
let timestamp = user_provided_timestamp; // User provides
let id = sequential_counter;             // Deterministic
let mut map = BTreeMap::new();           // Stable iteration order
```

**Related**: NFR-1, FR-E2

---

## ADR-010: Fixed Page Dimensions

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Engineering Team
**Tags**: #page #hardware #constraints

---

### Context

We need to decide if page dimensions should be:
1. **Fixed**: Always 160×51
2. **Configurable**: User specifies dimensions
3. **Printer-Specific**: Different sizes per printer model

**Hardware Context**:
- EPSON LQ-2090II in condensed mode: 160 columns × 51 lines
- Other modes: 80 columns (pica), 136 columns (elite)
- Other printers: Different dimensions

---

### Decision

**Page dimensions are fixed at 160×51** (non-configurable in V1).

```rust
pub struct Page {
    pub const WIDTH: usize = 160;
    pub const HEIGHT: usize = 51;
    // Not configurable
}
```

**Rationale**:
- Matches EPSON LQ-2090II condensed mode (12 CPI)
- Simplifies implementation (no dynamic sizing)
- Matches target use case (regulatory forms on this specific printer)

---

### Consequences

#### ✅ Positive

1. **Simplicity**: No configuration needed
2. **Type Safety**: Dimensions are compile-time constants
3. **Performance**: No runtime size checks
4. **Clarity**: Developer knows exact page size
5. **Testing**: Single configuration to test

#### ❌ Negative

1. **Inflexibility**: Can't use with other printers/modes
2. **Limited Scope**: Only LQ-2090II condensed mode

#### 🔄 Neutral

1. **V2 Path**: Can add configurable dimensions later

---

### Alternatives Considered

#### Alternative 1: Configurable Dimensions

**Description**: Allow user to specify page size.

**Example**:
```rust
let page = Page::new(80, 66)?; // 80 columns, 66 lines (pica mode)
```

**Pros**:
- Supports multiple printer modes
- Supports other printer models
- More flexible

**Cons**:
- More complex (dynamic sizing)
- Runtime overhead (size checks)
- Harder to optimize (can't assume fixed size)
- Larger scope (must test many configurations)

**Decision**: ❌ Rejected for V1 (can add in V2)

---

#### Alternative 2: Multiple Page Types

**Description**: Different page types for different modes.

**Example**:
```rust
pub struct PageCondensed;  // 160×51
pub struct PagePica;        // 80×66
pub struct PageElite;       // 136×?
```

**Pros**:
- Type-safe (can't mix page types)
- Compile-time optimization per type

**Cons**:
- Duplicated code
- Complex API (which type to use?)
- Overkill for V1 (only one target printer)

**Decision**: ❌ Rejected

---

### Validation

**Compile-Time Check**:
```rust
#[test]
fn test_page_dimensions_are_constant() {
    const WIDTH: usize = Page::WIDTH;
    const HEIGHT: usize = Page::HEIGHT;

    assert_eq!(WIDTH, 160);
    assert_eq!(HEIGHT, 51);

    // These should not compile (not mutable):
    // Page::WIDTH = 80;  // Compile error
}
```

**Documentation**:
```rust
/// Fixed-size page for EPSON LQ-2090II in condensed mode.
///
/// Dimensions: 160 columns × 51 lines
///
/// This size is **not configurable** in V1.
pub struct Page {
    pub const WIDTH: usize = 160;
    pub const HEIGHT: usize = 51;
    // ...
}
```

---

### Notes

**For V2**: Consider trait-based approach:
```rust
pub trait PageDimensions {
    const WIDTH: usize;
    const HEIGHT: usize;
}

pub struct Page<D: PageDimensions> {
    cells: [[Cell; D::WIDTH]; D::HEIGHT],
}
```

**Related**: FR-P1

---

## ADR-011: Immutable Document Model

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Engineering Team
**Tags**: #api #safety #concurrency

---

### Context

We need to decide if `Document` should be:
1. **Mutable**: Can modify after creation
2. **Immutable**: Frozen after `build()`
3. **Copy-on-Write**: Clone on modification

**Concerns**:
- Thread safety (`Send + Sync`)
- Prevent accidental modification after rendering
- Clear finalization boundary
- Caching rendered output

---

### Decision

**`Document` is immutable after `DocumentBuilder::build()`.**

```rust
let document = builder.build(); // Consumes builder, returns immutable Document

// ✅ Can do:
let bytes = document.render();
let count = document.page_count();
let thread_handle = thread::spawn(move || document.render());

// ❌ Cannot do (no methods exist):
// document.add_page();       // Compile error
// document.modify_page(0);   // Compile error
```

---

### Consequences

#### ✅ Positive

1. **Thread Safety**: `Document` is `Send + Sync` (safe to share across threads)
2. **No Accidental Modification**: Can't modify after "finalized"
3. **Cacheable**: Can cache rendered output (document won't change)
4. **Clear Lifecycle**: Build phase → Finalize → Render phase
5. **Compiler-Enforced**: Borrow checker prevents misuse

#### ❌ Negative

1. **No Modifications**: Must rebuild entire document to change it
2. **Less Flexible**: Can't adjust document after creation

#### 🔄 Neutral

1. **Rust Idiomatic**: Standard pattern in Rust

---

### Alternatives Considered

#### Alternative 1: Mutable Document

**Description**: Allow modification after creation.

**Example**:
```rust
let mut document = builder.build();
document.add_page(); // Add page to existing document
```

**Pros**:
- Flexible (can modify anytime)
- Incremental updates

**Cons**:
- Not thread-safe (would need `Arc<Mutex<Document>>`)
- Risk of modifying after rendering (inconsistent state)
- Harder to cache (document might change)
- No clear "finalized" state

**Decision**: ❌ Rejected

---

#### Alternative 2: Copy-on-Write

**Description**: Document clones itself on modification.

**Example**:
```rust
let doc1 = builder.build();
let doc2 = doc1.add_page(); // Returns new document (doc1 unchanged)
```

**Pros**:
- Immutability + flexibility
- Can create variations

**Cons**:
- Memory overhead (cloning pages)
- Confusing API (returns new document?)
- Not needed for use cases

**Decision**: ❌ Rejected

---

### Validation

**Immutability Test**:
```rust
#[test]
fn test_document_is_immutable() {
    let document = create_document();

    // Can render multiple times (immutable references OK)
    let render1 = document.render();
    let render2 = document.render();

    // Cannot modify (no mutable methods)
    // document.add_page(); // Compile error: no such method
}
```

**Thread Safety Test**:
```rust
#[test]
fn test_document_is_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Document>();
    assert_sync::<Document>();
}

#[test]
fn test_document_thread_safe_usage() {
    let document = create_document();

    let handle = thread::spawn(move || {
        document.render()
    });

    let bytes = handle.join().unwrap();
    assert!(!bytes.is_empty());
}
```

---

### Notes

**Lifecycle**:
```
DocumentBuilder (mutable)
    ↓
    .build()
    ↓
Document (immutable, Send + Sync)
    ↓
    .render()
    ↓
Vec<u8> (ESC/P bytes)
```

**Related**: FR-D3, NFR-6

---

## ADR-012: Widget Trait Design

**Date**: 2025-01-18
**Status**: ✅ Accepted
**Deciders**: Mohammad AlMechkor, Engineering Team
**Tags**: #widgets #extensibility #api

---

### Context

We need to design the widget system:
1. **Trait-based**: `Widget` trait, users implement for custom widgets
2. **Enum-based**: `enum Widget { Label(...), Table(...), Custom(...) }`
3. **Function-based**: Widgets are functions

**Requirements**:
- Extensibility (users can create custom widgets)
- Type safety
- Ergonomic usage
- Performance

---

### Decision

**Use a simple trait-based design:**

```rust
pub trait Widget {
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError>;
}
```

**Usage**:
```rust
// Built-in widgets implement Widget
impl Widget for Label { ... }
impl Widget for Table { ... }

// Users can implement custom widgets
struct MyWidget { ... }
impl Widget for MyWidget {
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError> {
        // Custom rendering logic
    }
}

// Using widgets
region.add_widget(Label::new("Text", Alignment::Center))?;
region.add_widget(MyWidget { ... })?;
```

---

### Consequences

#### ✅ Positive

1. **Extensibility**: Users can create custom widgets easily
2. **Type Safety**: Trait ensures correct signature
3. **Uniform API**: All widgets used same way
4. **Composition**: Widgets can contain other widgets
5. **Simple**: Single method trait

#### ❌ Negative

1. **Dynamic Dispatch**: (Mitigated: usually not performance-critical)
2. **No Widget Inspection**: Can't query widget type after creation

#### 🔄 Neutral

1. **Standard Rust**: Trait-based design is idiomatic

---

### Alternatives Considered

#### Alternative 1: Enum-Based

**Description**:
```rust
pub enum Widget {
    Label(LabelConfig),
    Table(TableConfig),
    Custom(Box<dyn CustomWidget>),
}
```

**Pros**:
- Exhaustive matching
- Can inspect widget type

**Cons**:
- Not extensible (users can't add enum variants)
- All widgets must be predefined
- `Custom` variant defeats type safety

**Decision**: ❌ Rejected

---

#### Alternative 2: Function-Based

**Description**:
```rust
type Widget = Box<dyn Fn(&mut RegionHandle) -> Result<(), LayoutError>>;
```

**Pros**:
- Flexible
- No trait needed

**Cons**:
- No state (widget config)
- Harder to compose
- Less clear API

**Decision**: ❌ Rejected

---

### Validation

**Custom Widget Test**:
```rust
#[test]
fn test_custom_widget() {
    struct HorizontalLine {
        character: char,
    }

    impl Widget for HorizontalLine {
        fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError> {
            let (_, _, width, _) = region.region.inner_bounds();
            let line = self.character.to_string().repeat(width as usize);
            region.write_text(0, 0, &line, Style::NORMAL);
            Ok(())
        }
    }

    let mut page = Page::new();
    let mut region = /* create region */;

    let line = HorizontalLine { character: '=' };
    region.add_widget(line).unwrap();

    // Verify line was rendered
}
```

---

### Notes

**Widget Composition**:
```rust
struct CompositeWidget {
    widgets: Vec<Box<dyn Widget>>,
}

impl Widget for CompositeWidget {
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError> {
        for widget in &self.widgets {
            widget.render(region)?;
        }
        Ok(())
    }
}
```

**Related**: FR-W1 through FR-W6

---

## 🔒 End of Architecture Decision Records

**Document Status**: ✅ Living Document (Updated as Decisions are Made)

**Next Steps**:
1. Review ADRs with architecture team
2. Ensure all implementation aligns with decisions
3. Update ADRs if decisions change (mark as Superseded)
4. Reference ADRs in code comments for context

---

**Revision Process**:
- New ADRs added as numbered records (ADR-013, ADR-014, ...)
- Deprecated decisions marked with **Status**: ❌ Deprecated
- Superseded decisions reference replacement (Superseded by ADR-XXX)

---

**For questions about architectural decisions, contact:**
**Architect**: Mohammad AlMechkor
**Document Location**: `/Users/mohammadalmechkor/Projects/matrix/specs/ADR.md`
**Related**: `PRD.md`, `TDD.md`, `API-SPEC.md`

---

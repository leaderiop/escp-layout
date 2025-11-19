# Research: Widget Composability System

**Feature**: Widget Composability System
**Branch**: `002-widget-composability`
**Date**: 2025-11-19
**Status**: Phase 0 Complete

## Overview

This document consolidates the research findings and technical decisions for the widget composability feature. All technical unknowns were resolved through clarification sessions documented in spec.md (sessions 2025-11-18 and 2025-11-19).

## Key Technical Decisions

### 1. Widget Dimension Specification Strategy

**Decision**: Use const generics with dual syntax support (turbofish + macros)

**Rationale**:
- **Compile-time type safety**: Const generics `Widget<const WIDTH: u16, const HEIGHT: u16>` enable the compiler to validate widget dimensions at compile time, preventing runtime errors
- **Zero runtime overhead**: Dimensions known at compile time eliminate need for runtime size checks in hot path
- **Ergonomics**: Dual syntax (turbofish `Box::<80, 30>::new()` vs macro `box_new!(80, 30)`) balances explicitness with usability
- **Constitution alignment**: Matches Principle VI validation hierarchy (compile-time > debug_assert! > runtime) and Principle VIII fixed-layout constraints

**Alternatives Considered**:
- **Runtime-sized widgets with validation**: Would require runtime dimension storage, increasing memory overhead and violating Constitution Principle VIII (fixed-layout constraints)
- **Builder pattern with dimension methods**: Less type-safe; dimensions not part of type signature; would allow widgets to be constructed without dimensions
- **Associated constants only (no const generics)**: Would require separate type for each widget size (e.g., `Box80x30`), leading to combinatorial explosion of types

**Implementation Notes**:
- Widget trait defines associated constants: `const WIDTH: u16; const HEIGHT: u16;`
- Implementations use const generic parameters to provide these values: `impl<const W: u16, const H: u16> Widget for Box<W, H> { const WIDTH: u16 = W; ... }`
- Macros expand to turbofish syntax at compile time (zero abstraction overhead)

---

### 2. Widget Tree Storage and Ownership Model

**Decision**: Use `Vec<WidgetNode>` with heap-allocated trait objects (`Box<dyn Widget>`)

**Rationale**:
- **Simplicity**: Straightforward ownership model (parent owns children via `Box<dyn Widget>`)
- **Memory overhead acceptable**: Per clarification (2025-11-19), Box heap allocation accepted for simplicity; optimize if needed later
- **Type erasure via trait objects**: Allows heterogeneous children (e.g., Box containing Label and Box widgets) without complex type signatures
- **Single-threaded model**: No need for `Rc<RefCell<>>` or `Arc<Mutex<>>` complexity; aligns with Constitution Principle IV (immutability after composition)

**Alternatives Considered**:
- **SmallVec optimization**: Would inline small widget counts to avoid heap allocation, but adds complexity and third-party dependency (violates Constitution Principle VII)
- **Arena allocation**: Would improve cache locality but requires lifetime management complexity and doesn't align with builder pattern
- **Rc/Arc for shared ownership**: Unnecessary complexity; widget tree has clear parent-child ownership hierarchy

**Memory Budget Analysis**:
- Estimated WidgetNode size: ~40 bytes (24 bytes `Box<dyn Widget>` + 4 bytes position + vtable overhead)
- For 100 widgets per page: ~4 KB widget tree overhead
- Well within 128 KB per-page budget (< 3% of budget)
- Leaves ~124 KB for character grid, style data, and other page allocations

**Implementation Notes**:
```rust
struct WidgetNode {
    widget: Box<dyn Widget>,  // Heap-allocated trait object
    position: (u16, u16),      // Relative to parent origin
}

impl<const WIDTH: u16, const HEIGHT: u16> Box<WIDTH, HEIGHT> {
    children: Vec<WidgetNode>  // Owned children
}
```

---

### 3. Validation Strategy: Compile-time vs Debug-time vs Runtime

**Decision**: Three-tier validation hierarchy per Constitution Principle VI

**Tier 1 - Compile-time validation (preferred)**:
- **Widget dimensions**: Const generics `Box::<WIDTH, HEIGHT>` enforce dimensions at type level
- **Trait bounds**: `impl Widget` ensures only valid widget types can be used
- **Lifetime safety**: Compiler prevents dangling references (e.g., `RegionHandle<'page>` cannot outlive `PageBuilder<'doc>`)

**Tier 2 - Development-time assertions (debug builds only)**:
- **Zero-size widget prevention**: `debug_assert!(WIDTH > 0 && HEIGHT > 0)` in `Box::new()`
- **Label HEIGHT constraint**: `debug_assert!(HEIGHT == 1)` in `Label::new()`
- **Zero cost in release builds**: All `debug_assert!` calls stripped by compiler

**Tier 3 - Runtime validation (only when compile-time impossible)**:
- **User-provided data**: Text content length validation in `Label::add_text(text) -> Result<Self, RenderError>`
- **Geometric validation**: AABB overlap detection during `add_child()` composition
- **Coordinate overflow**: Checked arithmetic in position calculations

**Rationale**:
- **Performance**: Compile-time validation has zero runtime cost; debug_assert! has zero release-build cost
- **Developer experience**: Compile-time errors caught by IDE immediately; debug_assert! catches violations during testing
- **Type safety**: Leverages Rust's type system to prevent entire classes of errors
- **Constitution compliance**: Aligns with Principle VI validation hierarchy and Principle IX zero-panic guarantee

**Alternatives Considered**:
- **Runtime validation for everything**: Would violate Constitution Principle XI performance targets (< 100 μs per page render)
- **Panic on all constraint violations**: Would violate Constitution Principle IX zero-panic guarantee in release builds
- **No validation (rely on documentation)**: Would violate Constitution Principle XV security requirements

**Undefined Behavior Policy**:
- Per Constitution Principle IX, documented constraint violations in release builds have undefined behavior
- Examples: `Box::<0, 10>::new()`, `Label::<20, 2>::new()`
- All constraints documented in API rustdoc `# Panics` sections
- Developers violating documented constraints accept responsibility

---

### 4. Error Handling Strategy for Widget Boundaries

**Decision**: Result-based error handling with specific error variants

**Rationale**:
- **Constitution Principle III Widget Exception**: Amendment (2025-11-19) explicitly allows widget boundary errors to bubble to caller
- **Developer control**: `Result<(), RenderError>` enables graceful error handling strategies (retry, fallback, user notification)
- **Early detection**: Composition-time validation (`add_child`) catches sizing mistakes before render phase
- **Debuggability**: Specific error variants (ChildExceedsParent, OutOfBounds, OverlappingChildren) provide actionable context

**Error Variants** (V1, marked `#[non_exhaustive]`):
```rust
#[non_exhaustive]
pub enum RenderError {
    ChildExceedsParent { child_width: u16, child_height: u16, parent_width: u16, parent_height: u16 },
    OutOfBounds { write_x: u16, write_y: u16, clip_width: u16, clip_height: u16 },
    OverlappingChildren { child1_id: String, child2_id: String, intersection: (u16, u16, u16, u16) },
    InsufficientSpace { requested: u16, available: u16 },
    IntegerOverflow { operation: String },
    TextExceedsWidth { text_len: usize, widget_width: u16 },
}
```

**Alternatives Considered**:
- **Silent truncation for widget boundaries**: Would align with PageBuilder/Region behavior but prevent developers from detecting layout bugs early
- **Panic on boundary violations**: Would violate Constitution Principle IX zero-panic guarantee
- **Generic error type**: Would lose contextual information needed for debugging (e.g., which children overlap, what the actual vs expected sizes were)

**Three-Layer Validation Architecture** (per FR-004):
- **Layer 1 (Widget Construction)**: `Label::add_text()` validates text.len() ≤ WIDTH; returns `Err(TextExceedsWidth)` if violated
- **Layer 2 (RenderContext)**: Validates write start position within clip_bounds; returns `Err(OutOfBounds)` if violated; delegates to PageBuilder when valid
- **Layer 3 (PageBuilder)**: Silently truncates content extending beyond bounds (ESC/P hardware compliance)

**Implementation Notes**:
- All errors include contextual fields for debugging
- `#[non_exhaustive]` allows future error variants without breaking changes (V2+)
- Errors returned during both composition phase (add_child) and render phase (render_to)

---

### 5. Layout Component Design: Returning Nested Boxes vs Area Objects

**Decision**: Layout components return `(Box<WIDTH, HEIGHT>, (u16, u16))` tuples

**Rationale**:
- **Simplicity**: Single widget type (Box) for all containers; no separate Area abstraction
- **Composability**: Returned Box widgets support `add_child()` for deep nesting
- **Constitution alignment**: Follows Principle VIII fixed-layout constraints (Layout components don't auto-size; dimensions specified via const generics at call site)
- **Developer clarity**: Explicit positioning via tuple `(widget, position)` makes layout structure visible

**API Design**:
```rust
// Column layout component
let column = Column::<80, 30>::new();
let (box1, pos1) = column.area::<10>()?;  // Box<80, 10> at calculated position
let (box2, pos2) = column.area::<20>()?;  // Box<80, 20> at calculated position
parent.add_child(box1, pos1)?;
parent.add_child(box2, pos2)?;

// Ergonomic macro alternative
let (box1, pos1) = column_area!(column, 10)?;
```

**Alternatives Considered**:
- **Separate Area type**: Would require additional type `Area { bounds: (u16, u16, u16, u16) }`, increasing API surface area without adding functionality
- **Layout components auto-add to parent**: Would hide positioning logic and prevent developers from reviewing layout before committing to widget tree
- **Layout components with runtime-sized areas**: Would violate Constitution Principle VIII fixed-layout constraints

**Generic Method Design**:
- Layout methods use generic parameters for dimensions: `fn area::<const H: u16>(&mut self) -> Result<(Box<W, H>, (u16, u16)), RenderError>`
- Developers specify dimensions at call site via turbofish syntax or macros
- Preserves compile-time dimension specification without requiring runtime-sized types

---

### 6. Overlap Detection Strategy

**Decision**: AABB (Axis-Aligned Bounding Box) collision detection with strict inequality

**Rationale**:
- **Performance**: O(1) per child pair; simple arithmetic operations
- **Determinism**: No floating-point calculations; integer-only arithmetic aligns with Constitution Principle I
- **Clarity**: Standard game-dev algorithm with well-understood semantics

**Algorithm** (per FR-005A):
```rust
// Two children overlap if rectangles intersect
fn overlaps(child1: &WidgetNode, child2: &WidgetNode) -> bool {
    let (x1, y1) = child1.position;
    let (x2, y2) = child2.position;
    let w1 = child1.widget.WIDTH;
    let h1 = child1.widget.HEIGHT;
    let w2 = child2.widget.WIDTH;
    let h2 = child2.widget.HEIGHT;

    // Strict inequality: touching edges (shared boundary) NOT overlap
    (x1 + w1) > x2 && x1 < (x2 + w2) && (y1 + h1) > y2 && y1 < (y2 + h2)
}
```

**Checked Arithmetic**:
- All position + size calculations use `checked_add()` to detect integer overflow
- Returns `IntegerOverflow` error when position + size exceeds `u16::MAX`
- Prevents wraparound bugs that could bypass boundary checks

**Alternatives Considered**:
- **No overlap detection**: Would allow overlapping children to corrupt layout; violates Constitution Principle XV security requirements
- **Inclusive boundaries (touching = overlap)**: Would prevent adjacent widgets from sharing edges (e.g., Column with no gaps)
- **Spatial indexing (quadtree, R-tree)**: Overkill for expected widget counts (< 100 per page); adds complexity

**Edge Cases Handled**:
- Negative coordinates prevented by type system (positions are `u16`, cannot be negative)
- Zero-size widgets prevented by `debug_assert!` in `Box::new()` (per FR-007 validation strategy)
- AABB assumes valid widget dimensions (WIDTH > 0, HEIGHT > 0) enforced at construction time

---

### 7. RenderContext Clip Bounds Management

**Decision**: Clip bounds intersection algorithm for nested widget clipping

**Rationale**:
- **Safety**: Constrains child rendering to parent bounds automatically
- **Performance**: Single intersection calculation per tree level; O(depth) complexity
- **Correctness**: Prevents children from rendering outside parent bounds even if PageBuilder would allow it

**Algorithm** (per RenderContext entity specification):
```rust
// When traversing into child widget at (child_abs_x, child_abs_y) with (child_width, child_height)
let new_clip_x = max(current_clip.x, child_abs_x);
let new_clip_y = max(current_clip.y, child_abs_y);
let new_clip_right = min(current_clip.x + current_clip.width, child_abs_x + child_width);
let new_clip_bottom = min(current_clip.y + current_clip.height, child_abs_y + child_height);
let new_clip_width = new_clip_right - new_clip_x;
let new_clip_height = new_clip_bottom - new_clip_y;

// After child rendering completes, restore parent's clip_bounds before processing next sibling
```

**Initial Clip Bounds**:
- Full page: `(0, 0, 160, 51)` per Constitution Principle II (160 columns × 51 lines)
- Updated during tree traversal to enforce nested clipping

**Delegation to PageBuilder** (per FR-004):
- **RenderContext returns OutOfBounds error when**: Write start position (x, y) falls outside clip_bounds
- **RenderContext delegates to PageBuilder when**: Write start position (x, y) is within clip_bounds
- **PageBuilder handles overflow**: Silent character-by-character truncation for content extending beyond bounds after valid start position

**Alternatives Considered**:
- **No clip bounds (rely on PageBuilder truncation)**: Would lose ability to detect when widgets render outside their allocated space; violates widget boundary safety contract
- **Clip bounds without intersection (just child bounds)**: Would allow children to render outside grandparent bounds in deeply nested trees
- **Per-cell clipping**: Too fine-grained; performance overhead not justified

**Restoration Strategy**:
- Save current clip_bounds before traversing into child
- Restore after child completes rendering
- Ensures siblings don't inherit child's clip bounds

---

### 8. Macro Wrapper Design for Ergonomics

**Decision**: Provide declarative macros that expand to turbofish syntax

**Rationale**:
- **Zero overhead**: Macros expand at compile time to identical code as turbofish syntax
- **Readability**: `box_new!(80, 30)` more concise than `Box::<80, 30>::new()`
- **Type safety preserved**: Macros are syntax sugar; all const generic validation still applies
- **Developer choice**: Both syntaxes officially supported; developers choose based on preference

**Macro Design Patterns**:
```rust
// Widget construction macros
macro_rules! box_new {
    ($w:expr, $h:expr) => { Box::<$w, $h>::new() };
}
macro_rules! label_new {
    ($w:expr) => { Label::<$w, 1>::new() };  // HEIGHT automatically set to 1
}

// Layout component macros
macro_rules! column_new {
    ($w:expr, $h:expr) => { Column::<$w, $h>::new() };
}
macro_rules! column_area {
    ($layout:expr, $h:expr) => { $layout.area::<$h>() };
}
```

**Alternatives Considered**:
- **Turbofish only (no macros)**: Less ergonomic; forces developers to use verbose syntax
- **Procedural macros for builder DSL**: Overkill complexity; would violate Constitution Principle VII (zero runtime dependencies) if requiring proc-macro crates
- **Generic functions with type inference**: Cannot infer const generic values; requires turbofish anyway

**Documentation Strategy** (per Constitution Principle XIV):
- All examples demonstrate both turbofish and macro syntax
- Rustdoc clearly states macros expand to turbofish syntax
- No functional difference; purely stylistic choice

---

## Technology Stack Summary

### Core Technologies
- **Language**: Rust 1.91.1+ (stable channel, 2021 edition)
- **Runtime Dependencies**: None (only Rust `std`)
- **Optional Dependencies**: `serde` (feature-gated for serialization support)

### Development & Testing
- **Testing Framework**: `cargo test` (built-in)
- **Property-Based Testing**: `proptest` (dev dependency)
- **Fuzzing**: `cargo-fuzz` with `libfuzzer-sys` (dev dependency)
- **Benchmarking**: `criterion` (dev dependency)

### Build & CI Requirements
- **MSRV**: Rust 1.91.1 (enforced via `rust-version` in Cargo.toml)
- **CI Checks**:
  - `cargo +1.91.1 fmt --check`
  - `cargo +1.91.1 clippy -- -D warnings`
  - `cargo +1.91.1 test --all-features`
  - `cargo +1.91.1 test --all-features --release` (verify debug_assert! behavior)
  - `cargo audit` (supply chain security)
  - `cargo tree --depth 1` (verify zero runtime deps)

---

## Best Practices Applied

### Rust API Design Guidelines
- **Const generics for compile-time dimensions**: Leverages Rust's type system for safety
- **Builder pattern**: `Label::<W, H>::new().add_text(text)?.bold()` for ergonomic configuration
- **Result-based error handling**: Idiomatic Rust error propagation via `?` operator
- **Trait objects for polymorphism**: `Box<dyn Widget>` enables heterogeneous widget trees
- **Lifetime annotations**: Explicit lifetimes prevent dangling references (existing `DocumentBuilder<'doc>` → `PageBuilder<'page>` hierarchy)

### Memory Safety
- **Zero unsafe code**: All code uses safe Rust abstractions
- **Ownership model**: Clear parent-child ownership via `Vec<WidgetNode>` (parent owns children)
- **Bounds checking**: Rust's automatic bounds checking prevents buffer overflows
- **Checked arithmetic**: `checked_add()` for all coordinate calculations prevents integer overflow

### Performance Optimization
- **Compile-time validation**: Dimensions known at compile time eliminate runtime checks
- **Inline hints**: `#[inline]` for hot path functions (RenderContext methods)
- **Pre-allocation**: Widget tree built once during composition; no allocations during render
- **Row-major memory layout**: Inherited from existing Page implementation for cache efficiency

### Documentation Standards
- **Rustdoc for all public APIs**: 100% public API coverage
- **`# Panics` sections**: Document all `debug_assert!` constraint violations
- **`# Errors` sections**: Document all `Result::Err` variants
- **Code examples**: Compilable examples tested via `cargo test --doc`
- **Both syntaxes demonstrated**: Turbofish and macro wrappers in all examples

---

## Implementation Risks & Mitigations

### Risk 1: Memory Budget Violation (< 128 KB per page)
**Likelihood**: Low
**Impact**: High (violates Constitution Principle X)
**Mitigation**:
- Memory profiling during Phase 1 implementation with `heaptrack` or `valgrind --tool=massif`
- Estimated widget tree overhead: ~4 KB for 100 widgets (well within budget)
- Benchmark memory usage in CI to detect regressions

### Risk 2: Performance Regression (> 100 μs per page render)
**Likelihood**: Low
**Impact**: Medium (violates Constitution Principle XI)
**Mitigation**:
- Inline hot path functions (RenderContext::write_text, etc.)
- Const generic dimensions eliminate runtime size checks
- Single-pass tree traversal with minimal allocations
- Criterion benchmarks in CI to detect regressions

### Risk 3: Developer Misuse of Undefined Behavior APIs
**Likelihood**: Medium
**Impact**: Medium (incorrect rendering in release builds)
**Mitigation**:
- Extensive rustdoc `# Panics` sections documenting all constraints
- Glossary section in spec.md explaining undefined behavior policy
- Debug assertions catch violations during testing
- Compile tests (e.g., T029) verify that invalid usage compiles (as expected)

### Risk 4: Breaking Changes to PageBuilder/Region APIs
**Likelihood**: Low (feature builds on existing infrastructure)
**Impact**: High (violates Constitution Principle II frozen spec)
**Mitigation**:
- Widget system delegates to PageBuilder via RenderContext wrapper (no PageBuilder modifications)
- Page::render() is new method (no existing API changes)
- Existing Region abstraction remains for backward compatibility

---

## References

### Feature Specification
- `/specs/002-widget-composability/spec.md` - Complete feature requirements and clarifications

### Constitution
- `.specify/memory/constitution.md` v1.3.0 (amended 2025-11-19)
  - Principle III: Widget Exception amendments (v1.1.0, v1.2.0)
  - Principle VI: Validation Strategy subsection (v1.2.1)
  - Principle VI: Widget Construction Syntax subsection (v1.2.2)
  - MSRV update to Rust 1.91.1 (v1.3.0)

### Rust Documentation
- Rust const generics: https://doc.rust-lang.org/reference/items/generics.html
- Trait objects: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
- Error handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html

---

**Document Status**: ✅ COMPLETE
**Phase 0 Deliverable**: All technical unknowns resolved; ready for Phase 1 design artifacts (data-model.md, contracts/, quickstart.md)

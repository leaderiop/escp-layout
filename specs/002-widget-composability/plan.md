# Implementation Plan: Widget Composability System

**Branch**: `002-widget-composability` | **Date**: 2025-11-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-widget-composability/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add React-like composability to the widget system while keeping the ESC/P page/backing grid model. Implement a two-phase rendering system (composition + render) with parent-child widget trees, automatic coordinate calculation, boundary enforcement via Result-based error handling, and Layout components (Column, Row, Stack) returning nested Box widgets. All widgets use compile-time const generic dimensions (WIDTH, HEIGHT) with both turbofish syntax and ergonomic macro wrappers. System enforces widget boundaries and content validation through errors while maintaining silent truncation for underlying PageBuilder/Region per Constitution Principle III Widget Exception.

## Technical Context

**Language/Version**: Rust 1.91.1+ (stable channel, 2021 edition)
**Primary Dependencies**: Zero runtime dependencies (only Rust `std`); optional `serde` feature-gated
**Storage**: N/A (in-memory layout engine)
**Testing**: `cargo test` (unit, integration, property-based with `proptest`, fuzzing with `cargo-fuzz`)
**Target Platform**: Cross-platform (Linux, macOS, Windows); ESC/P printer compatibility (EPSON LQ-2090II)
**Project Type**: Single library crate (Rust library)
**Performance Goals**: Single page render < 100 μs (p99); 100-page document < 10 ms (p99); zero allocations in render hot path
**Constraints**: Total page memory usage < 128 KB per page (including widget tree, character grid, style data); deterministic byte-identical output; zero runtime panics in release builds; compile-time validation preferred over runtime validation
**Scale/Scope**: Widget trees up to arbitrary nesting depth (stack limit ~250K levels theoretically); fixed-size layouts only (no dynamic sizing in V1)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I: Deterministic Behavior
✅ **PASS** - Widget composition uses deterministic tree traversal; no HashMap iteration; integer-only arithmetic; const generic dimensions ensure consistent sizing

### Principle II: V1 Specification Freeze
✅ **PASS** - Widget system builds on existing 160×51 page grid; no dynamic page sizing; static layout only; maintains ESC/P text mode

### Principle III: Strict Truncation and Clipping (with Widget Exception)
✅ **PASS (with documented exception)** - Feature explicitly uses Constitution Principle III Widget Exception amendment (added 2025-11-19): Widget boundary violations return `Result<(), RenderError>` with specific error variants (ChildExceedsParent, OutOfBounds, OverlappingChildren, TextExceedsWidth). Underlying PageBuilder/Region maintain silent truncation. Three-layer validation architecture documented in FR-004.

### Principle IV: Immutability Guarantees
✅ **PASS** - Widget tree built in composition phase (mutable `add_child` calls), then rendered immutably via `&impl Widget` borrows; widgets can be rendered multiple times without rebuilding tree; Page::render takes `&mut self, widget: &impl Widget`

### Principle V: ESC/P Text-Mode Compliance
✅ **PASS** - Widget rendering delegates to existing PageBuilder; maintains ESC/P command compliance; ASCII-only output preserved; no graphics modes

### Principle VI: Stable Rust Builder API Design
✅ **PASS** - MSRV: Rust 1.91.1+; const generics for compile-time dimensions; two equivalent syntaxes (turbofish `Box::<80, 30>::new()` vs macro `box_new!(80, 30)`); Result-based error handling; validation hierarchy: compile-time (const generics) > development-time (debug_assert!) > runtime (Result for user data); zero unsafe code planned

### Principle VII: Zero Runtime Dependencies
✅ **PASS** - Feature uses only Rust `std`; no external crates required; builds on existing Page/PageBuilder infrastructure

### Principle VIII: Fixed-Layout Constraints
✅ **PASS** - All widget dimensions specified at construction time via const generics; no content-based auto-sizing; no constraint solvers; Widget trait has associated constants `const WIDTH: u16; const HEIGHT: u16;`

### Principle IX: Zero-Panic Guarantee
✅ **PASS** - Release builds guaranteed panic-free; boundary violations return Result errors; debug_assert! used for contract violations (e.g., zero-size Box, Label HEIGHT ≠ 1); documented constraint violations have undefined behavior in release builds per constitution policy

### Principle X: Memory Efficiency and Predictability
⚠️ **REVIEW REQUIRED** - Total page memory budget: < 128 KB per page (updated from original < 16 KB page model constraint). Widget tree overhead: children stored as `Vec<WidgetNode>` where `WidgetNode { widget: Box<dyn Widget>, position: (u16, u16) }`. Need to verify widget tree memory usage stays within budget. Box heap allocation accepted for simplicity per clarification (2025-11-19).

### Principle XI: Performance Targets
✅ **PASS** - Single-pass tree traversal during render; const generics enable compile-time validation (zero runtime overhead); inline hints planned for hot path; no allocations in render loop (pre-allocated widget tree)

### Principle XII: Comprehensive Testing Requirements
✅ **PASS** - Success Criterion SC-006 mandates compilable example in examples/ directory with test coverage in tests/widget/integration.rs; unit tests for widget construction, composition errors, boundary validation; integration tests for multi-level nesting (≥3 levels per SC-006); property-based tests for overlap detection, coordinate overflow; golden master tests for widget output

### Principle XIII: API Stability and Versioning
✅ **PASS** - New public APIs added (Widget trait, Box, Label, Layout components, RenderError, RenderContext); no breaking changes to existing Page/PageBuilder APIs; RenderError marked #[non_exhaustive] for future expansion without breaking changes

### Principle XIV: Documentation Requirements
✅ **PASS** - FR-007C, Constitution Principle VI, and glossary require extensive rustdoc with `# Panics` sections documenting debug_assert! behavior and undefined behavior in release builds; both turbofish and macro syntaxes must be documented; quickstart.md generation planned in Phase 1

### Principle XV: Security and Safety
✅ **PASS** - Zero unsafe code; widget content validation at construction (TextExceedsWidth error); AABB overlap detection using checked arithmetic (IntegerOverflow error); const generics prevent many invalid states at compile-time; debug_assert! for contract violations

### Principle XVI: Specification Validation Process
✅ **PASS** - Feature approved via spec.md; no breaking changes to frozen V1 API; builds on existing infrastructure; Constitution amendments (Principle III Widget Exception v1.1.0, v1.2.0) approved and documented

### Principle XVII: Code Review Standards
✅ **PASS** - Feature plan follows Rust API guidelines; validation strategy documented (compile-time > debug_assert! > runtime); const generic widget construction; comprehensive test coverage planned; widget content validation via Result errors

### Principle XVIII: V2+ Feature Planning
✅ **PASS** - Widget composability is foundational for future V2 features (dynamic sizing, constraint-based layout); architecture maintains V1 fixed-layout constraints while enabling future extension

### Summary
**Status**: ✅ **APPROVED** (with memory review in Phase 1)
**Violations**: None
**Review Items**:
- Memory profiling during Phase 1 implementation to verify < 128 KB per page budget
- Verify widget tree overhead calculation: estimate ~40 bytes per WidgetNode (24 bytes Box<dyn Widget> + 4 bytes position + vtable overhead)
- For 100 widgets per page: ~4 KB widget tree overhead, well within 128 KB budget
- Document memory layout in data-model.md during Phase 1

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── widget/
│   ├── mod.rs              # Widget trait definition with associated constants
│   ├── box_widget.rs       # Box<WIDTH, HEIGHT> container implementation
│   ├── label.rs            # Label<WIDTH, HEIGHT> leaf widget
│   ├── tree.rs             # WidgetNode and widget tree structures
│   └── macros.rs           # Ergonomic macros (box_new!, label_new!, etc.)
├── layout/
│   ├── mod.rs              # Layout component traits
│   ├── column.rs           # Column<WIDTH, HEIGHT> layout component
│   ├── row.rs              # Row<WIDTH, HEIGHT> layout component
│   ├── stack.rs            # Stack<WIDTH, HEIGHT> layout component
│   └── macros.rs           # Layout macros (column_new!, column_area!, etc.)
├── render/
│   ├── context.rs          # RenderContext with clip bounds validation
│   ├── error.rs            # RenderError enum with variants
│   └── page.rs             # Page::render() method implementation
├── page_builder.rs         # Existing PageBuilder (unchanged)
├── region.rs               # Existing Region (unchanged)
└── lib.rs                  # Public API exports

tests/
├── widget/
│   ├── box_tests.rs        # Box widget unit tests
│   ├── label_tests.rs      # Label widget unit tests
│   ├── composition.rs      # Composition phase tests (add_child)
│   ├── validation.rs       # Boundary/overlap validation tests
│   └── integration.rs      # Multi-level nesting tests (SC-006)
├── layout/
│   ├── column_tests.rs     # Column layout component tests
│   ├── row_tests.rs        # Row layout component tests
│   └── stack_tests.rs      # Stack layout component tests
├── render/
│   ├── context_tests.rs    # RenderContext clipping tests
│   ├── error_tests.rs      # RenderError contextual info tests
│   └── determinism.rs      # Deterministic output property tests
└── golden_master/
    └── widget_output.rs    # Golden master tests for widget rendering

examples/
└── widget_composition.rs   # Multi-level nesting example (SC-006)
```

**Structure Decision**: Single Rust library crate structure (Option 1 adapted for Rust). Widget system implemented as a new module alongside existing Page/PageBuilder infrastructure. Tests organized by feature area (widget, layout, render) with integration tests and golden master tests. Compilable example required by SC-006 in examples/ directory.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**No violations detected.** Constitution Check passed with memory review item flagged for Phase 1.

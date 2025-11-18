# Implementation Plan: Rust ESC/P Layout Engine Library

**Branch**: `001-rust-escap-layout-engine` | **Date**: 2025-11-18 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-rust-escap-layout-engine/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a Rust library that implements a fully deterministic text-based layout engine for the Epson LQ-2090II dot-matrix printer using ESC/P condensed text mode. The engine renders content onto a fixed 160×51 character page matrix with strict boundaries, silent truncation, and byte-for-byte deterministic ESC/P output. The library must use a modular, clean architecture with minimal dependencies, zero heap allocations in hot loops, fixed-size data structures, immutable Pages/Documents after finalization, and a builder API that enforces geometry correctness at construction time.

## Technical Context

**Language/Version**: Rust 1.75+ (stable channel, 2021 edition)
**Primary Dependencies**: Zero runtime dependencies (only Rust `std`); optional `serde` feature-gated
**Storage**: In-memory only (fixed 160×51 Cell grid per page, contiguous row-major array)
**Testing**: `cargo test` (unit), `proptest` (property-based), `libfuzzer-sys` (fuzzing), `criterion` (benchmarks), golden-file tests for ESC/P output
**Target Platform**: Cross-platform (Linux, macOS, Windows), library crate (no binary target in V1)
**Project Type**: Single Rust library crate (no web/mobile components)
**Performance Goals**: Single page render <100μs (p99), 100-page document <10ms (p99), zero allocations in hot rendering loop
**Constraints**: Byte-for-byte deterministic output, no heap allocations in hot loop, immutable after finalization, no `unsafe` code, <2MB binary size with LTO
**Scale/Scope**: Library API surface ~20 public types, support for 6 widget types, ESC/P state machine with 4 style states, targeting industrial 24/7 environments

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I: Deterministic Behavior (NON-NEGOTIABLE)
✅ **PASS** - Feature spec explicitly requires byte-for-byte deterministic output (FR-018, SC-002). Technical approach enforces:
- No timestamps, UUIDs, or random values
- No HashMap (will use Vec/BTreeMap for stable ordering)
- No floating-point (integer arithmetic only)
- Fixed 160×51 Cell grid eliminates dynamic behavior

### Principle II: V1 Specification Freeze (NON-NEGOTIABLE)
✅ **PASS** - Feature scope strictly adheres to frozen V1 constraints:
- Fixed page dimensions: 160×51 (FR-001)
- ASCII-only output (FR-024)
- ESC/P text mode only (FR-021)
- Static layout only (FR-003-007)
- Manual pagination (FR-002, FR-020)
- Silent truncation (FR-008, FR-012)

### Principle III: Strict Truncation and Clipping (NON-NEGOTIABLE)
✅ **PASS** - Feature spec mandates silent truncation without errors:
- FR-008: Content never exceeds region boundaries (silent truncation)
- FR-012: Widgets respect boundaries and truncate when full
- User Story 2 (P1): Explicit overflow handling requirements
- Edge cases document zero-size region behavior

### Principle IV: Immutability Guarantees (NON-NEGOTIABLE)
✅ **PASS** - Feature spec requires immutability after finalization:
- FR-019: Pages and Documents immutable after finalization
- FR-022: Builder pattern for construction
- User Story 1 Scenario 4: Finalized pages cannot be modified
- User Story 3 Scenario 3: Finalized documents cannot be modified

### Principle V: ESC/P Text-Mode Compliance (NON-NEGOTIABLE)
✅ **PASS** - Feature spec constrains to ESC/P text mode:
- FR-014-017: ESC/P rendering with initialization, style codes, form-feeds
- FR-021: No bitmap or graphic modes
- FR-024: Non-ASCII character handling
- FR-025: Style state reset at line/page end
- SC-010: Output accepted by Epson LQ-2090II printers

### Principle VI: Stable Rust Builder API Design (NON-NEGOTIABLE)
✅ **PASS** - Feature spec emphasizes builder API:
- FR-022: Builder pattern for Pages and Documents
- FR-023: Geometry validation at construction time
- SC-001: <10 lines of code for single-page document
- User input emphasizes "builder APIs enforce geometry correctness at construction time"

### Principle VII: Zero Runtime Dependencies (NON-NEGOTIABLE)
✅ **PASS** - Feature spec and user input mandate minimal dependencies:
- User input: "minimal external dependencies"
- Technical Context: Zero runtime dependencies (only Rust std)
- Optional serde feature-gated only

### Principle VIII: Fixed-Layout Constraints
✅ **PASS** - Feature scope excludes auto-layout:
- FR-003-007: Explicit region dimensions
- FR-020: No automatic page creation
- User Story 4: Explicit vertical/horizontal splits with line/char counts
- V1 frozen spec excludes dynamic layout

### Principle IX: Zero-Panic Guarantee
✅ **PASS** - Feature requirements include fuzzing and robust error handling:
- FR-023: Geometry validation returns Result
- FR-008, FR-012: Silent truncation (no panics on overflow)
- Testing: libfuzzer-sys for fuzzing, proptest for property-based testing
- Edge cases documented with expected behaviors (no errors)

### Principle X: Memory Efficiency and Predictability
✅ **PASS** - User input and feature spec enforce memory efficiency:
- User input: "avoid heap allocations inside hot loop"
- User input: "fixed-size data structures"
- User input: "160×51 Cell grid stored in contiguous memory"
- Performance Goals: Zero allocations in hot rendering loop
- SC-009: 10-page document renders in <100ms

### Principle XI: Performance Targets (NON-NEGOTIABLE)
✅ **PASS** - Feature spec includes explicit performance targets:
- SC-009: 10-page document <100ms
- Performance Goals: Single page <100μs, 100-page document <10ms
- Benchmarking with criterion included in testing strategy

### Principle XII: Comprehensive Testing Requirements
✅ **PASS** - Feature spec includes extensive testing requirements:
- Unit tests: cargo test
- Property-based: proptest
- Fuzzing: libfuzzer-sys
- Golden-file tests for ESC/P output
- Hardware validation: SC-010 (Epson LQ-2090II acceptance)
- 6 user stories with 4 scenarios each = 24+ integration scenarios

### Principle XIII-XVIII: Governance, Documentation, Security
✅ **PASS** - Feature requirements align with governance principles:
- FR-019: Immutability supports API stability
- Constraints: No unsafe code
- Testing: Comprehensive coverage (unit, integration, property-based, fuzzing)
- Constitution itself provides architecture governance

### **GATE RESULT: ✅ ALL CHECKS PASS - PROCEED TO PHASE 0**

## Project Structure

### Documentation (this feature)

```text
specs/001-rust-escap-layout-engine/
├── plan.md              # This file (/speckit.plan command output) ✅
├── research.md          # Phase 0 output (/speckit.plan command) ✅
├── data-model.md        # Phase 1 output (/speckit.plan command) ✅
├── quickstart.md        # Phase 1 output (/speckit.plan command) ✅
├── contracts/           # Phase 1 output (/speckit.plan command) ✅
│   ├── public-api.md    # Complete public API contract ✅
│   └── escp-output-spec.md  # ESC/P byte stream specification ✅
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

**Structure Decision**: Single Rust library crate (no web/mobile/backend components). This is a library project targeting use as a dependency in other Rust applications.

```text
escp-layout/  (or matrix/ - root of repository)
├── Cargo.toml                  # Package manifest
├── Cargo.lock                  # Dependency lockfile
├── README.md                   # User-facing documentation
├── LICENSE                     # License file
├── .gitignore                  # Git ignore patterns
│
├── src/                        # Library source code
│   ├── lib.rs                  # Public API exports and module declarations
│   ├── cell.rs                 # Cell and StyleFlags types
│   ├── page.rs                 # Page and PageBuilder
│   ├── document.rs             # Document and DocumentBuilder
│   ├── region.rs               # Region geometry and operations
│   ├── error.rs                # LayoutError type
│   │
│   ├── widgets/                # Widget implementations
│   │   ├── mod.rs              # Widget trait and shared utilities
│   │   ├── label.rs            # Label widget
│   │   ├── text_block.rs       # TextBlock widget
│   │   ├── paragraph.rs        # Paragraph widget (with wrapping)
│   │   ├── ascii_box.rs        # ASCIIBox widget
│   │   ├── key_value.rs        # KeyValueList widget
│   │   └── table.rs            # Table widget
│   │
│   └── escp/                   # ESC/P rendering engine
│       ├── mod.rs              # Public rendering interface
│       ├── renderer.rs         # Main rendering logic
│       ├── state.rs            # RenderState state machine
│       └── constants.rs        # ESC/P command byte sequences
│
├── tests/                      # Integration and test files
│   ├── unit/                   # Unit tests
│   │   ├── cell_tests.rs
│   │   ├── region_tests.rs
│   │   ├── page_tests.rs
│   │   └── escp_tests.rs
│   │
│   ├── integration/            # Integration tests (end-to-end)
│   │   ├── single_page_tests.rs
│   │   ├── multi_page_tests.rs
│   │   ├── widget_tests.rs
│   │   └── nested_region_tests.rs
│   │
│   ├── property/               # Property-based tests (proptest)
│   │   ├── determinism_tests.rs
│   │   ├── truncation_tests.rs
│   │   └── no_panic_tests.rs
│   │
│   └── golden/                 # Golden master test files
│       ├── invoice.bin
│       ├── report.bin
│       ├── complex_layout.bin
│       └── golden_tests.rs     # Test harness for golden files
│
├── benches/                    # Criterion benchmarks
│   ├── render_bench.rs         # Document rendering benchmarks
│   └── widget_bench.rs         # Widget rendering benchmarks
│
├── examples/                   # Example usage code
│   ├── hello_world.rs          # Minimal example
│   ├── invoice.rs              # Invoice generation example
│   └── report.rs               # Multi-page report example
│
└── fuzz/                       # Fuzzing targets (cargo-fuzz)
    ├── Cargo.toml
    └── fuzz_targets/
        ├── fuzz_region.rs      # Fuzz region creation
        └── fuzz_render.rs      # Fuzz rendering pipeline
```

**Rationale**:
- **Modular separation**: Layout logic (region, page), widget logic (widgets/), and ESC/P rendering (escp/) are cleanly separated
- **Testability**: Each module has corresponding unit tests; integration tests validate end-to-end behavior
- **Future extensibility**: New widgets can be added to `widgets/` without affecting core; ESC/P extensions go in `escp/`
- **Best practices**: Follows Rust project conventions (src/, tests/, benches/, examples/)
- **Constitution compliance**: Structure supports zero dependencies, isolated modules, comprehensive testing

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**STATUS**: ✅ No constitution violations detected. All design choices align with constitution principles.

---

## Phase 0: Research Complete ✅

All technical decisions documented in `research.md`:
- Memory layout for Cell grid
- Cell structure and bit packing
- Region representation
- Builder API architecture
- ESC/P rendering state machine
- Widget trait design
- Error handling strategy
- Testing strategy
- Module structure
- ESC/P command reference

**No unresolved questions remain.**

---

## Phase 1: Design & Contracts Complete ✅

### Artifacts Generated

1. **data-model.md**: Complete data model with all types, relationships, and validation rules
2. **contracts/public-api.md**: Full API specification with signatures, contracts, examples
3. **contracts/escp-output-spec.md**: Byte-level ESC/P output format specification
4. **quickstart.md**: User-facing quickstart guide (< 5 min to first working example)
5. **CLAUDE.md**: Agent context updated with Rust, dependencies, and project structure

### Post-Design Constitution Re-Check

Re-evaluating all principles after design phase:

#### Principle I: Deterministic Behavior
✅ **MAINTAINED** - Design enforces:
- Fixed Cell grid (no dynamic allocation)
- BTreeMap/Vec for stable ordering (no HashMap)
- Integer-only calculations
- State machine with deterministic transitions
- SHA-256 verification built into testing strategy

#### Principle II: V1 Specification Freeze
✅ **MAINTAINED** - Design adheres to frozen constraints:
- Fixed 160×51 dimensions (compile-time guarantee via array types)
- ASCII-only (Cell::new converts non-ASCII to '?')
- ESC/P text mode only (escp/constants.rs defines allowed commands)
- Static layout (Region is value type with explicit dimensions)
- Manual pagination (Document::builder().add_page())
- Silent truncation (PageBuilder::write_at checks bounds, no panic)

#### Principle III: Strict Truncation and Clipping
✅ **MAINTAINED** - Design implements silent truncation:
- PageBuilder::write_at: `if x < 160 && y < 51` guard (silent return otherwise)
- Widget::render contract: MUST handle zero-size regions, MUST truncate
- No Result returns for overflow (only for geometry validation)

#### Principle IV: Immutability Guarantees
✅ **MAINTAINED** - Design enforces immutability:
- Builder pattern: PageBuilder → .build() → Page (no public mutable methods)
- Type state pattern considered (can be implemented with PhantomData)
- Document owns Vec<Page>, no public mutable access
- Cell, StyleFlags, Region are Copy types (inherently immutable per-instance)

#### Principle V: ESC/P Text-Mode Compliance
✅ **MAINTAINED** - Design constrains to text mode:
- escp/constants.rs defines only text-mode commands
- RenderState tracks bold/underline only (text styles)
- No bitmap/graphics commands defined
- Character validation: Cell::new enforces ASCII

#### Principle VI: Stable Rust Builder API Design
✅ **MAINTAINED** - Design follows Rust best practices:
- Builder pattern with method chaining
- Result<T, LayoutError> for geometry errors
- Lifetimes avoided via Copy semantics (Region) and owned data (builders)
- No unsafe code in design
- Trait bounds documented (Send + Sync for Document/Page)

#### Principle VII: Zero Runtime Dependencies
✅ **MAINTAINED** - Design uses only std:
- No external crates in core types
- Widget trait uses trait objects (no external dependencies)
- ESC/P rendering uses Vec<u8> and byte arrays
- Optional serde feature-gated (dev dependencies only: proptest, criterion)

#### Principle VIII: Fixed-Layout Constraints
✅ **MAINTAINED** - Design enforces static layout:
- Region::new requires explicit x, y, width, height
- Region::split_* requires explicit split dimensions
- No auto-sizing, no constraint solvers, no relative dimensions

#### Principle IX: Zero-Panic Guarantee
✅ **MAINTAINED** - Design prevents panics:
- Geometry validation returns Result
- Out-of-bounds writes silently ignored (if guards)
- Widget contract forbids panics
- Testing strategy includes fuzzing (1M+ iterations)

#### Principle X: Memory Efficiency and Predictability
✅ **MAINTAINED** - Design optimizes memory:
- Fixed Box<[[Cell; 160]; 51]> per page (~16 KB, predictable)
- Cell is 2 bytes (character + style)
- Region is 8 bytes (Copy type, stack-allocated)
- Row-major layout for cache efficiency
- Zero allocations in rendering loop (write directly to Vec<u8>)

#### Principle XI: Performance Targets
✅ **MAINTAINED** - Design supports performance goals:
- Direct array indexing (O(1) cell access)
- No intermediate allocations in hot path
- State machine minimizes ESC/P code emission
- Criterion benchmarks planned for validation

#### Principle XII: Comprehensive Testing Requirements
✅ **MAINTAINED** - Testing strategy defined:
- Unit tests: Per-module in tests/unit/
- Integration tests: tests/integration/
- Property-based: tests/property/ with proptest
- Golden master: tests/golden/
- Fuzzing: fuzz/fuzz_targets/
- Benchmarks: benches/

#### Principles XIII-XVIII: Governance, Documentation, Security
✅ **MAINTAINED** - Design aligns with governance:
- API stability: SemVer 2.0.0, no breaking changes in V1.x.x
- Documentation: All public APIs have rustdoc (per design)
- Security: Zero unsafe code, input validation, no external dependencies
- Versioning: Deprecation policy documented in public-api.md

### **POST-DESIGN GATE RESULT: ✅ ALL PRINCIPLES MAINTAINED**

---

## Phase 2: Task Generation (NOT INCLUDED IN THIS COMMAND)

**Next Command**: `/speckit.tasks` - Generate tasks.md with dependency-ordered implementation tasks

**Expected Outputs**:
- tasks.md with concrete implementation steps
- GitHub issues (if /speckit.taskstoissues used)

---

## Summary

**Implementation Plan Complete** ✅

### Deliverables

| Artifact | Status | Location |
|----------|--------|----------|
| Technical Context | ✅ Complete | plan.md (this file) |
| Constitution Check | ✅ Pass (pre & post design) | plan.md (this file) |
| Research | ✅ Complete | research.md |
| Data Model | ✅ Complete | data-model.md |
| Public API Contract | ✅ Complete | contracts/public-api.md |
| ESC/P Output Spec | ✅ Complete | contracts/escp-output-spec.md |
| Quickstart Guide | ✅ Complete | quickstart.md |
| Agent Context | ✅ Updated | CLAUDE.md |
| Project Structure | ✅ Defined | plan.md (this file) |

### Key Decisions

1. **Memory Layout**: Row-major Box<[[Cell; 160]; 51]> for cache efficiency
2. **Cell Structure**: 2-byte compact representation (character + bit-packed styles)
3. **Region Design**: Lightweight Copy value type (8 bytes, no lifetimes)
4. **Builder Pattern**: Type-safe construction with consuming .build()
5. **ESC/P State Machine**: Explicit state tracking, minimal transitions
6. **Widget System**: Trait objects for extensibility
7. **Error Strategy**: Result for geometry, silent truncation for overflow
8. **Testing**: Multi-layered (unit, integration, property-based, golden, fuzzing)
9. **Module Structure**: Clean separation (layout, widgets, escp)
10. **Zero Dependencies**: Only Rust std (optional serde feature-gated)

### Architecture Validation

- ✅ Deterministic rendering guaranteed
- ✅ Immutability enforced at API level
- ✅ Zero-panic guarantee via design contracts
- ✅ Fixed layout constraints respected
- ✅ ESC/P text-mode compliance verified
- ✅ Performance targets achievable
- ✅ Comprehensive testing planned
- ✅ Constitution compliance maintained

**Ready for Phase 2: Task Generation** (`/speckit.tasks`)

---

**Branch**: `001-rust-escap-layout-engine`
**Plan Document**: `/Users/mohammadalmechkor/Projects/matrix/specs/001-rust-escap-layout-engine/plan.md`
**Status**: Planning Phase Complete ✅

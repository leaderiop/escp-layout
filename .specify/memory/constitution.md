<!--
Version Change: 1.3.0 (Amendment: Rust Version Update to 1.91.1)
Modified Principles: None (technical specification update only)
Added Sections: None
Removed Sections: None
Templates Requiring Updates:
  ⚠ plan-template.md (update Rust version examples from 1.75 to 1.91.1)
  ⚠ Cargo.toml (update rust-version field)
  ⚠ specs/*/plan.md (update Language/Version field)
  ⚠ CLAUDE.md (auto-generated, will update on next run)
  ⚠ docs/PRD.md (update MSRV references)
  ⚠ docs/API-SPEC.md (update MSRV references)
  ⚠ specs/*/quickstart.md (update prerequisite version)
Follow-up TODOs:
  - Update Cargo.toml rust-version = "1.91.1"
  - Update all spec.md references from "Rust 1.75+" to "Rust 1.91.1+"
  - Update all plan.md Language/Version fields
  - Run CI to verify Rust 1.91.1 compatibility
  - Update docs/PRD.md MSRV requirement
  - Update docs/API-SPEC.md MSRV specification
Rationale for Amendment:
  Upgraded Minimum Supported Rust Version (MSRV) from 1.75.0 to 1.91.1 to leverage
  newer const generic improvements, better diagnostics, and stabilized features that
  improve the widget composability system. Rust 1.91.1 provides better const generic
  value constraints which align with Constitution Principle VI's preference for
  compile-time validation. This is a MINOR version bump because it changes a technical
  specification (MSRV) that affects compatibility but doesn't alter governance principles.
Previous Version: 1.2.2 (amended 2025-11-19)
Amendment Date: 2025-11-19
-->

# EPSON LQ-2090II Rust Layout Engine — Project Constitution

## Core Principles

### I. Deterministic Behavior (NON-NEGOTIABLE)

**Principle**: The library MUST produce byte-for-byte identical ESC/P output for identical inputs across all platforms, compiler versions (Rust 1.91.1+), and execution environments.

**Requirements**:

- Same `Document` input → Identical ESC/P byte stream output
- No timestamps, UUIDs, or random values in rendering pipeline
- No HashMap iteration (use Vec/BTreeMap for stable ordering)
- No floating-point calculations (use integer arithmetic exclusively)
- SHA-256 hash verification MUST pass across 1000+ rendering iterations
- Zero non-deterministic code paths in rendering engine

**Rationale**: Regulatory compliance, golden master testing, output caching, and predictable production behavior are foundational requirements. Non-deterministic output is incompatible with the compliance demands of industrial printing applications.

**Validation**: Every PR MUST include determinism tests. Rendering benchmarks MUST verify byte-identical output across multiple runs.

---

### II. V1 Specification Freeze (NON-NEGOTIABLE)

**Principle**: The V1 specification is frozen and MUST NOT be changed without explicit architectural review and approval.

**Fixed Constraints**:

- Page dimensions: 160 columns × 51 lines (EPSON LQ-2090II condensed mode)
- ASCII-only output (characters 32-126)
- ESC/P text mode only (no bitmap/graphics modes)
- Static layout only (no auto-layout or dynamic height)
- Manual pagination only (no automatic page breaks)
- Fixed truncation rules (horizontal and vertical)

**Prohibited in V1**:

- Dynamic page sizing
- Automatic pagination
- Auto-layout algorithms (flexrect-like systems)
- Unicode text shaping or bidirectional text
- Graphics modes (bitmap, raster, vector)
- Runtime configuration of page dimensions

**Rationale**: Simplicity, predictability, and achievable timeline. V1 establishes a solid foundation for future versions without introducing complexity that compromises correctness.

**Migration Path**: V2 MAY introduce dynamic features, but MUST maintain V1 compatibility through versioned APIs or feature flags.

---

### III. Strict Truncation and Clipping (NON-NEGOTIABLE, with Widget Exception)

**Principle**: All content overflow MUST be silently truncated. Truncation is NOT an error condition and MUST NOT produce warnings or panics.

**Truncation Rules**:

- **Horizontal truncation**: Content beyond region width is dropped character-by-character
- **Vertical truncation**: Lines beyond region height are discarded line-by-line
- **Page boundary truncation**: Writes outside (160×51) bounds are silently ignored
- **No errors**: Content overflow does NOT return `Result::Err`
- **No panics**: Overflow MUST NOT trigger panics in release builds

**Implementation**:

```rust
// ✅ Correct: Silent truncation (PageBuilder, Region)
if x >= width || y >= height {
    return; // Silent drop
}

// ❌ Wrong: Error on overflow (PageBuilder, Region)
if x >= width {
    return Err(LayoutError::Overflow); // NOT ALLOWED
}
```

**Widget Composability Exception** (Added 2025-11-19, Expanded 2025-11-19):
The widget composability system (feature 002) is **exempt** from silent truncation rules and MUST return `Result<(), RenderError>` for violations:

- **Boundary violations during composition**: `parent.add_child(child, pos)` returns errors for:

  - Child widget exceeds parent bounds (`ChildExceedsParent`)
  - Out-of-bounds positioning (`OutOfBounds`)
  - Zero-size parent widget (`ZeroSizeParent`)
  - Overlapping children (`OverlappingChildren`)

- **Content validation at widget construction**: Widget constructors return errors for:

  - Text content exceeding widget dimensions (e.g., `Label::<WIDTH, HEIGHT>::new(text)` where `text.len() > WIDTH` returns `Err(TextExceedsWidth)`)
  - Invalid content that cannot fit within declared widget bounds

- **Boundary violations during render phase**: `widget.render_to(context, pos)` returns errors for:
  - Writes outside widget bounds
  - Content positioning violations

**Validation Strategy** (Added 2025-11-19):

- **Prefer compile-time validation**: Use const generics and type system to prevent invalid states
- **Use debug_assert!**: For development-time contract checks (zero runtime cost in release builds)
- **Runtime Result errors**: Only when compile-time validation is impossible (e.g., text length checks)

**Rationale**: Widget boundary and content errors enable developers to detect layout bugs early and handle violations gracefully, supporting the composition mental model where parent-child relationships and content sizing have explicit safety contracts. Silent truncation remains for PageBuilder/Region to match hardware reality.

**Scope limitation**: This exception applies ONLY to `Widget` trait implementations, `RenderContext`, and widget constructors (Label, Rect, etc.). Underlying `PageBuilder` and `Region` APIs maintain silent truncation.

**Testing**: Property-based tests MUST verify zero panics with arbitrary out-of-bounds inputs for PageBuilder/Region. Widget tests MUST verify `Result::Err` for boundary and content violations.

---

### IV. Immutability Guarantees (NON-NEGOTIABLE)

**Principle**: Finalized `Document` objects MUST be immutable and thread-safe (`Send + Sync`).

**Requirements**:

- `DocumentBuilder::build()` consumes the builder and returns immutable `Document`
- No public mutable methods on `Document`
- Rendering operates on `&Document` (shared reference only)
- Thread safety verified via compile-time trait bounds
- No interior mutability (`RefCell`, `Mutex`) in `Document`

**Lifecycle**:

```
DocumentBuilder (mutable)
    ↓ .build()
Document (immutable, Send + Sync)
    ↓ .render()
Vec<u8> (ESC/P bytes)
```

**Rationale**: Enables multi-threaded rendering pipelines, output caching, and prevents accidental modifications after finalization. Immutability guarantees are enforced at compile-time via Rust's type system.

**Validation**: CI MUST include compile-time tests verifying `Send + Sync` bounds and rejection of mutable `Document` usage.

---

### V. ESC/P Text-Mode Compliance (NON-NEGOTIABLE)

**Principle**: The library MUST generate valid ESC/P text-mode commands compatible with EPSON LQ-2090II and equivalent printers.

**Supported ESC/P Commands**:

- `ESC @` (0x1B 0x40): Printer reset
- `SI` (0x0F): Condensed mode (12 CPI)
- `ESC E` / `ESC F` (0x1B 0x45 / 0x1B 0x46): Bold on/off
- `ESC - 1` / `ESC - 0` (0x1B 0x2D 0x01 / 0x1B 0x2D 0x00): Underline on/off
- `CR` (0x0D), `LF` (0x0A): Line termination
- `FF` (0x0C): Form feed (page separator)

**Prohibited Commands**:

- Bitmap/raster graphics commands (`ESC *`, `ESC K`)
- Vector graphics commands
- Font download commands
- Any graphics-mode sequences

**Character Handling**:

- Non-ASCII characters (value > 127) MUST be replaced with `'?'` (0x3F)
- ASCII printable range: 32-126

**Rationale**: Text-mode only keeps V1 simple, deterministic, and achievable. Graphics support can be added in V2 without breaking V1 guarantees.

**Validation**: Hardware tests on EPSON LQ-2090II MUST verify correct output. Byte-level ESC/P sequence inspection MUST be automated in CI.

---

### VI. Stable Rust Builder API Design (NON-NEGOTIABLE)

**Principle**: The builder API MUST use Rust lifetimes for compile-time safety and provide ergonomic, type-safe construction patterns.

**Minimum Supported Rust Version (MSRV)**: **1.91.1** (stable channel, 2021 edition)

**Lifetime Hierarchy**:

```
DocumentBuilder (owns data)
    ↓ lifetime 'doc
PageBuilder<'doc> (borrows from DocumentBuilder)
    ↓ lifetime 'page
RegionHandle<'page> (borrows from PageBuilder)
```

**API Contracts**:

- Builder pattern consumes on `build()` to prevent reuse
- Method chaining via `&mut Self` returns for ergonomics
- Lifetimes prevent dangling references at compile-time
- Zero runtime overhead (all checks compile-time)
- No `unsafe` code in public API

**Error Handling**:

- Geometry errors return `Result<T, LayoutError>`
- Content overflow silently truncates for PageBuilder/Region (no errors)
- Widget content validation returns `Result<T, RenderError>` per Principle III Widget Exception
- All public APIs documented with error conditions

**Widget Construction Syntax** (Added 2025-11-19):

The library provides **two equivalent syntaxes** for widget construction, both leveraging const generics for compile-time type safety:

**1. Turbofish Syntax** (Direct const generic instantiation):

```rust
// Widget creation with explicit const generics
let container = Rect::<80, 30>::new();
let label = Label::<20, 1>::new("Hello")?;

// Layout component creation
let column = Column::<80, 30>::new();
let (rect1, pos1) = column.area::<10>()?;
```

**2. Macro Wrapper Syntax** (Ergonomic alternative):

```rust
// Macro wrappers expand to turbofish syntax at compile time
let container = rect_new!(80, 30);
let label = label_new!("Hello", 20, 1)?;

// Layout component macros
let column = column_new!(80, 30);
let (rect1, pos1) = column_area!(column, 10)?;
```

**Macro Design Principles**:

- Macros MUST expand to turbofish syntax (zero abstraction overhead)
- Both syntaxes are officially supported and equivalent
- Macros provide ergonomic benefits without sacrificing type safety
- Developers MAY choose either syntax based on preference
- Documentation examples SHOULD demonstrate both approaches

**Rationale**: Turbofish syntax provides explicit control and clarity, while macro wrappers reduce visual noise and improve readability for common patterns. Supporting both syntaxes accommodates different developer preferences without fragmenting the API.

**Validation Strategy** (Added 2025-11-19):

The library MUST prioritize compile-time validation over runtime validation to minimize overhead and maximize type safety:

**Validation Hierarchy** (prefer in order):

1. **Compile-time validation** (preferred):

   - Const generics for dimensions: `Rect<WIDTH, HEIGHT>`, `Label<WIDTH, 1>`
   - Type system constraints via trait bounds
   - Lifetime constraints preventing invalid usage
   - Zero runtime cost, errors caught at compile-time

2. **Development-time assertions** (debug builds only):

   - `debug_assert!` for contract violations and invariant checks
   - Examples: non-zero dimensions, valid coordinate ranges
   - Zero cost in release builds (stripped by compiler)
   - Catches developer errors during testing

3. **Runtime validation** (only when compile-time impossible):
   - `Result<T, Error>` for user-provided data that cannot be validated at compile-time
   - Examples: text content length, dynamic overlap detection
   - Use sparingly to minimize runtime overhead
   - Document clearly in API why runtime check is necessary

**Application Examples**:

```rust
// ✅ Preferred: Compile-time validation via const generics
impl<const WIDTH: u16, const HEIGHT: u16> Rect<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        // Compile error if WIDTH or HEIGHT are 0
    }
}

// ✅ Good: Debug assertion for development-time checks
pub fn add_child(&mut self, widget: impl Widget, pos: (u16, u16)) -> Result<(), RenderError> {
    debug_assert!(WIDTH > 0 && HEIGHT > 0, "Zero-size parent");
    // Runtime validation for user-provided data
    if child_exceeds_bounds { return Err(...); }
}

// ⚠️ Use sparingly: Runtime validation only when compile-time impossible
pub fn new(text: &str) -> Result<Self, RenderError> {
    if text.len() > WIDTH {
        return Err(RenderError::TextExceedsWidth);
    }
}
```

**Rationale**: Compile-time validation leverages Rust's type system to prevent entire classes of errors without runtime cost. Debug assertions catch developer mistakes during testing with zero production overhead. Runtime validation is reserved for user-provided data that cannot be statically checked. Rust 1.91.1 provides improved const generic features that enable better compile-time validation.

**Validation**: Compile-time tests MUST verify rejection of invalid API usage. Examples MUST demonstrate idiomatic Rust patterns. CI MUST run tests in both debug and release modes to verify debug_assert! behavior. CI MUST test on Rust 1.91.1 stable.

---

### VII. Zero Runtime Dependencies (NON-NEGOTIABLE)

**Principle**: The library MUST have zero required runtime dependencies beyond Rust `std`.

**Allowed Dependencies**:

- **Runtime**: None (only Rust `std`)
- **Optional (feature-gated)**: `serde` for serialization support
- **Dev dependencies**: `criterion`, `proptest`, `libfuzzer-sys`

**Prohibited**:

- Required external crates at runtime
- C/C++ FFI dependencies
- Network-calling dependencies
- Cryptographic dependencies (use stdlib when needed)

**Rationale**: Minimizes supply chain attack surface, reduces binary size, accelerates compilation, enables future `no_std` support, and eliminates dependency breakage risk.

**Validation**: CI MUST run `cargo tree --depth 1` and fail if runtime dependencies detected. Binary size MUST be < 2 MB (release build with LTO).

---

### VIII. Fixed-Layout Constraints

**Principle**: All layout dimensions MUST be specified explicitly by the developer at construction time. No auto-sizing or dynamic layout in V1.

**Requirements**:

- Regions have fixed `(x, y, width, height)` specified by developer
- Widget dimensions fixed at construction time (via const generics or constructor parameters)
- No content-based auto-sizing
- No constraint solvers or layout algorithms
- No relative sizing (percent-based widths/heights)
- Padding reduces inner usable area (predictable calculation)

**Widget Dimension Specification** (Clarified 2025-11-19, Updated 2025-11-19):

Widget dimensions MUST be specified at construction time using **const generics** with either turbofish syntax or macro wrappers:

**Turbofish Syntax**:

```rust
// Widget construction with explicit dimensions
let container = Rect::<80, 30>::new();
let label = Label::<20, 1>::new("text")?;

// Layout component usage
let column = Column::<80, 30>::new();
let (nested_rect, pos) = column.area::<10>()?;  // Returns Rect<80, 10>
```

**Macro Wrapper Syntax**:

```rust
// Equivalent macro-based construction
let container = rect_new!(80, 30);
let label = label_new!("text", 20, 1)?;

// Layout component macros
let column = column_new!(80, 30);
let (nested_rect, pos) = column_area!(column, 10)?;  // Returns Rect<80, 10>
```

**Dimension Requirements**:

- All dimensions known at compile-time before widget tree traversal
- Both turbofish and macro syntaxes are officially supported and equivalent
- Developers MAY choose either syntax based on preference

**Rationale**: Predictable, testable, and matches regulatory form requirements where exact positioning is mandatory. Static layout eliminates non-deterministic rendering behavior. Fixed widget dimensions at construction enable early validation and error detection via compile-time checks (const generics) per Principle VI Validation Strategy. Dual syntax support provides flexibility without compromising type safety.

**Migration Path**: V2 MAY add `Region::auto_height()` or constraint-based layout without breaking V1 API.

---

## Code Quality Standards

### IX. Zero-Panic Guarantee

**Principle**: The library MUST NOT panic under any documented usage pattern in release builds.

**Requirements**:

- Invalid inputs return `Result<T, LayoutError>` or `Result<T, RenderError>`
- Out-of-bounds writes are silently clipped for PageBuilder/Region (no panics)
- Widget violations return errors (no panics) per Principle III Widget Exception
- Only `debug_assert!` may trigger panics (debug builds only)
- Fuzzing MUST run 1M+ iterations without crashes
- Property-based tests verify no panics with arbitrary inputs

**Allowed Panics**:

- **Debug builds only**: `debug_assert!` for documented API constraint violations (e.g., zero-size widgets, Label HEIGHT ≠ 1)
- **Debug builds only**: Developer misuse of private APIs (if any)
- **Never in release builds**: All panics stripped by compiler; undefined behavior acceptable for documented constraint violations

**Documented Constraint Violations** (APIs that compile but violate documented invariants):

- `Rect::<0, H>::new()` or `Rect::<W, 0>::new()` - zero-size widgets (panics in debug via `debug_assert!`, undefined in release)
- `Label::<W, H>::new()` where H ≠ 1 - multi-line labels (panics in debug via `debug_assert!`, undefined in release; developers MUST use H=1)
- `Label::<W, 1>::new().add_text(text)` where text contains newlines (`\n`, `\r\n`) - multi-line text content (panics in debug via `debug_assert!`, undefined in release)
- Future constraint violations added via `debug_assert!` must be documented in API rustdoc `# Panics` section

**Rationale**: Production stability for 24/7 industrial environments requires zero panics in release builds. Debug assertions provide development-time safety with zero production cost (per Principle VI Validation Strategy). Developers violating documented constraints in release builds accept undefined behavior risk.

**Validation**: Fuzzing with `cargo-fuzz` MUST be part of CI (release build). Every PR MUST pass property-based panic tests (release build). CI MUST test both debug and release builds on Rust 1.91.1+ to verify debug_assert! behavior differs correctly.

---

### X. Memory Efficiency and Predictability

**Principle**: Memory usage MUST be predictable, bounded, and minimal.

**Targets**:

- Page model: ≤ 16 KB per page (160 × 51 × 2 bytes)
- Document overhead: < 1 MB regardless of page count (for reasonable documents)
- Rendering MUST NOT allocate during cell-writing loops
- Zero-copy rendering where possible

**Memory Layout**:

- Row-major cell storage: `cells[y][x]` for cache-friendly iteration
- Bit-packed style storage (1 byte per cell)
- Rect allocation for page data (avoid stack overflow)

**Profiling Requirements**:

- Memory profiling with `heaptrack` or `valgrind --tool=massif`
- Benchmarks MUST track memory usage over time
- PRs MUST NOT regress memory usage by > 10%

**Rationale**: Embedded systems and resource-constrained environments require predictable memory usage. Cache-friendly layout improves performance.

---

### XI. Performance Targets (NON-NEGOTIABLE)

**Principle**: The library MUST meet performance targets for typical and stress-test workloads.

**Benchmark Targets (p99)**:
| Operation | Target | Measurement |
|-----------|--------|-------------|
| Single page render | < 100 μs | `criterion` |
| 100-page document | < 10 ms | `criterion` |
| Region lookup | < 1 μs | `criterion` |
| Page allocation | < 10 μs | `criterion` |

**Optimization Strategies**:

- Inline hints for hot path (`#[inline]`)
- Pre-allocation with capacity estimates
- Avoid allocations in tight loops
- Row-major memory layout for cache efficiency
- Compile-time validation eliminates runtime checks (per Principle VI Validation Strategy)

**Regression Prevention**:

- CI MUST run benchmarks on every PR
- Performance regressions > 10% MUST be investigated
- Flamegraphs MUST be generated for profiling

**Rationale**: Ensures the library is suitable for high-throughput industrial applications (1000+ forms/day). Compile-time validation contributes to performance by eliminating runtime overhead.

---

## Testing Standards

### XII. Comprehensive Testing Requirements

**Principle**: All code MUST be tested with unit, integration, property-based, and hardware validation tests.

**Coverage Targets**:

- Unit test coverage: ≥ 90% line coverage
- Public API coverage: 100%
- Integration tests: Minimum 50 scenarios
- Property-based tests: All core operations
- Fuzzing: 1M+ iterations per target

**Test Categories**:

**Unit Tests**:

- All public API functions
- Edge cases (empty regions, zero-width/height, boundary writes)
- Style state machine transitions
- Region splitting calculations
- Widget content validation (text exceeds dimensions)
- Debug assertion behavior (verify panics in debug builds)

**Integration Tests**:

- End-to-end document rendering
- Multi-page pagination
- Nested region hierarchies (5+ levels)
- All widgets in combination
- Widget composition with error handling
- Overlapping children detection

**Property-Based Tests** (`proptest`):

- Truncation correctness: Arbitrary strings → no overflow (PageBuilder/Region)
- Widget validation: Arbitrary content → proper errors or success
- Determinism: Same input → same output (1000 runs)
- No panics: Arbitrary valid geometries → no crash (release builds)

**Golden Master Tests**:

- 20+ golden master files
- SHA-256 hash verification
- Byte-level ESC/P output comparison

**Hardware Validation** (EPSON LQ-2090II):

- 10 test forms printed and visually inspected
- Alignment verification on multi-part forms
- All widgets validated on paper

**Rationale**: Comprehensive testing prevents regressions, validates correctness, and ensures production readiness. Both debug and release builds must be tested to verify debug_assert! behavior.

---

## Architectural Governance

### XIII. API Stability and Versioning

**Principle**: The library MUST follow Semantic Versioning 2.0.0 strictly.

**Version Format**: `MAJOR.MINOR.PATCH`

**Versioning Rules**:

- **MAJOR**: Breaking API changes, removed public APIs
- **MINOR**: Backward-compatible new features, new public APIs
- **PATCH**: Backward-compatible bug fixes

**Deprecation Policy**:

- Deprecated APIs MUST emit warnings for ≥ 1 minor version
- Migration path MUST be documented in deprecation message
- Removal requires MAJOR version bump

**API Stability Guarantees** (V1.x.x):

- Public API frozen except for additions
- ESC/P output format stable (byte-level compatibility)
- No breaking changes within 1.x.x series

**Example**:

```rust
#[deprecated(since = "1.2.0", note = "Use `split_horizontal_equal()` instead")]
pub fn split_equal(&mut self, count: u16) -> Result<...> { ... }
```

**Rationale**: Users rely on API stability for production systems. SemVer provides clear expectations for upgrades.

---

### XIV. Documentation Requirements

**Principle**: All public APIs MUST have comprehensive rustdoc documentation with examples.

**Documentation Standards**:

- 100% public API rustdoc coverage
- Every public function has `///` comments
- Code examples in docs (compilable and tested via `cargo test --doc`)
- Usage guidelines and edge cases documented
- Error conditions explicitly listed
- Validation strategy documented (compile-time vs runtime)
- Both turbofish and macro wrapper syntaxes demonstrated in examples

**Required Documentation Artifacts**:

- **README.md**: Quickstart guide (< 5 minutes to first working example)
- **API Specification**: Complete public API reference
- **User Guides**: Tutorials for beginners, intermediate, and advanced users
- **ADRs**: Architecture Decision Records for major design choices
- **Troubleshooting Guide**: Common issues and solutions

**Validation**:

- CI MUST run `cargo doc --no-deps && cargo deadlinks`
- Examples MUST compile and run
- `cargo doc --open` MUST produce readable documentation

**Rationale**: Well-documented APIs reduce support burden, accelerate onboarding, and prevent misuse. Documentation must explain validation strategy to help developers understand when to expect compile-time errors vs runtime errors. Demonstrating both widget construction syntaxes helps developers choose the approach that fits their style.

---

### XV. Security and Safety

**Principle**: The library MUST minimize security risks and unsafe code.

**Security Requirements**:

- Zero `unsafe` code blocks in V1 (unless absolutely necessary with justification)
- All user inputs validated (geometry bounds checks, widget content validation)
- No buffer overflows (prevented by Rust's bounds checking)
- Fuzzing integrated into CI (detect crashes and memory issues)
- Supply chain audits: `cargo audit` on every PR

**Input Validation**:

- All dimensions validated at creation time (prefer compile-time via const generics)
- Widget content validated at construction (per Principle III Widget Exception)
- Prevent integer overflow (width/height > u16::MAX)
- Prevent denial-of-service via excessive allocations

**Supply Chain Security**:

- Zero runtime dependencies (minimize attack surface)
- Dev dependencies vetted and pinned
- `cargo-deny` configured to reject untrusted crates

**Rationale**: Security is critical for industrial and government applications. Zero dependencies and minimal `unsafe` code reduce attack surface. Compile-time validation eliminates entire classes of runtime vulnerabilities.

---

## Change Management

### XVI. Specification Validation Process

**Principle**: All changes to specifications MUST be validated against the frozen V1 spec before implementation.

**Validation Process**:

1. **Proposal**: Changes proposed via GitHub issue or RFC
2. **Specification Review**: Check against frozen V1 constraints
3. **Impact Analysis**: Document affected components, APIs, tests
4. **Approval**: Requires sign-off from project maintainer
5. **Implementation**: PRs MUST reference approved specification change
6. **Validation**: All tests (unit, integration, hardware) MUST pass

**Prohibited Changes Without Major Version Bump**:

- Breaking public API changes
- Changes to ESC/P output format
- Changes to determinism guarantees
- Changes to fixed layout constraints

**Allowed Changes (Minor/Patch)**:

- Bug fixes that don't change output
- New optional features (feature-gated)
- Documentation improvements
- Performance optimizations (without behavior changes)
- MSRV updates (MINOR version bump for constitution)

**Rationale**: Prevents scope creep and ensures V1 remains stable and predictable.

---

### XVII. Code Review Standards

**Principle**: All code changes MUST pass rigorous code review before merging.

**Review Checklist**:

- [ ] Follows Rust API guidelines
- [ ] No `unsafe` code (or justified with safety invariants)
- [ ] All public APIs documented with rustdoc
- [ ] Tests added for new functionality (coverage ≥ 90%)
- [ ] Benchmarks added for performance-critical code
- [ ] No breaking changes (or approved for MAJOR version bump)
- [ ] Determinism verified (if touching rendering code)
- [ ] Hardware validated (if changing ESC/P output)
- [ ] Memory usage profiled (if adding allocations)
- [ ] Widget content validation implemented (if adding widget types)
- [ ] Validation strategy appropriate (compile-time preferred, debug_assert! for contracts, runtime only when necessary)
- [ ] Widget construction uses const generics (turbofish or macro syntax)
- [ ] Compiles and tests pass on Rust 1.91.1+

**Automated Checks (CI)**:

```yaml
- cargo +1.91.1 fmt --check
- cargo +1.91.1 clippy -- -D warnings
- cargo +1.91.1 test --all-features
- cargo +1.91.1 test --all-features --release # Verify release builds
- cargo +1.91.1 bench --no-run
- cargo audit
- cargo tree --depth 1 (verify zero runtime deps)
- cargo +1.91.1 doc --no-deps
```

**Rationale**: Code review catches bugs, enforces standards, and maintains code quality. Validation strategy review ensures optimal performance and type safety. Rust 1.91.1 CI checks ensure MSRV compliance.

---

## Version Evolution

### XVIII. V2+ Feature Planning

**Principle**: V1 architecture MUST NOT prevent future V2+ features, but V2 planning MUST NOT compromise V1 delivery.

**V2 Candidate Features** (NOT in V1):

- Automatic page breaks and pagination
- Dynamic-height regions (content-based sizing)
- Constraint-based layout system
- Unicode text shaping (complex scripts)
- Bitmap/graphics mode support
- Custom font support
- ESC/POS compatibility
- `no_std` support for embedded systems

**V1 → V2 Migration Path**:

- V1 API remains available (backward compatibility)
- V2 features introduced via new APIs or feature flags
- Clear migration guide provided
- Automated migration tools where possible

**Design Constraints for V1** (to enable V2):

- Avoid patterns that prevent dynamic layout extension
- Keep rendering logic pure and composable
- Modular architecture allows adding features without rewrites

**Rationale**: V1 focus ensures timely delivery while maintaining architectural flexibility for future enhancements.

---

## Governance

### Authority and Amendment Process

**Constitution Authority**:

- This constitution supersedes all other development practices and guidelines
- All PRs, code reviews, and architectural decisions MUST comply with this constitution
- Violations MUST be flagged and resolved before merge

**Amendment Procedure**:

1. **Proposal**: Amendments proposed via GitHub issue with `constitution-amendment` label
2. **Discussion**: Open discussion period (minimum 7 days)
3. **Approval**: Requires approval from project maintainer
4. **Documentation**: Amendment rationale documented in this file
5. **Migration Plan**: If breaking changes, migration guide MUST be provided

**Compliance Review**:

- All PRs MUST verify compliance with this constitution
- Quarterly constitution audits to verify adherence
- Complexity or deviations MUST be explicitly justified in code comments and PR descriptions

**Version History**:

- Version: 1.3.0 (Current)
- Ratified: 2025-01-18
- Last Amended: 2025-11-19
- Amendment Log:
  - 1.1.0 (2025-11-19): Added Widget Composability Exception to Principle III, allowing boundary errors for widget system while preserving silent truncation for PageBuilder/Region
  - 1.2.0 (2025-11-19): Extended Widget Composability Exception to include text content validation; clarified widget dimensions fixed at construction time in Principle VIII
  - 1.2.1 (2025-11-19): Added Validation Strategy subsection to Principle VI emphasizing compile-time validation preference hierarchy (compile-time → debug_assert! → runtime Result); updated Principles III, IX, XI, XII, XV, XVII to reference validation strategy
  - 1.2.2 (2025-11-19): Added Widget Construction Syntax subsection to Principle VI documenting dual API (turbofish vs macro wrappers); updated Principle VIII with syntax examples; updated Principle XIV to require both syntaxes in documentation; updated Principle XVII code review checklist
  - 1.3.0 (2025-11-19): Updated MSRV from Rust 1.75.0 to Rust 1.91.1; updated Principle I (determinism across compiler versions), Principle VI (MSRV specification, validation rationale, CI requirements), Principle IX (CI testing requirements), Principle XVI (MSRV updates policy), Principle XVII (CI checks with version); added propagation requirements for Cargo.toml, specs, and documentation

---

## Enforcement

**Responsibility**: All contributors, reviewers, and maintainers are responsible for upholding this constitution.

**Violation Response**:

1. **Detection**: Violations flagged during code review or CI
2. **Resolution**: PR author MUST address violations before merge
3. **Escalation**: Repeated or severe violations escalated to project maintainer

**Final Authority**: Project maintainer (Mohammad AlMechkor) has final authority on constitutional interpretation and amendments.

---

**Document Status**: ✅ RATIFIED

**For questions or amendments, contact:**
**Project Maintainer**: Mohammad AlMechkor
**Document Location**: `.specify/memory/constitution.md`

---

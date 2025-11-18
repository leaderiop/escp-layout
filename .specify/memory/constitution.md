<!--
Version Change: 1.0.0 (Initial Constitution)
Modified Principles: None (initial creation)
Added Sections: All sections (initial creation)
Removed Sections: None
Templates Requiring Updates:
  ✅ plan-template.md (will be validated in sync check)
  ✅ spec-template.md (will be validated in sync check)
  ✅ tasks-template.md (will be validated in sync check)
Follow-up TODOs: None
-->

# EPSON LQ-2090II Rust Layout Engine — Project Constitution

## Core Principles

### I. Deterministic Behavior (NON-NEGOTIABLE)

**Principle**: The library MUST produce byte-for-byte identical ESC/P output for identical inputs across all platforms, compiler versions, and execution environments.

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
- Auto-layout algorithms (flexbox-like systems)
- Unicode text shaping or bidirectional text
- Graphics modes (bitmap, raster, vector)
- Runtime configuration of page dimensions

**Rationale**: Simplicity, predictability, and achievable timeline. V1 establishes a solid foundation for future versions without introducing complexity that compromises correctness.

**Migration Path**: V2 MAY introduce dynamic features, but MUST maintain V1 compatibility through versioned APIs or feature flags.

---

### III. Strict Truncation and Clipping (NON-NEGOTIABLE)

**Principle**: All content overflow MUST be silently truncated. Truncation is NOT an error condition and MUST NOT produce warnings or panics.

**Truncation Rules**:
- **Horizontal truncation**: Content beyond region width is dropped character-by-character
- **Vertical truncation**: Lines beyond region height are discarded line-by-line
- **Page boundary truncation**: Writes outside (160×51) bounds are silently ignored
- **No errors**: Content overflow does NOT return `Result::Err`
- **No panics**: Overflow MUST NOT trigger panics in release builds

**Implementation**:
```rust
// ✅ Correct: Silent truncation
if x >= width || y >= height {
    return; // Silent drop
}

// ❌ Wrong: Error on overflow
if x >= width {
    return Err(LayoutError::Overflow); // NOT ALLOWED
}
```

**Rationale**: Graceful degradation for regulatory forms where partial content is better than no content. Printers clip content at physical margins; our behavior matches hardware reality.

**Testing**: Property-based tests MUST verify zero panics with arbitrary out-of-bounds inputs. Fuzzing MUST run 1M+ iterations without crashes.

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
- Content overflow silently truncates (no errors)
- All public APIs documented with error conditions

**Rationale**: Rust's type system enforces correctness at compile-time, preventing entire classes of runtime errors. Lifetime safety prevents use-after-free and dangling references without runtime overhead.

**Validation**: Compile-time tests MUST verify rejection of invalid API usage. Examples MUST demonstrate idiomatic Rust patterns.

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

**Principle**: All layout dimensions MUST be specified explicitly by the developer at build time. No auto-sizing or dynamic layout in V1.

**Requirements**:
- Regions have fixed `(x, y, width, height)` specified by developer
- No content-based auto-sizing
- No constraint solvers or layout algorithms
- No relative sizing (percent-based widths/heights)
- Padding reduces inner usable area (predictable calculation)

**Rationale**: Predictable, testable, and matches regulatory form requirements where exact positioning is mandatory. Static layout eliminates non-deterministic rendering behavior.

**Migration Path**: V2 MAY add `Region::auto_height()` or constraint-based layout without breaking V1 API.

---

## Code Quality Standards

### IX. Zero-Panic Guarantee

**Principle**: The library MUST NOT panic under any documented usage pattern in release builds.

**Requirements**:
- Invalid inputs return `Result<T, LayoutError>`
- Out-of-bounds writes are silently clipped (no panics)
- Only `debug_assert!` may trigger panics (debug builds only)
- Fuzzing MUST run 1M+ iterations without crashes
- Property-based tests verify no panics with arbitrary inputs

**Allowed Panics** (debug builds only):
- Contract violations caught by `debug_assert!`
- Developer misuse of private APIs (if any)

**Rationale**: Production stability for 24/7 industrial environments. Panics in embedded or critical systems are unacceptable.

**Validation**: Fuzzing with `cargo-fuzz` MUST be part of CI. Every PR MUST pass property-based panic tests.

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
- Box allocation for page data (avoid stack overflow)

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

**Regression Prevention**:
- CI MUST run benchmarks on every PR
- Performance regressions > 10% MUST be investigated
- Flamegraphs MUST be generated for profiling

**Rationale**: Ensures the library is suitable for high-throughput industrial applications (1000+ forms/day).

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

**Integration Tests**:
- End-to-end document rendering
- Multi-page pagination
- Nested region hierarchies (5+ levels)
- All widgets in combination

**Property-Based Tests** (`proptest`):
- Truncation correctness: Arbitrary strings → no overflow
- Determinism: Same input → same output (1000 runs)
- No panics: Arbitrary valid geometries → no crash

**Golden Master Tests**:
- 20+ golden master files
- SHA-256 hash verification
- Byte-level ESC/P output comparison

**Hardware Validation** (EPSON LQ-2090II):
- 10 test forms printed and visually inspected
- Alignment verification on multi-part forms
- All widgets validated on paper

**Rationale**: Comprehensive testing prevents regressions, validates correctness, and ensures production readiness.

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

**Rationale**: Well-documented APIs reduce support burden, accelerate onboarding, and prevent misuse.

---

### XV. Security and Safety

**Principle**: The library MUST minimize security risks and unsafe code.

**Security Requirements**:
- Zero `unsafe` code blocks in V1 (unless absolutely necessary with justification)
- All user inputs validated (geometry bounds checks)
- No buffer overflows (prevented by Rust's bounds checking)
- Fuzzing integrated into CI (detect crashes and memory issues)
- Supply chain audits: `cargo audit` on every PR

**Input Validation**:
- All dimensions validated at creation time
- Prevent integer overflow (width/height > u16::MAX)
- Prevent denial-of-service via excessive allocations

**Supply Chain Security**:
- Zero runtime dependencies (minimize attack surface)
- Dev dependencies vetted and pinned
- `cargo-deny` configured to reject untrusted crates

**Rationale**: Security is critical for industrial and government applications. Zero dependencies and minimal `unsafe` code reduce attack surface.

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

**Automated Checks (CI)**:
```yaml
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test --all-features
- cargo bench --no-run
- cargo audit
- cargo tree --depth 1 (verify zero runtime deps)
- cargo doc --no-deps
```

**Rationale**: Code review catches bugs, enforces standards, and maintains code quality.

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
- Version: 1.0.0
- Ratified: 2025-01-18
- Last Amended: 2025-01-18

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

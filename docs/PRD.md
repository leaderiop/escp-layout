📘 PRODUCT REQUIREMENTS DOCUMENT (PRD)

# EPSON LQ-2090II Rust Layout Engine — V1

**Deterministic Static Layout Engine for Dot-Matrix Text Printing**

---

## Document Control

| Field | Value |
|-------|-------|
| **Document Version** | 1.1 (Enhanced) |
| **Product Version** | V1.0 |
| **Owner** | Mohammad AlMechkor |
| **Status** | Approved for Implementation |
| **Classification** | Internal - Engineering |
| **Date Created** | 2025-01-18 |
| **Last Updated** | 2025-01-18 |

### Approval Chain

| Role | Name | Signature | Date |
|------|------|-----------|------|
| **Product Owner** | Mohammad AlMechkor | ✓ | 2025-01-18 |
| **Engineering Lead** | [Pending] | [ ] | — |
| **QA Lead** | [Pending] | [ ] | — |
| **Technical Architect** | [Pending] | [ ] | — |

### Reviewers & Contributors

- **Technical Reviewers**: [To be assigned]
- **Documentation Lead**: [To be assigned]
- **QA Reviewer**: [To be assigned]

### Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-01-XX | Mohammad AlMechkor | Initial PRD |
| 1.1 | 2025-01-18 | Mohammad AlMechkor | Enhanced with enterprise requirements |

### Related Documents

- **Technical Design Document (TDD)**: [Link pending]
- **API Specification**: [Link pending]
- **Test Plan**: [Link pending]
- **Architecture Decision Records (ADRs)**: [Link pending]
- **Security Review**: [Link pending]

### Next Review Date

**Quarterly Review**: 2025-04-18

---

## Table of Contents

1. [Overview](#1-overview)
2. [Background & Problem Statement](#2-background--problem-statement)
3. [Objectives & Success Criteria](#3-objectives--success-criteria)
4. [Stakeholders & Personas](#4-stakeholders--personas)
5. [Scope](#5-scope)
6. [Functional Requirements](#6-functional-requirements)
7. [API Requirements](#7-api-requirements)
8. [Non-Functional Requirements](#8-non-functional-requirements)
9. [Technical Constraints](#9-technical-constraints)
10. [Risks & Mitigations](#10-risks--mitigations)
11. [Testing Strategy](#11-testing-strategy)
12. [Security & Safety](#12-security--safety)
13. [Versioning & Compatibility](#13-versioning--compatibility)
14. [Distribution & Packaging](#14-distribution--packaging)
15. [Documentation Requirements](#15-documentation-requirements)
16. [Acceptance Criteria](#16-acceptance-criteria)
17. [Architecture & Design](#17-architecture--design)
18. [Traceability Matrix](#18-traceability-matrix)
19. [Appendices](#19-appendices)

---

## 1. Overview

### 1.1 Executive Summary

This document defines the comprehensive product requirements for **V1 of the EPSON LQ-2090II Rust Layout Engine**, a deterministic text-layout system targeting EPSON dot-matrix printers operating in ESC/P condensed text mode.

The library provides embedded and backend developers with a type-safe, deterministic, and ergonomic API for generating complex multi-page forms without manual ESC/P programming.

### 1.2 System Capabilities

The system provides:

- **Fixed-size virtual page**: 160 columns × 51 printable lines
- **Hierarchical layout mechanism**: Composable Regions with nesting support
- **Deterministic static widgets**: Label, TextBlock, Paragraph, Box, KeyValue, Table
- **Strict truncation-based rendering**: Predictable overflow handling
- **Manual multi-page document composition**: Explicit pagination control
- **ESC/P-compliant byte-level rendering**: Hardware-validated output
- **Stable and ergonomic Rust builder API**: Type-safe, lifetime-checked

### 1.3 Design Philosophy

V1 intentionally **excludes** dynamic height, automatic pagination, and auto-layout algorithms to guarantee:

- **Predictability**: Same input always produces identical output
- **Correctness**: No layout surprises or undefined behavior
- **Simplicity**: Minimal API surface with clear contracts
- **Foundation**: Clean architecture for V2+ advanced features

This PRD serves as the **authoritative specification** for engineering, QA, documentation, DevOps, and release planning.

---

## 2. Background & Problem Statement

### 2.1 Industry Context

Industrial printing for logistics, warehousing, manufacturing, and government sectors relies heavily on **dot-matrix impact printers** for multi-part carbonless forms. The EPSON LQ-2090II remains a workhorse in these environments due to:

- **Multi-part printing capability** (up to 7-part forms)
- **Reliability** and low maintenance
- **Wide carriage support** (up to 16.5" paper)
- **Cost-effectiveness** compared to laser alternatives

Despite this, software development for these printers remains **fragile and error-prone**.

### 2.2 Current State & Pain Points

#### Existing Approaches

1. **Manual ESC/P Programming**
   - Developers manually concatenate escape sequences
   - Byte-level calculations for positioning
   - No type safety or compile-time validation
   - High defect rate in production

2. **Legacy Libraries (C/Python)**
   - Memory-unsafe (buffer overflows)
   - Platform-specific quirks
   - No async/modern Rust ecosystem integration
   - Poor documentation

3. **Ad-hoc Text Formatting**
   - String padding and manual spacing
   - No guarantee of alignment
   - Breaks with font changes or printer variations

#### Critical Problems

| Problem | Impact | Severity |
|---------|--------|----------|
| **Misalignment bugs in production** | Forms rejected by regulatory authorities | HIGH |
| **Non-deterministic output** | Different output across environments/machines | HIGH |
| **High maintenance burden** | Changes require ESC/P expertise | MEDIUM |
| **No reusability** | Copy-paste code across projects | MEDIUM |
| **Testing difficulty** | No mocking or golden master tests | MEDIUM |

### 2.3 Problem Statement

**There is no modern, type-safe, deterministic layout abstraction for generating ESC/P dot-matrix forms in Rust.**

As a result, developers face:

- **Reliability issues**: Production bugs in critical business documents
- **Productivity loss**: Days spent debugging alignment issues
- **Knowledge gaps**: Scarce ESC/P expertise in modern development teams
- **Technical debt**: Unmaintainable escape sequence spaghetti code

### 2.4 Solution Approach

Build a **Rust-native layout engine** that:

1. **Abstracts ESC/P complexity** behind ergonomic builders
2. **Guarantees determinism** via pure functional rendering
3. **Enforces correctness** through the type system
4. **Provides testability** via in-memory page models
5. **Enables reusability** through composable widgets and regions

### 2.5 Success Vision

Developers should be able to generate a production-ready invoice in **< 50 lines of Rust code** without ever seeing an ESC/P sequence.

---

## 3. Objectives & Success Criteria

### 3.1 Primary Objectives

The library SHALL:

1. Provide a **deterministic text-layout system** for EPSON LQ-2090II condensed mode
2. Offer a **robust fixed-region layout model** with hierarchical composability
3. Guarantee **strict clipping** (horizontal + vertical truncation)
4. Allow developers to **manually assemble multi-page documents**
5. Produce **valid ESC/P sequences** for printing
6. Provide a **stable, predictable, ergonomic Rust API**
7. Serve as a **foundational library** for advanced features in V2+

### 3.2 Secondary Objectives

The library SHOULD:

- Minimize memory allocations during rendering
- Be easily testable and mockable (in-memory page model)
- Deliver reproducible byte-level output (fully deterministic)
- Integrate seamlessly with async Rust ecosystems
- Support zero-copy rendering where possible

### 3.3 Success Metrics

#### Functional Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| **ESC/P Compliance** | 100% | Hardware validation on LQ-2090II |
| **Crash Rate** | Zero panics | Fuzzing + stress tests |
| **Alignment Accuracy** | ±0 characters | Visual inspection on printer |
| **Determinism** | 100% | SHA-256 hash comparison |

#### Developer Experience Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| **API Complexity** | < 20 calls for standard invoice | Code review |
| **Learning Curve** | < 30 minutes to first working example | User testing |
| **ESC/P Knowledge Required** | None | Survey developers |
| **Documentation Coverage** | 100% public API | `cargo doc` analysis |

#### Performance Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| **Single Page Render** | < 100μs (p99) | `criterion` benchmarks |
| **100-Page Document** | < 10ms (p99) | Integration benchmarks |
| **Memory Footprint** | < 1MB per document | `heaptrack` profiling |

---

## 4. Stakeholders & Personas

### 4.1 Stakeholders

#### Primary Stakeholders

| Stakeholder | Role | Interest | Influence |
|-------------|------|----------|-----------|
| **Backend Engineers** | Library consumers | Integrate into production systems | HIGH |
| **Embedded Developers** | Library consumers | Deploy on resource-constrained devices | MEDIUM |
| **QA Engineers** | Testing & validation | Ensure correctness and reliability | HIGH |

#### Secondary Stakeholders

| Stakeholder | Role | Interest | Influence |
|-------------|------|----------|-----------|
| **DevOps/SRE** | Packaging & deployment | Reliable builds and distribution | MEDIUM |
| **Technical Writers** | Documentation | API docs and tutorials | MEDIUM |
| **Support Teams** | Troubleshooting | Debug printer integration issues | LOW |
| **Open Source Community** | Contributors | Code contributions and feedback | MEDIUM |

### 4.2 User Personas

#### Persona 1: Backend ERP Developer

**Name**: Alex Chen
**Experience**: 5+ years backend development (Python, Java, transitioning to Rust)
**Context**: Works on warehouse management system (WMS)

**Goals**:
- Generate picking/packing slips automatically
- Replace legacy Python ESC/P code
- Reduce production bugs in printed forms

**Pain Points**:
- Current Python library has memory leaks
- Alignment issues with certain form types
- No type safety leads to runtime errors

**Usage Pattern**:
- Generates 1000+ forms per day
- Requires multi-page invoices with tables
- Needs predictable output for auditing

**Success Criteria**: Can generate a 3-page invoice in < 50 lines of Rust without debugging alignment issues.

---

#### Persona 2: Embedded Systems Engineer

**Name**: Sarah Rodriguez
**Experience**: 10+ years embedded Rust
**Context**: Building kiosk system for government forms

**Goals**:
- Minimize binary size and memory usage
- Guarantee deterministic behavior
- Potential future `no_std` support

**Pain Points**:
- Existing libraries have too many dependencies
- Non-deterministic output unacceptable for compliance
- Needs offline operation (no network calls)

**Usage Pattern**:
- Runs on ARM Cortex-M4 microcontroller
- Memory budget: < 512KB
- Prints government tax forms

**Success Criteria**: Library compiles with minimal dependencies, uses < 100KB RAM, produces identical output across reboots.

---

#### Persona 3: Industrial Automation Developer

**Name**: Marcus Weber
**Experience**: 15+ years industrial control systems
**Context**: Manufacturing execution system (MES)

**Goals**:
- Print work orders and compliance labels
- Integrate with existing Rust-based MES
- Long-term maintainability

**Pain Points**:
- Previous C library caused buffer overflows
- Vendor lock-in with proprietary solutions
- Difficult to test without physical printer

**Usage Pattern**:
- Prints batch labels (50-200 per shift)
- Requires strict regulatory compliance
- 24/7 operation, zero downtime tolerance

**Success Criteria**: Zero runtime errors, testable without hardware, clear migration path for future printers.

---

### 4.3 Supported Use Cases

| Use Case ID | Description | Complexity | Priority |
|-------------|-------------|------------|----------|
| **UC-001** | Single-page invoice with header/footer | Low | P0 |
| **UC-002** | Multi-page invoice with tables | Medium | P0 |
| **UC-003** | Warehouse picking slip with barcode placeholder | Medium | P1 |
| **UC-004** | Government form with strict field alignment | High | P0 |
| **UC-005** | Packing slip with nested regions | Medium | P1 |
| **UC-006** | Multi-part carbonless forms (alignment-critical) | High | P0 |

---

## 5. Scope

### 5.1 In Scope (V1)

The following capabilities **SHALL be implemented** in V1:

#### Core Layout Engine
- Fixed-size page model (160×51 cells)
- Hierarchical region system with nesting
- Static rectangular geometry
- Padding and border support
- Horizontal and vertical region splitting

#### Widgets
- **Label**: Single-line text with alignment (left/center/right)
- **TextBlock**: Multi-line preformatted text
- **Paragraph**: Word-wrapping text block
- **Box**: ASCII borders with optional title
- **KeyValue**: Key-value pair list
- **Table**: Fixed-width columns with headers

#### Rendering
- ESC/P command generation
- Bold and underline styles
- Condensed mode (12 CPI → 160 columns)
- Deterministic byte-level output
- Multi-page document support with Form Feed separation

#### Developer Experience
- Builder pattern API
- Type-safe region handles
- Lifetime-checked references
- Comprehensive error types
- Zero-panic guarantee (except contract violations)

---

### 5.2 Out of Scope (V1)

The following capabilities **SHALL NOT be implemented** in V1:

#### Layout Features (V2+ Candidates)
- Automatic page breaks
- Dynamic-height regions
- Flow layout (text flowing across regions/pages)
- Relative sizing (percent-based widths/heights)
- Auto-layout algorithms (flexbox-like systems)

#### Typography Features (V2+ Candidates)
- Unicode text shaping (complex scripts)
- Bidirectional text (RTL support)
- Double-width/double-height fonts
- Custom fonts beyond condensed mode
- Kerning and advanced typography

#### Graphics Features (V2+ Candidates)
- Bitmap/raster graphics mode
- Vector graphics
- Barcodes (Code 128, QR codes, etc.)
- Logos and images

#### Protocol Features (Future)
- ESC/POS compatibility
- Non-EPSON printer support
- USB/network printer discovery

These items **MAY be introduced in V2+** based on user feedback and demand.

---

### 5.3 Assumptions & Dependencies

#### Assumptions
- Target printer is EPSON LQ-2090II or compatible model
- Condensed mode (12 CPI) is sufficient for all use cases
- ASCII character set covers required languages
- Developers can handle pagination logic manually in V1

#### Dependencies
- **Rust Standard Library**: `std` (future `no_std` support excluded from V1)
- **Optional Dependencies**: `serde` (feature-gated for serialization)
- **Development Dependencies**: `criterion` (benchmarking), `proptest` (property testing)

---

## 6. Functional Requirements

**Notation**: All requirements use the keywords **MUST**, **SHALL**, **SHOULD**, and **MAY** per [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

---

### 6.1 Page Model Requirements

#### FR-P1 — Page Dimensions

**Requirement**: The `Page` SHALL be a fixed matrix of:
- **160 columns** (condensed mode, 12 CPI)
- **51 printable lines**

**Rationale**: Matches physical EPSON LQ-2090II output in condensed mode.

**Verification**: Unit tests verify page initialization with exact dimensions.

**Priority**: P0 (Critical)

---

#### FR-P2 — Cell Model

**Requirement**: Each `Cell` SHALL store:
- One ASCII character (`u8` in range 32-126, or space)
- A `Style` object: `{ bold: bool, underline: bool }`

**Rationale**: Minimal memory footprint while supporting required text attributes.

**Verification**: Unit tests verify cell state transitions and style application.

**Priority**: P0 (Critical)

---

#### FR-P3 — Boundary Clipping

**Requirement**: Writes outside page bounds (x ≥ 160 or y ≥ 51) SHALL be silently ignored.

**Rationale**: Predictable overflow behavior without panics.

**Verification**: Property-based tests with out-of-bounds coordinates.

**Priority**: P0 (Critical)

---

#### FR-P4 — Immutability

**Requirement**: A `Page` SHALL become immutable after finalization. Rendering SHALL operate only on finalized `Page` objects.

**Rationale**: Prevents accidental modifications during rendering, enables `Send + Sync`.

**Verification**: Compile-time borrow checker enforcement.

**Priority**: P0 (Critical)

---

### 6.2 Region Requirements

#### FR-R1 — Static Rectangular Geometry

**Requirement**: A `Region` SHALL define:
- `x`, `y`: Origin coordinates relative to parent region or page
- `width`, `height`: Dimensions in columns/lines

**Rationale**: Enables predictable, static layout calculations.

**Verification**: Unit tests validate region geometry calculations.

**Priority**: P0 (Critical)

---

#### FR-R2 — Local Coordinates

**Requirement**: Region coordinates SHALL start at `(0, 0)` for the top-left corner.

**Rationale**: Simplifies widget logic (widgets don't need parent offset awareness).

**Verification**: Integration tests verify coordinate translation.

**Priority**: P0 (Critical)

---

#### FR-R3 — Region Clipping

**Requirement**: Writes that exceed region `width` or `height` SHALL be truncated.

**Rationale**: Prevents layout corruption from overflowing content.

**Verification**: Property-based tests with random content sizes.

**Priority**: P0 (Critical)

---

#### FR-R4 — Nesting

**Requirement**: Regions MUST support hierarchical children with unlimited nesting depth.

**Rationale**: Enables complex layouts (e.g., tables within sections within pages).

**Verification**: Integration tests with 10+ nesting levels.

**Priority**: P0 (Critical)

---

#### FR-R5 — Splitting API

**Requirement**: Regions SHALL support:
- **Horizontal splitting**: Divide into left/right subregions
- **Vertical splitting**: Divide into top/bottom subregions
- **Grid-based splitting**: M×N grid of subregions

**Rationale**: Common layout patterns for forms (headers, footers, tables).

**Verification**: Unit tests verify split geometry correctness.

**Priority**: P0 (Critical)

---

#### FR-R6 — Padding

**Requirement**: Regions MAY define padding (top, right, bottom, left) that reduces the inner usable area.

**Rationale**: Provides visual spacing without manual offset calculations.

**Verification**: Unit tests verify inner region dimensions after padding.

**Priority**: P1 (High)

---

#### FR-R7 — Default Style

**Requirement**: A `Region` MAY define a default `Style` applied to all text writes within that region.

**Rationale**: Enables style inheritance (e.g., bold headers, normal body).

**Verification**: Unit tests verify style application precedence.

**Priority**: P2 (Medium)

---

### 6.3 Document Requirements

#### FR-D1 — Document Structure

**Requirement**: A `Document` SHALL contain an ordered list of `Page` objects.

**Rationale**: Multi-page forms require explicit page collection.

**Verification**: Unit tests with 1, 10, and 100-page documents.

**Priority**: P0 (Critical)

---

#### FR-D2 — Manual Pagination

**Requirement**: The system SHALL NOT automatically create pages. Developers SHALL explicitly create each page.

**Rationale**: V1 prioritizes predictability over convenience.

**Verification**: API design review ensures no auto-pagination code paths.

**Priority**: P0 (Critical)

---

#### FR-D3 — Document Immutability

**Requirement**: Finalized `Document` SHALL be immutable.

**Rationale**: Enables thread-safe rendering and caching.

**Verification**: Compile-time borrow checker enforcement.

**Priority**: P0 (Critical)

---

#### FR-D4 — ESC/P Rendering

**Requirement**: A `Document` SHALL render to a single ESC/P byte stream (`Vec<u8>`).

**Rationale**: Printer expects continuous byte stream.

**Verification**: Hardware tests validate printer acceptance.

**Priority**: P0 (Critical)

---

#### FR-D5 — Page Separation

**Requirement**: A Form Feed (`\x0C`) SHALL separate each page in rendered output.

**Rationale**: Standard ESC/P page break mechanism.

**Verification**: Byte-level output inspection.

**Priority**: P0 (Critical)

---

### 6.4 Widget Requirements

**General Rule**: All widgets SHALL respect `Region` boundaries and apply truncation per FR-T1/FR-T2.

---

#### FR-W1 — Label Widget

**Requirement**: The `Label` widget SHALL support:
- Single-line text
- Alignment: left, center, right
- Horizontal truncation if text exceeds region width

**Priority**: P0 (Critical)

---

#### FR-W2 — TextBlock Widget

**Requirement**: The `TextBlock` widget SHALL support:
- Multi-line preformatted text (preserves whitespace)
- No automatic wrapping
- Strict horizontal and vertical truncation

**Use Case**: Code snippets, ASCII art, fixed-format data.

**Priority**: P0 (Critical)

---

#### FR-W3 — Paragraph Widget

**Requirement**: The `Paragraph` widget SHALL support:
- Soft word-wrapping at region width
- No hyphenation
- Vertical truncation (lines beyond region height discarded)

**Use Case**: Terms & conditions, descriptions.

**Priority**: P0 (Critical)

---

#### FR-W4 — Box Widget

**Requirement**: The `Box` widget SHALL:
- Draw ASCII border using `+`, `-`, `|` characters
- Support optional title in top border (e.g., `+--- Title ---+`)
- Provide an inner `Region` with dimensions `(width - 2, height - 2)`

**Use Case**: Visual grouping of content.

**Priority**: P1 (High)

---

#### FR-W5 — KeyValue Widget

**Requirement**: The `KeyValue` widget SHALL support:
- Fixed-width key column
- Value starts at defined offset
- Independent truncation of keys and values

**Example**:
```
Invoice Number: 12345
Date          : 2025-01-18
```

**Priority**: P1 (High)

---

#### FR-W6 — Table Widget

**Requirement**: The `Table` widget SHALL support:
- Fixed column widths (specified by developer)
- Header row with optional separators
- Data rows
- Horizontal truncation per column
- Vertical truncation (rows beyond region height discarded)

**Use Case**: Line items, product lists.

**Priority**: P0 (Critical)

---

### 6.5 Style Requirements

#### FR-S1 — Supported Styles

**Requirement**: The library SHALL support:
- **Normal**: No formatting
- **Bold**: ESC/P bold mode
- **Underline**: ESC/P underline mode

**Priority**: P0 (Critical)

---

#### FR-S2 — ESC/P Command Mapping

**Requirement**: The renderer SHALL emit:
- **Bold ON**: `ESC E` (`\x1B\x45`)
- **Bold OFF**: `ESC F` (`\x1B\x46`)
- **Underline ON**: `ESC - 1` (`\x1B\x2D\x01`)
- **Underline OFF**: `ESC - 0` (`\x1B\x2D\x00`)
- **Condensed Mode**: `SI` (`\x0F`)

**Verification**: Byte-level output validation against ESC/P specification.

**Priority**: P0 (Critical)

---

#### FR-S3 — Style State Machine

**Requirement**: The renderer MUST track current style state and emit transitions ONLY when the style changes.

**Rationale**: Minimizes output byte stream size and printer processing.

**Verification**: Unit tests verify minimal escape sequence emission.

**Priority**: P1 (High)

---

### 6.6 Truncation Requirements

**Critical**: These requirements define the library's predictability contract.

---

#### FR-T1 — Horizontal Truncation

**Requirement**: Content beyond region `width` SHALL be silently dropped.

**Verification**: Property tests with random long strings.

**Priority**: P0 (Critical)

---

#### FR-T2 — Vertical Truncation

**Requirement**: Lines beyond region `height` SHALL be silently discarded.

**Verification**: Property tests with random line counts.

**Priority**: P0 (Critical)

---

#### FR-T3 — Page Boundary Truncation

**Requirement**: Writes outside the (160×51) page bounds SHALL be dropped.

**Verification**: Unit tests with out-of-bounds regions.

**Priority**: P0 (Critical)

---

#### FR-T4 — No Error on Overflow

**Requirement**: Content overflow SHALL NOT produce errors, warnings, or panics.

**Rationale**: Silent truncation is predictable and matches V1 design philosophy.

**Verification**: Fuzzing tests confirm zero panics.

**Priority**: P0 (Critical)

---

### 6.7 ESC/P Rendering Requirements

#### FR-E1 — Rendering Order

**Requirement**: The renderer SHALL emit commands in this order:
1. **Reset**: `ESC @` (`\x1B\x40`)
2. **Condensed Mode**: `SI` (`\x0F`)
3. **Line-by-line rendering**: Emit text with style transitions
4. **Line Termination**: `CR + LF` (`\x0D\x0A`)
5. **Page Separation**: `FF` (`\x0C`) after each page

**Verification**: Byte sequence parser validates order.

**Priority**: P0 (Critical)

---

#### FR-E2 — Deterministic Output

**Requirement**: Rendering MUST be byte-for-byte deterministic (same input → same output).

**Rationale**: Enables golden master testing, regulatory compliance, caching.

**Verification**: SHA-256 hash comparison across 1000 runs.

**Priority**: P0 (Critical)

---

#### FR-E3 — Character Safety

**Requirement**: Non-ASCII characters (value > 127) SHALL be replaced by `'?'` (`0x3F`).

**Rationale**: ESC/P text mode is ASCII-only.

**Verification**: Unit tests with UTF-8 input.

**Priority**: P0 (Critical)

---

#### FR-E4 — Text Mode Only

**Requirement**: The renderer MUST NOT enter bitmap, raster, or graphics modes.

**Rationale**: V1 is text-only to minimize complexity.

**Verification**: Code review ensures no graphics ESC sequences.

**Priority**: P0 (Critical)

---

### 6.8 Dependency Requirements

#### FR-DEP1 — Zero Runtime Dependencies

**Requirement**: The library MUST have zero required runtime dependencies beyond Rust `std`.

**Rationale**: Minimizes supply chain risk and binary size.

**Verification**: `cargo tree` analysis.

**Priority**: P0 (Critical)

---

#### FR-DEP2 — Optional Serialization

**Requirement**: The library MAY support optional `serde` integration via feature flags.

**Use Case**: Serialize/deserialize documents for debugging.

**Priority**: P2 (Medium)

---

#### FR-DEP3 — Minimum Supported Rust Version (MSRV)

**Requirement**: MSRV SHALL be **Rust 1.75.0** (released 2023-12-28).

**Rationale**: Balances modern language features with ecosystem compatibility.

**Verification**: CI tests on Rust 1.75.0.

**Priority**: P1 (High)

---

#### FR-DEP4 — Platform Support

**Requirement**: The library MUST compile on stable Rust for:
- `x86_64-unknown-linux-gnu` (Linux)
- `x86_64-apple-darwin` (macOS)
- `x86_64-pc-windows-msvc` (Windows)

**Verification**: CI matrix builds.

**Priority**: P0 (Critical)

---

#### FR-DEP5 — Future `no_std` Compatibility

**Requirement**: The architecture MUST NOT prevent future `no_std` support in V2.

**Rationale**: Embedded use cases may require `no_std`.

**Verification**: Design review ensures no `std`-only patterns.

**Priority**: P2 (Medium)

---

### 6.9 Error Handling Requirements

#### FR-ERR1 — Error Type

**Requirement**: The library SHALL define a structured error enum:

```rust
pub enum LayoutError {
    RegionOutOfBounds { x: u16, y: u16, max_x: u16, max_y: u16 },
    InvalidDimensions { width: u16, height: u16 },
    BuilderNotFinalized,
    // ... additional variants
}
```

**Priority**: P0 (Critical)

---

#### FR-ERR2 — Result Types

**Requirement**: Builder methods that can fail MUST return `Result<T, LayoutError>`.

**Rationale**: Idiomatic Rust error handling.

**Priority**: P0 (Critical)

---

#### FR-ERR3 — Panic Policy

**Requirement**: The library MUST NOT panic except for:
- **Contract violations in debug builds** (`debug_assert!`)
- **Developer misuse of unsafe APIs** (if any, with clear documentation)

**Rationale**: Production stability.

**Verification**: Fuzzing with `cargo-fuzz`.

**Priority**: P0 (Critical)

---

#### FR-ERR4 — Overflow Behavior

**Requirement**: Content overflow (per FR-T1/FR-T2/FR-T3) is NOT an error.

**Rationale**: Silent truncation is the defined behavior.

**Priority**: P0 (Critical)

---

#### FR-ERR5 — Validation

**Requirement**: Region geometry MUST be validated at creation time:
- `width > 0`, `height > 0`
- `x + width ≤ parent width`
- `y + height ≤ parent height`

**Verification**: Unit tests with invalid inputs.

**Priority**: P0 (Critical)

---

## 7. API Requirements (Developer-Facing)

### 7.1 Builder Pattern

**Requirement**: The library SHALL expose a fluent builder pattern with the following types:

---

#### `DocumentBuilder`

```rust
pub struct DocumentBuilder {
    // Private fields
}

impl DocumentBuilder {
    /// Creates a new document builder
    pub fn new() -> Self;

    /// Adds a new page to the document
    /// Returns a PageBuilder for configuring the page
    pub fn add_page(&mut self) -> PageBuilder<'_>;

    /// Finalizes and builds the immutable document
    /// Consumes the builder
    pub fn build(self) -> Document;
}

impl Default for DocumentBuilder {
    fn default() -> Self;
}
```

---

#### `PageBuilder`

```rust
pub struct PageBuilder<'doc> {
    // Private fields, holds reference to parent DocumentBuilder
    _phantom: PhantomData<&'doc mut DocumentBuilder>,
}

impl<'doc> PageBuilder<'doc> {
    /// Returns a handle to the root region (160×51)
    pub fn root_region(&mut self) -> RegionHandle<'_>;

    /// Finalizes the page
    /// After finalization, no more modifications allowed
    pub fn finalize(self) -> Result<(), LayoutError>;
}
```

---

#### `RegionHandle`

```rust
pub struct RegionHandle<'page> {
    // Private fields
    _phantom: PhantomData<&'page mut Page>,
}

impl<'page> RegionHandle<'page> {
    // --- Text Writing ---

    /// Writes text at (x, y) with optional style
    pub fn write_text(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        style: Style,
    ) -> &mut Self;

    // --- Widgets ---

    /// Adds a widget to this region
    pub fn add_widget<W: Widget>(&mut self, widget: W) -> Result<(), LayoutError>;

    /// Convenience: Add a label
    pub fn label(
        &mut self,
        text: &str,
        alignment: Alignment,
    ) -> &mut Self;

    /// Convenience: Add a table
    pub fn table(
        &mut self,
        config: TableConfig,
    ) -> TableBuilder<'_>;

    // --- Region Splitting ---

    /// Splits region horizontally into N subregions
    /// `ratios`: Relative widths (e.g., &[1, 2, 1] = 25%, 50%, 25%)
    pub fn split_horizontal(
        &mut self,
        ratios: &[u16],
    ) -> Result<Vec<RegionHandle<'_>>, LayoutError>;

    /// Splits region vertically into N subregions
    pub fn split_vertical(
        &mut self,
        ratios: &[u16],
    ) -> Result<Vec<RegionHandle<'_>>, LayoutError>;

    /// Creates an M×N grid of subregions
    pub fn grid(
        &mut self,
        rows: u16,
        cols: u16,
    ) -> Result<Vec<Vec<RegionHandle<'_>>>, LayoutError>;

    // --- Styling ---

    /// Sets default style for this region
    pub fn set_default_style(&mut self, style: Style) -> &mut Self;

    // --- Layout ---

    /// Applies padding (reduces inner usable area)
    pub fn with_padding(
        &mut self,
        top: u16,
        right: u16,
        bottom: u16,
        left: u16,
    ) -> &mut Self;

    /// Creates a child region with specific geometry
    pub fn child_region(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> Result<RegionHandle<'_>, LayoutError>;
}
```

---

#### `Document`

```rust
pub struct Document {
    // Private fields
}

impl Document {
    /// Renders the document to ESC/P byte stream
    pub fn render(&self) -> Vec<u8>;

    /// Returns number of pages
    pub fn page_count(&self) -> usize;
}

// Thread-safe
unsafe impl Send for Document {}
unsafe impl Sync for Document {}
```

---

### 7.2 Core Types

#### `Style`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub bold: bool,
    pub underline: bool,
}

impl Style {
    pub const NORMAL: Style = Style { bold: false, underline: false };
    pub const BOLD: Style = Style { bold: true, underline: false };
    pub const UNDERLINE: Style = Style { bold: false, underline: true };
}

impl Default for Style {
    fn default() -> Self {
        Style::NORMAL
    }
}
```

---

#### `Alignment`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}
```

---

### 7.3 Widget Trait

```rust
pub trait Widget {
    /// Renders the widget into the given region
    fn render(&self, region: &mut RegionHandle) -> Result<(), LayoutError>;
}
```

**Built-in widgets**:
- `Label`
- `TextBlock`
- `Paragraph`
- `Box`
- `KeyValue`
- `Table`

---

### 7.4 Error Type

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    RegionOutOfBounds {
        x: u16,
        y: u16,
        max_x: u16,
        max_y: u16,
    },
    InvalidDimensions {
        width: u16,
        height: u16,
    },
    BuilderNotFinalized,
    InvalidSplitRatios {
        provided: usize,
        expected: usize,
    },
}

impl std::fmt::Display for LayoutError {
    // Implementation
}

impl std::error::Error for LayoutError {}
```

---

### 7.5 API Design Principles

1. **Lifetime Safety**: Builders use lifetimes to prevent dangling references
2. **Fluent API**: Method chaining for ergonomics
3. **Type State Pattern**: Prevent invalid state transitions at compile time
4. **Zero Cost Abstractions**: No runtime overhead for builder pattern
5. **Clear Ownership**: Consumes builder on `build()` to prevent misuse

---

## 8. Non-Functional Requirements (NFR)

### NFR-1 — Determinism

**Requirement**: Same inputs MUST produce byte-for-byte identical ESC/P output across all platforms and Rust compiler versions.

**Measurement**: SHA-256 hash comparison across 1000 rendering iterations.

**Target**: 100% reproducibility.

**Priority**: P0 (Critical)

---

### NFR-2 — Performance

**Requirement**: Rendering performance MUST meet the following targets:

| Operation | Complexity | Target Latency (p99) |
|-----------|------------|----------------------|
| Single page render | O(160 × 51) = O(8,160) | < 100μs |
| 100-page document | O(100 × 8,160) | < 10ms |
| Region lookup | O(depth) | < 1μs per lookup |

**Measurement**: `criterion` benchmarks on AMD Ryzen 5 5600X or equivalent (3.7GHz base clock).

**Priority**: P1 (High)

---

### NFR-3 — Memory Efficiency

**Requirement**: The library SHALL minimize heap allocations:
- Rendering MUST NOT allocate during inner cell-writing loops
- Page model: ≤ 8,160 cells × (1 byte char + 2 bytes style) ≈ 24 KB per page
- Document overhead: < 1MB regardless of page count

**Measurement**: `heaptrack`, `valgrind --tool=massif`, or `cargo-flamegraph`.

**Priority**: P1 (High)

---

### NFR-4 — Safety

**Requirement**: The library MUST NOT panic under any documented usage:
- Invalid inputs return `Result<T, LayoutError>`
- Out-of-bounds writes are silently truncated
- Only `debug_assert!` may trigger panics (debug builds only)

**Measurement**: Fuzzing with `cargo-fuzz` (1M+ inputs, 0 panics).

**Priority**: P0 (Critical)

---

### NFR-5 — Portability

**Requirement**: MUST compile and pass all tests on:
- **Linux**: `x86_64-unknown-linux-gnu` (Ubuntu 22.04 LTS)
- **macOS**: `x86_64-apple-darwin`, `aarch64-apple-darwin` (macOS 13+)
- **Windows**: `x86_64-pc-windows-msvc` (Windows 10+)

**Measurement**: CI matrix builds on GitHub Actions.

**Priority**: P0 (Critical)

---

### NFR-6 — Thread Safety

**Requirement**: Finalized `Document` MUST be `Send + Sync`.

**Rationale**: Enables multi-threaded rendering pipelines and caching.

**Verification**: Compile-time trait bounds, `cargo clippy`.

**Priority**: P1 (High)

---

### NFR-7 — Compile Time

**Requirement**: Full project build SHOULD complete in:
- **Clean build**: < 30 seconds (release mode)
- **Incremental build**: < 5 seconds (dev mode)

**Measurement**: CI build time tracking.

**Priority**: P2 (Medium)

---

### NFR-8 — Binary Size

**Requirement**: Statically linked binary SHOULD be:
- **Debug build**: < 10 MB
- **Release build**: < 2 MB (with LTO enabled)

**Measurement**: `ls -lh target/release/epson_lq2090_layout`.

**Priority**: P2 (Medium)

---

### NFR-9 — Documentation Coverage

**Requirement**: All public APIs MUST have rustdoc comments with examples.

**Target**: 100% public API coverage.

**Measurement**: `cargo doc --no-deps && cargo deadlinks`.

**Priority**: P1 (High)

---

## 9. Technical Constraints

The following constraints are **immutable** for V1:

| Constraint | Rationale | Impact |
|------------|-----------|--------|
| **Fixed page dimensions (160×51)** | Hardware limitation of LQ-2090II in condensed mode | No dynamic page sizing |
| **ASCII-only output** | ESC/P text mode limitation | No Unicode support |
| **ESC/P commands limited to text mode** | Avoid complexity of graphics modes | No bitmaps or vector graphics |
| **No external native dependencies** | Portability and supply chain security | Only Rust `std` |
| **Manual pagination only** | V1 design philosophy | Developer must create pages explicitly |
| **Static layout only** | Simplicity and determinism | No flexbox/auto-layout |

**Future Compatibility Constraint**:
The architecture MUST NOT prevent V2 features:
- Auto-layout engines
- Dynamic-height regions
- Automatic pagination
- `no_std` support

---

## 10. Risks & Mitigations

| Risk ID | Risk | Impact | Likelihood | Mitigation Strategy | Owner |
|---------|------|--------|------------|---------------------|-------|
| **R-001** | ESC/P variant differences across printer models | HIGH | MEDIUM | Use minimal compatible ESC/P subset; hardware test on multiple models | Engineering |
| **R-002** | Incorrect style state machine transitions | HIGH | MEDIUM | Implement formal state machine with unit tests; property-based testing | Engineering |
| **R-003** | Developers misusing Region geometry | MEDIUM | HIGH | Validate geometry at builder level; comprehensive API docs with examples | Engineering + Docs |
| **R-004** | Non-ASCII input causing output corruption | HIGH | MEDIUM | Force '?' replacement; document ASCII-only limitation prominently | Engineering |
| **R-005** | Performance degradation with deep nesting | MEDIUM | LOW | Benchmark deep nesting (10+ levels); optimize lookup if needed | Engineering |
| **R-006** | Memory leaks in long-running processes | HIGH | LOW | Valgrind testing; ensure proper `Drop` implementations | QA |
| **R-007** | Breaking API changes needed in V1.x | MEDIUM | MEDIUM | Design API with extensibility in mind; use SemVer strictly | Product + Engineering |
| **R-008** | Inadequate documentation leads to misuse | MEDIUM | HIGH | Invest in tutorials, examples, and rustdoc; user testing | Docs |
| **R-009** | Hardware printer unavailable for testing | MEDIUM | LOW | Source backup printer; use emulator for CI | QA |
| **R-010** | Rust MSRV conflicts with ecosystem | LOW | MEDIUM | Conservative MSRV choice (1.75.0); monitor ecosystem | Engineering |

---

## 11. Testing Strategy

### 11.1 Unit Tests

**Coverage Target**: ≥ 90% line coverage

**Scope**:
- All public API functions
- Edge cases: empty regions, zero-width/height, boundary writes
- Style state machine transitions
- Region splitting calculations
- Widget rendering logic

**Tools**: Built-in `cargo test`, `tarpaulin` for coverage

---

### 11.2 Integration Tests

**Scope**:
- End-to-end document rendering
- Multi-page pagination
- Nested region hierarchies (5+ levels)
- All widgets in combination
- Builder API workflow

**Test Cases**: Minimum 50 integration tests

---

### 11.3 Property-Based Tests

**Tool**: `proptest`

**Properties to test**:
- **Truncation correctness**: Arbitrary long strings → no overflow
- **Determinism**: Same input → Same output (1000 runs)
- **No panics**: Arbitrary valid region geometries → no crash
- **Idempotency**: Rendering twice produces identical bytes

**Fuzzing Targets**:
- Region geometry inputs
- Text content (arbitrary UTF-8)
- Style state transitions

---

### 11.4 Hardware Validation Tests

**Printer**: EPSON LQ-2090II

**Test Forms**:
1. Simple invoice (1 page)
2. Multi-page invoice (3 pages)
3. Nested regions stress test
4. All widgets showcase
5. Government form (strict alignment)
6. Table with 50 rows
7. Deep nesting (10 levels)
8. Style transitions (bold, underline combinations)
9. Boundary overflow test
10. Maximum page count (100 pages)

**Acceptance**: Visual inspection by QA, no misalignment

---

### 11.5 Golden Master Tests

**Approach**: Byte-level ESC/P output comparison

**Test Cases**: 20+ golden master files

**Verification**: SHA-256 hash match

**CI Integration**: Fail build if hash mismatch

---

### 11.6 Regression Tests

**Strategy**:
- All bugs get a regression test before fix
- Golden master suite prevents output regressions
- API compatibility tests prevent breaking changes

---

### 11.7 Performance Benchmarks

**Tool**: `criterion`

**Benchmarks**:
- Single page render (various widget combinations)
- 10/50/100-page documents
- Deep region nesting (1-15 levels)
- Large tables (10×100, 20×50)
- Style-heavy documents (50+ transitions)

**CI Integration**: Track performance over time, alert on >10% regression

---

## 12. Security & Safety

### 12.1 Input Validation

**SEC-001**: All user-provided dimensions MUST be validated to prevent:
- Integer overflow (width/height > u16::MAX)
- Out-of-bounds memory access
- Denial-of-service via excessive allocations

**Implementation**: Bounds checking in all builder methods.

---

### 12.2 Unsafe Code Policy

**SEC-002**: The library SHOULD minimize `unsafe` code.

**Requirements**:
- All `unsafe` blocks MUST be documented with safety invariants
- Prefer safe Rust alternatives
- Audit all `unsafe` code in code review

**Current Status**: V1 aims for **zero unsafe blocks**.

---

### 12.3 Dependency Audits

**SEC-003**: Run `cargo audit` in CI to detect vulnerable dependencies.

**Frequency**: On every pull request and daily scheduled builds.

**Response**: Patch within 7 days for HIGH/CRITICAL vulnerabilities.

---

### 12.4 Fuzzing

**SEC-004**: Integrate `cargo-fuzz` with the following targets:

| Target | Input | Objective |
|--------|-------|-----------|
| `fuzz_region_geometry` | Arbitrary (x, y, width, height) | No panics, no OOM |
| `fuzz_text_content` | Arbitrary UTF-8 strings | Correct ASCII conversion, no corruption |
| `fuzz_style_transitions` | Random style change sequences | Correct ESC/P state machine |

**CI Integration**: Run 1M iterations on PRs.

---

### 12.5 Supply Chain Security

**SEC-005**:
- Zero required runtime dependencies (only `std`)
- Development dependencies vetted and pinned
- `cargo-deny` configured to reject untrusted crates

---

## 13. Versioning & Compatibility

### 13.1 Semantic Versioning

The library SHALL follow [SemVer 2.0.0](https://semver.org/):

- **MAJOR** (X.0.0): Breaking API changes
- **MINOR** (1.X.0): Backward-compatible new features
- **PATCH** (1.0.X): Backward-compatible bug fixes

**Example**:
- `1.0.0` → `1.1.0`: Add new widget (backward-compatible)
- `1.1.0` → `1.1.1`: Fix rendering bug (backward-compatible)
- `1.x.x` → `2.0.0`: Change `RegionHandle` signature (breaking)

---

### 13.2 API Stability Guarantees

**V1.x.x Series**:
- Public API frozen except for additions
- Deprecation warnings MUST precede removal by ≥ 1 minor version
- ESC/P output format stability (byte-level compatibility)

**Deprecation Policy**:
```rust
#[deprecated(since = "1.2.0", note = "Use `new_method()` instead")]
pub fn old_method() { /* ... */ }
```

---

### 13.3 MSRV Policy

**Minimum Supported Rust Version (MSRV)**: 1.75.0

**Updates**:
- MSRV bump requires MINOR version increment (e.g., 1.2.0 → 1.3.0)
- MSRV documented in `Cargo.toml` and README
- CI tests on MSRV version

---

### 13.4 Compatibility Testing

**Requirement**: All releases MUST pass compatibility tests:
- Compile on MSRV
- All tests pass on stable and nightly Rust
- Cross-platform builds succeed (Linux/macOS/Windows)

---

## 14. Distribution & Packaging

### 14.1 Crate Publishing

**Package Name**: `epson-lq2090-layout` (subject to availability on crates.io)

**Repository**: Public GitHub repository (recommended: `anthropics/epson-lq2090-layout`)

**License**: MIT OR Apache-2.0 (dual license, standard Rust practice)

**Publishing**:
- Automated via GitHub Actions on git tags (e.g., `v1.0.0`)
- Requires manual approval from maintainer
- Changelog verification before publish

---

### 14.2 Documentation Hosting

**docs.rs**: Auto-generated from rustdoc comments

**GitHub Pages** (optional): Extended guides, tutorials, examples

---

### 14.3 CI/CD Pipeline

**Platform**: GitHub Actions

**Jobs**:
1. **Lint**: `cargo fmt --check`, `cargo clippy -- -D warnings`
2. **Test**: `cargo test --all-features`
3. **Build Matrix**: Linux/macOS/Windows × stable/MSRV/nightly
4. **Benchmarks**: Run criterion benchmarks, track regressions
5. **Security**: `cargo audit`, `cargo-deny`
6. **Fuzzing**: Run `cargo-fuzz` targets (1M iterations)
7. **Coverage**: `tarpaulin`, upload to codecov.io

---

### 14.4 Release Artifacts

Each release SHALL include:
- **Source tarball** (GitHub Releases)
- **Git tag** (signed, if possible)
- **CHANGELOG.md** entry
- **Migration guide** (for breaking changes)
- **Announcement** (GitHub Discussions, Reddit r/rust)

---

### 14.5 Examples

**Requirement**: Minimum **5 runnable examples** in `/examples`:
1. `simple_invoice.rs`: Basic single-page invoice
2. `multi_page.rs`: 3-page document with tables
3. `nested_regions.rs`: Complex nested layout
4. `all_widgets.rs`: Showcase all widget types
5. `custom_widget.rs`: Implementing the `Widget` trait

---

## 15. Documentation Requirements

### 15.1 API Documentation

**Rustdoc Coverage**: 100% of public APIs

**Requirements**:
- Every public function/struct/enum has `///` comments
- Code examples in docs (compilable, tested)
- Usage guidelines and edge cases documented

**Verification**: `cargo doc --no-deps && cargo deadlinks`

---

### 15.2 README

**Sections**:
1. **Quick Start**: Get printing in < 5 minutes
2. **Features**: High-level capabilities
3. **Installation**: `cargo add epson-lq2090-layout`
4. **Basic Example**: 10-line invoice
5. **Documentation Links**
6. **License**
7. **Contributing**

**Target Length**: < 500 lines

---

### 15.3 User Guides

| Guide | Audience | Topics |
|-------|----------|--------|
| **Tutorial: Your First Invoice** | Beginners | Page creation, regions, basic widgets |
| **How-To: Multi-Page Documents** | Intermediate | Manual pagination, headers/footers |
| **How-To: Custom Widgets** | Advanced | Implementing `Widget` trait |
| **Reference: ESC/P Commands** | All | Command mappings, troubleshooting |

---

### 15.4 Architecture Decision Records (ADRs)

**Tool**: Markdown files in `/docs/adr/`

**Topics**:
- Why static layout only in V1
- Why manual pagination
- Builder pattern choice
- Lifetime design rationale

---

### 15.5 Troubleshooting Guide

**Common Issues**:
- Misalignment on printer → Check condensed mode (SI) is emitted
- Truncation not working → Verify region geometry validation
- Panics → File bug report with reproduction

---

## 16. Acceptance Criteria

The following criteria **MUST be met** for V1 release approval:

---

### 16.1 Page Model Acceptance

| Criterion | Test Method | Status |
|-----------|-------------|--------|
| Page dimensions fixed at 160×51 | Unit tests | [ ] |
| Writes outside bounds are clipped | Property tests | [ ] |
| Page immutable after finalization | Compile-time verification | [ ] |
| Cell stores char + style correctly | Unit tests | [ ] |

---

### 16.2 Region Acceptance

| Criterion | Test Method | Status |
|-----------|-------------|--------|
| Nested regions map correctly to page coordinates | Integration tests | [ ] |
| Horizontal/vertical splits produce correct geometry | Unit tests | [ ] |
| All writes respect region boundaries | Property tests | [ ] |
| Padding reduces inner usable area | Unit tests | [ ] |
| Grid splitting produces M×N subregions | Unit tests | [ ] |

---

### 16.3 Widget Acceptance

| Criterion | Test Method | Status |
|-----------|-------------|--------|
| **Label**: Aligns and truncates correctly | Unit + hardware tests | [ ] |
| **Box**: Draws correct ASCII borders | Unit + hardware tests | [ ] |
| **Table**: Respects column widths, truncates rows | Integration + hardware tests | [ ] |
| **Paragraph**: Wraps within region width | Unit + hardware tests | [ ] |
| **TextBlock**: Preserves whitespace, no wrapping | Unit tests | [ ] |
| **KeyValue**: Fixed key width, independent truncation | Unit tests | [ ] |

---

### 16.4 Renderer Acceptance

| Criterion | Test Method | Status |
|-----------|-------------|--------|
| Correct ESC/P sequence ordering (ESC @, SI, ...) | Byte-level output inspection | [ ] |
| Bold/underline toggling emits correct ESC codes | Unit tests | [ ] |
| Non-ASCII characters replaced by '?' | Unit tests | [ ] |
| Deterministic output (100% SHA-256 match) | Golden master tests (1000 runs) | [ ] |
| Form Feed (FF) separates pages | Byte-level verification | [ ] |

---

### 16.5 API Acceptance

| Criterion | Test Method | Status |
|-----------|-------------|--------|
| Builder API is ergonomic (< 20 calls for invoice) | Code review + user testing | [ ] |
| No panics under documented usage | Fuzzing (1M+ inputs) | [ ] |
| Lifetimes prevent dangling references | Compile-time verification | [ ] |
| Error types are descriptive and actionable | Manual review | [ ] |

---

### 16.6 Hardware Acceptance

| Criterion | Test Method | Status |
|-----------|-------------|--------|
| 10 test forms print correctly on LQ-2090II | Visual inspection by QA | [ ] |
| No misalignment on multi-part forms | Hardware testing | [ ] |
| All widgets render correctly on paper | Visual inspection | [ ] |

---

### 16.7 Documentation Acceptance

| Criterion | Test Method | Status |
|-----------|-------------|--------|
| 100% public API rustdoc coverage | `cargo doc` + manual review | [ ] |
| README enables < 5min quickstart | User testing | [ ] |
| 5+ runnable examples provided | `cargo run --example` | [ ] |
| Troubleshooting guide addresses common issues | Manual review | [ ] |

---

### 16.8 Quality Assurance Acceptance

| Criterion | Test Method | Status |
|-----------|-------------|--------|
| ≥ 90% line coverage | `tarpaulin` | [ ] |
| Zero panics in fuzzing (1M+ iterations) | `cargo-fuzz` | [ ] |
| Zero critical/high vulnerabilities | `cargo audit` | [ ] |
| Passes on all target platforms | CI matrix | [ ] |
| Performance benchmarks pass | `criterion` | [ ] |

---

## 17. Architecture & Design

### 17.1 High-Level Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      USER APPLICATION                        │
│  (Rust code using the library)                              │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Uses builder API
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    BUILDER LAYER                             │
│  ┌────────────────┐   ┌──────────────┐   ┌────────────────┐ │
│  │DocumentBuilder │──>│ PageBuilder  │──>│ RegionHandle   │ │
│  └────────────────┘   └──────────────┘   └────────────────┘ │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Builds
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                     CORE MODEL                               │
│  ┌──────────┐     ┌──────────┐     ┌────────────────────┐  │
│  │ Document │────>│   Page   │────>│  Region (tree)     │  │
│  │(immutable│     │(160×51)  │     │                    │  │
│  │ pages)   │     │          │     │  Cells[8160]       │  │
│  └──────────┘     └──────────┘     └────────────────────┘  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Renders
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    WIDGET LAYER                              │
│  ┌──────┐ ┌─────────┐ ┌────────┐ ┌──────┐ ┌────────┐      │
│  │Label │ │TextBlock│ │Paragraph│ │ Box  │ │ Table  │ ...  │
│  └──────┘ └─────────┘ └────────┘ └──────┘ └────────┘      │
│  (All implement Widget trait)                                │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Widgets write to regions
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                   RENDERING LAYER                            │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              ESC/P Renderer                            │ │
│  │  • Style state machine (bold/underline tracking)      │ │
│  │  • ASCII sanitization                                 │ │
│  │  • Line-by-line output generation                     │ │
│  │  • Form Feed insertion                                │ │
│  └────────────────────────────────────────────────────────┘ │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Outputs
                     ▼
              ┌──────────────┐
              │   Vec<u8>    │
              │ (ESC/P bytes)│
              └──────────────┘
                     │
                     ▼
              ┌──────────────┐
              │ EPSON Printer│
              │   LQ-2090II  │
              └──────────────┘
```

---

### 17.2 Data Flow Diagram

```
1. BUILD PHASE
   User Code
      │
      ├─> DocumentBuilder::new()
      │
      ├─> add_page() ──> PageBuilder
      │                     │
      │                     ├─> root_region() ──> RegionHandle
      │                     │                          │
      │                     │                          ├─> split_horizontal([1,2,1])
      │                     │                          ├─> add_widget(Table { ... })
      │                     │                          └─> write_text(...)
      │                     │
      │                     └─> finalize() ──> Validates geometry
      │
      └─> build() ──> Immutable Document


2. RENDER PHASE
   Document
      │
      └─> render()
             │
             ├─> For each Page:
             │      │
             │      ├─> Emit ESC @ (reset)
             │      ├─> Emit SI (condensed mode)
             │      │
             │      ├─> For each line (0..51):
             │      │      │
             │      │      ├─> For each column (0..160):
             │      │      │      │
             │      │      │      ├─> Read Cell (char, style)
             │      │      │      ├─> Emit style transitions if needed
             │      │      │      └─> Emit character (ASCII)
             │      │      │
             │      │      └─> Emit CR + LF
             │      │
             │      └─> Emit FF (form feed)
             │
             └─> Return Vec<u8>
```

---

### 17.3 Style State Machine

```
┌─────────────────────────────────────────────────────┐
│            Style Transition State Machine            │
└─────────────────────────────────────────────────────┘

States: Normal, Bold, Underline, Bold+Underline

Transitions:
  Normal  ───(Bold requested)───>  Bold       [Emit: ESC E]
  Bold    ───(Normal requested)──> Normal     [Emit: ESC F]
  Normal  ───(Underline req.)───>  Underline  [Emit: ESC - 1]
  Underline─(Normal requested)──>  Normal     [Emit: ESC - 0]
  Bold    ───(Bold+Underline)───>  B+U        [Emit: ESC - 1]
  etc.

Optimization: Only emit ESC codes when state changes
```

---

### 17.4 Module Structure

```
epson_lq2090_layout/
├── src/
│   ├── lib.rs               // Public API exports
│   ├── builder/
│   │   ├── mod.rs
│   │   ├── document.rs      // DocumentBuilder
│   │   ├── page.rs          // PageBuilder
│   │   └── region.rs        // RegionHandle
│   ├── core/
│   │   ├── mod.rs
│   │   ├── document.rs      // Document struct
│   │   ├── page.rs          // Page struct (160×51 cells)
│   │   ├── region.rs        // Region tree structure
│   │   ├── cell.rs          // Cell (char + style)
│   │   └── style.rs         // Style struct
│   ├── widgets/
│   │   ├── mod.rs
│   │   ├── trait.rs         // Widget trait
│   │   ├── label.rs
│   │   ├── text_block.rs
│   │   ├── paragraph.rs
│   │   ├── box_widget.rs
│   │   ├── key_value.rs
│   │   └── table.rs
│   ├── renderer/
│   │   ├── mod.rs
│   │   ├── escp.rs          // ESC/P byte generation
│   │   └── state_machine.rs // Style state tracking
│   ├── error.rs             // LayoutError enum
│   └── utils/
│       ├── mod.rs
│       └── alignment.rs     // Text alignment helpers
├── examples/
│   ├── simple_invoice.rs
│   ├── multi_page.rs
│   ├── nested_regions.rs
│   ├── all_widgets.rs
│   └── custom_widget.rs
├── tests/
│   ├── integration/
│   ├── golden_masters/
│   └── hardware/
├── benches/
│   └── rendering.rs
├── fuzz/
│   └── fuzz_targets/
└── docs/
    ├── adr/
    └── guides/
```

---

## 18. Traceability Matrix

| Requirement ID | Description | Test Cases | Implementation Module | Status |
|----------------|-------------|------------|----------------------|--------|
| **FR-P1** | Page dimensions 160×51 | `TC-PAGE-DIM-001` | `core::page` | [ ] |
| **FR-P2** | Cell model (char + style) | `TC-CELL-001`, `TC-CELL-002` | `core::cell` | [ ] |
| **FR-P3** | Boundary clipping | `TC-CLIP-001` to `TC-CLIP-010` | `core::page` | [ ] |
| **FR-P4** | Page immutability | `TC-IMMUT-001` | `core::page` | [ ] |
| **FR-R1** | Region geometry | `TC-REGION-GEO-001` | `core::region` | [ ] |
| **FR-R2** | Local coordinates | `TC-REGION-COORD-001` | `core::region` | [ ] |
| **FR-R3** | Region clipping | `TC-REGION-CLIP-001` | `core::region` | [ ] |
| **FR-R4** | Nesting support | `TC-NEST-001` to `TC-NEST-010` | `core::region` | [ ] |
| **FR-R5** | Splitting API | `TC-SPLIT-H-001`, `TC-SPLIT-V-001`, `TC-GRID-001` | `builder::region` | [ ] |
| **FR-R6** | Padding | `TC-PADDING-001` | `builder::region` | [ ] |
| **FR-R7** | Default style | `TC-STYLE-DEFAULT-001` | `core::region` | [ ] |
| **FR-D1** | Document structure | `TC-DOC-001` | `core::document` | [ ] |
| **FR-D2** | Manual pagination | `TC-MANUAL-PAG-001` | `builder::document` | [ ] |
| **FR-D3** | Document immutability | `TC-DOC-IMMUT-001` | `core::document` | [ ] |
| **FR-D4** | ESC/P rendering | `TC-RENDER-001` to `TC-RENDER-020` | `renderer::escp` | [ ] |
| **FR-D5** | Page separation (FF) | `TC-FF-001` | `renderer::escp` | [ ] |
| **FR-W1** | Label widget | `TC-LABEL-001` to `TC-LABEL-005` | `widgets::label` | [ ] |
| **FR-W2** | TextBlock widget | `TC-TEXTBLOCK-001` | `widgets::text_block` | [ ] |
| **FR-W3** | Paragraph widget | `TC-PARA-001` to `TC-PARA-003` | `widgets::paragraph` | [ ] |
| **FR-W4** | Box widget | `TC-BOX-001` to `TC-BOX-003` | `widgets::box_widget` | [ ] |
| **FR-W5** | KeyValue widget | `TC-KV-001` | `widgets::key_value` | [ ] |
| **FR-W6** | Table widget | `TC-TABLE-001` to `TC-TABLE-010` | `widgets::table` | [ ] |
| **FR-S1** | Supported styles | `TC-STYLE-001` | `core::style` | [ ] |
| **FR-S2** | ESC/P command mapping | `TC-ESC-MAP-001` to `TC-ESC-MAP-005` | `renderer::escp` | [ ] |
| **FR-S3** | Style state machine | `TC-STATE-MACH-001` | `renderer::state_machine` | [ ] |
| **FR-T1** | Horizontal truncation | `TC-TRUNC-H-001` | `core::region` | [ ] |
| **FR-T2** | Vertical truncation | `TC-TRUNC-V-001` | `core::region` | [ ] |
| **FR-T3** | Page boundary truncation | `TC-TRUNC-PAGE-001` | `core::page` | [ ] |
| **FR-T4** | No error on overflow | `TC-NO-PANIC-001` | All modules | [ ] |
| **FR-E1** | Rendering order | `TC-RENDER-ORDER-001` | `renderer::escp` | [ ] |
| **FR-E2** | Deterministic output | `TC-DETERM-001` (1000 runs) | `renderer::escp` | [ ] |
| **FR-E3** | Character safety | `TC-ASCII-SAFE-001` | `renderer::escp` | [ ] |
| **FR-E4** | Text mode only | `TC-TEXT-MODE-001` | `renderer::escp` | [ ] |
| **FR-DEP1** | Zero runtime deps | `TC-DEPS-001` | `Cargo.toml` | [ ] |
| **FR-DEP3** | MSRV 1.75.0 | `TC-MSRV-001` | CI config | [ ] |
| **FR-DEP4** | Platform support | `TC-PLATFORM-001` to `TC-PLATFORM-003` | CI matrix | [ ] |
| **FR-ERR1** | Error type | `TC-ERROR-001` | `error` | [ ] |
| **FR-ERR2** | Result types | `TC-RESULT-001` | All builders | [ ] |
| **FR-ERR3** | Panic policy | `TC-FUZZ-001` (1M inputs) | All modules | [ ] |
| **FR-ERR5** | Validation | `TC-VALID-001` to `TC-VALID-005` | `builder::region` | [ ] |
| **NFR-1** | Determinism | `TC-DETERM-001` | `renderer::escp` | [ ] |
| **NFR-2** | Performance | `BENCH-RENDER-001` to `BENCH-RENDER-005` | Benchmarks | [ ] |
| **NFR-4** | Safety (no panics) | `TC-FUZZ-001` | All modules | [ ] |
| **NFR-5** | Portability | `TC-PLATFORM-001` to `TC-PLATFORM-003` | CI | [ ] |
| **NFR-6** | Thread safety | `TC-SEND-SYNC-001` | `core::document` | [ ] |

**Total Requirements**: 46 functional + 9 non-functional = **55 requirements**

---

## 19. Appendices

### 19.1 Glossary

| Term | Definition |
|------|------------|
| **Region** | Rectangular sub-area of a Page with defined (x, y, width, height) |
| **Widget** | Reusable content renderer (e.g., Label, Table) implementing the Widget trait |
| **ESC/P** | EPSON Standard Code for Printers - control language for dot-matrix printers |
| **Condensed Mode** | 12 CPI (characters per inch) producing 160 columns on LQ-2090II |
| **Truncation** | Silent drop of content that exceeds region or page boundaries |
| **Builder Pattern** | Creational design pattern for constructing complex objects step-by-step |
| **MSRV** | Minimum Supported Rust Version |
| **CPI** | Characters Per Inch - print density metric |
| **Form Feed (FF)** | ASCII control character `\x0C` signaling printer to advance to next page |
| **LQ-2090II** | EPSON wide-carriage 24-pin dot-matrix printer model |

---

### 19.2 ESC/P Command Reference

| Command | Hex Sequence | Purpose |
|---------|--------------|---------|
| **ESC @** | `\x1B\x40` | Reset printer to default state |
| **ESC E** | `\x1B\x45` | Enable bold mode |
| **ESC F** | `\x1B\x46` | Disable bold mode |
| **ESC - 1** | `\x1B\x2D\x01` | Enable underline |
| **ESC - 0** | `\x1B\x2D\x00` | Disable underline |
| **SI** | `\x0F` | Select condensed mode (12 CPI) |
| **CR** | `\x0D` | Carriage return |
| **LF** | `\x0A` | Line feed |
| **FF** | `\x0C` | Form feed (page eject) |

**Reference**: EPSON ESC/P Reference Manual (1997 Edition)

---

### 19.3 References

1. **EPSON LQ-2090II User Manual**: Hardware specifications and ESC/P command set
2. **EPSON ESC/P Reference Manual**: Complete command reference
3. **RFC 2119**: "Key words for use in RFCs to Indicate Requirement Levels"
   [https://www.ietf.org/rfc/rfc2119.txt](https://www.ietf.org/rfc/rfc2119.txt)
4. **SemVer 2.0.0**: Semantic Versioning specification
   [https://semver.org/](https://semver.org/)
5. **Rust API Guidelines**: Best practices for Rust library design
   [https://rust-lang.github.io/api-guidelines/](https://rust-lang.github.io/api-guidelines/)
6. **The Rust Programming Language**: Official Rust book
   [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)

---

### 19.4 Acronyms & Abbreviations

| Acronym | Expansion |
|---------|-----------|
| **ADR** | Architecture Decision Record |
| **API** | Application Programming Interface |
| **ASCII** | American Standard Code for Information Interchange |
| **CI/CD** | Continuous Integration / Continuous Deployment |
| **CPI** | Characters Per Inch |
| **CR** | Carriage Return |
| **ERP** | Enterprise Resource Planning |
| **ESC/P** | EPSON Standard Code for Printers |
| **FF** | Form Feed |
| **LF** | Line Feed |
| **LTO** | Link-Time Optimization |
| **MES** | Manufacturing Execution System |
| **MSRV** | Minimum Supported Rust Version |
| **NFR** | Non-Functional Requirement |
| **OOM** | Out Of Memory |
| **PRD** | Product Requirements Document |
| **QA** | Quality Assurance |
| **SemVer** | Semantic Versioning |
| **SI** | Shift In (ESC/P condensed mode command) |
| **SRE** | Site Reliability Engineering |
| **TDD** | Technical Design Document |
| **UTF-8** | Unicode Transformation Format - 8-bit |
| **WMS** | Warehouse Management System |

---

### 19.5 Change Log

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| **1.0** | 2025-01-XX | Mohammad AlMechkor | Initial PRD |
| **1.1** | 2025-01-18 | Mohammad AlMechkor | Enhanced with:<br>• Document control & approval chain<br>• Background & problem statement<br>• Detailed personas<br>• Dependency requirements (FR-DEP*)<br>• Error handling requirements (FR-ERR*)<br>• Comprehensive API specifications<br>• Testing strategy<br>• Security & safety requirements<br>• Versioning & compatibility policy<br>• Distribution & packaging<br>• Documentation requirements<br>• Enhanced NFRs<br>• Traceability matrix<br>• Expanded appendices |

---

## 🔒 End of Document

**Document Status**: ✅ Ready for Review
**Next Steps**:
1. Stakeholder review and approval signatures
2. Technical architecture review
3. QA test plan development
4. Implementation kick-off

---

**For questions or clarifications, contact:**
**Product Owner**: Mohammad AlMechkor
**Document Location**: `/Users/mohammadalmechkor/Projects/matrix/specs/PRD.md`

---

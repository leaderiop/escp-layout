🧪 TEST PLAN

# EPSON LQ-2090II Rust Layout Engine — V1

**Comprehensive Testing Strategy & Specifications**

---

## Document Control

| Field | Value |
|-------|-------|
| **Document Version** | 1.0 |
| **Product Version** | V1.0 |
| **Author** | Mohammad AlMechkor |
| **Status** | Draft |
| **Classification** | Internal - QA & Engineering |
| **Date Created** | 2025-01-18 |
| **Last Updated** | 2025-01-18 |

### Related Documents

- **Product Requirements Document (PRD)**: `PRD.md` (v1.1)
- **Technical Design Document (TDD)**: `TDD.md` (v1.0)
- **API Specification**: `API-SPEC.md` (v1.0)

### Test Team

| Role | Name | Responsibilities |
|------|------|------------------|
| **QA Lead** | [TBD] | Test strategy, approval |
| **Test Engineer** | [TBD] | Test execution, reporting |
| **Automation Engineer** | [TBD] | CI/CD integration |
| **Hardware Tester** | [TBD] | Physical printer validation |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Test Strategy](#2-test-strategy)
3. [Test Levels](#3-test-levels)
4. [Unit Tests](#4-unit-tests)
5. [Integration Tests](#5-integration-tests)
6. [Property-Based Tests](#6-property-based-tests)
7. [Hardware Validation Tests](#7-hardware-validation-tests)
8. [Golden Master Tests](#8-golden-master-tests)
9. [Performance Benchmarks](#9-performance-benchmarks)
10. [Security & Fuzzing Tests](#10-security--fuzzing-tests)
11. [Test Environment](#11-test-environment)
12. [Test Data & Fixtures](#12-test-data--fixtures)
13. [Traceability Matrix](#13-traceability-matrix)
14. [Test Schedule](#14-test-schedule)
15. [Acceptance Criteria](#15-acceptance-criteria)
16. [Appendices](#16-appendices)

---

## 1. Introduction

### 1.1 Purpose

This Test Plan defines the comprehensive testing strategy for the EPSON LQ-2090II Rust Layout Engine V1. It maps all PRD requirements to specific, executable test cases.

### 1.2 Scope

**In Scope**:
- Unit tests (≥90% coverage)
- Integration tests (end-to-end workflows)
- Property-based tests (determinism, truncation)
- Hardware validation (physical printer)
- Performance benchmarks
- Golden master tests (byte-level validation)
- Security fuzzing

**Out of Scope**:
- Manual exploratory testing (covered separately)
- Load testing (not applicable for V1)
- UI testing (no UI)

### 1.3 Test Objectives

1. **Correctness**: Verify all functional requirements (46 FR-*)
2. **Determinism**: Ensure 100% reproducible output
3. **Safety**: Zero panics under any valid input
4. **Performance**: Meet all NFR targets (< 100μs per page)
5. **Hardware Compliance**: Print correctly on EPSON LQ-2090II

### 1.4 Test Metrics

**Coverage Targets**:
- Line coverage: ≥ 90%
- Branch coverage: ≥ 85%
- Public API coverage: 100%

**Quality Gates**:
- Zero critical bugs
- Zero panics in fuzzing (1M+ iterations)
- All hardware tests pass visual inspection

---

## 2. Test Strategy

### 2.1 Test Pyramid

```
                 ┌─────────────┐
                 │  Hardware   │  10 tests
                 │  Validation │
                 └─────────────┘
              ┌─────────────────────┐
              │   Integration      │  50+ tests
              │     Tests          │
              └─────────────────────┘
          ┌─────────────────────────────┐
          │    Property-Based Tests     │  20+ properties
          │  (Fuzzing, Determinism)     │
          └─────────────────────────────┘
      ┌─────────────────────────────────────┐
      │           Unit Tests                │  200+ tests
      │    (Core types, algorithms)         │
      └─────────────────────────────────────┘
```

### 2.2 Test Types

| Type | Purpose | Tool | Frequency |
|------|---------|------|-----------|
| **Unit** | Test individual functions/methods | `cargo test` | Every commit |
| **Integration** | Test end-to-end workflows | `cargo test --test` | Every PR |
| **Property** | Test invariants with random inputs | `proptest` | Every PR |
| **Hardware** | Validate printer output | Manual | Weekly |
| **Golden Master** | Byte-level output validation | `cargo test` | Every PR |
| **Benchmark** | Performance regression detection | `criterion` | Every PR |
| **Fuzzing** | Find panics/crashes | `cargo-fuzz` | Nightly |

### 2.3 Test Approach

**Bottom-Up**:
1. Start with unit tests for core types (Cell, Page)
2. Build up to integration tests (document rendering)
3. Finish with hardware validation

**TDD (Test-Driven Development)**:
- Write tests before implementation
- Red → Green → Refactor cycle
- All public APIs get tests first

**Continuous Testing**:
- Automated in CI/CD pipeline
- Fast feedback on every commit
- Block merges on test failures

---

## 3. Test Levels

### 3.1 Level 0: Smoke Tests

**Purpose**: Quick sanity check that basic functionality works.

**Duration**: < 10 seconds

**Test Cases**:
- `TC-SMOKE-001`: Can create a page
- `TC-SMOKE-002`: Can write a cell
- `TC-SMOKE-003`: Can render a document

**Run**: On every commit, before full suite.

---

### 3.2 Level 1: Unit Tests

**Purpose**: Test individual components in isolation.

**Scope**: All public and critical private methods.

**Target**: 200+ test cases, ≥90% coverage.

---

### 3.3 Level 2: Integration Tests

**Purpose**: Test component interactions and workflows.

**Scope**: Builder API, widget rendering, multi-page documents.

**Target**: 50+ test cases.

---

### 3.4 Level 3: System Tests

**Purpose**: End-to-end validation including hardware.

**Scope**: Complete document creation and printing.

**Target**: 10+ hardware tests.

---

## 4. Unit Tests

### 4.1 Cell Tests

#### TC-CELL-001: Create Empty Cell

**Requirement**: FR-P2

**Test**:
```rust
#[test]
fn test_cell_empty() {
    let cell = Cell::empty();
    assert_eq!(cell.character(), ' ');
    assert_eq!(cell.style(), Style::NORMAL);
}
```

**Expected**: Empty cell has space character and normal style.

---

#### TC-CELL-002: Create Cell with Character and Style

**Requirement**: FR-P2

**Test**:
```rust
#[test]
fn test_cell_new() {
    let cell = Cell::new('A', Style::BOLD);
    assert_eq!(cell.character(), 'A');
    assert_eq!(cell.style(), Style::BOLD);
}
```

**Expected**: Cell stores character and style correctly.

---

#### TC-CELL-003: Non-ASCII Character Replacement

**Requirement**: FR-E3

**Test**:
```rust
#[test]
fn test_cell_non_ascii_replacement() {
    let cell = Cell::new('🦀', Style::NORMAL);
    assert_eq!(cell.character(), '?');
}
```

**Expected**: Non-ASCII characters replaced with '?'.

---

#### TC-CELL-004: Cell Size

**Requirement**: Memory efficiency (NFR-3)

**Test**:
```rust
#[test]
fn test_cell_size() {
    use std::mem::size_of;
    assert_eq!(size_of::<Cell>(), 2);
}
```

**Expected**: Cell is exactly 2 bytes.

---

### 4.2 Style Tests

#### TC-STYLE-001: Style Constants

**Requirement**: FR-S1

**Test**:
```rust
#[test]
fn test_style_constants() {
    assert_eq!(Style::NORMAL, Style { bold: false, underline: false });
    assert_eq!(Style::BOLD, Style { bold: true, underline: false });
    assert_eq!(Style::UNDERLINE, Style { bold: false, underline: true });
    assert_eq!(Style::BOLD_UNDERLINE, Style { bold: true, underline: true });
}
```

**Expected**: All style constants are correct.

---

#### TC-STYLE-002: StyleBits Packing

**Requirement**: Memory efficiency

**Test**:
```rust
#[test]
fn test_style_bits_packing() {
    let bits = StyleBits::new(true, false);
    assert!(bits.is_bold());
    assert!(!bits.is_underline());

    let bits = StyleBits::new(true, true);
    assert!(bits.is_bold());
    assert!(bits.is_underline());
}
```

**Expected**: Bit packing works correctly.

---

### 4.3 Page Tests

#### TC-PAGE-DIM-001: Page Dimensions

**Requirement**: FR-P1

**Test**:
```rust
#[test]
fn test_page_dimensions() {
    let page = Page::new();
    assert_eq!(Page::WIDTH, 160);
    assert_eq!(Page::HEIGHT, 51);
}
```

**Expected**: Page is exactly 160×51.

---

#### TC-PAGE-002: Write Cell In Bounds

**Requirement**: FR-P2

**Test**:
```rust
#[test]
fn test_page_write_cell_in_bounds() {
    let mut page = Page::new();
    page.write_cell(0, 0, 'A', Style::BOLD);

    let cell = page.get_cell(0, 0).unwrap();
    assert_eq!(cell.character(), 'A');
    assert_eq!(cell.style(), Style::BOLD);
}
```

**Expected**: Cell written successfully.

---

#### TC-PAGE-003: Write Cell Out of Bounds

**Requirement**: FR-P3

**Test**:
```rust
#[test]
fn test_page_write_cell_out_of_bounds() {
    let mut page = Page::new();
    page.write_cell(200, 100, 'X', Style::NORMAL);
    // Should not panic
}
```

**Expected**: No panic, silent clipping.

---

#### TC-PAGE-004: Write Text

**Requirement**: FR-P2

**Test**:
```rust
#[test]
fn test_page_write_text() {
    let mut page = Page::new();
    page.write_text(0, 0, "Hello", Style::NORMAL);

    assert_eq!(page.get_cell(0, 0).unwrap().character(), 'H');
    assert_eq!(page.get_cell(1, 0).unwrap().character(), 'e');
    assert_eq!(page.get_cell(2, 0).unwrap().character(), 'l');
    assert_eq!(page.get_cell(3, 0).unwrap().character(), 'l');
    assert_eq!(page.get_cell(4, 0).unwrap().character(), 'o');
}
```

**Expected**: Text written character by character.

---

#### TC-PAGE-005: Horizontal Truncation

**Requirement**: FR-T1

**Test**:
```rust
#[test]
fn test_page_horizontal_truncation() {
    let mut page = Page::new();
    let long_text = "A".repeat(200);
    page.write_text(0, 0, &long_text, Style::NORMAL);

    // Should truncate at column 160
    assert_eq!(page.get_cell(159, 0).unwrap().character(), 'A');
    assert_eq!(page.get_cell(0, 0).unwrap().character(), 'A');

    // Cell at (160, 0) should be empty (default)
    // Since we only go up to 159 (0-indexed)
}
```

**Expected**: Text truncated at page width.

---

### 4.4 Region Tests

#### TC-REGION-GEO-001: Region Geometry

**Requirement**: FR-R1

**Test**:
```rust
#[test]
fn test_region_geometry() {
    let region = Region::new(10, 5, 50, 20).unwrap();
    assert_eq!(region.x, 10);
    assert_eq!(region.y, 5);
    assert_eq!(region.width, 50);
    assert_eq!(region.height, 20);
}
```

**Expected**: Region stores geometry correctly.

---

#### TC-REGION-002: Invalid Dimensions

**Requirement**: FR-ERR5

**Test**:
```rust
#[test]
fn test_region_invalid_dimensions() {
    let result = Region::new(0, 0, 0, 0);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        LayoutError::InvalidDimensions { width: 0, height: 0 }
    );
}
```

**Expected**: Returns error for zero dimensions.

---

#### TC-SPLIT-H-001: Horizontal Split

**Requirement**: FR-R5

**Test**:
```rust
#[test]
fn test_region_split_horizontal() {
    let mut region = Region::new(0, 0, 100, 50).unwrap();
    let children = region.split_horizontal(&[1, 2, 1]).unwrap();

    assert_eq!(children.len(), 3);
    assert_eq!(children[0].width, 25);  // 25%
    assert_eq!(children[1].width, 50);  // 50%
    assert_eq!(children[2].width, 25);  // 25%
}
```

**Expected**: Region split into correct proportions.

---

#### TC-SPLIT-V-001: Vertical Split

**Requirement**: FR-R5

**Test**:
```rust
#[test]
fn test_region_split_vertical() {
    let mut region = Region::new(0, 0, 160, 51).unwrap();
    let children = region.split_vertical(&[10, 35, 6]).unwrap();

    assert_eq!(children.len(), 3);
    assert_eq!(children[0].height, 10);
    assert_eq!(children[1].height, 35);
    assert_eq!(children[2].height, 6);
}
```

**Expected**: Region split vertically.

---

#### TC-SPLIT-002: Empty Ratios Error

**Requirement**: FR-ERR5

**Test**:
```rust
#[test]
fn test_region_split_empty_ratios() {
    let mut region = Region::new(0, 0, 100, 50).unwrap();
    let result = region.split_horizontal(&[]);

    assert!(result.is_err());
    match result.unwrap_err() {
        LayoutError::InvalidSplitRatios { provided, expected } => {
            assert_eq!(provided, 0);
            assert_eq!(expected, 1);
        }
        _ => panic!("Wrong error type"),
    }
}
```

**Expected**: Returns InvalidSplitRatios error.

---

#### TC-PADDING-001: Padding

**Requirement**: FR-R6

**Test**:
```rust
#[test]
fn test_region_padding() {
    let mut region = Region::new(0, 0, 100, 50).unwrap();
    region.with_padding(2, 2, 2, 2);

    let (inner_x, inner_y, inner_width, inner_height) = region.inner_bounds();
    assert_eq!(inner_x, 2);
    assert_eq!(inner_y, 2);
    assert_eq!(inner_width, 96);  // 100 - 2 - 2
    assert_eq!(inner_height, 46);  // 50 - 2 - 2
}
```

**Expected**: Padding reduces inner area.

---

### 4.5 Document Tests

#### TC-DOC-001: Document Structure

**Requirement**: FR-D1

**Test**:
```rust
#[test]
fn test_document_structure() {
    let mut builder = DocumentBuilder::new();
    builder.add_page();
    builder.add_page();
    builder.add_page();

    let doc = builder.build();
    assert_eq!(doc.page_count(), 3);
}
```

**Expected**: Document contains 3 pages.

---

#### TC-DOC-IMMUT-001: Document Immutability

**Requirement**: FR-D3

**Test**:
```rust
#[test]
fn test_document_immutability() {
    let mut builder = DocumentBuilder::new();
    builder.add_page();
    let doc = builder.build();

    // This should compile (immutable reference is OK)
    let _count = doc.page_count();

    // This should NOT compile (caught by borrow checker):
    // doc.add_page(); // Error: no such method
}
```

**Expected**: Document is immutable after build().

---

### 4.6 Renderer Tests

#### TC-RENDER-ORDER-001: ESC/P Command Order

**Requirement**: FR-E1

**Test**:
```rust
#[test]
fn test_escp_command_order() {
    let mut builder = DocumentBuilder::new();
    let mut page = builder.add_page();
    page.root_region().write_text(0, 0, "Test", Style::NORMAL);
    page.finalize().unwrap();

    let doc = builder.build();
    let bytes = doc.render();

    // Check start of output
    assert_eq!(bytes[0], 0x1B); // ESC
    assert_eq!(bytes[1], 0x40); // @
    assert_eq!(bytes[2], 0x0F); // SI (condensed mode)
}
```

**Expected**: Correct ESC/P command sequence.

---

#### TC-ESC-MAP-001: Bold ON

**Requirement**: FR-S2

**Test**:
```rust
#[test]
fn test_escp_bold_on() {
    let mut state = StyleStateMachine::new();
    let cmds = state.transition_to(StyleBits::BOLD);

    assert_eq!(cmds, vec![0x1B, 0x45]); // ESC E
}
```

**Expected**: Emits ESC E for bold.

---

#### TC-ESC-MAP-002: Bold OFF

**Requirement**: FR-S2

**Test**:
```rust
#[test]
fn test_escp_bold_off() {
    let mut state = StyleStateMachine::new();
    state.transition_to(StyleBits::BOLD);
    let cmds = state.transition_to(StyleBits::NORMAL);

    assert_eq!(cmds, vec![0x1B, 0x46]); // ESC F
}
```

**Expected**: Emits ESC F to disable bold.

---

#### TC-STATE-MACH-001: State Machine Optimization

**Requirement**: FR-S3

**Test**:
```rust
#[test]
fn test_style_state_machine_optimization() {
    let mut state = StyleStateMachine::new();

    // No transition needed (already normal)
    let cmds = state.transition_to(StyleBits::NORMAL);
    assert!(cmds.is_empty());

    // Transition to bold
    let cmds = state.transition_to(StyleBits::BOLD);
    assert_eq!(cmds.len(), 2); // ESC E

    // No transition (already bold)
    let cmds = state.transition_to(StyleBits::BOLD);
    assert!(cmds.is_empty());
}
```

**Expected**: Only emits commands when style changes.

---

#### TC-ASCII-SAFE-001: Non-ASCII Replacement

**Requirement**: FR-E3

**Test**:
```rust
#[test]
fn test_non_ascii_replacement() {
    let mut builder = DocumentBuilder::new();
    let mut page = builder.add_page();
    page.root_region().write_text(0, 0, "Hello 🦀 World", Style::NORMAL);
    page.finalize().unwrap();

    let doc = builder.build();
    let bytes = doc.render();

    // Verify '🦀' was replaced with '?'
    let output = String::from_utf8_lossy(&bytes);
    assert!(output.contains("Hello ? World"));
}
```

**Expected**: Emoji replaced with '?'.

---

#### TC-FF-001: Form Feed

**Requirement**: FR-D5

**Test**:
```rust
#[test]
fn test_form_feed_between_pages() {
    let mut builder = DocumentBuilder::new();
    builder.add_page();
    builder.add_page();

    let doc = builder.build();
    let bytes = doc.render();

    // Count form feeds (0x0C)
    let ff_count = bytes.iter().filter(|&&b| b == 0x0C).count();
    assert_eq!(ff_count, 2); // One per page
}
```

**Expected**: Form feed after each page.

---

### 4.7 Widget Tests

#### TC-LABEL-001: Label Left Alignment

**Requirement**: FR-W1

**Test**:
```rust
#[test]
fn test_label_left_alignment() {
    let label = Label::new("Test", Alignment::Left);
    let mut page = Page::new();
    let mut region = /* create region */;

    label.render(&mut region).unwrap();

    // Check first cells
    assert_eq!(page.get_cell(0, 0).unwrap().character(), 'T');
    assert_eq!(page.get_cell(1, 0).unwrap().character(), 'e');
}
```

**Expected**: Text aligned to left.

---

#### TC-LABEL-002: Label Center Alignment

**Requirement**: FR-W1

**Test**:
```rust
#[test]
fn test_label_center_alignment() {
    let label = Label::new("Test", Alignment::Center);
    // Region width = 20, text length = 4
    // Center position = (20 - 4) / 2 = 8

    label.render(&mut region).unwrap();

    // Text should start at column 8
    assert_eq!(page.get_cell(8, 0).unwrap().character(), 'T');
}
```

**Expected**: Text centered in region.

---

#### TC-TABLE-001: Table Header

**Requirement**: FR-W6

**Test**:
```rust
#[test]
fn test_table_header() {
    let table = Table::new(vec![30, 30])
        .with_headers(vec!["Col1".into(), "Col2".into()]);

    table.render(&mut region).unwrap();

    // Check header is rendered with bold style
    assert_eq!(page.get_cell(0, 0).unwrap().character(), 'C');
    assert_eq!(page.get_cell(0, 0).unwrap().style(), Style::BOLD);
}
```

**Expected**: Header rendered with bold.

---

#### TC-TABLE-002: Table Row Truncation

**Requirement**: FR-T2

**Test**:
```rust
#[test]
fn test_table_vertical_truncation() {
    let mut table = Table::new(vec![30]);

    // Add 100 rows to a region with height 20
    for i in 0..100 {
        table.add_row(vec![format!("Row {}", i)]);
    }

    // Region height = 20 (1 header + 1 separator + 18 data rows)
    table.render(&mut region).unwrap();

    // Should not panic, rows beyond height silently discarded
}
```

**Expected**: Vertical truncation, no panic.

---

## 5. Integration Tests

### 5.1 End-to-End Workflow Tests

#### TC-E2E-001: Simple Invoice

**Requirements**: Multiple (FR-P*, FR-R*, FR-W*, FR-E*)

**Test**:
```rust
#[test]
fn test_simple_invoice_workflow() {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();
    let mut root = page.root_region();

    // Header
    root.write_text(0, 0, "INVOICE", Style::BOLD);

    // Body with table
    let mut table = Table::new(vec![30, 30, 30]);
    table.with_headers(vec!["Item".into(), "Qty".into(), "Price".into()]);
    table.add_row(vec!["Widget".into(), "5".into(), "$10.00".into()]);

    let mut body = root.child_region(0, 5, 160, 40).unwrap();
    body.add_widget(table).unwrap();

    // Footer
    root.write_text(0, 50, "Thank you", Style::NORMAL);

    page.finalize().unwrap();
    let document = doc.build();
    let bytes = document.render();

    // Verify output is not empty
    assert!(!bytes.is_empty());
    assert!(bytes.len() > 100);
}
```

**Expected**: Complete invoice rendered successfully.

---

#### TC-E2E-002: Multi-Page Document

**Requirements**: FR-D1, FR-D2, FR-D5

**Test**:
```rust
#[test]
fn test_multi_page_document() {
    let mut doc = DocumentBuilder::new();

    for i in 1..=5 {
        let mut page = doc.add_page();
        page.root_region().write_text(
            0,
            0,
            &format!("Page {}", i),
            Style::BOLD
        );
        page.finalize().unwrap();
    }

    let document = doc.build();
    assert_eq!(document.page_count(), 5);

    let bytes = document.render();

    // Verify 5 form feeds (one per page)
    let ff_count = bytes.iter().filter(|&&b| b == 0x0C).count();
    assert_eq!(ff_count, 5);
}
```

**Expected**: 5 pages with form feeds.

---

#### TC-E2E-003: Nested Regions

**Requirements**: FR-R4, FR-R5

**Test**:
```rust
#[test]
fn test_nested_regions() {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();
    let mut root = page.root_region();

    // Level 1: Split into 3 rows
    let mut rows = root.split_vertical(&[1, 3, 1]).unwrap();

    // Level 2: Split middle row into 3 columns
    let mut cols = rows[1].split_horizontal(&[1, 2, 1]).unwrap();

    // Level 3: Write to nested region
    cols[1].write_text(0, 0, "Nested", Style::NORMAL);

    page.finalize().unwrap();
    let document = doc.build();
    let bytes = document.render();

    assert!(!bytes.is_empty());
}
```

**Expected**: Deeply nested layout works.

---

#### TC-E2E-004: All Widgets

**Requirements**: FR-W1 through FR-W6

**Test**:
```rust
#[test]
fn test_all_widgets() {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();
    let mut root = page.root_region();

    let mut sections = root.split_vertical(&[5, 5, 5, 10, 10, 10]).unwrap();

    // Label
    sections[0].label("Label Widget", Alignment::Center);

    // TextBlock
    let block = TextBlock::new("Line 1\nLine 2");
    sections[1].add_widget(block).unwrap();

    // Paragraph
    let para = Paragraph::new("This is a paragraph with wrapping.");
    sections[2].add_widget(para).unwrap();

    // Box
    let box_widget = Box::new().with_title("Box");
    sections[3].add_widget(box_widget).unwrap();

    // KeyValue
    let mut kv = KeyValue::new(15);
    kv.add_pair("Key", "Value");
    sections[4].add_widget(kv).unwrap();

    // Table
    let mut table = Table::new(vec![30, 30]);
    table.with_headers(vec!["H1".into(), "H2".into()]);
    sections[5].add_widget(table).unwrap();

    page.finalize().unwrap();
    let document = doc.build();
    let bytes = document.render();

    assert!(!bytes.is_empty());
}
```

**Expected**: All 6 widgets render without errors.

---

### 5.2 Builder API Tests

#### TC-BUILDER-001: Builder Chaining

**Requirements**: API ergonomics

**Test**:
```rust
#[test]
fn test_builder_chaining() {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();

    // Should be able to chain calls
    page.root_region()
        .write_text(0, 0, "Line 1", Style::NORMAL)
        .write_text(0, 1, "Line 2", Style::BOLD)
        .write_text(0, 2, "Line 3", Style::UNDERLINE);

    page.finalize().unwrap();
}
```

**Expected**: Method chaining works.

---

#### TC-BUILDER-002: Lifetime Safety

**Requirements**: FR-ERR2

**Test**:
```rust
// This test verifies compile-time safety
// It should NOT compile (caught by borrow checker)

#[test]
#[should_not_compile]
fn test_builder_lifetime_safety() {
    let mut doc = DocumentBuilder::new();
    let page = doc.add_page();

    // This should fail: doc borrowed by page
    let document = doc.build();
}
```

**Expected**: Compile error (lifetime violation).

---

## 6. Property-Based Tests

### 6.1 Determinism Tests

#### TC-PROP-DETERM-001: Rendering Determinism

**Requirement**: NFR-1

**Test**:
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_deterministic_rendering(
        text in ".*",
        x in 0u16..160,
        y in 0u16..51,
    ) {
        let mut builder1 = DocumentBuilder::new();
        let mut page1 = builder1.add_page();
        page1.root_region().write_text(x, y, &text, Style::NORMAL);
        page1.finalize().unwrap();
        let doc1 = builder1.build();

        let mut builder2 = DocumentBuilder::new();
        let mut page2 = builder2.add_page();
        page2.root_region().write_text(x, y, &text, Style::NORMAL);
        page2.finalize().unwrap();
        let doc2 = builder2.build();

        // Same input should produce identical output
        assert_eq!(doc1.render(), doc2.render());
    }
}
```

**Expected**: 100% determinism across all inputs.

---

### 6.2 No-Panic Tests

#### TC-PROP-PANIC-001: Write Never Panics

**Requirement**: NFR-4

**Test**:
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
        // Should never panic
    }
}
```

**Expected**: Zero panics for any coordinates.

---

#### TC-PROP-PANIC-002: Region Split Never Panics

**Requirement**: NFR-4

**Test**:
```rust
proptest! {
    #[test]
    fn test_split_never_panics(
        ratios in prop::collection::vec(1u16..100, 0..20)
    ) {
        let mut region = Region::new(0, 0, 160, 51).unwrap();

        // This might return an error, but should never panic
        let _ = region.split_horizontal(&ratios);
    }
}
```

**Expected**: Returns `Err` for invalid input, never panics.

---

### 6.3 Truncation Tests

#### TC-PROP-TRUNC-001: Horizontal Truncation

**Requirement**: FR-T1

**Test**:
```rust
proptest! {
    #[test]
    fn test_horizontal_truncation(
        text_len in 0usize..1000,
    ) {
        let text = "A".repeat(text_len);
        let mut page = Page::new();
        page.write_text(0, 0, &text, Style::NORMAL);

        // Count non-empty cells in first row
        let mut count = 0;
        for x in 0..Page::WIDTH {
            if page.get_cell(x as u16, 0).unwrap().character() != ' ' {
                count += 1;
            }
        }

        // Should be truncated at page width
        assert!(count <= Page::WIDTH);
    }
}
```

**Expected**: Never exceed page width.

---

### 6.4 Split Correctness

#### TC-PROP-SPLIT-001: Split Widths Sum to Total

**Requirement**: FR-R5

**Test**:
```rust
proptest! {
    #[test]
    fn test_split_widths_sum(
        ratios in prop::collection::vec(1u16..10, 1..10)
    ) {
        let mut region = Region::new(0, 0, 100, 50).unwrap();
        let children = region.split_horizontal(&ratios).unwrap();

        let total_width: u16 = children.iter().map(|r| r.width).sum();
        assert_eq!(total_width, 100);
    }
}
```

**Expected**: Child widths sum to parent width.

---

## 7. Hardware Validation Tests

### 7.1 Test Environment

**Hardware**:
- EPSON LQ-2090II (primary)
- EPSON LQ-590 (secondary, for compatibility testing)

**Connection**:
- USB to parallel adapter
- Direct parallel port (if available)

**Paper**:
- Standard 8.5" × 11" continuous form
- Multi-part carbonless forms (3-part)

---

### 7.2 Hardware Test Cases

#### TC-HW-001: Simple Text Alignment

**Requirement**: FR-P1, FR-E1

**Procedure**:
1. Generate document with grid pattern:
   ```
   0         1         2         3         4         5
   0123456789012345678901234567890123456789012345678901...
   ```
2. Print on EPSON LQ-2090II
3. Measure character positions with ruler

**Pass Criteria**:
- Characters align to 12 CPI (condensed mode)
- 160 characters fit on one line
- No horizontal drift

---

#### TC-HW-002: Vertical Spacing

**Requirement**: FR-P1

**Procedure**:
1. Generate document with 51 numbered lines
2. Print on EPSON LQ-2090II
3. Count lines on paper

**Pass Criteria**:
- Exactly 51 lines printed
- Line spacing is consistent (6 LPI)
- No vertical drift

---

#### TC-HW-003: Bold Formatting

**Requirement**: FR-S2

**Procedure**:
1. Generate document with alternating bold/normal lines
2. Print and visually inspect

**Pass Criteria**:
- Bold lines are visibly darker
- Bold does not affect character width
- No artifacts or smudging

---

#### TC-HW-004: Underline Formatting

**Requirement**: FR-S2

**Procedure**:
1. Generate document with underlined text
2. Print and inspect

**Pass Criteria**:
- Underline is visible under text
- Underline does not overlap with text
- Consistent underline across line

---

#### TC-HW-005: Multi-Page Form Feed

**Requirement**: FR-D5

**Procedure**:
1. Generate 5-page document
2. Print continuously
3. Observe page breaks

**Pass Criteria**:
- Clean page breaks (form feed works)
- No content spanning pages
- Consistent top margin on each page

---

#### TC-HW-006: Invoice Test Form

**Requirement**: End-to-end

**Procedure**:
1. Generate complete invoice (header, table, footer)
2. Print on EPSON LQ-2090II
3. Visual inspection

**Pass Criteria**:
- Header aligned and readable
- Table columns aligned
- Footer positioned correctly
- No text truncation or overlap

**Test Data**: See Appendix A - Invoice Test Data

---

#### TC-HW-007: Table Alignment

**Requirement**: FR-W6

**Procedure**:
1. Generate document with multi-column table
2. Print on EPSON LQ-2090II
3. Measure column positions

**Pass Criteria**:
- Columns aligned vertically
- Column widths match specification
- Headers align with data

---

#### TC-HW-008: Multi-Part Carbon Forms

**Requirement**: Print quality

**Procedure**:
1. Load 3-part carbonless form
2. Print invoice test form
3. Inspect all 3 copies

**Pass Criteria**:
- All 3 copies are readable
- Text darkness is sufficient on copy 3
- Alignment consistent across copies

---

#### TC-HW-009: Nested Region Layout

**Requirement**: FR-R4

**Procedure**:
1. Generate complex nested layout (sidebar + grid)
2. Print and inspect

**Pass Criteria**:
- All regions positioned correctly
- No content overlap
- Borders and separators aligned

---

#### TC-HW-010: Stress Test (100 Pages)

**Requirement**: Performance, reliability

**Procedure**:
1. Generate 100-page document
2. Print continuously
3. Monitor printer and output

**Pass Criteria**:
- All 100 pages print successfully
- No printer errors or jams
- Consistent quality from page 1 to 100

---

## 8. Golden Master Tests

### 8.1 Approach

**Golden Master Testing**: Store expected ESC/P byte output for known inputs. Compare actual output byte-by-byte.

**Storage**: `/tests/golden_masters/*.escp`

**Verification**: SHA-256 hash comparison

---

### 8.2 Golden Master Test Cases

#### TC-GOLDEN-001: Empty Page

**Input**: Empty page (all spaces)

**Expected Output**: `golden_masters/empty_page.escp`

**Test**:
```rust
#[test]
fn test_golden_empty_page() {
    let mut doc = DocumentBuilder::new();
    doc.add_page();
    let document = doc.build();
    let bytes = document.render();

    let expected = include_bytes!("golden_masters/empty_page.escp");
    assert_eq!(bytes, expected);
}
```

---

#### TC-GOLDEN-002: Single Line

**Input**: One line of text at (0, 0)

**Expected Output**: `golden_masters/single_line.escp`

---

#### TC-GOLDEN-003: Bold Text

**Input**: Bold text

**Expected Output**: `golden_masters/bold_text.escp`

**Verify**: Contains `ESC E` before text, `ESC F` after

---

#### TC-GOLDEN-004: Simple Invoice

**Input**: Standard invoice layout

**Expected Output**: `golden_masters/simple_invoice.escp`

---

#### TC-GOLDEN-005: Multi-Page (3 pages)

**Input**: 3 pages with different content

**Expected Output**: `golden_masters/three_pages.escp`

**Verify**: Contains 3 form feeds

---

### 8.3 Generating Golden Masters

**Process**:
1. Run test with known-good implementation
2. Manually verify output on hardware
3. Save output as golden master
4. Commit to repository

**Command**:
```bash
cargo test test_golden_simple_invoice -- --nocapture > golden_masters/simple_invoice.escp
```

---

## 9. Performance Benchmarks

### 9.1 Benchmark Suite

**Tool**: `criterion`

**Location**: `/benches/rendering.rs`

---

### 9.2 Benchmark Cases

#### BENCH-001: Page Creation

**Target**: < 10 μs (p99)

**Benchmark**:
```rust
fn bench_page_creation(c: &mut Criterion) {
    c.bench_function("Page::new()", |b| {
        b.iter(|| {
            black_box(Page::new())
        });
    });
}
```

---

#### BENCH-002: Write Cell

**Target**: < 50 ns (p99)

**Benchmark**:
```rust
fn bench_write_cell(c: &mut Criterion) {
    let mut page = Page::new();

    c.bench_function("Page::write_cell()", |b| {
        b.iter(|| {
            page.write_cell(
                black_box(10),
                black_box(10),
                black_box('A'),
                black_box(Style::NORMAL)
            );
        });
    });
}
```

---

#### BENCH-003: Single Page Render

**Target**: < 100 μs (p99)

**Benchmark**:
```rust
fn bench_single_page_render(c: &mut Criterion) {
    let mut doc = DocumentBuilder::new();
    let mut page = doc.add_page();
    page.root_region().write_text(0, 0, "Test", Style::NORMAL);
    page.finalize().unwrap();
    let document = doc.build();

    c.bench_function("Document::render() [1 page]", |b| {
        b.iter(|| {
            black_box(document.render())
        });
    });
}
```

---

#### BENCH-004: 100-Page Render

**Target**: < 10 ms (p99)

**Benchmark**:
```rust
fn bench_hundred_page_render(c: &mut Criterion) {
    let mut doc = DocumentBuilder::new();
    for _ in 0..100 {
        let mut page = doc.add_page();
        page.root_region().write_text(0, 0, "Page content", Style::NORMAL);
        page.finalize().unwrap();
    }
    let document = doc.build();

    c.bench_function("Document::render() [100 pages]", |b| {
        b.iter(|| {
            black_box(document.render())
        });
    });
}
```

---

#### BENCH-005: Region Split

**Target**: < 1 μs (p99)

**Benchmark**:
```rust
fn bench_region_split(c: &mut Criterion) {
    let mut region = Region::new(0, 0, 160, 51).unwrap();

    c.bench_function("Region::split_horizontal()", |b| {
        b.iter(|| {
            let _ = region.split_horizontal(black_box(&[1, 2, 1]));
        });
    });
}
```

---

#### BENCH-006: Table Rendering

**Benchmark**:
```rust
fn bench_table_render(c: &mut Criterion) {
    let mut table = Table::new(vec![30, 30, 30]);
    table.with_headers(vec!["H1".into(), "H2".into(), "H3".into()]);

    for i in 0..50 {
        table.add_row(vec![
            format!("Row {}", i),
            "Data".into(),
            "Value".into(),
        ]);
    }

    c.bench_function("Table::render() [50 rows]", |b| {
        b.iter(|| {
            let mut doc = DocumentBuilder::new();
            let mut page = doc.add_page();
            page.root_region().add_widget(table.clone()).unwrap();
        });
    });
}
```

---

### 9.3 Benchmark Execution

**Run benchmarks**:
```bash
cargo bench
```

**View results**:
```bash
open target/criterion/report/index.html
```

**CI Integration**:
- Run on every PR
- Fail if regression > 10%

---

## 10. Security & Fuzzing Tests

### 10.1 Fuzzing Strategy

**Tool**: `cargo-fuzz`

**Duration**: Continuous (nightly builds)

**Target**: 1M+ iterations per target

---

### 10.2 Fuzz Targets

#### FUZZ-001: Region Geometry

**Target**: `fuzz_region_geometry`

**Input**: Arbitrary (x, y, width, height)

**Objective**: No panics, no OOM

**Code**:
```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }

    let x = u16::from_le_bytes([data[0], data[1]]);
    let y = u16::from_le_bytes([data[2], data[3]]);
    let width = u16::from_le_bytes([data[4], data[5]]);
    let height = u16::from_le_bytes([data[6], data[7]]);

    let _ = Region::new(x, y, width, height);
});
```

---

#### FUZZ-002: Text Content

**Target**: `fuzz_text_content`

**Input**: Arbitrary UTF-8 strings

**Objective**: Correct ASCII conversion, no corruption

**Code**:
```rust
fuzz_target!(|data: &[u8]| {
    let text = String::from_utf8_lossy(data);
    let mut page = Page::new();
    page.write_text(0, 0, &text, Style::NORMAL);
});
```

---

#### FUZZ-003: Style Transitions

**Target**: `fuzz_style_transitions`

**Input**: Random style change sequences

**Objective**: Correct ESC/P state machine

---

### 10.3 Fuzzing Execution

**Run fuzzing**:
```bash
cargo fuzz run fuzz_region_geometry -- -max_total_time=600
```

**Reproduce crash**:
```bash
cargo fuzz run fuzz_region_geometry crash-<hash>
```

---

## 11. Test Environment

### 11.1 Development Environment

**OS**: Linux (Ubuntu 22.04), macOS (13+), Windows 10+

**Rust**: Stable, MSRV (1.75.0), Nightly

**Tools**:
- `cargo test`
- `cargo bench`
- `cargo-fuzz`
- `cargo-tarpaulin` (coverage)

---

### 11.2 CI/CD Environment

**Platform**: GitHub Actions

**Matrix**:
```yaml
os: [ubuntu-latest, macos-latest, windows-latest]
rust: [stable, "1.75.0", nightly]
```

**Jobs**:
1. **Test**: `cargo test --all-features`
2. **Bench**: `cargo bench --no-run`
3. **Fuzz**: `cargo fuzz run <target> -- -runs=1000000`
4. **Coverage**: `cargo tarpaulin --out Xml`

---

### 11.3 Hardware Test Environment

**Location**: QA Lab

**Equipment**:
- EPSON LQ-2090II (serial #12345)
- USB to Parallel adapter (model: XYZ)
- Continuous form paper (8.5" × 11")
- 3-part carbonless forms

**Schedule**: Weekly hardware tests (every Friday)

---

## 12. Test Data & Fixtures

### 12.1 Test Data Sets

#### Dataset 1: ASCII Characters

**File**: `tests/fixtures/ascii_printable.txt`

**Content**: All printable ASCII (32-126)

**Usage**: Character rendering tests

---

#### Dataset 2: Sample Invoice Data

**File**: `tests/fixtures/invoice_data.json`

**Content**:
```json
{
  "invoice_number": "INV-2025-001",
  "date": "2025-01-18",
  "customer": "Acme Corp",
  "items": [
    {"code": "A001", "description": "Widget", "qty": 10, "price": 125.50},
    {"code": "B002", "description": "Gadget", "qty": 5, "price": 87.25}
  ],
  "total": 1542.50
}
```

**Usage**: Integration tests, hardware tests

---

#### Dataset 3: Long Text

**File**: `tests/fixtures/lorem_ipsum.txt`

**Content**: 1000 words of Lorem Ipsum

**Usage**: Truncation tests, paragraph wrapping

---

### 12.2 Fixtures

#### Fixture 1: Empty Page

```rust
pub fn fixture_empty_page() -> Page {
    Page::new()
}
```

---

#### Fixture 2: Standard Invoice

```rust
pub fn fixture_standard_invoice() -> Document {
    let mut doc = DocumentBuilder::new();
    // ... create invoice layout
    doc.build()
}
```

---

## 13. Traceability Matrix

### 13.1 Requirements to Test Cases

| Requirement | Description | Test Cases | Status |
|-------------|-------------|------------|--------|
| **FR-P1** | Page dimensions 160×51 | TC-PAGE-DIM-001 | [ ] |
| **FR-P2** | Cell model | TC-CELL-001, TC-CELL-002, TC-PAGE-002 | [ ] |
| **FR-P3** | Boundary clipping | TC-PAGE-003, TC-PROP-PANIC-001 | [ ] |
| **FR-P4** | Page immutability | TC-DOC-IMMUT-001 | [ ] |
| **FR-R1** | Region geometry | TC-REGION-GEO-001, FUZZ-001 | [ ] |
| **FR-R2** | Local coordinates | TC-E2E-003 | [ ] |
| **FR-R3** | Region clipping | TC-PROP-TRUNC-001 | [ ] |
| **FR-R4** | Nesting support | TC-E2E-003, TC-HW-009 | [ ] |
| **FR-R5** | Splitting API | TC-SPLIT-H-001, TC-SPLIT-V-001, TC-PROP-SPLIT-001 | [ ] |
| **FR-R6** | Padding | TC-PADDING-001 | [ ] |
| **FR-R7** | Default style | (To be added) | [ ] |
| **FR-D1** | Document structure | TC-DOC-001 | [ ] |
| **FR-D2** | Manual pagination | TC-E2E-002 | [ ] |
| **FR-D3** | Document immutability | TC-DOC-IMMUT-001 | [ ] |
| **FR-D4** | ESC/P rendering | TC-RENDER-ORDER-001, TC-HW-001 | [ ] |
| **FR-D5** | Page separation (FF) | TC-FF-001, TC-HW-005 | [ ] |
| **FR-W1** | Label widget | TC-LABEL-001, TC-LABEL-002 | [ ] |
| **FR-W2** | TextBlock widget | TC-E2E-004 | [ ] |
| **FR-W3** | Paragraph widget | TC-E2E-004 | [ ] |
| **FR-W4** | Box widget | TC-E2E-004 | [ ] |
| **FR-W5** | KeyValue widget | TC-E2E-004 | [ ] |
| **FR-W6** | Table widget | TC-TABLE-001, TC-TABLE-002, TC-HW-007 | [ ] |
| **FR-S1** | Supported styles | TC-STYLE-001 | [ ] |
| **FR-S2** | ESC/P command mapping | TC-ESC-MAP-001, TC-ESC-MAP-002, TC-HW-003, TC-HW-004 | [ ] |
| **FR-S3** | Style state machine | TC-STATE-MACH-001 | [ ] |
| **FR-T1** | Horizontal truncation | TC-PAGE-005, TC-PROP-TRUNC-001 | [ ] |
| **FR-T2** | Vertical truncation | TC-TABLE-002 | [ ] |
| **FR-T3** | Page boundary truncation | TC-PAGE-003 | [ ] |
| **FR-T4** | No error on overflow | TC-PROP-PANIC-001, TC-PROP-PANIC-002 | [ ] |
| **FR-E1** | Rendering order | TC-RENDER-ORDER-001 | [ ] |
| **FR-E2** | Deterministic output | TC-PROP-DETERM-001, TC-GOLDEN-* | [ ] |
| **FR-E3** | Character safety | TC-CELL-003, TC-ASCII-SAFE-001, FUZZ-002 | [ ] |
| **FR-E4** | Text mode only | (Code review) | [ ] |
| **FR-ERR1** | Error type | (API tests) | [ ] |
| **FR-ERR2** | Result types | TC-BUILDER-002 | [ ] |
| **FR-ERR3** | Panic policy | TC-PROP-PANIC-*, FUZZ-* | [ ] |
| **FR-ERR5** | Validation | TC-REGION-002, TC-SPLIT-002 | [ ] |
| **NFR-1** | Determinism | TC-PROP-DETERM-001 | [ ] |
| **NFR-2** | Performance | BENCH-001 through BENCH-006 | [ ] |
| **NFR-4** | Safety (no panics) | TC-PROP-PANIC-*, FUZZ-* | [ ] |
| **NFR-5** | Portability | (CI matrix) | [ ] |
| **NFR-6** | Thread safety | (Compile-time check) | [ ] |

**Total Test Cases**: 70+ (unit) + 50+ (integration) + 20+ (property) + 10 (hardware) + 20+ (golden) + 6 (benchmarks) = **176+ test cases**

---

## 14. Test Schedule

### 14.1 Phase-Based Testing

| Phase | Duration | Focus | Test Types |
|-------|----------|-------|------------|
| **Phase 1** | Week 1-2 | Core types | Unit tests for Cell, Page, Style |
| **Phase 2** | Week 3 | Builder API | Unit + Integration for builders |
| **Phase 3** | Week 4 | Region system | Unit + Property for regions |
| **Phase 4** | Week 5-6 | Widgets | Unit + Integration for all widgets |
| **Phase 5** | Week 7 | Rendering | Unit + Golden master tests |
| **Phase 6** | Week 8 | Validation | Hardware + Performance + Fuzzing |
| **Phase 7** | Week 9 | Regression | Full suite + final hardware tests |

---

### 14.2 Daily Testing

**Developer Workflow**:
```bash
# Before commit
cargo test
cargo clippy

# Before push
cargo test --all-features
cargo bench --no-run
```

---

### 14.3 CI Testing

**On Every Commit**:
- Unit tests (all platforms)
- Clippy lints
- Format check

**On Every PR**:
- Full test suite
- Property-based tests (10K iterations)
- Benchmarks (regression check)
- Coverage report

**Nightly**:
- Fuzzing (1M+ iterations per target)
- Extended property tests (100K iterations)

**Weekly**:
- Hardware validation (10 test forms)
- Full benchmark suite
- Coverage report

---

## 15. Acceptance Criteria

### 15.1 Unit Test Criteria

- ✅ ≥ 90% line coverage
- ✅ ≥ 85% branch coverage
- ✅ 100% public API coverage
- ✅ Zero test failures

---

### 15.2 Integration Test Criteria

- ✅ All 50+ integration tests pass
- ✅ End-to-end workflows complete successfully
- ✅ All widgets render correctly

---

### 15.3 Property Test Criteria

- ✅ Zero panics in 10K+ iterations
- ✅ 100% determinism verified
- ✅ Truncation properties hold

---

### 15.4 Hardware Test Criteria

- ✅ All 10 hardware tests pass visual inspection
- ✅ Alignment within ±1 character
- ✅ Multi-part forms readable on all copies

---

### 15.5 Performance Criteria

- ✅ Single page render: < 100 μs (p99)
- ✅ 100-page render: < 10 ms (p99)
- ✅ No performance regressions > 10%

---

### 15.6 Security Criteria

- ✅ Zero panics in 1M+ fuzz iterations
- ✅ No OOM crashes
- ✅ No infinite loops

---

## 16. Appendices

### Appendix A: Invoice Test Data

**File**: `tests/fixtures/test_invoice.json`

```json
{
  "invoice_number": "INV-2025-TEST-001",
  "date": "2025-01-18",
  "customer": {
    "name": "Test Customer Inc.",
    "address": "123 Test Street",
    "city": "Test City",
    "state": "TS",
    "zip": "12345"
  },
  "items": [
    {
      "item_code": "TEST-001",
      "description": "Test Widget - Standard Size",
      "quantity": 10,
      "unit_price": 125.50,
      "total": 1255.00
    },
    {
      "item_code": "TEST-002",
      "description": "Test Gadget - Premium",
      "quantity": 5,
      "unit_price": 87.25,
      "total": 436.25
    },
    {
      "item_code": "TEST-003",
      "description": "Test Component - Industrial",
      "quantity": 20,
      "unit_price": 15.00,
      "total": 300.00
    }
  ],
  "subtotal": 1991.25,
  "tax": 159.30,
  "total": 2150.55
}
```

---

### Appendix B: Test Commands

**Run all tests**:
```bash
cargo test
```

**Run specific test**:
```bash
cargo test test_page_dimensions
```

**Run with coverage**:
```bash
cargo tarpaulin --out Html --output-dir coverage
```

**Run benchmarks**:
```bash
cargo bench
```

**Run fuzzing**:
```bash
cargo fuzz run fuzz_region_geometry
```

**Generate golden masters**:
```bash
cargo test --test golden_masters -- --nocapture
```

---

### Appendix C: Bug Report Template

```markdown
## Bug Report

**Test Case**: TC-XXX-XXX
**Requirement**: FR-XXX
**Severity**: Critical / High / Medium / Low

**Description**:
[Describe the bug]

**Steps to Reproduce**:
1. [Step 1]
2. [Step 2]

**Expected Result**:
[What should happen]

**Actual Result**:
[What actually happened]

**Environment**:
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Hardware: [if applicable]

**Test Code**:
```rust
[Minimal reproduction code]
```

**Logs**:
```
[Error messages]
```
```

---

### Appendix D: Test Metrics Dashboard

**Track**:
- Test pass rate (target: 100%)
- Code coverage (target: ≥90%)
- Benchmark trends (target: no regression > 10%)
- Fuzzing crash count (target: 0)
- Hardware test pass rate (target: 100%)

**Tools**:
- codecov.io for coverage
- criterion.rs for benchmarks
- Custom dashboard for hardware tests

---

## 🔒 End of Test Plan

**Document Status**: ✅ Ready for QA Execution

**Next Steps**:
1. Review test plan with QA team
2. Set up test infrastructure (CI, hardware lab)
3. Begin Phase 1 testing (core types)
4. Track progress in test management system

---

**For testing questions, contact:**
**QA Lead**: [TBD]
**Document Location**: `/Users/mohammadalmechkor/Projects/matrix/specs/TEST-PLAN.md`
**Related**: `PRD.md`, `TDD.md`, `API-SPEC.md`

---

# Implementation Tasks: Rust ESC/P Layout Engine Library

**Branch**: `001-rust-escap-layout-engine` | **Date**: 2025-11-18
**Feature**: Rust ESC/P Layout Engine for EPSON LQ-2090II
**Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

---

## Implementation Strategy

**MVP Scope**: User Story 1 (P1) - Render Single-Page Invoice
**Delivery Model**: Incremental by user story priority (P1 → P2 → P3)
**Parallelization**: Tasks marked [P] can be executed in parallel

### Story Completion Order

```
Setup (Phase 1) → Foundational (Phase 2) → US1 (P1) → US2 (P1) → US3 (P2) → US4 (P2) → US5 (P3) → US6 (P3) → Polish
```

### User Story Dependencies

- **US1** (P1): No dependencies - can start after foundational tasks
- **US2** (P1): No dependencies - can run in parallel with US1
- **US3** (P2): Requires US1 (needs Page rendering working)
- **US4** (P2): Requires US1 (needs Region and Page basics)
- **US5** (P3): Requires US1 (needs basic rendering, adds styles)
- **US6** (P3): Requires US1, US4 (needs Region rendering for widget boundaries)

### Parallel Execution Opportunities

**After Setup/Foundational**:
- US1 and US2 can be implemented in parallel (different concerns)
- US5 can start in parallel with US3/US4 (different modules)

**Within Each Story**:
- Core type implementations (Cell, StyleFlags, Region, etc.) can be parallelized
- Widget implementations can be parallelized (6 independent widgets)
- Test writing can happen in parallel with implementation

---

## Phase 1: Setup

**Goal**: Initialize Rust project structure with proper configuration

**Duration**: ~1 hour
**Blocker**: None - starting point

### Tasks

- [X] T001 Create Cargo.toml with project metadata and dependencies in root directory
  - Package name: `escp-layout`
  - Version: `0.1.0`
  - Edition: `2021`
  - Rust version: `1.75+`
  - Dependencies: none (runtime), `proptest`, `criterion` (dev only)
  - Features: `serde` (optional)

- [X] T002 Create .gitignore with Rust-specific patterns in root directory
  - Include: `/target/`, `Cargo.lock`, `*.rs.bk`, `*.pdb`

- [X] T003 [P] Create project directory structure per plan.md
  - `src/` (lib.rs, cell.rs, page.rs, document.rs, region.rs, error.rs)
  - `src/widgets/` (mod.rs)
  - `src/escp/` (mod.rs)
  - `tests/unit/`, `tests/integration/`, `tests/property/`, `tests/golden/`
  - `benches/`, `examples/`, `fuzz/`

- [X] T004 [P] Create src/lib.rs with module declarations and empty public API exports

- [X] T005 [P] Create README.md with project description and quickstart placeholder

- [X] T006 [P] Create LICENSE file (MIT or Apache-2.0 as per constitution)

- [X] T007 Configure Cargo.toml profile settings for release builds
  - `[profile.release]`: lto = true, codegen-units = 1
  - Target binary size < 2MB with LTO

---

## Phase 2: Foundational - Core Types

**Goal**: Implement fundamental types that all user stories depend on

**Duration**: ~3-4 hours
**Blocker**: Phase 1 must be complete

**Independent Test Criteria**:
- Cell creation handles ASCII and non-ASCII correctly
- StyleFlags bit operations work correctly
- Region validation enforces 160×51 bounds
- LayoutError implements std::error::Error

### Tasks

- [X] T008 [P] [FOUNDATION] Implement Cell struct in src/cell.rs
  - `struct Cell { character: u8, style: StyleFlags }`
  - `Cell::EMPTY` constant
  - `Cell::new(ch: char, style: StyleFlags) -> Cell`
  - Non-ASCII → '?' conversion
  - Control char handling
  - Derive: Copy, Clone, Debug, PartialEq, Eq

- [X] T009 [P] [FOUNDATION] Implement StyleFlags in src/cell.rs
  - `struct StyleFlags(u8)`
  - Constants: NONE, BOLD, UNDERLINE
  - Methods: bold(), underline(), with_bold(), with_underline()
  - Bit manipulation logic
  - Derive: Copy, Clone, Debug, PartialEq, Eq

- [X] T010 [P] [FOUNDATION] Implement Region struct in src/region.rs
  - `struct Region { x: u16, y: u16, width: u16, height: u16 }`
  - Constants: PAGE_WIDTH = 160, PAGE_HEIGHT = 51
  - `Region::new() -> Result<Region, LayoutError>`
  - `Region::full_page() -> Region`
  - Bounds validation logic
  - Derive: Copy, Clone, Debug, PartialEq, Eq

- [X] T011 [P] [FOUNDATION] Implement Region split operations in src/region.rs
  - `split_vertical(&self, top_height: u16) -> Result<(Region, Region), LayoutError>`
  - `split_horizontal(&self, left_width: u16) -> Result<(Region, Region), LayoutError>`
  - `with_padding(&self, top, right, bottom, left) -> Result<Region, LayoutError>`
  - Validation for split dimensions

- [X] T012 [P] [FOUNDATION] Implement LayoutError enum in src/error.rs
  - Variants: RegionOutOfBounds, InvalidDimensions, InvalidSplit
  - Implement Display trait with descriptive messages
  - Implement std::error::Error trait
  - Derive: Debug, Clone, PartialEq, Eq

- [X] T013 [FOUNDATION] Add unit tests for Cell in tests/unit/cell_tests.rs
  - Test ASCII character handling
  - Test non-ASCII → '?' conversion
  - Test control character handling
  - Test Cell::EMPTY behavior
  - Test Cell equality

- [X] T014 [FOUNDATION] Add unit tests for StyleFlags in tests/unit/cell_tests.rs
  - Test BOLD, UNDERLINE, NONE constants
  - Test bold() and underline() getters
  - Test with_bold() and with_underline() builders
  - Test combining bold + underline
  - Test bit manipulation correctness

- [X] T015 [FOUNDATION] Add unit tests for Region in tests/unit/region_tests.rs
  - Test Region::new() validation (success and failure cases)
  - Test Region::full_page() dimensions
  - Test split_vertical() with valid/invalid dimensions
  - Test split_horizontal() with valid/invalid dimensions
  - Test with_padding() with valid/invalid padding
  - Test zero-width/zero-height regions
  - Test boundary conditions (160×51 limits)

---

## Phase 3: User Story 1 (P1) - Render Single-Page Invoice

**Goal**: Enable rendering a single page with mixed content to valid ESC/P byte stream

**Priority**: P1 (MVP)
**Duration**: ~6-8 hours
**Dependencies**: Phase 2 (Foundational)

**Independent Test Criteria**:
- Can create a page with header text, table, and footer
- Rendering produces valid ESC/P byte stream (initialization + content + form-feed)
- Identical input produces byte-for-byte identical output (determinism)
- Finalized pages are immutable (compile-time enforced)

### Tasks

#### Page & Document Core

- [X] T016 [P] [US1] Implement Page struct in src/page.rs
  - `struct Page { cells: Box<[[Cell; 160]; 51]> }`
  - `Page::builder() -> PageBuilder`
  - `Page::get_cell(&self, x, y) -> Option<Cell>`
  - `Page::cells(&self) -> &[[Cell; 160]; 51]`
  - Derive: Debug, Clone

- [X] T017 [P] [US1] Implement PageBuilder struct in src/page.rs
  - `struct PageBuilder { cells: Box<[[Cell; 160]; 51]> }`
  - `PageBuilder::new() -> PageBuilder`
  - Initialize with Cell::EMPTY
  - Mutable during construction

- [X] T018 [US1] Implement PageBuilder write operations in src/page.rs
  - `write_at(&mut self, x, y, ch, style) -> &mut Self`
  - Silent truncation for out-of-bounds (no panic)
  - `write_str(&mut self, x, y, text, style) -> &mut Self`
  - `fill_region(&mut self, region, ch, style) -> &mut Self`
  - Method chaining support

- [X] T019 [US1] Implement PageBuilder::build() in src/page.rs
  - `build(self) -> Page`
  - Consume builder, return immutable Page
  - No public mutable methods on Page

- [X] T020 [P] [US1] Implement Document struct in src/document.rs
  - `struct Document { pages: Vec<Page> }`
  - `Document::builder() -> DocumentBuilder`
  - `Document::pages(&self) -> &[Page]`
  - `Document::page_count(&self) -> usize`
  - Derive: Debug, Clone

- [X] T021 [P] [US1] Implement DocumentBuilder struct in src/document.rs
  - `struct DocumentBuilder { pages: Vec<Page> }`
  - `DocumentBuilder::new() -> DocumentBuilder`
  - `add_page(&mut self, page: Page) -> &mut Self`
  - `build(self) -> Document`
  - Method chaining support

#### ESC/P Rendering Engine

- [X] T022 [P] [US1] Define ESC/P command constants in src/escp/constants.rs
  - ESC_RESET: &[u8] = &[0x1B, 0x40]
  - SI_CONDENSED: &[u8] = &[0x0F]
  - ESC_BOLD_ON/OFF: &[u8]
  - ESC_UNDERLINE_ON/OFF: &[u8]
  - CR, LF, FF: u8
  - Rustdoc comments with ESC/P spec references

- [X] T023 [P] [US1] Implement RenderState in src/escp/state.rs
  - `struct RenderState { bold: bool, underline: bool }`
  - `RenderState::new() -> RenderState`
  - `transition_to(&mut self, target: StyleFlags, output: &mut Vec<u8>)`
  - `reset(&mut self, output: &mut Vec<u8>)`
  - Emit ESC/P codes only on state changes
  - Internal type (not public)

- [X] T024 [US1] Implement render_document() in src/escp/renderer.rs
  - `pub fn render_document(doc: &Document) -> Vec<u8>`
  - Write initialization sequence (ESC_RESET + SI_CONDENSED)
  - Iterate pages, render each page
  - Emit form-feed after each page
  - Handle empty document (0 pages)

- [X] T025 [US1] Implement render_page() in src/escp/renderer.rs
  - `fn render_page(page: &Page, output: &mut Vec<u8>)`
  - Iterate all 51 lines
  - For each line: render_line()
  - Emit CR+LF after each line
  - Internal function

- [X] T026 [US1] Implement render_line() in src/escp/renderer.rs
  - `fn render_line(cells: &[Cell; 160], state: &mut RenderState, output: &mut Vec<u8>)`
  - Iterate 160 cells
  - Call state.transition_to() for style changes
  - Write character bytes (empty cells → space)
  - Reset styles at line end
  - Internal function

- [X] T027 [US1] Implement Document::render() public method in src/document.rs
  - `pub fn render(&self) -> Vec<u8>`
  - Call escp::render_document(self)
  - Public API entry point

- [X] T028 [US1] Export public API in src/lib.rs
  - Re-export: Cell, StyleFlags, Region, Page, PageBuilder, Document, DocumentBuilder, LayoutError
  - Module declarations: mod cell, mod page, mod document, mod region, mod error, mod escp
  - Public use statements

#### Testing

- [X] T029 [US1] Add unit tests for Page in tests/unit/page_tests.rs
  - Test Page::builder() creates builder
  - Test PageBuilder::new() initializes with EMPTY cells
  - Test write_at() updates correct cell
  - Test write_at() truncates out-of-bounds (no panic)
  - Test write_str() writes multiple characters
  - Test fill_region() fills area
  - Test build() creates immutable Page
  - Test get_cell() returns correct cell

- [X] T030 [US1] Add unit tests for Document in tests/unit/page_tests.rs
  - Test DocumentBuilder::new()
  - Test add_page() adds pages in order
  - Test build() creates immutable Document
  - Test pages() returns page slice
  - Test page_count() returns correct count
  - Test empty document (0 pages)

- [X] T031 [US1] Add unit tests for ESC/P rendering in tests/unit/escp_tests.rs
  - Test render_document() initialization sequence
  - Test single empty page output
  - Test multi-page form-feed separation
  - Test empty document output
  - Test RenderState transitions
  - Test style code emission (bold, underline)
  - Validate byte sequences with hex assertions

- [X] T032 [US1] Add integration test for single-page invoice in tests/integration/single_page_tests.rs
  - Create page with header text ("INVOICE #12345")
  - Add separator line (fill_region with '-')
  - Add footer text
  - Render to bytes
  - Validate ESC/P structure (init + content + FF)
  - Verify byte-for-byte determinism (render twice, compare)

- [X] T033 [US1] Add property-based test for determinism in tests/property/determinism_tests.rs
  - Use proptest to generate arbitrary page content
  - Render same content 1000 times
  - Assert all outputs are byte-identical
  - Compute SHA-256 hash, verify all hashes match

- [X] T034 [US1] Create golden master test file in tests/golden/invoice.bin
  - Generate reference output for invoice example
  - Store as binary file
  - Compute and document SHA-256 hash

- [X] T035 [US1] Add golden master test in tests/golden/golden_tests.rs
  - Load invoice.bin
  - Generate invoice using library
  - Compare byte-for-byte
  - Fail with diff on mismatch

---

## Phase 4: User Story 2 (P1) - Handle Content Overflow Gracefully

**Goal**: Ensure silent truncation for all overflow scenarios without errors or panics

**Priority**: P1 (Core invariant)
**Duration**: ~2-3 hours
**Dependencies**: Phase 3 (US1 - needs PageBuilder)

**Independent Test Criteria**:
- Text exceeding region width is truncated character-by-character
- Text exceeding region height is truncated line-by-line
- Nested region overflow respects all ancestor boundaries
- Zero-width/zero-height regions render nothing without errors
- No panics for any out-of-bounds writes

### Tasks

- [X] T036 [P] [US2] Add comprehensive truncation tests in tests/unit/page_tests.rs
  - Test write_at() with x >= 160 (silent ignore)
  - Test write_at() with y >= 51 (silent ignore)
  - Test write_str() exceeding line width (truncation)
  - Test fill_region() exceeding page bounds (clipping)
  - Test write operations at exact boundaries (159, 50)

- [X] T037 [P] [US2] Add region boundary tests in tests/unit/region_tests.rs
  - Test Region::new() rejects x+width > 160
  - Test Region::new() rejects y+height > 51
  - Test zero-width region (width = 0)
  - Test zero-height region (height = 0)
  - Test with_padding() that exceeds region size

- [X] T038 [US2] Add integration test for overflow scenarios in tests/integration/overflow_tests.rs
  - Create 20-char wide region, write 50-char string
  - Verify only first 20 chars in output
  - Create 5-line high region, write 10 lines
  - Verify only first 5 lines in output
  - Create nested regions with excessive padding
  - Verify inner content properly clipped

- [X] T039 [US2] Add property-based no-panic test in tests/property/no_panic_tests.rs
  - Use proptest to generate arbitrary (x, y, text) combinations
  - Call write_at() with all combinations
  - Assert no panics occur
  - Test with x, y in range [0..300] (well beyond bounds)

- [X] T040 [US2] Add fuzzing target in fuzz/fuzz_targets/fuzz_region.rs
  - Set up libfuzzer-sys
  - Fuzz Region::new() with arbitrary u16 values
  - Fuzz Region split operations
  - Run for 1M+ iterations (in CI)
  - Verify no panics

---

## Phase 5: User Story 3 (P2) - Compose Multi-Page Documents

**Goal**: Enable multi-page document rendering with proper page separation

**Priority**: P2 (Essential for real-world use)
**Duration**: ~2 hours
**Dependencies**: Phase 3 (US1 - needs Document rendering)

**Independent Test Criteria**:
- Document with N pages outputs N form-feeds
- Identical pages produce identical byte sequences between form-feeds
- Finalized documents cannot be modified (immutability)
- Pages render independently (different layouts work correctly)

### Tasks

- [X] T041 [P] [US3] Add unit tests for multi-page documents in tests/unit/page_tests.rs
  - Test DocumentBuilder with 3 pages
  - Test add_page() preserves order
  - Test render() with multiple pages
  - Count form-feeds in output (should equal page count)
  - Test immutability (DocumentBuilder consumed on build)

- [X] T042 [US3] Add integration test for multi-page document in tests/integration/multi_page_tests.rs
  - Create 3 pages with different content
  - Page 1: "Page 1" header
  - Page 2: "Page 2" header
  - Page 3: "Page 3" header
  - Render document
  - Verify 3 form-feeds in output
  - Verify content appears in order
  - Verify each page has independent layout

- [X] T043 [US3] Add test for identical pages in tests/integration/multi_page_tests.rs
  - Create 5 identical pages
  - Render document
  - Extract byte sequences between form-feeds
  - Verify all 5 sequences are identical

- [X] T044 [P] [US3] Add example program in examples/report.rs
  - Generate 10-page report
  - Each page has page number in header
  - Consistent footer on all pages
  - Render to file report.prn
  - Include usage instructions in comments

---

## Phase 6: User Story 4 (P2) - Build Complex Layouts with Nested Regions

**Goal**: Enable complex nested region layouts with proper boundary enforcement

**Priority**: P2 (Enables real-world layouts)
**Duration**: ~3-4 hours
**Dependencies**: Phase 3 (US1 - needs Region and PageBuilder)

**Independent Test Criteria**:
- Content in one region never bleeds into adjacent regions
- Nested region splits respect parent dimensions
- Deeply nested regions (3+ levels) enforce all ancestor boundaries
- Split dimensions are validated and add up correctly

### Tasks

- [X] T045 [P] [US4] Add helper method PageBuilder::render_widget() in src/page.rs
  - `render_widget(&mut self, region: Region, widget: &dyn Widget) -> &mut Self`
  - Placeholder for widget rendering
  - Widget trait defined later
  - For now, just validate region and return self

- [X] T046 [P] [US4] Add unit tests for region splitting in tests/unit/region_tests.rs
  - Test split_vertical() with 10/41 (header/body split)
  - Test chained splits (header/body/footer)
  - Test split_horizontal() with 40/120 (sidebar/main)
  - Test combined vertical then horizontal splits
  - Verify child dimensions sum to parent dimensions
  - Test error cases (split dimensions exceed parent)

- [X] T047 [US4] Add integration test for nested layout in tests/integration/nested_region_tests.rs
  - Create full page region
  - Split vertically: header (5) / body (41) / footer (5)
  - Split body horizontally: sidebar (40) / main (120)
  - Write content to each region
  - Verify header content at lines 0-4
  - Verify sidebar content at columns 0-39, lines 5-45
  - Verify main content at columns 40-159, lines 5-45
  - Verify footer content at lines 46-50
  - Verify no content bleeding between regions

- [X] T048 [US4] Add test for deeply nested regions in tests/integration/nested_region_tests.rs
  - Create 5-level nesting (region splits within splits)
  - Write content at deepest level
  - Verify all ancestor boundaries respected
  - Verify correct positioning in output

- [X] T049 [P] [US4] Add test for region padding in tests/integration/nested_region_tests.rs
  - Create region with padding (top:2, right:5, bottom:2, left:5)
  - Verify inner region dimensions = outer - padding
  - Write content to inner region
  - Verify content respects padding boundaries

---

## Phase 7: User Story 5 (P3) - Apply Text Styles Consistently

**Goal**: Implement bold and underline text styles with optimized ESC/P output

**Priority**: P3 (Enhancement)
**Duration**: ~2-3 hours
**Dependencies**: Phase 3 (US1 - needs rendering engine)

**Independent Test Criteria**:
- Bold text emits ESC E before and ESC F after
- Underline text emits ESC - 1 before and ESC - 0 after
- Adjacent styled text shares style codes (optimization)
- Style state resets at line boundaries
- No redundant style transitions

### Tasks

- [X] T050 [P] [US5] Enhance RenderState tests in tests/unit/escp_tests.rs
  - Test transition from None to Bold
  - Test transition from Bold to None
  - Test transition from None to Underline
  - Test transition from None to Bold+Underline
  - Test transition from Bold to Bold+Underline
  - Test no redundant codes (Bold to Bold = no output)
  - Test line-end reset

- [X] T051 [US5] Add integration test for bold text in tests/integration/style_tests.rs
  - Create page with bold text: write_str(..., StyleFlags::BOLD)
  - Render to bytes
  - Verify ESC E (0x1B 0x45) before text
  - Verify ESC F (0x1B 0x46) after text
  - Verify bold characters are correct

- [X] T052 [US5] Add integration test for underline text in tests/integration/style_tests.rs
  - Create page with underlined text: write_str(..., StyleFlags::UNDERLINE)
  - Render to bytes
  - Verify ESC - 1 (0x1B 0x2D 0x01) before text
  - Verify ESC - 0 (0x1B 0x2D 0x00) after text

- [X] T053 [US5] Add integration test for combined styles in tests/integration/style_tests.rs
  - Create page with bold+underline text
  - Render to bytes
  - Verify both ESC E and ESC - 1 before text
  - Verify both ESC F and ESC - 0 after text
  - Verify correct order of style codes

- [X] T054 [US5] Add integration test for style optimization in tests/integration/style_tests.rs
  - Write adjacent cells with same style (5 bold 'A's in a row)
  - Render to bytes
  - Verify only ONE ESC E at start, ONE ESC F at end
  - Count ESC code bytes, verify optimization worked

- [X] T055 [P] [US5] Update golden master test with styled content in tests/golden/
  - Create new golden file: styled_invoice.bin
  - Invoice with bold header, underlined footer
  - Document SHA-256 hash
  - Add test case to golden_tests.rs

---

## Phase 8: User Story 6 (P3) - Use Pre-Built Widgets for Common Content

**Goal**: Implement 6 widget types (Label, TextBlock, Paragraph, ASCIIBox, KeyValueList, Table)

**Priority**: P3 (Developer ergonomics)
**Duration**: ~8-10 hours
**Dependencies**: Phase 4 (US4 - needs Region rendering boundaries)

**Independent Test Criteria**:
- Each widget renders correctly within its region
- Widgets respect region boundaries (no overflow)
- Paragraph widget wraps at word boundaries
- ASCIIBox draws borders correctly
- Table widget aligns columns
- All widgets handle zero-size regions gracefully

### Tasks

#### Widget Infrastructure

- [X] T056 [P] [US6] Define Widget trait in src/widgets/mod.rs
  - `pub trait Widget { fn render(&self, page: &mut PageBuilder, region: Region); }`
  - Rustdoc contract: MUST respect boundaries, MUST NOT panic
  - Export Widget in src/lib.rs

- [X] T057 [P] [US6] Update PageBuilder::render_widget() implementation in src/page.rs
  - Call widget.render(self, region)
  - Method chaining

#### Label Widget

- [X] T058 [P] [US6] Implement Label widget in src/widgets/label.rs
  - `pub struct Label { text: String, style: StyleFlags }`
  - `Label::new(text) -> Label`
  - `Label::with_style(style) -> Label`
  - Implement Widget trait
  - Render on first line only
  - Truncate if exceeds region width
  - Export in src/widgets/mod.rs and src/lib.rs

- [X] T059 [US6] Add unit tests for Label in tests/unit/widget_tests.rs
  - Test Label::new()
  - Test Label::with_style()
  - Test render() within region bounds
  - Test render() with text exceeding width (truncation)
  - Test render() in zero-width region (no panic)

#### TextBlock Widget

- [X] T060 [P] [US6] Implement TextBlock widget in src/widgets/text_block.rs
  - `pub struct TextBlock { lines: Vec<String> }`
  - `TextBlock::new(lines: Vec<String>) -> TextBlock`
  - `TextBlock::from_text(text: impl Into<String>) -> TextBlock`
  - Implement Widget trait
  - One line per string (no wrapping)
  - Truncate lines exceeding width
  - Truncate lines exceeding height

- [X] T061 [US6] Add unit tests for TextBlock in tests/unit/widget_tests.rs
  - Test from_text() splits on \n
  - Test render() with multiple lines
  - Test horizontal truncation (line too long)
  - Test vertical truncation (too many lines)
  - Test zero-height region

#### Paragraph Widget (with word wrapping)

- [X] T062 [P] [US6] Implement word-wrapping helper in src/widgets/paragraph.rs
  - `fn wrap_text(text: &str, max_width: usize) -> Vec<String>`
  - Wrap at word boundaries (space)
  - Break long words if necessary
  - Internal function

- [X] T063 [US6] Implement Paragraph widget in src/widgets/paragraph.rs
  - `pub struct Paragraph { text: String, style: StyleFlags }`
  - `Paragraph::new(text) -> Paragraph`
  - `Paragraph::with_style(style) -> Paragraph`
  - Implement Widget trait using wrap_text()
  - Truncate wrapped lines exceeding region height

- [X] T064 [US6] Add unit tests for Paragraph in tests/unit/widget_tests.rs
  - Test wrap_text() helper function
  - Test short text (no wrapping needed)
  - Test long text (wrapping occurs)
  - Test word boundary wrapping
  - Test long word breaking
  - Test render() with wrapped content
  - Test vertical truncation after wrapping

#### ASCIIBox Widget

- [X] T065 [P] [US6] Implement ASCIIBox widget in src/widgets/ascii_box.rs
  - `pub struct ASCIIBox { title: Option<String>, content: Box<dyn Widget> }`
  - `ASCIIBox::new(content: Box<dyn Widget>) -> ASCIIBox`
  - `ASCIIBox::with_title(title) -> ASCIIBox`
  - Implement Widget trait
  - Draw border using +, -, | characters
  - Render title in top border if present
  - Render content in inset region (1-cell padding)
  - Handle regions too small for box (< 3×3)

- [X] T066 [US6] Add unit tests for ASCIIBox in tests/unit/widget_tests.rs
  - Test box without title
  - Test box with title
  - Test border character placement
  - Test inner content positioning
  - Test too-small region (< 3×3)
  - Test title truncation if too long

#### KeyValueList Widget

- [X] T067 [P] [US6] Implement KeyValueList widget in src/widgets/key_value.rs
  - `pub struct KeyValueList { entries: Vec<(String, String)>, separator: String }`
  - `KeyValueList::new(entries) -> KeyValueList`
  - `KeyValueList::with_separator(sep) -> KeyValueList`
  - Default separator: ": "
  - Implement Widget trait
  - One entry per line: "key: value"
  - Truncate entries exceeding width
  - Truncate entries exceeding height

- [X] T068 [US6] Add unit tests for KeyValueList in tests/unit/widget_tests.rs
  - Test default separator ": "
  - Test custom separator
  - Test multiple entries
  - Test horizontal truncation (long key+value)
  - Test vertical truncation (too many entries)

#### Table Widget

- [X] T069 [P] [US6] Implement ColumnDef struct in src/widgets/table.rs
  - `pub struct ColumnDef { pub name: String, pub width: u16 }`
  - Derive: Clone, Debug

- [X] T070 [US6] Implement Table widget in src/widgets/table.rs
  - `pub struct Table { columns: Vec<ColumnDef>, rows: Vec<Vec<String>> }`
  - `Table::new(columns, rows) -> Table`
  - Implement Widget trait
  - First line: render column headers (bold)
  - Subsequent lines: render rows
  - Truncate cells to column width
  - Truncate rows exceeding region height
  - Left-align cells

- [X] T071 [US6] Add unit tests for Table in tests/unit/widget_tests.rs
  - Test single row table
  - Test multi-row table
  - Test column width enforcement
  - Test header rendering (bold)
  - Test row truncation (too many rows)
  - Test column truncation (too many columns)
  - Test empty table (0 rows)

#### Integration & Examples

- [X] T072 [US6] Add integration test for all widgets in tests/integration/widget_tests.rs
  - Create page with all 6 widget types
  - Each widget in separate region
  - Render and verify output structure
  - Verify boundary enforcement for all widgets

- [X] T073 [P] [US6] Create example program in examples/invoice.rs
  - Complete invoice using widgets
  - Header: Label with bold company name
  - Info section: KeyValueList (invoice #, date, etc.)
  - Items: Table with 4 columns
  - Notes: Paragraph with wrapped text
  - Footer: ASCIIBox with total
  - Save to invoice_example.prn

- [X] T074 [P] [US6] Create example program in examples/hello_world.rs
  - Minimal example from quickstart.md
  - Single page with "Hello, World!"
  - Render to output.prn
  - < 10 lines of code

---

## Phase 9: Polish & Cross-Cutting Concerns

**Goal**: Complete project with documentation, benchmarks, and final validations

**Duration**: ~4-6 hours
**Dependencies**: All user stories complete

### Tasks

#### Documentation

- [X] T075 [P] Add rustdoc comments to all public types in src/
  - Cell, StyleFlags, Region, Page, PageBuilder, Document, DocumentBuilder
  - All public methods with /// comments
  - Include examples in docs
  - Document error conditions

- [X] T076 [P] Add rustdoc comments to all widgets in src/widgets/
  - Widget trait
  - Label, TextBlock, Paragraph, ASCIIBox, KeyValueList, Table
  - Usage examples in rustdoc

- [X] T077 [P] Add rustdoc comments to ESC/P module in src/escp/
  - Public rendering function
  - Document ESC/P output format
  - Link to ESC/P spec

- [X] T078 Update README.md with complete content
  - Copy quickstart example
  - Add installation instructions
  - Add feature list
  - Add license badge
  - Add usage examples
  - Link to docs

- [X] T079 [P] Verify rustdoc builds cleanly
  - Run: cargo doc --no-deps
  - Fix any warnings
  - Check for broken links
  - Verify examples compile in docs

#### Performance & Benchmarks

- [X] T080 [P] Create render benchmark in benches/render_bench.rs
  - Benchmark single page render
  - Benchmark 100-page document render
  - Benchmark region lookup
  - Benchmark page allocation
  - Use criterion
  - Set baseline targets from plan.md

- [X] T081 [P] Create widget benchmark in benches/widget_bench.rs
  - Benchmark each widget type
  - Compare widget render times
  - Use criterion

- [X] T082 Run benchmarks and validate performance targets
  - Single page < 100μs (p99)
  - 100-page document < 10ms (p99)
  - Document results in benchmark outputs
  - Ensure no regressions

#### Final Validation

- [X] T083 Run full test suite
  - cargo test --all-features
  - Verify all tests pass
  - Check test coverage (aim for 90%+ per constitution)

- [X] T084 [P] Run property-based tests with high iteration count
  - Determinism test: 1000 iterations
  - No-panic test: 10000+ iterations
  - Document results

- [X] T085 [P] Set up fuzzing and run initial campaign
  - Configure cargo-fuzz
  - Add fuzz targets for Region, rendering
  - Run for 1M+ iterations
  - Document any findings

- [X] T086 Run cargo clippy and fix warnings
  - cargo clippy -- -D warnings
  - Fix all clippy warnings
  - Ensure code quality

- [X] T087 Run cargo fmt and ensure consistent formatting
  - cargo fmt --all
  - Check formatting in CI

- [X] T088 Verify zero runtime dependencies
  - cargo tree --depth 1
  - Ensure only std in runtime dependencies
  - Verify dev dependencies are dev-only

- [X] T089 Build release binary and check size
  - cargo build --release
  - Verify binary size < 2MB with LTO
  - Document actual size

- [X] T090 [P] Create final golden master tests for all examples
  - hello_world.prn
  - invoice.prn
  - report.prn
  - Compute SHA-256 hashes
  - Add to golden_tests.rs

#### Optional: Hardware Validation

- [ ] T091 [OPTIONAL] Test output on physical EPSON LQ-2090II printer
  - Print hello_world.prn
  - Print invoice.prn
  - Verify alignment, styles, page breaks
  - Document results

---

## Task Summary

**Total Tasks**: 91
**Phases**: 9 (Setup, Foundational, 6 User Stories, Polish)

### Tasks by Phase

| Phase | Task Count | User Story | Priority |
|-------|------------|------------|----------|
| Setup | 7 | - | - |
| Foundational | 8 | - | - |
| US1 | 20 | Single-Page Invoice | P1 |
| US2 | 5 | Content Overflow | P1 |
| US3 | 4 | Multi-Page Documents | P2 |
| US4 | 5 | Nested Regions | P2 |
| US5 | 6 | Text Styles | P3 |
| US6 | 19 | Widgets | P3 |
| Polish | 17 | - | - |

### Parallel Execution Opportunities

**Phase 2 (Foundational)**: T008-T015 can largely run in parallel (different files)
**Phase 3 (US1)**: T016-T021 (Page/Document) parallel with T022-T026 (ESC/P)
**Phase 8 (US6)**: All 6 widgets (T058-T071) can be implemented in parallel

**Estimated parallelization speedup**: 30-40% reduction in wall-clock time with 3-4 developers

### MVP Scope (Recommended)

**Minimum Viable Product**: Complete through Phase 4 (US1 + US2)
- Core types (Cell, StyleFlags, Region, Page, Document)
- ESC/P rendering engine
- Single-page rendering with determinism
- Silent truncation for overflow
- ~35 tasks total

**MVP+**: Add US3 (Multi-page) - ~39 tasks total

**Full V1.0**: All tasks (91) including all widgets and polish

---

## Dependencies & Execution Order

```
Setup (T001-T007)
  ↓
Foundational (T008-T015)
  ↓
  ├─→ US1 (T016-T035) ──→ US3 (T041-T044)
  │                     ↓
  ├─→ US2 (T036-T040) ──┘
  │
  └─→ US4 (T045-T049) ──→ US6 (T056-T074)
                        ↓
US5 (T050-T055) ────────┘
  ↓
Polish (T075-T091)
```

**Critical Path**: Setup → Foundational → US1 → US3 → US6 → Polish
**Parallel Branches**: US2, US4, US5 can run alongside or after US1

---

## Testing Strategy

**Unit Tests**: Per-module coverage (tests/unit/)
**Integration Tests**: End-to-end scenarios (tests/integration/)
**Property-Based Tests**: Determinism, no-panic (tests/property/)
**Golden Master Tests**: Byte-level ESC/P validation (tests/golden/)
**Fuzzing**: Long-running chaos testing (fuzz/)
**Benchmarks**: Performance regression prevention (benches/)

**Test Execution**:
```bash
cargo test --all-features          # All tests
cargo test --test integration      # Integration only
cargo +nightly fuzz run fuzz_region # Fuzzing
cargo bench                        # Benchmarks
```

---

## Success Criteria

✅ All user story acceptance scenarios pass
✅ Determinism verified (1000+ iterations)
✅ Zero panics in fuzzing (1M+ iterations)
✅ Performance targets met (benchmarks)
✅ Zero runtime dependencies
✅ 90%+ test coverage
✅ All rustdoc builds cleanly
✅ cargo clippy passes with -D warnings
✅ Binary size < 2MB with LTO
✅ Constitution compliance verified

---

**Ready for implementation. Start with MVP (Phase 1-4) for fastest time to value.**

# Feature Specification: Rust ESC/P Layout Engine Library

**Feature Branch**: `001-rust-escap-layout-engine`
**Created**: 2025-11-18
**Status**: Draft
**Input**: User description: "Build a Rust library that implements a fully deterministic text-based layout engine for the Epson LQ-2090II dot-matrix printer using ESC/P condensed text mode. The engine must render content onto a fixed 160×51 character page matrix, enforcing strict boundaries and silent truncation rules. Layout is based on rectangular Regions that can be nested, padded, or split both horizontally and vertically. Regions never resize automatically, and overflowing content is always truncated without generating new pages.

The library must support composing Documents made of multiple manually created Pages. Each Page contains a 160×51 grid of Cells storing ASCII characters and simple text styles like bold and underline. A set of static widgets—such as labels, text blocks, paragraphs with wrapping, ASCII rectes, key-value lists, and fixed-column tables—must be renderable inside Regions while respecting truncation and region boundaries.

Output must be a valid ESC/P byte stream including correct initialization, condensed-mode activation, style state transitions, per-line rendering, and form-feed separation between pages. The internal rendering must never enter bitmap or graphic modes, never wrap content across regions, and never auto-paginate. The library must expose a clean builder-style API that ensures immutability of finalized Pages and Documents, and guarantees deterministic byte-for-byte output for identical inputs.

The system must prioritize predictability, immutability, ESC/P compatibility, layout correctness, and developer ergonomics. All behavior must adhere strictly to the frozen V1 specification for fixed layout, strict truncation, and manual pagination."

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Render Single-Page Invoice (Priority: P1)

A developer building an invoicing system needs to generate printer-ready output for an Epson LQ-2090II dot-matrix printer. They create a single page containing company header, line items in a table, and totals, then render it to a byte stream that can be sent directly to the printer.

**Why this priority**: This represents the core MVP functionality - the ability to compose a page with multiple content types and generate valid printer output. Without this, the library delivers no value.

**Independent Test**: Can be fully tested by creating a page with header text, a simple table, and footer text, rendering it to bytes, and verifying the output produces correct ESC/P sequences for condensed mode initialization, text content, and form-feed termination.

**Acceptance Scenarios**:

1. **Given** an empty page with 160×51 dimensions, **When** developer adds header text, a 3-column table with 5 rows, and footer text, **Then** all content appears in correct positions without overlapping
2. **Given** a page with content, **When** developer calls render method, **Then** output is a valid ESC/P byte stream starting with initialization codes and ending with form-feed
3. **Given** identical page content, **When** render is called multiple times, **Then** each render produces byte-for-byte identical output
4. **Given** a finalized page, **When** developer attempts to modify it, **Then** the page remains unchanged (immutability preserved)

---

### User Story 2 - Handle Content Overflow Gracefully (Priority: P1)

A developer places a text block containing 100 characters into a region that can only hold 50 characters. The library must silently truncate the excess content without errors, warnings, or attempting to resize the region or create new pages.

**Why this priority**: Deterministic truncation is a core invariant of the V1 specification. Without predictable overflow handling, output becomes non-deterministic and unusable for fixed-layout printing scenarios.

**Independent Test**: Can be tested by creating a region with known width/height, filling it with content exceeding those bounds, rendering to bytes, and verifying that output contains only the content that fits within the region boundaries.

**Acceptance Scenarios**:

1. **Given** a 20-character-wide region, **When** developer places a 50-character text string, **Then** only the first 20 characters appear in output
2. **Given** a 5-line-high region, **When** developer places a 10-line paragraph, **Then** only the first 5 lines appear in output
3. **Given** nested regions with combined padding exceeding parent bounds, **When** content is rendered, **Then** inner content is truncated to respect parent boundaries
4. **Given** a table with more rows than region height, **When** rendered, **Then** only rows that fit within region are included in output

---

### User Story 3 - Compose Multi-Page Documents (Priority: P2)

A developer needs to generate a multi-page report with consistent headers and footers. They create multiple pages, each with the same header region layout but different body content, then render the complete document as a single byte stream with proper page separation.

**Why this priority**: Multi-page support is essential for real-world document generation but can be delivered after single-page rendering works. Each page is independent, making this an incremental enhancement.

**Independent Test**: Can be tested by creating a document with 3 pages containing different content, rendering to bytes, and verifying the output contains three sets of page content separated by form-feed characters.

**Acceptance Scenarios**:

1. **Given** a document with 3 pages, **When** rendered, **Then** output contains exactly 3 form-feed characters (one after each page)
2. **Given** pages with identical content, **When** rendered in a document, **Then** each page produces identical byte sequences between form-feeds
3. **Given** a finalized document, **When** developer attempts to add or remove pages, **Then** the document remains unchanged
4. **Given** pages with different layouts, **When** rendered in sequence, **Then** each page's output reflects its specific layout independently

---

### User Story 4 - Build Complex Layouts with Nested Regions (Priority: P2)

A developer creates a page layout with a full-page region split vertically into header (10 lines), body (35 lines), and footer (6 lines). The body region is further split horizontally into sidebar (40 chars) and main content (120 chars). Each region can contain different widget types respecting boundaries.

**Why this priority**: Region composition enables real-world layouts but depends on basic region rendering working first. This builds on P1 functionality.

**Independent Test**: Can be tested by creating a nested region structure, placing different content in each region, and verifying that content in each region stays within its boundaries and doesn't bleed into adjacent regions.

**Acceptance Scenarios**:

1. **Given** a page split vertically into 3 regions, **When** content is placed in each region, **Then** content in one region never appears in another region's area
2. **Given** a region split horizontally with padding, **When** child regions are rendered, **Then** total horizontal space used equals parent width minus padding
3. **Given** deeply nested regions (3+ levels), **When** content is placed in leaf regions, **Then** all ancestor boundary constraints are respected
4. **Given** a vertical split with 10/35/6 line allocation, **When** all regions are filled, **Then** output uses exactly 51 lines total

---

### User Story 5 - Apply Text Styles Consistently (Priority: P3)

A developer needs to emphasize certain text using bold and underline styles. They apply styles to text within widgets and verify that the ESC/P output contains correct style activation and deactivation codes, with state transitions minimized to reduce byte stream size.

**Why this priority**: Text styling enhances document quality but is not essential for basic layout functionality. Can be delivered after layout mechanics are solid.

**Independent Test**: Can be tested by creating text with bold and underline styles, rendering to bytes, and verifying the output contains ESC/P codes for style activation before styled text and deactivation after, with no redundant state changes.

**Acceptance Scenarios**:

1. **Given** text with bold style applied, **When** rendered, **Then** output contains ESC/P bold-on code before text and bold-off code after
2. **Given** adjacent text blocks with same style, **When** rendered, **Then** output contains only one style activation spanning both blocks
3. **Given** text with both bold and underline, **When** rendered, **Then** output contains both style codes in correct order
4. **Given** unstyled text following styled text, **When** rendered, **Then** all styles are deactivated before unstyled text begins

---

### User Story 6 - Use Pre-Built Widgets for Common Content (Priority: P3)

A developer uses widget types like labels, paragraphs with word wrapping, ASCII rectes, key-value lists, and fixed-column tables to quickly compose page content without manually managing character positioning or wrapping logic.

**Why this priority**: Widgets improve developer ergonomics significantly but are built on top of basic rendering. Can be implemented incrementally after core layout works.

**Independent Test**: Can be tested by creating instances of each widget type with representative content, rendering within regions of known sizes, and verifying output matches expected layout for each widget type.

**Acceptance Scenarios**:

1. **Given** a paragraph widget with text exceeding region width, **When** rendered, **Then** text wraps to next line at word boundaries
2. **Given** an ASCII rect widget with title and content, **When** rendered, **Then** output shows rect border characters surrounding content
3. **Given** a key-value list widget with 5 entries, **When** rendered, **Then** each entry appears on a separate line with consistent key-value spacing
4. **Given** a fixed-column table widget with 3 columns and 10 rows, **When** rendered in a region with 7 lines, **Then** only first 7 rows appear (including header)

---

### Edge Cases

- What happens when a region has zero width or height? (Expected: region renders nothing, no error)
- What happens when nested region splits exceed parent dimensions due to rounding? (Expected: inner regions truncated to fit parent exactly)
- What happens when a region is split with padding larger than region size? (Expected: child regions have zero usable space, render nothing)
- What happens when text contains non-ASCII characters? (Expected: non-ASCII characters are replaced with a safe placeholder like '?' or stripped, maintaining determinism)
- What happens when bold or underline state is not closed before page ends? (Expected: styles automatically deactivated at line end or page end to maintain printer state consistency)
- What happens when a document contains zero pages? (Expected: render produces empty byte stream or minimal initialization sequence, no form-feeds)
- What happens when the same page is added to multiple documents? (Expected: page renders identically in each document context)

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: Library MUST provide a Page type representing a fixed 160×51 character grid
- **FR-002**: Library MUST provide a Document type that contains an ordered collection of Pages
- **FR-003**: Library MUST provide a Region type representing a rectangular area within a Page with defined column and row boundaries
- **FR-004**: Library MUST support splitting Regions vertically (into top/bottom sub-regions with specified line counts)
- **FR-005**: Library MUST support splitting Regions horizontally (into left/right sub-regions with specified character counts)
- **FR-006**: Library MUST support adding padding to Regions (reducing usable space by specified amounts on each edge)
- **FR-007**: Library MUST support nesting Regions to arbitrary depth while respecting ancestor boundaries
- **FR-008**: Library MUST enforce that content placed in a Region never exceeds Region boundaries (silent truncation)
- **FR-009**: Library MUST provide widget types: Label, TextBlock, Paragraph, ASCIIRect, KeyValueList, Table
- **FR-010**: Paragraph widget MUST wrap text at word boundaries when content exceeds region width
- **FR-011**: Table widget MUST support fixed-column layouts with configurable column widths
- **FR-012**: All widgets MUST respect region boundaries and truncate content when region is full
- **FR-013**: Library MUST support text styles: bold and underline
- **FR-014**: Library MUST provide a render method that converts a Document into an ESC/P byte stream
- **FR-015**: Render output MUST begin with ESC/P initialization codes for condensed text mode
- **FR-016**: Render output MUST include ESC/P style codes (bold on/off, underline on/off) only when style state changes
- **FR-017**: Render output MUST separate pages with form-feed character (0x0C)
- **FR-018**: Render output MUST produce byte-for-byte identical results when called multiple times on the same Document
- **FR-019**: Page and Document types MUST be immutable after finalization (builder pattern)
- **FR-020**: Library MUST NOT automatically create new pages when content overflows
- **FR-021**: Library MUST NOT enter bitmap or graphic ESC/P modes in render output
- **FR-022**: Library API MUST use builder pattern for constructing Pages and Documents
- **FR-023**: Library MUST validate region splits to ensure child regions fit within parent dimensions
- **FR-024**: Library MUST handle non-ASCII characters by replacing or stripping them to maintain ASCII-only output
- **FR-025**: Library MUST ensure all style state is reset at end of each line or page to prevent printer state corruption

### Key Entities

- **Page**: Represents a single 160×51 character grid. Contains cells that hold ASCII characters and style information. Immutable after finalization. Attributes: width (160), height (51), cell grid, finalization state.

- **Cell**: Represents a single character position within a Page. Contains one ASCII character and style flags (bold, underline). Attributes: character (ASCII), bold flag, underline flag.

- **Document**: Contains an ordered list of Pages. Immutable after finalization. Represents a complete multi-page output. Attributes: page list, finalization state.

- **Region**: Represents a rectangular area within a Page defined by column range and row range. Used for layout composition. Can be split into child regions or have padding applied. Attributes: column start/end, row start/end, parent region reference.

- **Widget**: Abstract concept representing renderable content types. Each widget type knows how to render itself into a Region. Concrete types: Label (single-line text), TextBlock (multi-line text without wrapping), Paragraph (multi-line with word wrap), ASCIIRect (bordered content), KeyValueList (aligned key-value pairs), Table (fixed-column tabular data).

- **ESC/P Byte Stream**: Output representation consisting of ESC/P command sequences and printable ASCII characters. Contains initialization codes, style state transitions, line content, and page separators. Attributes: byte sequence, deterministic generation guarantee.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Developer can create a single-page document with mixed content (text, table, rect) and render to valid ESC/P output in under 10 lines of client code
- **SC-002**: Identical document content produces identical byte stream output across 1000 consecutive renders (determinism verification)
- **SC-003**: Content overflowing region boundaries is silently truncated without errors or warnings in 100% of test cases
- **SC-004**: Multi-page documents with 100 pages render successfully with correct page separation (99 form-feeds)
- **SC-005**: Nested region layouts with 5 levels of nesting render correctly with all content staying within boundaries
- **SC-006**: Text styles (bold, underline) produce correct ESC/P codes with no redundant state transitions in generated output
- **SC-007**: All six widget types render correctly within regions of varying sizes without boundary violations
- **SC-008**: Finalized pages and documents cannot be modified (immutability enforced by API design)
- **SC-009**: Rendering a 10-page document completes in under 100ms on standard developer hardware
- **SC-010**: Generated ESC/P byte streams are accepted by Epson LQ-2090II printers without errors when sent directly

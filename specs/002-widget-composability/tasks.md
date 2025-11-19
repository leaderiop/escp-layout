# Tasks: Widget Composability System

**Input**: Design documents from `/specs/002-widget-composability/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md
**Feature Branch**: `002-widget-composability`

**Tests**: Test tasks are integrated within each phase and user story to ensure Constitutional compliance (Principle XII: ≥90% coverage, 100% public API coverage). Additional comprehensive test suite in Phase 2 (Foundational) and Phase 6 (Polish).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

**Updates (2025-11-19)**: Tasks updated to reflect spec.md clarifications from `/speckit.analyze` findings:

- Added explicit clip_bounds intersection algorithm specification (T008 - already present)
- Added PageBuilder Layer 3 validation tests for FR-004 Three-Layer Architecture (T014E1, T014E2, T014E3 - analysis finding C2)
- Expanded Label HEIGHT constraint testing to comprehensive test suite (T029A - analysis finding C1)
- Added Label HEIGHT rustdoc `# Panics` section (T029B - analysis finding C1)
- Added SC-005 code reduction comparison test (T055I - analysis finding C3)
- Added cargo doc validation task for Constitution Principle XIV (T057A - analysis finding C4)
- Enhanced API documentation requirements with syntax examples (T057 updated)
- Updated RenderContext tests to verify clip_bounds intersection algorithm (T014E - analysis finding U1)

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

Single library project structure:

- Source: `src/` at repository root
- Tests: `tests/` at repository root
- Examples: `examples/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create widget module structure: src/widget/mod.rs, src/widget/tree.rs, src/widget/rect_widget.rs, src/widget/label.rs, src/widget/context.rs, src/widget/layout/mod.rs
- [ ] T002 [P] Add widget module re-exports to src/lib.rs (Widget, Rect, Label, RenderError, Column, Row, Stack, rect_new!, label_new!, column_new!, row_new!, stack_new!, column_area!, row_area!)
- [ ] T003 [P] Create test directory structure: tests/widget/composition.rs, tests/widget/rendering.rs, tests/widget/boundary.rs, tests/widget/layouts.rs, tests/widget/integration.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 Define Widget trait in src/widget/mod.rs with associated constants (const WIDTH: u16; const HEIGHT: u16;) and render_to(&self, context, position) method per FR-001 and data-model.md specification
- [ ] T005 [P] Define RenderError enum in src/widget/mod.rs with #[non_exhaustive] attribute and these V1 variants: ChildExceedsParent, OutOfBounds, OverlappingChildren, InsufficientSpace, IntegerOverflow, TextExceedsWidth per Constitution Principle III Widget Exception amendment v1.2.0 and data-model.md (note: ZeroSizeParent error variant intentionally excluded - zero-size prevention uses compile-time const generics when stable + debug_assert! in Rect::new() per Constitution Principle VI validation hierarchy and FR-007 removal rationale)
- [ ] T006 [P] Implement Display and Error traits for RenderError in src/widget/mod.rs with contextual error messages per data-model.md format
- [ ] T007 Define WidgetNode struct in src/widget/tree.rs with position: (u16, u16) and widget: Rect<dyn Widget> fields per data-model.md
- [ ] T008 Define RenderContext struct in src/widget/context.rs with page_builder and clip_bounds fields per data-model.md specification; implement clip_bounds intersection algorithm per spec.md RenderContext entity: new_clip_x = max(current_clip.x, child_abs_x), new_clip_y = max(current_clip.y, child_abs_y), new_clip_right = min(current_clip.x + current_clip.width, child_abs_x + child_width), new_clip_bottom = min(current_clip.y + current_clip.height, child_abs_y + child_height), new_clip_width = new_clip_right - new_clip_x, new_clip_height = new_clip_bottom - new_clip_y
- [ ] T009 Implement RenderContext::new(page_builder) in src/widget/context.rs initializing clip_bounds to (0, 0, 160, 51) per data-model.md
- [ ] T010 Implement RenderContext::write_text() in src/widget/context.rs with pre-validation before PageBuilder delegation (check against clip_bounds, return OutOfBounds error on violation) per data-model.md
- [ ] T011 [P] Implement RenderContext::write_styled() in src/widget/context.rs with pre-validation before PageBuilder delegation (check against clip_bounds, return OutOfBounds error on violation) per data-model.md
- [ ] T012 [P] Define Label<const WIDTH: u16, const HEIGHT: u16> struct in src/widget/label.rs with text: Option<String>, style: Style fields per data-model.md (text is Option because builder pattern starts with None and add_text() sets it)
- [ ] T013 [P] Implement Label builder pattern in src/widget/label.rs: (1) Label::new() -> Self with debug_assert!(HEIGHT == 1, "Label HEIGHT must be 1") per Constitution Principle IX Documented Constraint Violations, initializing text: None and default style; (2) add_text(text: &str) -> Result<Self, RenderError> validating text.len() as u16 <= WIDTH and text contains no newlines (`\n`, `\r\n`), returning TextExceedsWidth error if violated; (3) bold() -> Self and underline() -> Self methods modifying style per data-model.md
- [ ] T014 Implement Widget trait for Label<const WIDTH, const HEIGHT> in src/widget/label.rs with associated constants (const WIDTH = WIDTH; const HEIGHT = HEIGHT;) and render_to() calling context.write_styled() per data-model.md
- [ ] T014A [P] Define rect_new! macro in src/widget/rect_widget.rs that expands rect_new!(W, H) to Rect::<W, H>::new() for clean API syntax
- [ ] T014B [P] Define label_new! macro in src/widget/label.rs that expands label_new!(W) to Label::<W, 1>::new() for clean API syntax (macro automatically sets HEIGHT=1)
- [ ] T014C [P] Add unit tests for Widget trait in tests/widget/mod.rs: verify associated constants WIDTH/HEIGHT accessible, verify render_to() signature correctness (Constitution Principle XII: 100% public API coverage)
- [ ] T014D [P] Add unit tests for RenderError in tests/widget/mod.rs: verify all error variants (ChildExceedsParent, OutOfBounds, OverlappingChildren, InsufficientSpace, IntegerOverflow, TextExceedsWidth), verify Display trait output includes contextual information per data-model.md
- [ ] T014E [P] Add unit tests for RenderContext in tests/widget/context_tests.rs: verify clip_bounds initialization to (0,0,160,51), verify write_text() pre-validation, verify write_styled() pre-validation, verify OutOfBounds error returned for invalid positions, verify clip_bounds intersection algorithm correctness per spec.md formula (test nested clipping scenarios with known parent/child bounds and verify calculated clip region matches formula)
- [ ] T014E1 [P] Add PageBuilder Layer 3 validation test in tests/widget/context_tests.rs: verify PageBuilder silently truncates text extending beyond bounds after valid start position (validates FR-004 Three-Layer Validation Architecture Layer 3 delegation contract); create RenderContext with clip_bounds, write text starting at valid position but extending beyond bounds, verify PageBuilder truncates without returning error (analysis finding C2)
- [ ] T014E2 [P] Add PageBuilder Layer 3 integration test in tests/integration/pagebuilder_truncation.rs: verify PageBuilder silent truncation behavior unchanged after RenderContext introduction - create Page with direct PageBuilder usage (bypassing widgets), write text extending beyond page bounds, verify no errors returned and output truncated correctly; ensures Constitution Principle III Layer 3 contract preserved
- [ ] T014E3 [P] Add PageBuilder Layer 3 property-based test in tests/widget/context_tests.rs: use proptest to generate arbitrary text strings (length 1-500) with valid start positions within clip_bounds, write via RenderContext to PageBuilder, verify no errors returned (silent truncation only) for all 1000+ iterations; validates FR-004 Three-Layer Architecture Layer 3 delegation contract
- [ ] T014F [P] Add unit tests for Label in tests/widget/label_tests.rs: verify Label::<20, 1>::new() initializes with text=None, verify add_text("valid text") succeeds when text.len() <= WIDTH and contains no newlines, verify TextExceedsWidth error when text.len() > WIDTH, verify TextExceedsWidth error when text contains newlines (`\n`, `\r\n`), verify bold() and underline() methods modify style correctly, verify HEIGHT=1 constraint via debug_assert! (test Label::<20, 2>::new() panics in debug build), verify render_to() delegates to context.write_styled(), verify Widget trait implementation has const HEIGHT = 1, verify Label (as leaf widget per FR-009) has no child-related methods (compilation test: `Label::<20, 1>::new().add_text("text")?.add_child(...)` must not compile - use compile_fail doc test)
- [ ] T014G [P] Add unit tests for macros in tests/widget/macro_tests.rs: verify rect_new!(W, H) expands correctly, verify label_new!(W) expands correctly to Label::<W, 1>::new()

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Nested Widget Composition (Priority: P1) 🎯 MVP

**Goal**: Enable developers to compose multiple widgets into hierarchical structures (parent-child relationships) so that complex UI layouts can be built from simple, reusable components without manually managing coordinate offsets for each child

**Independent Test**: Create a simple parent widget containing two child widgets, render it, and verify that children appear at correct relative positions within the parent's bounds

### Implementation for User Story 1

- [ ] T015 [US1] Define Rect<const WIDTH: u16, const HEIGHT: u16> struct in src/widget/rect_widget.rs with children: Vec<WidgetNode> field per data-model.md
- [ ] T016 [US1] Implement Rect::new() in src/widget/rect_widget.rs with debug_assert! validating WIDTH > 0 and HEIGHT > 0 (compile-time const validation not stable yet) per data-model.md
- [ ] T017 [US1] Implement Rect::add_child(widget, position) in src/widget/rect_widget.rs with boundary validation (ChildExceedsParent, OutOfBounds, IntegerOverflow) using checked_add and widget.WIDTH/HEIGHT constants per data-model.md
- [ ] T017A [US1] Implement overlap detection in Rect::add_child() in src/widget/rect_widget.rs using AABB (Axis-Aligned Bounding Rect) collision detection algorithm: for each existing child, check if new child rectangle intersects using formula (child1_right > child2_left && child1_left < child2_right && child1_bottom > child2_top && child1_top < child2_bottom); return OverlappingChildren error if intersection detected
- [ ] T018 [US1] Implement Widget trait for Rect<const WIDTH, const HEIGHT> in src/widget/rect_widget.rs with associated constants (const WIDTH = WIDTH; const HEIGHT = HEIGHT;) and render_to() traversing children and calculating cumulative positions per data-model.md; when traversing into child, update RenderContext clip_bounds to intersection of current clip_bounds and child dimensions (constrains child writes to allocated region), then restore parent clip_bounds after child rendering completes before processing next sibling
- [ ] T020 [US1] Add Page::render(widget) method in src/page/mod.rs creating RenderContext and calling widget.render_to(context, (0,0)) per data-model.md page enhancement
- [ ] T021 [US1] Add integration test in tests/widget/composition.rs: verify parent with two children renders at correct relative positions (acceptance scenario 1)
- [ ] T022 [US1] Add integration test in tests/widget/composition.rs: verify three-level nesting hierarchy with cumulative position calculation (acceptance scenario 2)
- [ ] T023 [US1] Add integration test in tests/widget/composition.rs: verify developer does not need manual absolute coordinate calculation (acceptance scenario 3)
- [ ] T023A [US1] Add deep nesting integration test in tests/widget/integration.rs: verify 10-level nested Rect hierarchy with cumulative position calculation - create hierarchy where each level offsets by (5, 5), verify deepest child renders at position (45, 45) absolute coordinates, manually validate cumulative offset math (validates FR-013 arbitrary depth requirement)

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Automatic Boundary Enforcement (Priority: P1)

**Goal**: Ensure child widgets are constrained to their parent's boundaries so that layout bugs (children escaping bounds) are detected and reported, allowing developers to handle violations gracefully

**Independent Test**: Render a child widget that extends beyond its parent's bounds and verify that an error is returned indicating the boundary violation

### Implementation for User Story 2

- [ ] T024 [US2] Add ChildExceedsParent validation in Rect::add_child() in src/widget/rect_widget.rs checking if child_right > WIDTH or child_bottom > HEIGHT using widget.WIDTH/HEIGHT constants (acceptance scenario 1)
- [ ] T025 [US2] Add OutOfBounds validation in Rect::add_child() in src/widget/rect_widget.rs checking if position places child outside parent WIDTH/HEIGHT bounds (acceptance scenario 2)
- [ ] T026 [US2] Add zero-size prevention in Rect::new() in src/widget/rect_widget.rs using debug_assert!(WIDTH > 0 && HEIGHT > 0, "Zero-size parent widget") per Constitution Principle VI validation hierarchy; note: ZeroSizeParent error variant removed from RenderError per FR-007 - validation is compile-time (const generic constraints) + development-time (debug_assert!) only (acceptance scenario 3)
- [ ] T027 [US2] Add boundary test in tests/widget/boundary.rs: verify ChildExceedsParent error when child widget with WIDTH=20, HEIGHT=20 added to Rect<10, 10> parent (acceptance scenario 1)
- [ ] T028 [US2] Add boundary test in tests/widget/boundary.rs: verify OutOfBounds error when child positioned at (8,8) with size (5×5) in Rect<10, 10> parent (acceptance scenario 2)
- [ ] T029 [US2] Add boundary test in tests/widget/boundary.rs: verify debug_assert! panic in debug build when creating Rect::new() with WIDTH=0 or HEIGHT=0 (if const generics allow instantiation); add compile-time test attempting Rect::<0, 10>::new() to verify compiler error; document that const generic constraints provide primary defense (acceptance scenario 3)
- [ ] T029A [P] [US2] Add Label constraint test suite in tests/widget/boundary.rs: **(1) HEIGHT constraint**: verify debug_assert! panic in debug build when creating Label::<20, 2>::new() with HEIGHT ≠ 1, verify panic message contains "Label HEIGHT must be 1"; **(2) Text content constraint**: verify Label::<20, 1>::new().add_text("Line1\nLine2") returns Err(TextExceedsWidth) in both debug and release builds when text contains newlines; verify Label::<20, 1>::new().add_text("Valid text") succeeds; **(3) Valid usage**: verify Label::<20, 1>::new().add_text("text") succeeds (HEIGHT==1 and single-line text); validates FR-007C two-layer enforcement per Constitution Principle VI validation hierarchy and Principle IX Documented Constraint Violations (analysis finding C1)
- [ ] T029B [P] [US2] Add Label rustdoc documentation in src/widget/label.rs: **(1) Label::new() `# Panics` section**: document HEIGHT must always be 1, panic occurs in debug builds if HEIGHT ≠ 1 via debug_assert!, release build behavior undefined if violated; **(2) add_text() `# Errors` section**: document returns TextExceedsWidth if text.len() > WIDTH or text contains newlines (`\n`, `\r\n`); **(3) Examples**: show correct usage `Label::<20, 1>::new().add_text("Hello")?.bold()` or `label_new!(20).add_text("Hello")?`, show compile_fail examples for HEIGHT ≠ 1 and multi-line text (Constitution Principle XIV documentation requirement; analysis finding C1)
- [ ] T030 [US2] Add error context validation test in tests/widget/boundary.rs: verify all error variants include contextual information (sizes, positions, bounds) per data-model.md
- [ ] T030A [US2] Add edge case test in tests/widget/boundary.rs: verify IntegerOverflow error when adding child at position causing u16 overflow during composition phase (edge case from spec.md L102)
- [ ] T030B [US2] Add edge case test in tests/widget/boundary.rs: verify IntegerOverflow error when deep nesting (e.g., 1000+ levels each offsetting by large values) causes cumulative coordinate overflow during render phase tree traversal (edge case from spec.md L102 - validates cumulative coordinate calculation safety)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Layout Components Returning Nested Rectes (Priority: P2)

**Goal**: Provide Layout components (Column, Row, Stack) that extract parent Rect size and return nested Rect widgets with calculated dimensions, so that developers can build structured layouts by positioning children within the returned Rectes

**Independent Test**: Create a Column layout component, obtain nested Rect widgets from it, position children within those Rectes, and verify the entire hierarchy renders correctly

### Implementation for User Story 3

- [ ] T031 [P] [US3] Define Column<const WIDTH: u16, const HEIGHT: u16> struct in src/widget/layout/column.rs with current_y: u16 field per data-model.md
- [ ] T032 [P] [US3] Define Row<const WIDTH: u16, const HEIGHT: u16> struct in src/widget/layout/row.rs with current_x: u16 field per data-model.md
- [ ] T033 [P] [US3] Define Stack<const WIDTH: u16, const HEIGHT: u16> struct in src/widget/layout/stack.rs per data-model.md
- [ ] T034 [US3] Implement Column::new() in src/widget/layout/column.rs initializing current_y to 0 per data-model.md
- [ ] T035 [US3] Implement Column::area<const H: u16>() in src/widget/layout/column.rs returning (Rect<WIDTH, H>, (u16, u16)) with InsufficientSpace error if current_y + H exceeds HEIGHT per data-model.md
- [ ] T036 [US3] Implement Row::new() in src/widget/layout/row.rs initializing current_x to 0 per data-model.md
- [ ] T037 [US3] Implement Row::area<const W: u16>() in src/widget/layout/row.rs returning (Rect<W, HEIGHT>, (u16, u16)) with InsufficientSpace error if current_x + W exceeds WIDTH per data-model.md
- [ ] T038 [US3] Implement Stack::new() in src/widget/layout/stack.rs per data-model.md
- [ ] T039 [US3] Implement Stack::area() in src/widget/layout/stack.rs returning (Rect<WIDTH, HEIGHT>, (0, 0)) for overlapping layers per data-model.md
- [ ] T039A [P] [US3] Define column_new! macro in src/widget/layout/column.rs that expands column_new!(W, H) to Column::<W, H>::new() for clean API syntax
- [ ] T039B [P] [US3] Define row_new! macro in src/widget/layout/row.rs that expands row_new!(W, H) to Row::<W, H>::new() for clean API syntax
- [ ] T039C [P] [US3] Define stack_new! macro in src/widget/layout/stack.rs that expands stack_new!(W, H) to Stack::<W, H>::new() for clean API syntax
- [ ] T039D [P] [US3] Define column_area! macro in src/widget/layout/column.rs that expands column_area!(col, H) to col.area::<H>() for clean API syntax
- [ ] T039E [P] [US3] Define row_area! macro in src/widget/layout/row.rs that expands row_area!(row, W) to row.area::<W>() for clean API syntax
- [ ] T040 [US3] Add layout module re-exports in src/widget/layout/mod.rs (Column, Row, Stack, column_new!, row_new!, stack_new!, column_area!, row_area!)
- [ ] T041 [US3] Add Column layout test in tests/widget/layouts.rs: verify three nested Rect widgets from Column<80, 30>::new().area::<10>() calls positioned at rows 0, 10, 20 (acceptance scenario 1)
- [ ] T042 [US3] Add Row layout test in tests/widget/layouts.rs: verify two nested Rect widgets from Row<20, HEIGHT>::new().area::<W>() calls positioned at columns 0 and 5 (acceptance scenario 2)
- [ ] T043 [US3] Add Stack layout test in tests/widget/layouts.rs: verify Stack<WIDTH, HEIGHT>::new().area() overlapping nested rectes all positioned at (0,0) with later rectes obscuring earlier ones (acceptance scenario 3)
- [ ] T044 [US3] Add InsufficientSpace error test in tests/widget/layouts.rs: verify Column and Row return error when area() exceeds remaining space
- [ ] T044A [US3] Add edge case test in tests/widget/layouts.rs: verify Layout::area() returns InsufficientSpace error when requested size exceeds parent Rect dimensions (edge case from spec.md L99)
- [ ] T044B [US3] Add edge case test in tests/widget/layouts.rs: verify developer can manually add children to parent Rect after Layout component created nested Rectes without errors (edge case from spec.md L100-101)
- [ ] T044C [US3] Add edge case test in tests/widget/layouts.rs: verify nested Rectes returned by Layouts can themselves use Layout components for deep nesting (edge case from spec.md L101)

**Checkpoint**: All P1-P2 user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories, plus comprehensive testing for Constitutional compliance

- [ ] T051 [P] Create comprehensive example in examples/widget_composition.rs demonstrating multi-level nesting with Column and Row layouts using macro API (rect_new!, column_new!, etc.) per project structure in plan.md
- [ ] T052 [P] Add property-based test in tests/widget/integration.rs: verify arbitrary widget trees (depth 1-20, width 1-10 children) produce no panics and correct cumulative coordinates using proptest (constitutional zero-panic guarantee + FR-013 arbitrary depth validation)
- [ ] T053 [P] Add property-based test in tests/widget/integration.rs: verify arbitrary widget trees produce deterministic output across 1000 iterations (constitutional deterministic behavior)
- [ ] T054 [P] Add criterion benchmark in benches/widget_traversal.rs: measure traversal overhead for trees with 10, 50, 100, 500 widget nodes (validate < 100 μs p99 target per Constitution Principle XI; benchmark MUST fail if p99 exceeds target)
- [ ] T054A [P] Add memory profiling task in benches/widget_memory.rs: measure total page memory usage with criterion-based memory profiling for typical widget hierarchies (5-level nesting, 10-20 widgets); verify total page memory (including widget tree, character grid, style data, and all page allocations) remains < 128 KB per page per Constitution Principle X and plan.md validation requirements
- [ ] T055 [P] Add golden master tests in tests/integration/: add complex composed layout test cases (3-column invoice, nested tables) per research.md testing strategy
- [ ] T055A [P] Add unit tests for Rect<WIDTH, HEIGHT> in tests/widget/rect_tests.rs: verify Rect::new() with valid dimensions, verify add_child() boundary checks (ChildExceedsParent, OutOfBounds), verify overlap detection (OverlappingChildren via AABB), verify Widget trait implementation, verify render_to() tree traversal and cumulative coordinate calculation (Constitution Principle XII: 100% public API coverage)
- [ ] T055B [P] Add unit tests for Column layout in tests/widget/column_tests.rs: verify Column::new() initialization, verify area::<H>() returns correct Rect<WIDTH, H> and position, verify InsufficientSpace error when exceeding HEIGHT, verify current_y tracking, verify macro column_new! and column_area! expansions
- [ ] T055C [P] Add unit tests for Row layout in tests/widget/row_tests.rs: verify Row::new() initialization, verify area::<W>() returns correct Rect<W, HEIGHT> and position, verify InsufficientSpace error when exceeding WIDTH, verify current_x tracking, verify macro row_new! and row_area! expansions
- [ ] T055D [P] Add unit tests for Stack layout in tests/widget/stack_tests.rs: verify Stack::new(), verify area() returns overlapping Rect<WIDTH, HEIGHT> at (0,0), verify multiple area() calls all return (0,0) position, verify macro stack_new! expansion
- [ ] T055E [P] Add unit tests for Page::render in tests/widget/page_tests.rs: verify Page::render() accepts &impl Widget, verify render starts at (0,0), verify widgets can be rendered multiple times (immutable borrow), verify RenderError propagation from widget tree
- [ ] T055F [P] Add fuzzing target in fuzz/fuzz_targets/widget_tree.rs: arbitrary widget tree generation (random depth, random child counts, random positions), verify no panics in composition or rendering, target 1M+ iterations per Constitution Principle XII
- [ ] T055G [P] Add integration test in tests/widget/assumptions.rs: verify Widget::render_to uses &self (immutable borrow assumption), verify widgets can be rendered multiple times (call page.render() twice on same widget), verify no interior mutability required, verify single-pass rendering (no multiple traversals required) - validates spec.md Assumptions section lines 160-170
- [ ] T055H [P] Add hardware validation test plan document in tests/hardware/validation_plan.md: specify 10 test forms to print on EPSON LQ-2090II, include widget composition examples (nested rectes, layouts, labels), define visual inspection criteria per Constitution Principle XII
- [ ] T055I [P] Add SC-005 code reduction comparison test in tests/widget/code_reduction_test.rs: create two implementations of same complex layout (e.g., 3-column invoice with Column containing multiple Rows, each with multiple Labels) - one using manual positioning with explicit coordinates, one using composability API; measure lines of code (LOC) for each; verify composability API achieves ≥50% LOC reduction compared to manual positioning (validates Success Criterion SC-005 from spec.md; analysis finding C3)
- [ ] T056 [P] Update CLAUDE.md with widget composability technologies and commands (already auto-generated but verify accuracy)
- [ ] T057 [P] Add API documentation with examples to all public traits and structs in src/widget/ using /// doc comments (target: 100% public API rustdoc coverage per Constitution Principle XIV); include code examples demonstrating both turbofish and macro wrapper syntaxes; document panics, errors, and validation strategy (compile-time vs runtime) for each API
- [ ] T057A [P] Run cargo doc --no-deps to generate documentation, then run cargo deadlinks or similar tool to validate all documentation links are valid (Constitution Principle XIV compliance; analysis finding C4)
- [ ] T058 Run cargo clippy and fix all warnings across src/widget/
- [ ] T059 Run cargo fmt across src/widget/
- [ ] T060 Run cargo test and ensure all tests pass (Constitution Principle XII: ≥90% line coverage target)
- [ ] T060A Run cargo test --release and ensure all tests pass (verify debug_assert! behavior differs between debug/release builds per Principle VI validation strategy)
- [ ] T061 Validate quickstart.md examples are executable: compile and run all code snippets from quickstart.md
- [ ] T061A Run cargo tarpaulin or similar coverage tool and verify ≥90% line coverage (Constitution Principle XII compliance); identify any uncovered code paths and add targeted tests

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - User Story 1 (P1) and User Story 2 (P1) can proceed in parallel after Phase 2
  - User Story 3 (P2) can start after Phase 2, may reference US1 Rect implementation
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Enhances US1 but independently testable
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Uses Rect from US1 but independently testable

### Within Each User Story

**User Story 1**:

- T015-T020 sequential (define Rect<const WIDTH, const HEIGHT> → implement methods → implement Widget trait → add Page::render)
- T021-T023A can run in parallel after T015-T020

**User Story 2**:

- T024-T026 sequential (add validations to existing Rect<WIDTH, HEIGHT>)
- T027-T030 can run in parallel after T024-T026

**User Story 3**:

- T031-T033 can run in parallel (different files - Column, Row, Stack structs)
- T034-T039 must follow T031-T033 sequentially within each layout component
- T040 after T034-T039
- T041-T044 can run in parallel after T040

### Parallel Opportunities

- **Phase 1**: T002 and T003 can run in parallel after T001
- **Phase 2**: T005-T006 can run in parallel after T004; T010-T011 can run in parallel after T008-T009; T012-T013 can run in parallel (Label struct and methods)
- **After Phase 2 completes**: User Stories 1, 2, and 3 can all start in parallel if team capacity allows
- **User Story 3**: T031, T032, T033 can run in parallel (different layout components)
- **Phase 6**: T051-T057 can all run in parallel

---

## Parallel Example: User Story 1

```bash
# Phase 2: Launch Label tasks in parallel:
Task: "Define Label struct in src/widget/label.rs"
Task: "Implement Label::new(), Label::with_size(), bold(), underline()"

# After T015-T020 complete (User Story 1), launch all integration tests together:
Task: "Add integration test: verify parent with two children renders at correct relative positions"
Task: "Add integration test: verify three-level nesting hierarchy with cumulative position calculation"
Task: "Add integration test: verify developer does not need manual absolute coordinate calculation"
```

## Parallel Example: User Story 3

```bash
# Launch all layout struct definitions together (T031-T033):
Task: "Define Column struct in src/widget/layout/column.rs"
Task: "Define Row struct in src/widget/layout/row.rs"
Task: "Define Stack struct in src/widget/layout/stack.rs"

# After implementations complete, launch all layout tests together (T041-T044):
Task: "Add Column layout test in tests/widget/layouts.rs"
Task: "Add Row layout test in tests/widget/layouts.rs"
Task: "Add Stack layout test in tests/widget/layouts.rs"
Task: "Add InsufficientSpace error test in tests/widget/layouts.rs"
```

## Parallel Example: Phase 6 (Polish)

```bash
# Launch all polish tasks together (T051-T057):
Task: "Create comprehensive example in examples/widget_composition.rs"
Task: "Add property-based test for zero-panic guarantee"
Task: "Add property-based test for deterministic output"
Task: "Add criterion benchmark for traversal overhead"
Task: "Add golden master tests for complex layouts"
Task: "Update CLAUDE.md with widget composability"
Task: "Add API documentation with examples"
```

---

## Implementation Strategy

### MVP First (User Story 1 + User Story 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Nested Widget Composition)
4. Complete Phase 4: User Story 2 (Automatic Boundary Enforcement)
5. **STOP and VALIDATE**: Test US1 and US2 independently
6. Deploy/demo if ready - **Core composability system is functional**

**Rationale**: US1 and US2 are both P1 priority and provide the fundamental value proposition (composition + safety). This is the true MVP.

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 + User Story 2 → Test independently → Deploy/Demo (MVP! - Core composition with boundary safety)
3. Add User Story 3 → Test independently → Deploy/Demo (Layout helpers added)
4. Add Phase 6 (Polish) → Final validation and documentation

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - **Developer A**: User Story 1 (T015-T023)
   - **Developer B**: User Story 2 (T024-T030) - can start in parallel, adds to same files as US1
   - **Developer C**: User Story 3 (T031-T044) - independent module
3. Stories complete and integrate independently
4. Team collaborates on Phase 6 (Polish) with all parallel tasks

**Note**: US1 and US2 modify the same files (Rect<const WIDTH, const HEIGHT> in src/widget/rect_widget.rs), so coordinate to avoid merge conflicts or run sequentially.

---

## Task Count Summary

- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (Foundational)**: 24 tasks (includes Label widget + macros + comprehensive unit tests + PageBuilder Layer 3 validation tests - T004-T014G, T014E1-T014E3)
- **Phase 3 (US1 - P1)**: 9 tasks (T015-T023A, T019 removed as redundant, T023A added for deep nesting)
- **Phase 4 (US2 - P1)**: 11 tasks (T024-T030B, includes IntegerOverflow edge cases for composition and render phases, T029A expanded for comprehensive Label HEIGHT constraint testing, T029B for rustdoc)
- **Phase 5 (US3 - P2)**: 22 tasks (includes layout macros and edge case tests - T031-T044C)
- **Phase 6 (Polish)**: 24 tasks (includes comprehensive unit tests for all components, fuzzing, hardware validation, memory profiling, SC-005 code reduction test, cargo doc validation - T051-T061A, T055I, T057A)

**Total**: 93 tasks (previously 99, reduced by 6 after removing backward compatibility user story)

**Test Task Count**: 36+ test tasks (Constitution Principle XII compliance: unit tests, integration tests, property-based tests, fuzzing, golden master tests, hardware validation, coverage verification, code reduction validation)

**New Tasks Added (2025-11-19)**: Based on /speckit.analyze findings:

- **T014E1**: PageBuilder Layer 3 validation test (validates FR-004 Three-Layer Architecture Layer 3 delegation; analysis finding C2)
- **T014E2**: PageBuilder Layer 3 integration test (validates PageBuilder behavior unchanged after RenderContext; analysis finding C2)
- **T014E3**: PageBuilder Layer 3 property-based test (validates silent truncation with arbitrary text; analysis finding C2)
- **T014F Updated**: Added FR-009 leaf widget compile_fail test (analysis finding G1)
- **T029A Expanded**: Label HEIGHT constraint comprehensive test suite (validates FR-007C single-line Label enforcement; analysis finding C1)
- **T029B**: Label HEIGHT rustdoc `# Panics` section (Constitution Principle XIV documentation; analysis finding C1)
- **T055I**: SC-005 code reduction comparison test (validates Success Criterion SC-005; analysis finding C3)
- **T057A**: cargo doc --no-deps validation (Constitution Principle XIV compliance; analysis finding C4)
- **T057 Updated**: Enhanced to require both turbofish and macro syntax examples, validation strategy documentation
- **T014E Updated**: Added clip_bounds intersection algorithm correctness validation (analysis finding U1)

### Tasks per User Story

- **US1 (Nested Widget Composition)**: 9 tasks (T015-T023A, T019 removed, T023A added)
- **US2 (Automatic Boundary Enforcement)**: 11 tasks (T024-T030B, includes T029A expanded and T029B for Label HEIGHT constraint)
- **US3 (Layout Components)**: 22 tasks (T031-T044C, includes layout macros and edge cases)

### MVP Scope (US1 + US2 only)

- **MVP Task Count**: 3 (Setup) + 24 (Foundational with Label + macros + unit tests + Layer 3 validation) + 9 (US1) + 11 (US2) = **47 tasks**
- **MVP Delivers**: Core widget composition with automatic coordinate calculation + boundary safety with Result-based error handling + Label<WIDTH, HEIGHT> widget for text rendering with content validation (including HEIGHT==1 constraint enforcement with comprehensive test suite and rustdoc) + Rect<WIDTH, HEIGHT> container widget (compile-time const generic sizing) + Clean macro API (rect_new!, label_new!) + TextExceedsWidth error for constitution compliance + Deep nesting validation (10+ levels) + Comprehensive unit tests for foundational components (Widget trait, RenderError, RenderContext with clip_bounds intersection algorithm, Label with HEIGHT constraint and leaf widget validation, macros) + Three-Layer Validation Architecture verification (Widget Construction → RenderContext → PageBuilder) with unit, integration, and property-based tests ensuring Constitutional compliance

---

## API Usage Example (Macro Wrapper Syntax)

All widgets use const generic parameters for compile-time sizing, with macro wrappers providing clean ergonomic API:

```rust
// Create widgets with clean macro syntax (expands to const generics)
let mut root = rect_new!(80, 30);           // Expands to Rect::<80, 30>::new()
let label = label_new!(20)                 // Expands to Label::<20, 1>::new()
    .add_text("Hello")?                    // Builder pattern, returns Result
    .bold();                               // Styling method
root.add_child(label, (0, 0))?;

// Create layout helpers with clean macro syntax
let mut column = column_new!(80, 30);              // Expands to Column::<80, 30>::new()
let (rect1, pos1) = column_area!(column, 10)?;     // Expands to column.area::<10>()?
let (rect2, pos2) = column_area!(column, 20)?;     // Expands to column.area::<20>()?

root.add_child(rect1, pos1)?;
root.add_child(rect2, pos2)?;

// Alternative: Direct turbofish syntax (also supported)
let mut root = Rect::<80, 30>::new();
let label = Label::<20, 1>::new()          // Constructor returns empty Label
    .add_text("Hello")?                    // Builder pattern, returns Result
    .bold();                               // Styling method
let mut column = Column::<80, 30>::new();
let (rect1, pos1) = column.area::<10>()?;
```

**Key Points:**

- Dimensions specified at type level using const generic parameters
- Macro wrappers (`rect_new!`, `label_new!`, `column_new!`, etc.) provide clean ergonomic API
- Macros expand to turbofish syntax at compile time
- Widget trait uses associated constants: `const WIDTH: u16; const HEIGHT: u16;` (Label uses HEIGHT=1)
- Layout `area()` macros (`column_area!`, `row_area!`) wrap generic method calls
- Both macro and direct turbofish syntax are supported
- Label uses builder pattern: `label_new!(WIDTH)` or `Label::<WIDTH, 1>::new()` returns empty Label, then `.add_text(text)?` adds content, then optional styling (`.bold()`, `.underline()`), then render

---

## Notes

- [P] tasks = different files, no dependencies - can run in parallel
- [Story] label (US1, US2, US3, US4) maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Tests are NOT generated per specification (not explicitly requested)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- All tasks follow strict checklist format: `- [ ] [TaskID] [P?] [Story?] Description with file path`

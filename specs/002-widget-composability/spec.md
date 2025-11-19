# Feature Specification: Widget Composability System

**Feature Branch**: `002-widget-composability`
**Created**: 2025-11-18
**Status**: Draft
**Input**: User description: "add React-like composability to the widget system while keeping the ESC/P page/backing grid model. Constraints: keep the rendering backend (Page/PageBuilder/Region) intact; avoid dynamic sizing if not needed, but enable parent/child composition and nesting."

## Clarifications

### Session 2025-11-19 (Memory Constraint)

- Q: How should the memory constraint scope be defined - is "< 16 KB per page" for widget tree overhead only or total page memory including everything? → A: < 128 KB per page including everything (total page memory including widget tree, character grid, style data, and all page-related allocations)
- Q: What is the observability strategy for widget composition failures in production - should the library log errors or remain logging-agnostic? → A: Errors bubble to caller with context, no logging (library code should not log)
- Q: Is backward compatibility needed for existing widgets, or can all widgets be updated to the new Widget trait? → A: No backward compatibility needed; project is in design phase, all existing widgets can be updated directly to new Widget trait
- Q: What memory allocation strategy should be used for widget tree children storage - SmallVec optimization or standard Rect heap allocation? → A: Rect allocates on heap, accept overhead for simplicity (optimize if needed later)

### Session 2025-11-18

- Q: How should the system handle boundary violations when a child widget's requested size exceeds its parent's allocated region? → A: Rendering returns a Result that developers must handle, allowing graceful recovery
- Q: How should containers allocate space among children - dynamic division or fixed sizes? → A: Widgets are static with fixed declared sizes
- Q: How should the Region concept relate to the widget system? → A: Replace Region with Rect widget; Widget is the only container/component type (eliminate separate Region concept)
- Q: How specific should rendering error types be? → A: Specific error variants for each violation type (ChildExceedsParent, OutOfBounds, OverlappingChildren, etc.); ZeroSizeParent enforced via debug_assert! (development-time) + future const generic constraints (compile-time when stable in Rust 1.75+)
- Q: How should child widgets specify their position within a Rect parent? → A: Relative positioning in character-grid units (column, row offset from parent origin)
- Q: How should the Widget system interact with the existing Page/PageBuilder backend? → A: RenderContext wraps PageBuilder; widgets call context methods which translate coordinates and delegate to PageBuilder
- Q: How should boundary clipping be enforced when widgets render? → A: Pre-validate before delegation; RenderContext checks bounds before calling PageBuilder and returns error if out-of-bounds write attempted
- Q: What is the API design for widget composition and rendering? → A: Two-phase system: (1) Composition phase - `parent.add_child(child, (x,y))?` builds widget tree with positions stored; (2) Render phase - `page.render(parent)` traverses tree and draws everything to PageBuilder at cumulative positions, always starting at (0,0)
- Q: How do Layout components work with Areas and child widget construction? → A: Layout components extract parent Rect size, divide into Areas (containing size+position), Areas passed to child widget constructors which automatically append children to parent at Area's position
- Q: What type do Layout components return when creating areas? → A: Layouts return Rect<WIDTH, HEIGHT> widgets (not separate Area objects); returned Rect widgets support `rect.add_child(child, (x,y))?` for positioning children, creating nested Rect tree structure

### Session 2025-11-19

- Q: How do Layout components specify dimensions for returned Rect widgets given const generic constraints? → A: Layout components use generic methods requiring turbofish syntax at call site (e.g., `column.area::<10>()` for height 10), preserving compile-time size specification without runtime-sized types
- Q: Should Widget trait use const generic parameters or associated constants for WIDTH/HEIGHT? → A: Widget trait MUST define WIDTH and HEIGHT as associated constants (const WIDTH: u16; const HEIGHT: u16;)
- Q: Should overlapping children be validated during composition? → A: Yes, overlapping children is an error condition that must be detected and reported
- Q: What validation strategy should be used - compile-time or runtime? → A: Prefer compile-time validation wherever possible; use debug_assert! for development-time checks; avoid runtime validation overhead
- Q: Can Label widget support multiple lines of text? → A: No, Label can only be 1 line (HEIGHT must be 1)
- Q: What syntax should developers use to create Rect and other widgets? → A: All widgets use const generic parameters for WIDTH and HEIGHT. Developers can use either (1) turbofish syntax: `Rect::<80, 30>::new()`, `Label::<20, 1>::new().add_text("Hello")?`, or (2) macro wrappers for cleaner syntax: `rect_new!(80, 30)`, `label_new!(20).add_text("Hello")?` (macro automatically sets HEIGHT=1). Both approaches are equivalent - macros expand to turbofish syntax at compile time. Widgets use builder pattern for configuration (e.g., Label has add_text(), bold(), underline() methods). Same pattern applies to Layout components: direct turbofish `Column::<80, 30>::new()` or macro `column_new!(80, 30)`.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Nested Widget Composition (Priority: P1)

As a developer, I need to compose multiple widgets into hierarchical structures (parent-child relationships) so that I can build complex UI layouts from simple, reusable components without manually managing coordinate offsets for each child.

**Why this priority**: This is the core value proposition - enabling composition is the foundation that all other features depend on. Without this, developers must manually calculate positions for every nested element, which is error-prone and non-scalable.

**Independent Test**: Can be fully tested by creating a simple parent widget containing two child widgets, rendering it, and verifying that children appear at correct relative positions within the parent's bounds.

**Acceptance Scenarios**:

1. **Given** a parent Rect widget and two child widgets, **When** developer calls `parent.add_child(child1, (2, 3))?` and `parent.add_child(child2, (5, 7))?` then `page.render(parent)`, **Then** child1 appears at (2,3) and child2 at (5,7) relative to parent origin
2. **Given** a three-level nesting hierarchy built via composition API, **When** `page.render(grandparent)` is called, **Then** Page calculates cumulative positions and renders deeply nested child at correct absolute position without developer intervention
3. **Given** a Rect widget with children added via `parent.add_child(child, pos)?`, **When** render phase executes, **Then** developer did not need to manually calculate absolute coordinates for any child

---

### User Story 2 - Automatic Boundary Enforcement (Priority: P1)

As a developer, I need child widgets to be constrained to their parent's boundaries so that layout bugs (children escaping bounds) are detected and reported, allowing me to handle violations gracefully.

**Why this priority**: Critical safety feature that prevents rendering corruption and maintains layout integrity. Result-based error handling enables developers to catch sizing mistakes early while maintaining control over error recovery strategies.

**Independent Test**: Can be fully tested by rendering a child widget that extends beyond its parent's bounds and verifying that an error is returned indicating the boundary violation.

**Acceptance Scenarios**:

1. **Given** a parent Rect widget with 10x10 bounds and a child widget with width=20, height=20, **When** developer calls `parent.add_child(child, (0,0))`, **Then** composition returns ChildExceedsParent error
2. **Given** a parent Rect widget with 10x10 bounds and a child positioned at (8, 8) with size 5x5, **When** developer calls `parent.add_child(child, (8,8))`, **Then** composition returns OutOfBounds error (child extends beyond parent)
3. **Given** a parent Rect widget with width=0 or height=0, **When** developer attempts to instantiate it, **Then** debug_assert! panics in debug builds (primary enforcement); developers MUST NOT create zero-size Rect widgets per project policy (undefined behavior in release builds)

---

### User Story 3 - Layout Components Returning Nested Rectes (Priority: P2)

As a developer, I need Layout components (Column, Row, Stack) that extract parent Rect size and return nested Rect widgets with calculated dimensions, so that I can build structured layouts by positioning children within the returned Rectes.

**Why this priority**: Provides practical value on top of the composition foundation. While developers could manually position children using P1 features (Rect with explicit positions), Layout components accelerate common use cases by automating space division and returning pre-sized, pre-positioned nested Rectes.

**Independent Test**: Can be fully tested by creating a Column layout component, obtaining nested Rect widgets from it, positioning children within those Rectes, and verifying the entire hierarchy renders correctly.

**Acceptance Scenarios**:

1. **Given** a Rect widget (height=30) and Column layout component, **When** developer calls `column = Column::<80, 30>::new()`, obtains three nested Rect widgets via `(rect1, pos1) = column.area::<10>()`, `(rect2, pos2) = column.area::<10>()`, `(rect3, pos3) = column.area::<10>()`, adds them to parent via `parent.add_child(rect1, pos1)?`, `parent.add_child(rect2, pos2)?`, `parent.add_child(rect3, pos3)?`, then positions children within each rect via `rect1.add_child(label_a, (0,0))?`, `rect2.add_child(label_b, (0,0))?`, `rect3.add_child(label_c, (0,0))?`, **Then** labels appear in parent at rows 0, 10, 20
2. **Given** a Rect widget (width=20) and Row layout component, **When** developer obtains two nested Rect widgets (`(rect1, pos1) = row.area::<5>()`, `(rect2, pos2) = row.area::<15>()`), positions children within them, **Then** children are automatically positioned at columns 0 and 5 within parent Rect
3. **Given** a Rect widget and Stack layout component, **When** developer obtains overlapping nested Rectes via `stack.area::<WIDTH, HEIGHT>()` calls, positions children within returned Rectes, **Then** all children render at (0,0) in parent with later Rectes obscuring earlier ones

---

### Edge Cases

- What happens when a child widget is positioned completely outside its parent widget's bounds? (Expected: returns OutOfBounds error indicating boundary violation)
- How does the system handle deeply nested widget hierarchies (e.g., 10+ levels)? (Expected: correct rendering with cumulative offset calculation, no artificial depth limit imposed beyond Rust call stack limits (~1MB default, ~250K nesting levels theoretically), or error if any level violates bounds)
- What happens when a parent widget has zero width or height? (Expected: **Validation Strategy**: Zero-size Rect widgets use Constitution Principle VI validation hierarchy: (1) **Compile-time prevention (not yet available)**: Const generic value constraints would prevent `Rect::<0, H>` and `Rect::<W, 0>` instantiation but require unstable Rust feature `generic_const_exprs` with no stabilization timeline (not available in Rust 1.75+ stable channel); this is a future enhancement only. (2) **Development-time enforcement (PRIMARY MECHANISM for Rust 1.75+)**: `debug_assert!(WIDTH > 0 && HEIGHT > 0, "Zero-size Rect")` in `Rect::new()` panics in debug builds, providing immediate feedback during development with zero runtime cost in release builds per Principle VI; (3) **Compile test T029**: Verifies that `Rect::<0, 10>::new()` compiles successfully in Rust 1.75+ stable, confirming debug_assert! is the active enforcement mechanism. **Project Policy**: Zero-size Rect widgets are PROHIBITED and developers MUST NOT instantiate them (documented in API rustdoc with `# Panics` section). **Release build behavior**: Zero-size Rect instantiation in release builds has undefined behavior per Constitution Principle IX.)
- How does the system handle coordinate overflow with very large offset values? (Expected: returns IntegerOverflow error if resulting coordinates exceed valid bounds)
- What happens when two or more children overlap within a parent Rect? (Expected: returns OverlappingChildren error detected via AABB intersection check during composition; touching edges without intersection does NOT count as overlap per FR-005A formula using strict inequality)
- What happens when a Layout component tries to create a nested Rect larger than the parent Rect? (Expected: Layout returns InsufficientSpace error indicating insufficient space)
- What happens if developer manually adds a child to parent Rect after Layout component created nested Rectes? (Expected: allowed, but overlap detection will return OverlappingChildren error if new child intersects existing children)
- Can nested Rectes returned by Layouts themselves use Layout components? (Expected: yes, Layouts can be applied to any Rect, enabling deep nesting)
- What happens when Label is instantiated with HEIGHT > 1 or passed multi-line text? (Expected: **HEIGHT ≠ 1**: Label<WIDTH, HEIGHT> allows HEIGHT as type parameter but Label::new() enforces HEIGHT==1 via debug_assert! which panics in debug builds if HEIGHT ≠ 1; release build behavior is undefined. **Multi-line text**: Label::add_text(text) validates text contains no newline characters (`\n`, `\r\n`) and returns Err(TextExceedsWidth) in both debug and release builds if validation fails. Developers MUST use HEIGHT=1 and single-line text per FR-007C.)

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST provide two-phase rendering: (1) composition phase where widgets are added to parents with positions (`parent.add_child(child, (x,y))?`) building an immutable widget tree, and (2) render phase where Page traverses the widget tree and outputs to PageBuilder (`page.render(parent)`). See Widget entity for associated constants specification.
- **FR-002**: System MUST automatically calculate cumulative widget coordinates during render phase by traversing the widget tree; Page translates child positions relative to parent origins without manual offset calculation by widget implementors
- **FR-003**: System MUST enforce boundary safety during both composition and render phases by returning Result errors for boundary violations (ChildExceedsParent, OutOfBounds) per Constitution Principle III Widget Exception; composition phase validates children don't exceed parent bounds; render phase validates content writes stay within widget bounds; zero-size parents prevented via debug_assert! (development-time check) with const generics providing compile-time prevention
- **FR-004**: RenderContext MUST validate that write positions are within clip bounds and return OutOfBounds error if position exceeds bounds; text content that extends beyond bounds after a valid start position is truncated character-by-character by PageBuilder per Constitution Principle III (silent truncation for PageBuilder, errors only for invalid start positions in RenderContext). **Three-Layer Validation Architecture**:
  - **Layer 1 (Widget Construction - `Label::add_text`)**: Validates text.len() ≤ WIDTH before widget creation; returns `Result::Err(TextExceedsWidth)` if violated; enables early detection at composition time
  - **Layer 2 (Render Phase - `RenderContext`)**: Validates write start position within clip_bounds during render phase; returns `Result::Err(OutOfBounds)` if violated; enforces boundary safety for widget tree
  - **Layer 3 (Underlying - `PageBuilder`)**: Silently truncates content extending beyond bounds after valid start position (no error returned); maintains hardware compatibility (ESC/P compliance)
  - **Layer 2→3 Delegation Rules** (precise boundary specification):
    - **RenderContext returns OutOfBounds error when**: Write start position (x, y) falls outside clip_bounds (x < clip_bounds.x OR x >= clip_bounds.x + clip_bounds.width OR y < clip_bounds.y OR y >= clip_bounds.y + clip_bounds.height)
    - **RenderContext delegates to PageBuilder when**: Write start position (x, y) is within clip_bounds; PageBuilder then handles any content overflow beyond bounds via silent character-by-character truncation
    - **Example**: clip_bounds=(0, 0, 80, 30), write_text("Hello", (79, 5)) → RenderContext delegates to PageBuilder → PageBuilder writes 'H' at column 79, silently truncates "ello" (columns 80-83 out of bounds)
    - **Example**: clip_bounds=(0, 0, 80, 30), write_text("Hello", (81, 5)) → RenderContext returns Err(OutOfBounds) without delegating (start position invalid)
  - **Responsibility Boundary**: Layers 1-2 return errors for widget violations per Constitution Principle III Widget Exception; Layer 3 maintains silent truncation for PageBuilder/Region per original Constitution Principle III
- **FR-005**: Parent widgets MUST support adding child widgets with explicit relative positions via composition API (`parent.add_child(child, (x,y))?`); positions are stored in the widget tree
- **FR-005A**: Composition phase MUST return Result to allow validation errors (child exceeds bounds, overlapping children detected via AABB intersection checks, etc.); overlapping children is an error condition that must be detected and reported using Axis-Aligned Bounding Rect (AABB) collision detection: two children overlap if rectangles intersect (child1_right > child2_left && child1_left < child2_right && child1_bottom > child2_top && child1_top < child2_bottom); touching edges (shared boundary without intersection) does NOT count as overlap per strict inequality in formula. All position and size calculations MUST use checked arithmetic (checked_add) to detect integer overflow, returning IntegerOverflow error when position + size exceeds u16::MAX; render phase cumulative coordinate calculation MUST also use checked arithmetic to prevent overflow during tree traversal. **AABB Edge Case Handling**: Negative coordinates are prevented by type system (positions use u16, cannot be negative); zero-size widgets prevented by debug_assert! in Rect::new() per FR-007 validation strategy (developers MUST NOT create zero-size widgets); AABB overlap check assumes valid widget dimensions (WIDTH > 0, HEIGHT > 0) enforced at construction time.
- **FR-006**: System MUST provide Layout components (Column, Row, Stack) that extract parent Rect size and return nested Rect widgets with specific dimensions based on layout rules
- **FR-006A**: Layout components MUST provide generic methods (e.g., `area::<H>()`) that return Rect<WIDTH, HEIGHT> widgets where developers specify dimensions via turbofish syntax at call site, preserving compile-time size specification (returned as tuple with position). Ergonomic macro wrappers MUST be provided to simplify common usage patterns: **column_area!(layout, H)** expands to `layout.area::<H>()` for Column layouts; **row_area!(layout, W)** expands to `layout.area::<W>()` for Row layouts. Macros preserve compile-time dimension specification and Result-based error handling. Example: `let (rect1, pos1) = column_area!(column, 10)?;` expands to `let (rect1, pos1) = column.area::<10>()?;`
- **FR-006B**: Layout components MUST return `(Rect<WIDTH, HEIGHT>, (u16, u16))` tuples containing a nested Rect widget and its calculated position; developers manually add the returned Rect to the parent via `parent.add_child(rect, pos)?`; returned Rect widgets support further child positioning via `rect.add_child(child, (x,y))?` for deep nesting
- **FR-006C**: All Rect widgets MUST support child positioning via `add_child(child, (x,y))?` calls, enabling nested Rect hierarchies
- **FR-007**: System MUST return Result with specific error variants for each invalid rendering condition. V1 RenderError enum has exactly these variants (marked #[non_exhaustive] for future expansion): ChildExceedsParent (child size exceeds parent bounds), OutOfBounds (positioning outside bounds), OverlappingChildren (child widgets overlap detected via intersection check), InsufficientSpace (layout widget cannot fit all children), IntegerOverflow (coordinate overflow), TextExceedsWidth (text content exceeds widget width). Additional variants may be added in V2+ without breaking changes due to #[non_exhaustive] attribute. **Note on ZeroSizeParent removal**: ZeroSizeParent error variant is NOT included in V1 RenderError because zero-size prevention uses the Constitution Principle VI validation hierarchy (formalized in Constitution v1.2.1, 2025-11-19):
  1. **Compile-time (preferred but not yet stable)**: const generic value constraints would prevent Rect::<0, H> and Rect::<W, 0> instantiation, but this requires unstable Rust features not available in Rust 1.75+
  2. **Development-time (PRIMARY MECHANISM)**: debug_assert!(WIDTH > 0 && HEIGHT > 0) in Rect::new() catches violations in debug builds with zero runtime cost in release builds per Principle VI validation strategy
  3. **Project Policy**: Zero-size Rect instantiation in release builds is undefined behavior and developers MUST NOT create zero-size widgets; compile test T029 verifies current compiler behavior
- **FR-007A**: Each error variant MUST provide contextual information about the violation (requested size, available size, widget identifiers)
- **FR-007B**: Widget constructors MUST validate content dimensions and return TextExceedsWidth error when text content exceeds widget WIDTH bounds per Constitution Principle III Widget Exception (amended 2025-11-19)
- **FR-007C**: Label widget MUST be single-line only (HEIGHT must always be 1); multi-line text support is out of scope for V1. **Enforcement Strategy**: Label<const WIDTH: u16, const HEIGHT: u16> struct created via turbofish syntax `Label::<20, 1>::new()` or macro `label_new!(20)` (macro automatically sets HEIGHT=1). Two-layer validation: **(1) HEIGHT parameter validation**: `Label::new()` enforces HEIGHT==1 via `debug_assert!(HEIGHT == 1, "Label HEIGHT must be 1")` which panics in debug builds if HEIGHT ≠ 1; **(2) Text content validation**: `Label::add_text(text)` validates text contains no newline characters (`\n`, `\r\n`) and returns `Result::Err(TextExceedsWidth)` if text.len() > WIDTH or contains newlines. Both violations align with Constitution Principle IX Documented Constraint Violations policy - panics/errors in debug builds (immediate developer feedback), undefined behavior in release builds if violated. Developers MUST use HEIGHT=1 and single-line text per API rustdoc `# Panics` documentation. Per Constitution Principle VI validation hierarchy (compile-time constraint not yet available in Rust 1.91.1+ stable; debug_assert! provides development-time safety with zero production cost in release builds).
- **FR-008**: Page MUST provide `render(&mut self, widget: &impl Widget) -> Result<(), RenderError>` method that traverses widget tree and outputs to PageBuilder; maintains compatibility with existing PageBuilder backend; always renders root widget at position (0,0); widgets can be rendered multiple times (immutable borrow)
- **FR-009**: Leaf widgets (widgets without children) MUST be able to render without providing child-related functionality
- **FR-010**: Layout widgets MUST enforce that children cannot write outside their allocated bounds via RenderContext pre-validation; RenderContext checks all write operations against clip_bounds and returns OutOfBounds error for violations (enforcement mechanism detailed in FR-004)
- **FR-011**: System MUST provide Rect<WIDTH, HEIGHT> widget as the primary container; Layout components (Column, Row, Stack) divide Rect space and return nested Rect widgets for automatic child positioning
- **FR-012**: System MUST handle coordinate calculations correctly for arbitrary widget nesting depths

### Key Entities

- **Widget**: Trait defining renderable components with WIDTH and HEIGHT as associated constants (`const WIDTH: u16; const HEIGHT: u16;`) for compile-time size specification and render_to method signature `fn render_to(&self, context: &mut RenderContext, position: (u16, u16)) -> Result<(), RenderError>`; implementations use const generic parameters to provide these associated constant values at the type level (example: `impl<const WIDTH: u16, const HEIGHT: u16> Widget for Rect<WIDTH, HEIGHT> { const WIDTH: u16 = WIDTH; const HEIGHT: u16 = HEIGHT; }`); supports two-phase rendering: composition (building tree) and output (Page traverses tree); all components implement Widget (Rect<WIDTH, HEIGHT>, Label<WIDTH, HEIGHT>, etc.); render_to uses immutable &self reference enabling multiple renders without rebuilding widget tree
- **Widget Tree**: In-memory structure built during composition phase; stores parent-child relationships and relative positions for each child; can have multiple levels of nested Rect widgets; traversed by Page during render phase. **Ownership Model**: Parent widgets own their children via `Rect<dyn Widget>` (heap allocation for trait objects; note that const generic widgets like Rect<80,30> are sized types and can be rected as dyn Widget trait objects). **Lifetime Model**: Widget tree is built once during composition phase (mutable operations via add_child) and then rendered immutably multiple times (Page::render borrows &impl Widget via `fn render(&mut self, widget: &impl Widget)`); root widget must outlive all render calls. **Concrete Type Specification**: `Rect<WIDTH, HEIGHT>` stores children as `Vec<WidgetNode>` where `WidgetNode` contains `widget: Rect<dyn Widget>` (owning pointer to heap-allocated trait object) and `position: (u16, u16)`. Single-threaded ownership model (no Rc/Arc required); aligns with plan.md lifetime hierarchy and Constitution Principle IV immutability guarantees.
- **RenderContext**: Struct used by Page during render phase to track cumulative positions while traversing widget tree; wraps PageBuilder and validates bounds before delegating writes, returning errors for out-of-bounds writes; provides public API for widget rendering (write_text, write_styled, bounds checking); clip_bounds initialized to full page (0, 0, 160, 51) and updated during tree traversal to enforce nested widget clipping. **Clip bounds intersection algorithm**: When Page traverses into a child widget at absolute position `(child_abs_x, child_abs_y)` with dimensions `(child_width, child_height)`, new clip_bounds calculated as: `new_clip_x = max(current_clip.x, child_abs_x)`, `new_clip_y = max(current_clip.y, child_abs_y)`, `new_clip_right = min(current_clip.x + current_clip.width, child_abs_x + child_width)`, `new_clip_bottom = min(current_clip.y + current_clip.height, child_abs_y + child_height)`, `new_clip_width = new_clip_right - new_clip_x`, `new_clip_height = new_clip_bottom - new_clip_y`; this constrains child to the intersection of parent's clip region and child's allocated region; after child rendering completes, clip_bounds is restored to parent's clip_bounds before processing next sibling.
- **Rect<WIDTH, HEIGHT> Widget**: Primary container widget with compile-time const generic dimensions; created via turbofish syntax `Rect::<80, 30>::new()` or macro wrapper `rect_new!(80, 30)` (both equivalent); stores children with explicit (x,y) positions set via `rect.add_child(child, (x,y))?`; Rect widgets can be nested within other Rect widgets to create hierarchical layouts; all dimensions are const generic type parameters known at compile time
- **Label<WIDTH, HEIGHT> Widget**: Leaf widget for rendering single-line text content with compile-time const generic WIDTH and HEIGHT dimensions. **Construction**: Created via turbofish syntax `Label::<20, 1>::new()` or macro wrapper `label_new!(20)` (macro automatically sets HEIGHT=1); uses builder pattern with `add_text(text) -> Result<Self, RenderError>` method that validates text length ≤ WIDTH and contains no newline characters, returning TextExceedsWidth error if violated; supports text styling methods `bold() -> Self` and `underline() -> Self`; renders text via RenderContext at given position. **HEIGHT Constraint**: Label<WIDTH, HEIGHT> accepts HEIGHT as a type parameter for Widget trait compatibility (all widgets require WIDTH and HEIGHT associated constants), but Label::new() enforces HEIGHT==1 via `debug_assert!(HEIGHT == 1, "Label HEIGHT must be 1")` which panics in debug builds if HEIGHT ≠ 1; **developers MUST always use HEIGHT=1** (e.g., turbofish: `Label::<20, 1>::new().add_text("text")?`, macro: `label_new!(20).add_text("text")?`). Using HEIGHT > 1 compiles but has undefined behavior in release builds per Constitution Principle VI validation hierarchy (compile-time constraint not yet available in Rust 1.91.1+ stable; debug_assert! provides development-time safety). **Text Constraint**: add_text() validates single-line text (no `\n` or `\r\n` characters) in both debug and release builds via Result-based error handling.
- **Layout Components**: Helper components (Column, Row, Stack) that extract parent Rect size and provide generic methods returning nested Rect<W, H> widgets with compile-time dimensions; created via turbofish syntax `Column::<80, 30>::new()` or macro wrapper `column_new!(80, 30)` (both equivalent); area allocation uses generic methods requiring turbofish at call site `column.area::<10>()` or macro wrapper `column_area!(column, 10)` (both equivalent); each returned Rect is automatically positioned within parent and can have its own children positioned via `rect.add_child(child, pos)?`; all dimensions are const generic type parameters known at compile time
- **Page**: Enhanced with `render(widget)` method that traverses widget tree and outputs to PageBuilder; calculates cumulative positions for all widgets in tree; always renders root widget at (0,0)
- **PageBuilder**: Existing rendering backend that manages the character grid (unchanged)
- **RenderError**: Error type with specific variants for rendering violations (ChildExceedsParent, OutOfBounds, OverlappingChildren, InsufficientSpace, IntegerOverflow, TextExceedsWidth) marked #[non_exhaustive] for future expansion; returned during both composition and render phases; includes contextual information about each violation. Note: ZeroSizeParent variant removed in favor of two-layer validation (compile-time const generic constraints + debug_assert! in Rect::new()) per Constitution Principle VI validation hierarchy.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Developers can create nested widget hierarchies at least 5 levels deep without manual coordinate calculation
- **SC-002**: Child widgets that exceed parent bounds are detected and reported via errors in 100% of cases
- **SC-003**: Container widgets correctly position fixed-size children according to layout rules (stacking, horizontal arrangement)
- **SC-004**: Developers can compose complex layouts (e.g., a Column containing multiple Rows, each with multiple Labels) in under 50% of the code compared to manual positioning
- **SC-005**: Boundary violations return appropriate errors in 100% of test cases, enabling graceful error handling; zero-size widgets prevented via debug_assert! (panics in debug builds, undefined behavior in release builds per documented usage patterns)
- **SC-006**: Documentation includes at least one compilable, executable example (in examples/ directory) of multi-level nested composition (≥3 nesting levels) that developers can run with `cargo run --example widget_composition` and modify; example MUST have test coverage in tests/widget/integration.rs verifying output correctness

## Assumptions

- The existing ESC/P page model and character grid remain the primary rendering model (no switch to pixel-based or dynamic layout)
- Widgets declare their size via const WIDTH and HEIGHT generic parameters at compile-time; all widget dimensions are fixed and specified at type level; no content-based auto-sizing or runtime measurement
- All widgets are created using const generic parameters: developers use either turbofish syntax (`Rect::<80, 30>::new()`, `Label::<20, 1>::new().add_text("text")?`) or equivalent macro wrappers (`rect_new!(80, 30)`, `label_new!(20).add_text("text")?`) for ergonomic API
- Label widget is restricted to single-line text only (**HEIGHT is always 1, enforced via debug_assert! in Label::new()**); Label uses builder pattern with add_text() method that validates text contains no newline characters (`\n`, `\r\n`) and returns TextExceedsWidth error if text exceeds WIDTH or contains newlines; developers MUST use HEIGHT=1 (e.g., turbofish: `Label::<20, 1>::new().add_text("text")?`, macro: `label_new!(20).add_text("text")?`); multi-line text rendering is deferred to future versions
- Widget is the only container/component type
- Two-phase rendering model: composition phase builds immutable widget tree with positions; render phase outputs tree to Page
- Widget trees are built once and can be rendered multiple times (widgets borrowed immutably via &impl Widget); no reactive re-rendering when widget state changes (widgets must be rebuilt for content updates)
- The primary use case is thermal receipt printing or similar fixed-width text output, not responsive web-style layouts
- Developers are comfortable with a React-like composition mental model (declarative parent-child hierarchies)
- Developers are comfortable with Rust turbofish syntax or macro wrappers for const generic type parameters
- Coordinate systems use character-grid units (column, row) where each position represents one character cell in the matrix
- Performance requirements are satisfied by single-pass rendering without optimization for thousands of widgets
- Total page memory usage (including widget tree, character grid, style data, and all page allocations) must not exceed 128 KB per page

## Constraints

- **MUST NOT** modify the fundamental Page/PageBuilder rendering backend
- **MUST NOT** introduce dynamic sizing or measurement of rendered content
- **MUST NOT** include logging or tracing in library code (errors bubble to caller with context; application code decides observability strategy)
- **MUST** use Widget as the only container/component abstraction (no separate Region concept)
- **MUST** use Result-based error handling for boundary violations and invalid rendering conditions
- **MUST** prefer compile-time validation over runtime validation wherever possible; use debug_assert! for development-time checks to avoid runtime overhead
- **SHOULD** minimize allocations and overhead in rendering hot paths
- **SHOULD** keep the API simple enough for straightforward migration from manual positioning

## Dependencies

- No external dependencies introduced; feature builds on existing Page/PageBuilder rendering backend
- Existing widgets may need minor updates to implement the new Widget trait signature with size declaration

## Out of Scope

- Dynamic sizing based on content measurement
- Reactive re-rendering when widget state changes
- CSS-like layout algorithms (flexrect, grid)
- Animation or transitions
- Responsive layouts that adapt to different page sizes
- Performance optimization for rendering thousands of widgets
- Visual design system or theming capabilities
- Input handling or interactivity

## Glossary

### Undefined Behavior (Release Builds)

When this specification states that certain API usage results in "undefined behavior in release builds", this means:

- **No panic guarantee**: The code will NOT panic (all `debug_assert!` checks are compiled out in release builds)
- **No specified outcome**: The library does not guarantee any particular behavior - widget may render incorrectly, produce garbage output, corrupt internal state, or appear to work correctly
- **No error returned**: Operations complete without returning `Result::Err` (silent undefined state)
- **Developer responsibility**: Developers violating documented API constraints (e.g., Label HEIGHT ≠ 1, zero-size Rect, multi-line Label text) accept full responsibility for consequences in release builds
- **Constitution compliance**: Aligns with Constitution Principle IX Documented Constraint Violations policy - debug_assert! provides development-time safety, release builds prioritize zero-panic guarantee over validation

**Examples of Undefined Behavior**:

- `Label::<20, 2>::new()` (HEIGHT ≠ 1) in release build: Compiles successfully, but widget behavior undefined - may render incorrectly, may corrupt layout, may cause out-of-bounds writes
- `Label::<20, 1>::new().add_text("Line1\nLine2")` (multi-line text) in release build: May render only first line, may render all lines overlapping, may corrupt layout - behavior not specified
- `Rect::<0, 10>::new()` in release build: May cause integer overflows in child calculations, may render nothing, may corrupt page state - behavior not specified

**Prevention**: Always test in debug builds during development to catch constraint violations via `debug_assert!` panics.

# Tasks: ESC/P2 Printer Driver

**Input**: Design documents from `/specs/001-escp2-printer-driver/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/printer-api.md, quickstart.md

**Tests**: NOT REQUESTED - No test tasks included per specification

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic Rust library structure

- [ ] T001 Create Cargo.toml with package metadata (name="escp-layout", edition="2021", rust-version="1.91.1")
- [ ] T002 Add thiserror dependency to Cargo.toml (required for error types)
- [ ] T003 Add optional dependencies to Cargo.toml (serde and tracing features)
- [ ] T004 Create src/lib.rs with module declarations and feature gates
- [ ] T005 [P] Create src/printer.rs placeholder for Printer struct
- [ ] T006 [P] Create src/errors.rs placeholder for error types
- [ ] T007 [P] Create src/types/ directory with mod.rs
- [ ] T008 [P] Create src/commands/ directory with mod.rs
- [ ] T009 [P] Create src/io/ directory with mod.rs
- [ ] T010 [P] Create tests/unit/ directory structure
- [ ] T011 [P] Create tests/integration/ directory structure
- [ ] T012 [P] Create examples/ directory for code examples
- [ ] T013 Configure .gitignore for Rust projects (target/, Cargo.lock for libraries)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until error types, basic I/O utilities, and Printer struct foundation are complete

### Error Types (Foundation for all operations)

- [ ] T014 Define ValidationError enum in src/errors.rs with MicroFeedZero variant
- [ ] T015 Add GraphicsWidthExceeded and GraphicsWidthMismatch variants to ValidationError in src/errors.rs
- [ ] T016 Add InvalidPageLength variant to ValidationError in src/errors.rs
- [ ] T017 Define PrinterError enum in src/errors.rs with Io, Permission, DeviceNotFound variants
- [ ] T018 Add Disconnected, Timeout, BufferFull variants to PrinterError in src/errors.rs
- [ ] T019 Add Validation(ValidationError) variant to PrinterError in src/errors.rs
- [ ] T020 Implement thiserror derives and error messages for all error variants in src/errors.rs
- [ ] T021 Implement From<io::Error> and From<ValidationError> for PrinterError in src/errors.rs

### Basic Types (Foundation for all commands)

- [ ] T022 [P] Define PrinterStatus struct in src/types/status.rs with online, paper_out, error fields
- [ ] T023 [P] Implement PrinterStatus::from_byte() parser in src/types/status.rs
- [ ] T024 [P] Define Font enum in src/types/font.rs (Roman, SansSerif, Courier, Script, Prestige)
- [ ] T025 [P] Define Pitch enum in src/types/pitch.rs (Pica, Elite, Condensed)
- [ ] T026 [P] Define GraphicsMode enum in src/types/graphics.rs (SingleDensity, DoubleDensity, HighDensity)
- [ ] T027 [P] Define LineSpacing enum in src/types/spacing.rs (Default, Custom(u8))
- [ ] T028 Export all types from src/types/mod.rs

### I/O Utilities (Foundation for all communication)

- [ ] T029 [P] Implement write_all_with_retry() in src/io/retry.rs for partial write handling
- [ ] T030 [P] Implement read_byte_with_timeout() in src/io/timeout.rs for status query timeout handling
- [ ] T031 [P] Create MockWriter for testing in src/io/mock.rs (cfg(test) only)
- [ ] T032 [P] Create MockReader for testing in src/io/mock.rs (cfg(test) only)
- [ ] T033 Export I/O utilities from src/io/mod.rs

### Printer Struct Foundation

- [ ] T034 Define Printer<W: Write, R: Read> struct in src/printer.rs with writer, reader, max_graphics_width fields
- [ ] T035 Implement Printer::new() constructor in src/printer.rs
- [ ] T036 Implement Printer::open_device() for File-based I/O in src/printer.rs with permission error handling
- [ ] T037 Implement Printer::send() low-level method using write_all_with_retry in src/printer.rs
- [ ] T038 Implement Printer::esc() low-level method for ESC/P2 commands in src/printer.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 5 - Error Recovery and Device Management (Priority: P2) 🎯 FOUNDATIONAL

**Goal**: Implement robust error handling, device management, and status queries to enable production-ready error resilience

**Why First**: This story provides the error handling and status query infrastructure needed by all other stories. Implementing it early ensures all subsequent features have proper error recovery.

**Independent Test**: Can be tested by opening devices with invalid paths, querying printer status, and verifying error messages and status parsing without requiring any text/graphics printing functionality.

### Implementation for User Story 5

- [ ] T039 [US5] Implement Printer::reset() device control method in src/printer.rs sending ESC @ command
- [ ] T040 [US5] Implement Printer::query_status() with timeout in src/printer.rs using read_byte_with_timeout and PrinterStatus::from_byte
- [ ] T041 [US5] Add detailed permission error messages with remediation instructions in Printer::open_device in src/printer.rs
- [ ] T042 [US5] Add device not found error handling in Printer::open_device in src/printer.rs
- [ ] T043 [US5] Implement disconnection detection (EOF on read) in read_byte_with_timeout in src/io/timeout.rs
- [ ] T044 [US5] Implement timeout error handling in read_byte_with_timeout in src/io/timeout.rs
- [ ] T045 [US5] Add buffer full error detection (implementation may be placeholder in V1) in src/printer.rs
- [ ] T046 [US5] Create example examples/error_handling.rs demonstrating permission errors, status queries, and error recovery

**Checkpoint**: Error handling and device management complete - all error scenarios handled gracefully

---

## Phase 4: User Story 1 - Basic Text Printing (Priority: P1) 🎯 MVP

**Goal**: Enable POS system developers to print receipts with formatted text (bold, underline, different fonts) to thermal printers

**Independent Test**: Can be fully tested by opening a printer device, sending text with formatting commands (bold, underline, fonts, pitch), and verifying printed output. Delivers a working receipt printer driver.

### Text Formatting Commands

- [ ] T047 [P] [US1] Implement bold_on() in src/commands/text.rs sending ESC E command
- [ ] T048 [P] [US1] Implement bold_off() in src/commands/text.rs sending ESC F command
- [ ] T049 [P] [US1] Implement underline_on() in src/commands/text.rs sending ESC - 1 command
- [ ] T050 [P] [US1] Implement underline_off() in src/commands/text.rs sending ESC - 0 command
- [ ] T051 [P] [US1] Implement double_strike_on() in src/commands/text.rs sending ESC G command
- [ ] T052 [P] [US1] Implement double_strike_off() in src/commands/text.rs sending ESC H command

### Font and Pitch Commands

- [ ] T053 [P] [US1] Implement select_10cpi() in src/commands/text.rs sending ESC P command
- [ ] T054 [P] [US1] Implement select_12cpi() in src/commands/text.rs sending ESC M command
- [ ] T055 [P] [US1] Implement select_15cpi() in src/commands/text.rs sending ESC g command
- [ ] T056 [US1] Implement select_font(Font) in src/commands/text.rs sending ESC k n command
- [ ] T057 [US1] Implement write_text(&str) in src/commands/text.rs with ASCII validation (replace non-ASCII with '?')

### Basic Examples and Integration

- [ ] T058 [US1] Export text formatting methods from src/commands/mod.rs
- [ ] T059 [US1] Create example examples/hello_world.rs demonstrating basic text printing with bold and underline
- [ ] T060 [US1] Create example examples/receipt.rs demonstrating complete receipt printing workflow

**Checkpoint**: Basic text printing complete - can print formatted receipts

---

## Phase 5: User Story 2 - Page Layout Control (Priority: P2)

**Goal**: Enable invoice printing systems to control page dimensions, margins, and paper feeding for multi-page invoices

**Independent Test**: Can be tested independently by setting page length, margins, line spacing, and printing multiple pages with form feeds. Delivers a complete invoice printing solution.

### Page Control Commands

- [ ] T061 [P] [US2] Implement set_page_length_lines(u8) in src/commands/layout.rs with validation (must be >= 1)
- [ ] T062 [P] [US2] Implement set_page_length_dots(u16) in src/commands/layout.rs with validation (must be >= 1)
- [ ] T063 [P] [US2] Implement form_feed() in src/commands/layout.rs sending FF command (0x0C)
- [ ] T064 [P] [US2] Implement line_feed() in src/commands/layout.rs sending LF command (0x0A)
- [ ] T065 [P] [US2] Implement carriage_return() in src/commands/layout.rs sending CR command (0x0D)

### Spacing and Positioning Commands

- [ ] T066 [P] [US2] Implement set_line_spacing(u8) in src/commands/layout.rs sending ESC 3 n command
- [ ] T067 [P] [US2] Implement set_default_line_spacing() in src/commands/layout.rs sending ESC 2 command
- [ ] T068 [US2] Implement micro_forward(u8) in src/commands/positioning.rs with validation (1-255, reject 0)
- [ ] T069 [US2] Implement micro_reverse(u8) in src/commands/positioning.rs with validation (1-255, reject 0)

### Margin and Horizontal Positioning Commands

- [ ] T070 [P] [US2] Implement set_left_margin(u8) in src/commands/layout.rs sending ESC l n command
- [ ] T071 [P] [US2] Implement set_right_margin(u8) in src/commands/layout.rs sending ESC Q n command
- [ ] T072 [P] [US2] Implement move_absolute_x(u16) in src/commands/positioning.rs sending ESC $ nL nH command
- [ ] T073 [P] [US2] Implement move_relative_x(i16) in src/commands/positioning.rs sending ESC \ nL nH command

### Examples and Integration

- [ ] T074 [US2] Export layout and positioning methods from src/commands/mod.rs
- [ ] T075 [US2] Create example examples/invoice.rs demonstrating multi-page invoice with margins and positioning

**Checkpoint**: Page layout control complete - can print multi-page formatted invoices

---

## Phase 6: User Story 3 - Graphics Printing (Priority: P3)

**Goal**: Enable retail systems to print logos and barcodes using raster graphics on receipts and labels

**Independent Test**: Can be tested independently by sending bitmap data in different graphics modes and verifying printed images. Delivers logo/barcode printing capability.

### Graphics Commands

- [ ] T076 [US3] Implement print_graphics(GraphicsMode, u16, &[u8]) in src/commands/graphics.rs
- [ ] T077 [US3] Add graphics width validation against max_graphics_width in print_graphics in src/commands/graphics.rs
- [ ] T078 [US3] Add graphics data length validation (must equal width) in print_graphics in src/commands/graphics.rs
- [ ] T079 [US3] Implement ESC K (SingleDensity 60 DPI) command construction in src/commands/graphics.rs
- [ ] T080 [US3] Implement ESC L (DoubleDensity 120 DPI) command construction in src/commands/graphics.rs
- [ ] T081 [US3] Implement ESC Y (HighDensity 180 DPI) command construction in src/commands/graphics.rs

### Examples and Integration

- [ ] T082 [US3] Export graphics methods from src/commands/mod.rs
- [ ] T083 [US3] Create example examples/graphics_logo.rs demonstrating logo printing with different density modes
- [ ] T084 [US3] Create example examples/barcode.rs demonstrating barcode printing (simple 1D barcode as bitmap)

**Checkpoint**: Graphics printing complete - can print logos and barcodes

---

## Phase 7: User Story 4 - Advanced Text Formatting (Priority: P3)

**Goal**: Enable document printing applications to use different character pitches and multiple fonts for professional document layouts

**Independent Test**: Can be tested independently by switching between character pitches and fonts, then verifying character density and appearance. Delivers advanced typography control.

**Note**: Most functionality already implemented in User Story 1. This phase adds comprehensive examples and documentation.

### Additional Font/Pitch Support (if needed)

- [ ] T085 [US4] Verify all Font enum variants (Roman, SansSerif, Courier, Script, Prestige) work correctly in select_font
- [ ] T086 [US4] Verify all pitch methods (10cpi, 12cpi, 15cpi) produce correct character density
- [ ] T087 [US4] Verify double_strike mode produces darker text than bold mode

### Examples and Documentation

- [ ] T088 [US4] Create example examples/typography.rs demonstrating all fonts and pitches combinations
- [ ] T089 [US4] Create example examples/document_layout.rs showing professional document with mixed fonts and pitches
- [ ] T090 [US4] Update quickstart.md with font selection and pitch selection examples

**Checkpoint**: Advanced text formatting complete - full typography control available

---

## Phase 8: Observability (Cross-Cutting Feature)

**Purpose**: Feature-gated tracing integration for debugging and observability (FR-032)

- [ ] T091 [P] Add tracing instrumentation to Printer::send() in src/printer.rs with cfg(feature = "tracing")
- [ ] T092 [P] Add tracing instrumentation to Printer::reset() in src/printer.rs with cfg(feature = "tracing")
- [ ] T093 [P] Add tracing instrumentation to Printer::query_status() in src/printer.rs with cfg(feature = "tracing")
- [ ] T094 [P] Add debug-level tracing for write_all_with_retry showing partial writes in src/io/retry.rs
- [ ] T095 [P] Add debug-level tracing for raw bytes in Printer::send() in src/printer.rs with cfg(feature = "tracing")
- [ ] T096 Define no-op tracing macros for when feature is disabled in src/lib.rs
- [ ] T097 Create example examples/tracing_demo.rs showing how to enable tracing in application code

**Checkpoint**: Tracing integration complete - zero overhead when disabled, comprehensive spans when enabled

---

## Phase 9: Documentation & Polish

**Purpose**: API documentation, examples, and final refinements

### API Documentation

- [ ] T098 [P] Add rustdoc comments to all Printer methods in src/printer.rs with examples
- [ ] T099 [P] Add rustdoc comments to all error types in src/errors.rs with examples
- [ ] T100 [P] Add rustdoc comments to all public types in src/types/
- [ ] T101 [P] Add crate-level documentation in src/lib.rs with quickstart example
- [ ] T102 Create prelude module in src/lib.rs exporting commonly used types

### Examples and Guides

- [ ] T103 [P] Create examples/mock_testing.rs demonstrating testing without physical printer
- [ ] T104 Update quickstart.md with all example references and troubleshooting section
- [ ] T105 Verify all quickstart.md examples work correctly by running them
- [ ] T106 Update README.md with installation instructions, features, and quickstart link

### Code Quality

- [ ] T107 Run cargo clippy and fix all warnings
- [ ] T108 Run cargo fmt to ensure consistent code style
- [ ] T109 Verify MSRV 1.91.1 compatibility by testing with minimum Rust version
- [ ] T110 Check all public APIs are exported correctly in src/lib.rs
- [ ] T111 Verify zero dependencies when no features enabled (cargo tree --no-default-features)
- [ ] T112 Add CI configuration (.github/workflows/ci.yml) for testing and linting

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 5 (Phase 3)**: Depends on Foundational - Provides error handling for all other stories
- **User Story 1 (Phase 4)**: Depends on Foundational and US5 - MVP functionality
- **User Story 2 (Phase 5)**: Depends on Foundational and US5 - Can run parallel to US1 with team capacity
- **User Story 3 (Phase 6)**: Depends on Foundational and US5 - Can run parallel to US1/US2 with team capacity
- **User Story 4 (Phase 7)**: Depends on US1 completion (builds on text formatting)
- **Observability (Phase 8)**: Can run parallel to user stories once Foundational complete
- **Documentation (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 5 (P2) - Error Recovery**: Independent, provides foundation for others
- **User Story 1 (P1) - Basic Text**: Depends on US5 for error handling
- **User Story 2 (P2) - Page Layout**: Depends on US5 for error handling, independent of US1
- **User Story 3 (P3) - Graphics**: Depends on US5 for error handling, independent of US1/US2
- **User Story 4 (P3) - Advanced Text**: Depends on US1 for text formatting foundation

### Critical Path (Sequential MVP)

1. Phase 1: Setup (T001-T013)
2. Phase 2: Foundational (T014-T038) - BLOCKING
3. Phase 3: User Story 5 (T039-T046) - Error handling foundation
4. Phase 4: User Story 1 (T047-T060) - MVP text printing
5. **MVP CHECKPOINT**: Can now print formatted receipts with error handling

### Parallel Opportunities

**Within Foundational Phase (T014-T038)**:
- Error types (T014-T021) can run sequentially
- Basic types (T022-T028) marked [P] can run in parallel
- I/O utilities (T029-T033) marked [P] can run in parallel
- Printer struct (T034-T038) depends on errors and I/O utilities

**Within User Stories**:
- US1 text formatting (T047-T052) marked [P] can run in parallel
- US1 font/pitch (T053-T055) marked [P] can run in parallel
- US2 page control (T061-T065) marked [P] can run in parallel
- US2 spacing/positioning (T066-T069) can run in parallel with margins (T070-T073)

**Across User Stories** (once Foundational + US5 complete):
- US1 (Phase 4) and US2 (Phase 5) can run in parallel
- US1 (Phase 4) and US3 (Phase 6) can run in parallel
- US2 (Phase 5) and US3 (Phase 6) can run in parallel
- Observability (Phase 8) can run in parallel with any user story

**Documentation Phase**:
- All rustdoc tasks (T098-T101) marked [P] can run in parallel
- All examples (T103-T104) marked [P] can run in parallel

---

## Implementation Strategy

### MVP First (User Story 1 + Error Handling)

1. Complete Phase 1: Setup (T001-T013)
2. Complete Phase 2: Foundational (T014-T038) - CRITICAL
3. Complete Phase 3: User Story 5 - Error Recovery (T039-T046)
4. Complete Phase 4: User Story 1 - Basic Text Printing (T047-T060)
5. **STOP and VALIDATE**: Test receipt printing with error handling
6. Deploy/demo MVP: Receipt printer driver with robust error handling

**MVP Deliverable**: A working ESC/P2 driver that can print formatted receipts (bold, underline, fonts, pitch) with comprehensive error handling (permission errors, status queries, disconnection handling).

### Incremental Delivery

1. **Foundation** (Setup + Foundational + US5) → Error handling ready
2. **MVP** (+ US1) → Receipt printing → Deploy/Demo
3. **Invoice Support** (+ US2) → Page layout control → Deploy/Demo
4. **Branding** (+ US3) → Logo/barcode printing → Deploy/Demo
5. **Professional Docs** (+ US4) → Advanced typography → Deploy/Demo
6. **Observability** (+ Phase 8) → Debugging support → Deploy/Demo
7. **Polish** (+ Phase 9) → Complete documentation → v1.0 Release

### Parallel Team Strategy

With 3 developers after Foundational + US5 complete:

1. **Team completes Setup + Foundational + US5 together** (T001-T046)
2. **Parallel user story development**:
   - Developer A: User Story 1 - Basic Text (T047-T060)
   - Developer B: User Story 2 - Page Layout (T061-T075)
   - Developer C: User Story 3 - Graphics (T076-T084)
3. Developer D (optional): Observability (T091-T097) in parallel
4. **Integration**: All stories work independently, minimal integration needed
5. **Team reconvenes for User Story 4** (builds on US1): (T085-T090)
6. **Team completes Documentation together**: (T098-T112)

---

## Task Summary

**Total Tasks**: 112
**MVP Tasks (Setup + Foundational + US5 + US1)**: 60 tasks
**Full Implementation**: 112 tasks

### Task Count by Phase

- Phase 1 (Setup): 13 tasks
- Phase 2 (Foundational): 25 tasks
- Phase 3 (User Story 5): 8 tasks
- Phase 4 (User Story 1): 14 tasks
- Phase 5 (User Story 2): 15 tasks
- Phase 6 (User Story 3): 9 tasks
- Phase 7 (User Story 4): 6 tasks
- Phase 8 (Observability): 7 tasks
- Phase 9 (Documentation): 15 tasks

### Parallel Opportunities

- **Setup Phase**: 8 of 13 tasks can run in parallel
- **Foundational Phase**: 12 of 25 tasks can run in parallel
- **User Stories**: All 4 user stories can run in parallel after Foundational + US5 complete
- **Within Each Story**: 30-60% of tasks per story can run in parallel
- **Documentation Phase**: 10 of 15 tasks can run in parallel

### Recommended MVP Scope

**Essential (Must Have for v1.0)**:
- Phase 1: Setup ✓
- Phase 2: Foundational ✓
- Phase 3: User Story 5 (Error Recovery) ✓
- Phase 4: User Story 1 (Basic Text Printing) ✓
- Selected tasks from Phase 9 (Basic documentation) ✓

**Target (Should Have for v1.0)**:
- Phase 5: User Story 2 (Page Layout)
- Phase 6: User Story 3 (Graphics)

**Optional (Nice to Have for v1.0)**:
- Phase 7: User Story 4 (Advanced Text)
- Phase 8: Observability
- Complete Phase 9: Full documentation

---

## Notes

- **[P] tasks**: Different files, no dependencies, safe to run in parallel
- **[Story] label**: Maps task to specific user story for traceability
- **Tests**: NOT included per specification (no TDD approach requested)
- **Each user story**: Independently completable and testable
- **Commit strategy**: Commit after each task or logical group
- **Validation checkpoints**: Stop at any checkpoint to validate story independently
- **File conflicts**: Avoided by using separate files per command category
- **Zero dependencies**: Verified by T111 (only thiserror required, serde/tracing optional)
- **MSRV compliance**: Verified by T109 (Rust 1.91.1+)
- **Constitution compliance**: Zero runtime dependencies, validation before I/O, comprehensive error handling

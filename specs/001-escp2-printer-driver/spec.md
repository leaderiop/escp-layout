# Feature Specification: ESC/P2 Printer Driver

**Feature Branch**: `001-escp2-printer-driver`
**Created**: 2025-11-20
**Status**: Draft
**Input**: User description: "Create a Printer struct and a full driver module that abstracts every ESC/P2 command into safe, typed Rust methods. The driver must communicate with the printer through raw bytes written to a file descriptor or any Write/Read trait object."

## Clarifications

### Session 2025-11-20

- Q: Should the Printer struct support concurrent access from multiple threads? → A: Requires external synchronization (not thread-safe, users must wrap in Mutex if needed)
- Q: Should the driver support bidirectional communication (reading printer status, responses, or error codes)? → A: Required bidirectional (require Read trait, implement status query methods)
- Q: How should the driver handle printer buffer overflow conditions? → A: Return error and let caller retry (explicit error handling)
- Q: What should the driver do if the device file requires elevated permissions (e.g., root access)? → A: Return clear permission error with instructions (e.g., suggest adding user to lp group)
- Q: What are the acceptable performance characteristics for printer command execution? → A: Best-effort with no hard limits
- Q: What error should be returned when micro-feed values are 0 or >255? → A: Return validation error immediately (before I/O)
- Q: How should the driver handle graphics data that exceeds maximum printer width? → A: Return validation error with maximum width information
- Q: How should the driver handle mixing text and graphics commands? → A: Allow freely, maintain separate state for each (text formatting vs graphics mode)
- Q: How should the driver handle partial write scenarios? → A: Retry partial writes automatically until complete or error
- Q: How should the driver handle printer disconnection during an operation? → A: Return I/O error immediately, let caller decide next action (reconnect, fail, etc.)
- Q: How should observability (logging/tracing) be integrated into the driver? → A: Feature-gated optional tracing dependency (similar to existing serde feature pattern) - adds ~0 overhead when disabled
- Q: Which tracing library should be used for structured observability? → A: `tracing` crate (tokio-rs ecosystem standard)
- Q: What level of detail should be included in tracing output? → A: High-level operations (commands, status queries, errors) with optional detailed I/O in debug spans
- Q: What testing strategy should be used for the driver? → A: Both unit tests (internal logic, validation) and integration tests (real device simulation or mock I/O)
- Q: What documentation approach should be provided for the driver? → A: Rustdoc API documentation with examples for each command, plus README with quick start guide
- Q: Which ESC/P2 command set should be implemented? → A: Core command set covering user stories only (text formatting, page layout, basic graphics, status queries) - approximately 30-40 commands
- Q: How should the maximum graphics width limit be determined for validation? → A: Accept maximum width as constructor parameter when creating Printer instance (caller-specified)

## Scope Definition

The driver implements a **core ESC/P2 command set** (approximately 30-40 commands) sufficient to satisfy all user stories. This includes:
- Text formatting commands (bold, underline, double-strike, fonts, pitch)
- Page layout commands (margins, line spacing, page length, positioning)
- Basic graphics commands (bitmap printing in multiple density modes)
- Status query commands (paper out, offline, error conditions, buffer status)
- Device control commands (reset, initialization, paper feed)

Out of scope for initial implementation:
- Extended character sets (kanji, international characters)
- Advanced graphics features (color, complex drawing primitives)
- Barcode-specific commands
- Printer-specific extended commands beyond core ESC/P2 standard

The architecture supports future expansion to additional command sets without breaking changes to existing APIs.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Text Printing (Priority: P1)

A POS system developer needs to print receipts with formatted text (bold, underline, different fonts) to a thermal printer connected via USB.

**Why this priority**: Core functionality - printing formatted text is the most fundamental use case for any printer driver and delivers immediate value for receipt printing systems.

**Independent Test**: Can be fully tested by opening a printer device, sending text with formatting commands, and verifying printed output. Delivers a working receipt printer driver.

**Acceptance Scenarios**:

1. **Given** a printer connected at `/dev/usb/lp0`, **When** developer creates a Printer instance and sends text "Hello World", **Then** the text appears on the printed page
2. **Given** a Printer instance, **When** developer enables bold mode and prints text, **Then** the text appears in bold on the printed page
3. **Given** a Printer instance, **When** developer sets underline mode and prints text, **Then** the text appears underlined on the printed page
4. **Given** a Printer instance, **When** developer sends multiple formatting commands in sequence, **Then** all formatting is correctly applied to the printed output

---

### User Story 2 - Page Layout Control (Priority: P2)

An invoice printing system needs to control page dimensions, margins, and paper feeding to print multi-page invoices with consistent formatting.

**Why this priority**: Essential for professional document printing but builds on basic text printing. Required for invoice/report generation systems.

**Independent Test**: Can be tested independently by setting page length, margins, and printing multiple pages with form feeds. Delivers a complete invoice printing solution.

**Acceptance Scenarios**:

1. **Given** a Printer instance, **When** developer sets page length to 66 lines and sends form feed, **Then** the paper advances exactly 66 lines
2. **Given** a Printer instance, **When** developer sets left margin to 10 characters and prints text, **Then** all text starts 10 character positions from the left edge
3. **Given** a Printer instance, **When** developer sets line spacing to 8 dots and prints multiple lines, **Then** lines are separated by exactly 8 dots
4. **Given** a Printer instance, **When** developer performs micro-feed forward and reverse, **Then** paper position adjusts by the specified micro-feed units

---

### User Story 3 - Graphics Printing (Priority: P3)

A retail system developer needs to print logos and barcodes using raster graphics on receipts and labels.

**Why this priority**: Advanced feature that enhances visual output but not required for basic text printing. Adds branding and barcode capabilities.

**Independent Test**: Can be tested independently by sending bitmap data in different graphics modes and verifying printed images. Delivers logo/barcode printing capability.

**Acceptance Scenarios**:

1. **Given** a Printer instance and bitmap data, **When** developer sends graphics in single-density mode, **Then** the bitmap is printed at correct resolution
2. **Given** a Printer instance and bitmap data, **When** developer sends graphics in double-density mode, **Then** the bitmap is printed at higher resolution
3. **Given** a Printer instance and bitmap data of specific width, **When** developer sends graphics command with width parameter, **Then** the graphics are printed at the specified width
4. **Given** a Printer instance, **When** developer positions graphics using absolute X positioning, **Then** the graphics appear at the correct horizontal position

---

### User Story 4 - Advanced Text Formatting (Priority: P3)

A document printing application needs to print with different character pitches (10cpi, 12cpi, 15cpi), multiple fonts, and double-strike mode for emphasis.

**Why this priority**: Enhances text formatting capabilities but not critical for basic operation. Useful for professional document layouts.

**Independent Test**: Can be tested independently by switching between character pitches and fonts, then verifying character density and appearance. Delivers advanced typography control.

**Acceptance Scenarios**:

1. **Given** a Printer instance, **When** developer selects 10cpi pitch and prints text, **Then** exactly 10 characters fit per inch horizontally
2. **Given** a Printer instance, **When** developer selects 12cpi pitch and prints text, **Then** exactly 12 characters fit per inch horizontally
3. **Given** a Printer instance, **When** developer selects 15cpi pitch and prints text, **Then** exactly 15 characters fit per inch horizontally
4. **Given** a Printer instance, **When** developer enables double-strike mode and prints text, **Then** the text appears darker/bolder than normal text
5. **Given** a Printer instance, **When** developer selects a different font and prints text, **Then** the text appears in the selected font style

---

### User Story 5 - Error Recovery and Device Management (Priority: P2)

A POS system needs to handle printer errors gracefully, reset the printer when needed, and recover from communication failures.

**Why this priority**: Critical for production reliability but builds on basic printing. Required for any system that needs to handle printer failures gracefully.

**Independent Test**: Can be tested independently by triggering error conditions (disconnected printer, invalid parameters) and verifying error handling. Delivers production-ready error resilience.

**Acceptance Scenarios**:

1. **Given** a Printer instance with invalid device path, **When** developer attempts to create the printer, **Then** a clear error is returned indicating the device cannot be opened
2. **Given** a device file without proper permissions, **When** developer attempts to create a Printer instance, **Then** a permission error with remediation instructions (e.g., "Add user to 'lp' group") is returned
3. **Given** a Printer instance, **When** developer calls reset method, **Then** the printer returns to default initialization state
4. **Given** a Printer instance, **When** developer sends commands with invalid parameters (e.g., micro-feed value 0 or > 255), **Then** a validation error is returned before sending to printer
5. **Given** a Printer instance with communication failure, **When** developer sends commands, **Then** a descriptive error is returned indicating communication failure
6. **Given** a Printer instance, **When** developer queries printer status, **Then** current status conditions (paper out, offline, error, ready) are returned
7. **Given** a Printer instance with printer not responding, **When** developer queries status with timeout, **Then** a timeout error is returned after specified duration
8. **Given** a Printer instance with full printer buffer, **When** developer sends a command, **Then** a buffer full error is returned allowing the developer to retry
9. **Given** a Printer instance with active operation, **When** printer is disconnected, **Then** an I/O error is returned immediately allowing the developer to decide recovery strategy

---

### Edge Cases

- Micro-feed validation: Values of 0 or >255 return immediate validation error before any I/O operation
- Device permissions: Driver returns clear permission error with instructions (e.g., "Add user to 'lp' group or run with appropriate permissions") when device file is not accessible
- Graphics width overflow: Driver returns validation error with maximum width information when graphics data exceeds the maximum width specified during Printer construction (caller is responsible for knowing their printer's capabilities)
- Interleaved commands: Driver allows freely mixing text and graphics commands, maintaining separate state for text formatting and graphics mode
- Buffer overflow: Driver returns an error when printer buffer is full; caller is responsible for retry logic
- Partial writes: Driver automatically retries partial writes until complete or error occurs
- Printer disconnection: Driver returns I/O error immediately when printer disconnects; caller decides recovery strategy (reconnect, fail, queue, etc.)
- Concurrent access: Printer struct is not thread-safe; users must provide external synchronization (Arc<Mutex<Printer>>) if accessing from multiple threads

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a Printer struct that accepts any type implementing both Write and Read traits for bidirectional communication
- **FR-002**: System MUST provide two constructors: (1) `new(writer, reader, max_graphics_width)` accepting any Write+Read trait objects, and (2) `open_device(path, max_graphics_width)` that opens a device file path (e.g., `/dev/usb/lp0`) in raw mode with read/write access; both accept a maximum graphics width parameter (in dots) for validation purposes
- **FR-003**: System MUST provide a low-level `send` method to write arbitrary byte sequences to the printer
- **FR-004**: System MUST provide a low-level `esc` method to construct and send ESC/P2 escape sequences with command code and arguments
- **FR-005**: System MUST implement a `reset` method that sends the printer initialization command
- **FR-006**: System MUST provide methods to set page length in both lines (`set_page_length_lines`) and dots (`set_page_length_dots`)
- **FR-007**: System MUST provide a `form_feed` method to advance paper to the next page
- **FR-008**: System MUST provide micro-feed methods (`micro_forward`, `micro_reverse`) with validation for 1-255 range
- **FR-009**: System MUST provide absolute and relative horizontal positioning methods (`move_absolute_x`, `move_relative_x`)
- **FR-010**: System MUST provide a `set_line_spacing` method to control vertical spacing between lines
- **FR-011**: System MUST provide text formatting methods: `bold_on`, `bold_off`, `underline_on`, `underline_off`, `double_strike_on`, `double_strike_off`
- **FR-012**: System MUST provide margin setting methods: `set_left_margin`, `set_right_margin`
- **FR-013**: System MUST provide character pitch selection methods: `select_10cpi`, `select_12cpi`, `select_15cpi`
- **FR-014**: System MUST provide a font selection method accepting standard ESC/P2 font types (Roman, SansSerif, Courier, Script, Prestige) via strongly-typed enum
- **FR-015**: System MUST provide a graphics printing method that accepts graphics mode, bitmap data, and width
- **FR-016**: System MUST provide a `write_text` method that sends text data to the printer
- **FR-017**: System MUST use strongly-typed enums for printer settings (Pitch, GraphicsMode, etc.) instead of raw integer values
- **FR-018**: System MUST validate all numeric parameters against ESC/P2 valid ranges before sending to printer
- **FR-019**: System MUST return Result types with descriptive errors for all fallible operations
- **FR-020**: System MUST handle I/O errors from the underlying Write and Read implementations
- **FR-021**: System MUST provide methods to query printer status (e.g., paper out, offline, error conditions)
- **FR-022**: System MUST provide methods to read printer responses and error codes
- **FR-023**: System MUST handle timeout scenarios when reading from printer (printer not responding); status query methods MUST accept an optional timeout parameter (Option<Duration>) where None blocks indefinitely and Some(duration) returns timeout error after specified duration
- **FR-024**: System MUST return a distinct error when printer buffer is full, allowing caller to implement retry logic
- **FR-025**: System MUST return descriptive permission errors when device file is not accessible, including remediation instructions (e.g., "Add user to 'lp' group")
- **FR-026**: System MUST operate on best-effort basis for command execution with no hard performance limits; actual print speed is hardware-dependent
- **FR-027**: System MUST validate graphics data width against the maximum width specified during Printer construction and return error with maximum width information if exceeded
- **FR-028**: System MUST allow freely interleaving text and graphics commands, maintaining separate state for text formatting and graphics mode
- **FR-029**: System MUST automatically retry partial writes to the device until all data is written or an error occurs
- **FR-030**: System MUST return I/O error immediately when printer disconnects during operation, without attempting automatic reconnection
- **FR-031**: Printer struct MUST NOT provide internal thread synchronization; concurrent access from multiple threads requires external synchronization (e.g., wrapping in Arc<Mutex<Printer>>)
- **FR-032**: System MUST provide optional structured tracing using the `tracing` crate via feature-gated dependency (`tracing` feature flag); when disabled, zero runtime overhead; when enabled, emit spans for high-level operations (command execution, status queries, errors) and debug-level events for detailed I/O operations (raw bytes, partial writes)
- **FR-033**: System MUST be tested with both unit tests (validating parameter validation, command construction, error handling logic) and integration tests (verifying complete command sequences using mock Write/Read implementations)
- **FR-034**: System MUST provide comprehensive Rustdoc API documentation for all public types and methods with usage examples, plus a README file containing a quick start guide and basic usage patterns

### Key Entities

- **Printer**: Represents a bidirectional connection to an ESC/P2 printer, encapsulates Write and Read trait objects, stores maximum graphics width configuration (specified at construction), and provides all command and status query methods
- **PrinterStatus**: Enumeration or struct representing printer status conditions (ready, paper out, offline, error, buffer full)
- **Pitch**: Enumeration of character pitch settings (10cpi, 12cpi, 15cpi) representing characters per inch
- **Font**: Enumeration of available font typefaces (Roman, SansSerif, Courier, Script, Prestige) supported by ESC/P2 printers
- **GraphicsMode**: Enumeration of graphics density modes (single-density, double-density, high-density) for bitmap printing
- **LineSpacing**: Representation of line spacing configuration, either default or custom dot spacing
- **Bold/Underline/DoubleStrike**: Boolean state representations for text formatting modes

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can print formatted text (bold, underline, different pitches) with fewer than 10 lines of code
- **SC-002**: All core ESC/P2 commands (30-40 commands covering user stories) are abstracted with type-safe method calls requiring zero manual escape sequence construction
- **SC-003**: Invalid parameters are caught at compile time (via enums) or runtime (via validation) before being sent to printer, preventing 100% of malformed command sequences
- **SC-004**: Developers can switch from one printer to another (file, USB device, network stream) by providing a different Write+Read implementation without code changes
- **SC-005**: All printer errors and I/O failures return descriptive error messages that enable developers to diagnose issues within 5 minutes
- **SC-006**: Example code demonstrates complete invoice printing workflow from initialization to final form feed in under 50 lines
- **SC-007**: Graphics printing supports at least 3 different density modes with automatic width handling
- **SC-008**: Printer status queries with a 2-second timeout parameter complete within 2 seconds or return timeout error, preventing application hangs; queries with no timeout block indefinitely until response or I/O error
- **SC-009**: Command execution performance is best-effort with no artificial delays; actual speed determined by printer hardware capabilities
- **SC-010**: Unit tests cover 100% of parameter validation logic and command construction; integration tests verify all user stories can be executed end-to-end using mock I/O
- **SC-011**: All public APIs have Rustdoc documentation with at least one usage example; README provides a complete quick start that gets a developer printing "Hello World" in under 5 minutes

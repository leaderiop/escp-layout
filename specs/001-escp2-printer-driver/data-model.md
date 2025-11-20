# Data Model: ESC/P2 Printer Driver

**Feature**: 001-escp2-printer-driver
**Created**: 2025-11-20
**Status**: Phase 1 Design

This document defines all entities, their fields, validation rules, and relationships for the ESC/P2 printer driver implementation.

---

## Entity: Printer

**Purpose**: Represents a bidirectional connection to an ESC/P2 printer device, encapsulating Write and Read trait objects for communication.

**Fields**:
- `writer: W` (where `W: Write`) - Output stream for sending commands to printer
- `reader: R` (where `R: Read`) - Input stream for receiving status responses from printer
- `max_graphics_width: u16` - Maximum graphics width in dots, specified at construction for validation purposes

**Validation**:
- `max_graphics_width` must be > 0 (enforced at construction via validation or debug_assert!)
- Generic type parameters `W` and `R` must implement `std::io::Write` and `std::io::Read` respectively

**Relationships**:
- Creates `PrinterError` instances for all error conditions
- Uses `ValidationError` for parameter validation failures
- Sends commands to printer that may affect `PrinterStatus`
- Constructs `GraphicsMode`, `Font`, `Pitch` enums for typed command parameters

**State Machine**:
The Printer maintains internal state for:
1. **Text formatting state**: Bold (on/off), Underline (on/off), DoubleStrike (on/off)
2. **Character pitch state**: 10cpi, 12cpi, or 15cpi
3. **Font state**: Roman, Sans Serif, Courier, Script, or Prestige
4. **Line spacing state**: Default (1/6 inch) or custom (N/180 inch)
5. **Margin state**: Left margin (0-255 chars), Right margin (0-255 chars)
6. **Current position state**: Absolute horizontal position, current line

All state transitions are triggered by explicit method calls and affect subsequent text rendering. Reset command (`reset()`) returns all state to default values.

---

## Entity: PrinterStatus

**Purpose**: Represents the current operational status of the printer at a point in time, obtained through status query commands.

**Fields**:
- `online: bool` - True if printer is online and ready, false if offline
- `paper_out: bool` - True if printer has no paper loaded
- `error: bool` - True if printer has encountered an error condition

**Validation**:
- No runtime validation needed - status bytes are parsed directly from printer response
- All boolean fields default to safe values if status byte cannot be interpreted

**Relationships**:
- Created by `Printer::query_status()` method
- Parsed from status byte response (ESC/P2 DLE EOT 1 command)
- Used by application code to determine if printer is ready for operations

**Bit Layout** (from status byte):
```
Bit 3: 1 = offline, 0 = online
Bit 5: 1 = paper out, 0 = paper loaded
Bit 6: 1 = error occurred, 0 = no error
```

**State Interpretation**:
- Ready to print: `online == true && paper_out == false && error == false`
- Offline: `online == false`
- Paper out: `paper_out == true` (may still be online)
- Error condition: `error == true`

---

## Entity: Pitch

**Purpose**: Enumeration of character pitch settings representing characters per inch horizontally.

**Variants**:
- `Pitch::Pica` - 10 characters per inch (ESC P command, byte 0x50)
- `Pitch::Elite` - 12 characters per inch (ESC M command, byte 0x4D)
- `Pitch::Condensed` - 15 characters per inch (ESC g command, byte 0x67)

**Validation**:
- Type-safe via Rust enum - invalid values impossible at compile-time
- No runtime validation needed

**Relationships**:
- Used by `Printer::select_10cpi()`, `select_12cpi()`, `select_15cpi()` methods
- Affects text rendering density and horizontal spacing

**ESC/P2 Mapping**:
```rust
impl Pitch {
    fn as_command(&self) -> &[u8] {
        match self {
            Pitch::Pica => &[0x1B, 0x50],       // ESC P
            Pitch::Elite => &[0x1B, 0x4D],      // ESC M
            Pitch::Condensed => &[0x1B, 0x67],  // ESC g
        }
    }
}
```

---

## Entity: Font

**Purpose**: Enumeration of available font typefaces supported by ESC/P2 printers.

**Variants**:
- `Font::Roman` - Roman typeface (value 0)
- `Font::SansSerif` - Sans Serif typeface (value 1)
- `Font::Courier` - Courier typeface (value 2)
- `Font::Script` - Script typeface (value 3)
- `Font::Prestige` - Prestige typeface (value 4)

**Validation**:
- Type-safe via Rust enum - invalid values impossible at compile-time
- Values 0-4 map directly to ESC/P2 font selection parameter

**Relationships**:
- Used by `Printer::select_font()` method
- Affects character appearance and style

**ESC/P2 Mapping**:
```rust
impl Font {
    fn as_byte(&self) -> u8 {
        match self {
            Font::Roman => 0,
            Font::SansSerif => 1,
            Font::Courier => 2,
            Font::Script => 3,
            Font::Prestige => 4,
        }
    }

    fn as_command(&self) -> [u8; 3] {
        [0x1B, 0x6B, self.as_byte()]  // ESC k n
    }
}
```

---

## Entity: GraphicsMode

**Purpose**: Enumeration of graphics density modes for bitmap printing, specifying resolution in dots per inch.

**Variants**:
- `GraphicsMode::SingleDensity` - 60 DPI (ESC K command, byte 0x4B)
- `GraphicsMode::DoubleDensity` - 120 DPI (ESC L command, byte 0x4C)
- `GraphicsMode::HighDensity` - 180 DPI (ESC Y command, byte 0x59)

**Validation**:
- Type-safe via Rust enum - invalid values impossible at compile-time
- Graphics width validation performed separately against `Printer::max_graphics_width`

**Relationships**:
- Used by `Printer::print_graphics()` method
- Combined with width parameter and bitmap data to construct graphics command

**ESC/P2 Mapping**:
```rust
impl GraphicsMode {
    fn as_command_byte(&self) -> u8 {
        match self {
            GraphicsMode::SingleDensity => 0x4B,  // ESC K
            GraphicsMode::DoubleDensity => 0x4C,  // ESC L
            GraphicsMode::HighDensity => 0x59,    // ESC Y
        }
    }
}
```

**Resolution Details**:
- Single density: 60 DPI horizontal × 60 DPI vertical
- Double density: 120 DPI horizontal × 60 DPI vertical
- High density: 180 DPI horizontal × 180 DPI vertical

---

## Entity: LineSpacing

**Purpose**: Representation of line spacing configuration in 1/180-inch units.

**Variants** (conceptual - actual implementation may use different structure):
- `LineSpacing::Default` - 1/6 inch (30/180) spacing (ESC 2 command)
- `LineSpacing::Custom(u8)` - Custom spacing in 1/180-inch units (ESC 3 n command, where n = 0-255)

**Validation**:
- `Custom(n)` variant: `n` must be 0-255 (enforced by u8 type)
- No additional validation needed

**Relationships**:
- Used by `Printer::set_line_spacing()` method
- Affects vertical spacing between consecutive lines of text

**ESC/P2 Mapping**:
```rust
impl LineSpacing {
    fn as_command(&self) -> Vec<u8> {
        match self {
            LineSpacing::Default => vec![0x1B, 0x32],      // ESC 2
            LineSpacing::Custom(dots) => vec![0x1B, 0x33, *dots],  // ESC 3 n
        }
    }
}
```

---

## Entity: PrinterError

**Purpose**: Comprehensive error type representing all failure modes for printer operations.

**Variants**:

### `PrinterError::Io(std::io::Error)`
**Description**: I/O error communicating with printer device
**Causes**: Write failure, read failure, connection loss, interrupted system call
**Automatic conversion**: Via `#[from]` attribute from `std::io::Error`

### `PrinterError::Permission { path: String, message: String }`
**Description**: Permission denied accessing printer device
**Fields**:
- `path: String` - Device file path that was inaccessible
- `message: String` - Detailed error with remediation instructions
**Example message**: "Cannot access printer device '/dev/usb/lp0'. Add your user to 'lp' group: sudo usermod -aG lp $USER"

### `PrinterError::DeviceNotFound { path: String }`
**Description**: Printer device file does not exist
**Fields**:
- `path: String` - Device file path that was not found
**Causes**: Incorrect path, printer not connected, driver not loaded

### `PrinterError::Disconnected`
**Description**: Printer disconnected during operation
**Causes**: USB cable unplugged, network connection lost, power loss
**Detection**: Read returns 0 bytes (EOF)

### `PrinterError::Timeout { timeout: Duration }`
**Description**: Timeout waiting for printer response
**Fields**:
- `timeout: Duration` - Duration that elapsed before timeout
**Causes**: Printer not responding, printer powered off, communication failure

### `PrinterError::BufferFull`
**Description**: Printer buffer full, operation cannot proceed
**Causes**: Sending data faster than printer can process
**Recovery**: Caller should retry after delay

### `PrinterError::Validation(ValidationError)`
**Description**: Parameter validation failure
**Automatic conversion**: Via `#[from]` attribute from `ValidationError`
**Causes**: See `ValidationError` entity below

**Validation**:
- All error variants must be constructible from appropriate underlying errors
- Permission errors must include actionable remediation instructions
- All errors must implement `std::error::Error` and `Debug` traits

**Relationships**:
- Returned by all fallible `Printer` methods
- Contains `ValidationError` as nested error type
- Wraps `std::io::Error` for I/O failures

---

## Entity: ValidationError

**Purpose**: Specialized error type for parameter validation failures that occur before any I/O operation.

**Variants**:

### `ValidationError::MicroFeedZero`
**Description**: Micro-feed value must be 1-255, got 0
**Causes**: Calling `micro_forward(0)` or `micro_reverse(0)`
**Prevention**: Validate input is non-zero before calling method

### `ValidationError::GraphicsWidthExceeded { width: u16, max_width: u16 }`
**Description**: Graphics width exceeds maximum supported by printer
**Fields**:
- `width: u16` - Requested graphics width in dots
- `max_width: u16` - Maximum width configured for this Printer instance
**Causes**: Bitmap data wider than printer capability

### `ValidationError::GraphicsWidthMismatch { width: u16, data_len: usize }`
**Description**: Graphics data length doesn't match specified width
**Fields**:
- `width: u16` - Width parameter passed to `print_graphics()`
- `data_len: usize` - Actual length of data slice
**Causes**: Programming error - width parameter must equal data.len()

### `ValidationError::InvalidPageLength { value: u8 }`
**Description**: Page length must be at least 1 line/dot
**Fields**:
- `value: u8` - Invalid page length value provided
**Causes**: Calling `set_page_length_lines(0)` or equivalent

**Validation**:
- All validation errors must be detected before any I/O operation
- Validation errors represent programming errors or invalid user input, not I/O failures

**Relationships**:
- Nested within `PrinterError::Validation`
- Triggered by parameter validation in `Printer` methods
- All errors provide specific information about what was invalid

---

## Entity: FormattingState (Internal)

**Purpose**: Internal state tracking text formatting modes (not exposed as public API entity).

**Fields**:
- `bold: bool` - Bold mode enabled (ESC E) or disabled (ESC F)
- `underline: bool` - Underline mode enabled (ESC - 1) or disabled (ESC - 0)
- `double_strike: bool` - Double-strike mode enabled (ESC G) or disabled (ESC H)

**Validation**:
- All fields are simple booleans - no validation needed

**Relationships**:
- Tracked internally by `Printer` struct
- Modified by `bold_on()`, `bold_off()`, `underline_on()`, `underline_off()`, `double_strike_on()`, `double_strike_off()`
- Reset to default (all false) by `reset()` method

**State Transitions**:
```
Initial state: { bold: false, underline: false, double_strike: false }

bold_on() → { bold: true, ... }
bold_off() → { bold: false, ... }
underline_on() → { underline: true, ... }
underline_off() → { underline: false, ... }
double_strike_on() → { double_strike: true, ... }
double_strike_off() → { double_strike: false, ... }
reset() → { bold: false, underline: false, double_strike: false }
```

**Note**: Formatting state may be maintained implicitly by the printer hardware rather than as explicit struct fields in V1 implementation. Driver sends commands and trusts printer to track state.

---

## Entity: PositionState (Internal)

**Purpose**: Internal state tracking current printer position (optional - may not be explicitly tracked in V1).

**Fields** (conceptual):
- `current_x: u16` - Current horizontal position in 1/60-inch units
- `current_y: u16` - Current vertical position in 1/180-inch units
- `left_margin: u8` - Left margin in characters (0-255)
- `right_margin: u8` - Right margin in characters (0-255)

**Validation**:
- Margins: 0-255 range (enforced by u8 type)
- Position values: Within printer physical limits

**Relationships**:
- Modified by `move_absolute_x()`, `move_relative_x()`, `set_left_margin()`, `set_right_margin()`
- Affected by line feeds, carriage returns, form feeds
- Reset by `reset()` method

**Note**: Position tracking may not be required in V1 if driver operates in stateless command-sending mode. Printer hardware maintains its own position state.

---

## Data Flow Diagrams

### Command Execution Flow

```
Application Code
    ↓
Printer::method() (e.g., bold_on())
    ↓
Parameter Validation (if applicable)
    ↓ (if valid)
Command Construction (ESC/P2 byte sequence)
    ↓
Printer::send() (low-level write)
    ↓
write_all_with_retry() (handles partial writes)
    ↓
std::io::Write::write() (to device)
    ↓
Printer Hardware
```

**Validation occurs before send** - ensures no invalid commands reach printer.

### Status Query Flow

```
Application Code
    ↓
Printer::query_status(timeout)
    ↓
Send status request command (DLE EOT 1)
    ↓
std::io::Write::write()
    ↓
Printer Hardware
    ↓
std::io::Read::read() (with timeout)
    ↓
read_byte_with_timeout()
    ↓
PrinterStatus::from_byte() (parse response)
    ↓
Return PrinterStatus to caller
```

**Timeout handling** prevents application hangs if printer doesn't respond.

### Error Propagation Flow

```
I/O Error or Validation Failure
    ↓
Construct PrinterError or ValidationError
    ↓
Return Result::Err(error)
    ↓
Application Code (error handling)
```

All errors use `Result<T, PrinterError>` - no panics in release builds.

---

## Type Relationships Summary

```
Printer<W: Write, R: Read>
    ├── max_graphics_width: u16
    ├── writer: W
    └── reader: R

Printer methods return:
    ├── Result<(), PrinterError>        (most commands)
    ├── Result<PrinterStatus, PrinterError>  (status query)
    └── void (none - all methods are fallible)

PrinterError variants:
    ├── Io(std::io::Error)
    ├── Permission { path, message }
    ├── DeviceNotFound { path }
    ├── Disconnected
    ├── Timeout { timeout }
    ├── BufferFull
    └── Validation(ValidationError)

ValidationError variants:
    ├── MicroFeedZero
    ├── GraphicsWidthExceeded { width, max_width }
    ├── GraphicsWidthMismatch { width, data_len }
    └── InvalidPageLength { value }

Type-safe command enums:
    ├── Pitch { Pica, Elite, Condensed }
    ├── Font { Roman, SansSerif, Courier, Script, Prestige }
    ├── GraphicsMode { SingleDensity, DoubleDensity, HighDensity }
    └── LineSpacing { Default, Custom(u8) }

PrinterStatus:
    ├── online: bool
    ├── paper_out: bool
    └── error: bool
```

---

## Validation Rules Summary

### Compile-Time Validation (Type System)
- Font selection: Only valid Font enum variants accepted
- Pitch selection: Only valid Pitch enum variants accepted
- GraphicsMode: Only valid GraphicsMode enum variants accepted
- Write/Read traits: Generic bounds enforce correct I/O capabilities

### Runtime Validation (Before I/O)
- Micro-feed range: 1-255 (reject 0)
- Graphics width: Must not exceed `max_graphics_width`
- Graphics data length: Must equal width parameter
- Page length: Must be at least 1 line/dot

### Runtime Validation (During I/O)
- Device file permissions: Check on open, return helpful error
- Timeout: Monitor elapsed time during status reads
- Partial writes: Automatically retry until complete
- Disconnection: Detect EOF and return error

### No Validation Needed (Safe by Design)
- Boolean formatting flags: `true`/`false` always valid
- Margin values: u8 type enforces 0-255 range
- Status byte parsing: All bit patterns valid (interpret as-is)

---

## Concurrency and Thread Safety

### Thread Safety Constraints
- `Printer` struct: **NOT thread-safe** (no internal synchronization)
- Application responsibility: Wrap in `Arc<Mutex<Printer>>` for multi-threaded access
- Rationale: Avoids overhead for single-threaded use cases

### Immutability Constraints
- All command methods require `&mut self` (exclusive access)
- No shared mutable state between operations
- Prevents data races at compile-time via Rust borrow checker

### Send/Sync Traits
- `Printer<W, R>` is `Send` if `W: Send` and `R: Send`
- `Printer<W, R>` is NOT `Sync` (mutable methods require exclusive access)
- Status types (`PrinterStatus`, errors) are `Send + Sync` (can be shared across threads)

---

## Memory Layout Considerations

### Stack vs Heap Allocation
- `Printer` struct: Small fixed size (two trait objects + u16), suitable for stack
- Command buffers: Temporary allocations during command construction (Vec<u8> for graphics)
- Error types: Small enums, typically stack-allocated
- No persistent heap allocations for printer state

### Buffer Management
- Graphics commands: Allocate temporary buffer matching data size
- Text output: Direct write of string bytes (no intermediate buffer)
- Status queries: Single-byte read (no buffering)

### Resource Cleanup
- `Printer::open_device()` opens file descriptor
- Drop implementation should close file (handled by `File::drop()`)
- No manual resource management needed (RAII)

---

## Future Extension Points

### V2 Candidate Additions
- **Extended character sets**: Add enum variants for international character sets
- **Color support**: Extend `GraphicsMode` with color density modes
- **Async I/O**: Add `Printer<W: AsyncWrite, R: AsyncRead>` variant
- **Status monitoring**: Add event-based status change notifications
- **Command batching**: Add `Printer::batch()` for optimized multi-command sequences

### Backward Compatibility Strategy
- V1 types remain unchanged (frozen API)
- V2 additions use new types or feature flags
- No breaking changes to core entities

---

**Document Status**: Phase 1 Complete
**Next Steps**: Create API contracts (contracts/printer-api.md) and quickstart guide

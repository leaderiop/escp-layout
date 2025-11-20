# API Contract: ESC/P2 Printer Driver (Rust Library)

**Feature**: 001-escp2-printer-driver
**Created**: 2025-11-20
**Status**: Phase 1 Design

This document defines the public Rust API surface for the ESC/P2 printer driver library. This is NOT a web API - it's a Rust library providing type-safe abstractions over ESC/P2 commands.

---

## Module Structure

```rust
escp_driver
├── printer         // Core Printer struct and methods
├── error           // Error types (PrinterError, ValidationError)
├── types           // Public types (PrinterStatus, Font, Pitch, etc.)
└── prelude         // Common imports
```

---

## Public Types

### `Printer<W, R>`

Generic printer driver supporting any Write/Read implementation.

**Type Parameters**:
- `W: std::io::Write` - Output stream for commands
- `R: std::io::Read` - Input stream for status responses

**Trait Implementations**:
- Does NOT implement `Send` or `Sync` (users must wrap in Arc<Mutex<>> for multi-threaded access)
- Does NOT implement `Clone` (owns I/O handles)

---

## Constructor and Initialization Methods

### Method: `new`

**Signature**:
```rust
pub fn new(writer: W, reader: R, max_graphics_width: u16) -> Self
```

**Parameters**:
- `writer: W` - Write implementation for sending commands to printer
- `reader: R` - Read implementation for receiving status from printer
- `max_graphics_width: u16` - Maximum graphics width in dots (for validation)

**Returns**: `Printer<W, R>` instance

**Errors**: None (infallible constructor)

**Example**:
```rust
use std::fs::File;
use escp_driver::Printer;

// Using file descriptors
let file = File::open("/dev/usb/lp0")?;
let printer = Printer::new(file.try_clone()?, file, 1440);
```

---

### Method: `open_device`

**Signature**:
```rust
impl Printer<File, File> {
    pub fn open_device(path: &str, max_graphics_width: u16) -> Result<Self, PrinterError>
}
```

**Parameters**:
- `path: &str` - Device file path (e.g., "/dev/usb/lp0", "/dev/ttyUSB0")
- `max_graphics_width: u16` - Maximum graphics width in dots

**Returns**: `Result<Printer<File, File>, PrinterError>`

**Errors**:
- `PrinterError::Permission` - Insufficient permissions to access device
- `PrinterError::DeviceNotFound` - Device file does not exist
- `PrinterError::Io` - Other I/O errors during open

**Example**:
```rust
use escp_driver::Printer;

// Open USB printer device
let mut printer = Printer::open_device("/dev/usb/lp0", 1440)?;

// Permission error includes helpful remediation
match Printer::open_device("/dev/usb/lp0", 1440) {
    Err(PrinterError::Permission { message, .. }) => {
        eprintln!("{}", message);
        // Prints: "Cannot access printer device '/dev/usb/lp0'.
        //          Add your user to 'lp' group: sudo usermod -aG lp $USER"
    }
    Ok(printer) => { /* use printer */ }
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## Low-Level Command Methods

### Method: `send`

**Signature**:
```rust
pub fn send(&mut self, data: &[u8]) -> Result<(), PrinterError>
```

**Parameters**:
- `data: &[u8]` - Raw byte sequence to send to printer

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Write failure or device error
- `PrinterError::Disconnected` - Printer disconnected during write

**Example**:
```rust
// Send raw ESC/P2 command (use high-level methods instead when available)
printer.send(&[0x1B, 0x40])?;  // Reset command
```

**Note**: Prefer high-level typed methods over `send()` for safety and clarity.

---

### Method: `esc`

**Signature**:
```rust
pub fn esc(&mut self, command: u8, args: &[u8]) -> Result<(), PrinterError>
```

**Parameters**:
- `command: u8` - ESC/P2 command byte (follows ESC prefix 0x1B)
- `args: &[u8]` - Command arguments/parameters

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Write failure or device error
- `PrinterError::Disconnected` - Printer disconnected during write

**Example**:
```rust
// Send ESC E (bold on) command
printer.esc(0x45, &[])?;

// Send ESC k n (select font) command
printer.esc(0x6B, &[2])?;  // Font::Courier
```

**Note**: Prefer high-level typed methods over `esc()` for safety and clarity.

---

## Device Control Methods

### Method: `reset`

**Signature**:
```rust
pub fn reset(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Resets printer to default initialization state (ESC @ command). Clears all formatting, resets margins, line spacing, and position.

**Example**:
```rust
// Initialize printer to known state
printer.reset()?;
```

---

### Method: `query_status`

**Signature**:
```rust
pub fn query_status(&mut self, timeout: Duration) -> Result<PrinterStatus, PrinterError>
```

**Parameters**:
- `timeout: Duration` - Maximum time to wait for printer response

**Returns**: `Result<PrinterStatus, PrinterError>`

**Errors**:
- `PrinterError::Timeout` - Printer did not respond within timeout duration
- `PrinterError::Disconnected` - Printer disconnected during query
- `PrinterError::Io` - Communication failure

**Example**:
```rust
use std::time::Duration;

// Query printer status with 2-second timeout
let status = printer.query_status(Duration::from_secs(2))?;

if status.online && !status.paper_out && !status.error {
    println!("Printer ready");
} else {
    println!("Printer not ready: online={}, paper_out={}, error={}",
             status.online, status.paper_out, status.error);
}
```

---

## Text Formatting Methods

### Method: `bold_on`

**Signature**:
```rust
pub fn bold_on(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Enables bold text mode (ESC E command). Subsequent text will print in bold.

**Example**:
```rust
printer.bold_on()?;
printer.write_text("BOLD TEXT")?;
printer.bold_off()?;
```

---

### Method: `bold_off`

**Signature**:
```rust
pub fn bold_off(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Disables bold text mode (ESC F command).

**Example**: See `bold_on` example above.

---

### Method: `underline_on`

**Signature**:
```rust
pub fn underline_on(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Enables underline mode (ESC - 1 command). Subsequent text will be underlined.

**Example**:
```rust
printer.underline_on()?;
printer.write_text("Underlined text")?;
printer.underline_off()?;
```

---

### Method: `underline_off`

**Signature**:
```rust
pub fn underline_off(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Disables underline mode (ESC - 0 command).

---

### Method: `double_strike_on`

**Signature**:
```rust
pub fn double_strike_on(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Enables double-strike mode (ESC G command). Text will be printed twice for darker/bolder appearance.

**Example**:
```rust
printer.double_strike_on()?;
printer.write_text("Extra bold text")?;
printer.double_strike_off()?;
```

---

### Method: `double_strike_off`

**Signature**:
```rust
pub fn double_strike_off(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Disables double-strike mode (ESC H command).

---

## Character Pitch Methods

### Method: `select_10cpi`

**Signature**:
```rust
pub fn select_10cpi(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Selects 10 characters per inch pitch (Pica, ESC P command).

**Example**:
```rust
printer.select_10cpi()?;
printer.write_text("10 CPI text")?;
```

---

### Method: `select_12cpi`

**Signature**:
```rust
pub fn select_12cpi(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Selects 12 characters per inch pitch (Elite, ESC M command).

**Example**:
```rust
printer.select_12cpi()?;
printer.write_text("12 CPI text - more characters per line")?;
```

---

### Method: `select_15cpi`

**Signature**:
```rust
pub fn select_15cpi(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Selects 15 characters per inch pitch (Condensed, ESC g command).

**Example**:
```rust
printer.select_15cpi()?;
printer.write_text("15 CPI condensed text - maximum density")?;
```

---

## Font Selection Method

### Method: `select_font`

**Signature**:
```rust
pub fn select_font(&mut self, font: Font) -> Result<(), PrinterError>
```

**Parameters**:
- `font: Font` - Font typeface to select (Roman, Sans Serif, Courier, Script, Prestige)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Selects font typeface (ESC k n command).

**Example**:
```rust
use escp_driver::Font;

printer.select_font(Font::Courier)?;
printer.write_text("Courier font text")?;

printer.select_font(Font::Script)?;
printer.write_text("Script font text")?;
```

---

## Page Layout Methods

### Method: `set_page_length_lines`

**Signature**:
```rust
pub fn set_page_length_lines(&mut self, lines: u8) -> Result<(), PrinterError>
```

**Parameters**:
- `lines: u8` - Page length in lines (1-127)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Validation(ValidationError::InvalidPageLength)` - Lines value is 0
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Sets page length in lines (ESC C n command). Defines when form feed advances to next page.

**Example**:
```rust
// Set page to 66 lines (standard 11-inch page at 6 lines/inch)
printer.set_page_length_lines(66)?;

// Validation error for zero
let err = printer.set_page_length_lines(0).unwrap_err();
assert!(matches!(err, PrinterError::Validation(ValidationError::InvalidPageLength { .. })));
```

---

### Method: `set_page_length_dots`

**Signature**:
```rust
pub fn set_page_length_dots(&mut self, dots: u16) -> Result<(), PrinterError>
```

**Parameters**:
- `dots: u16` - Page length in 1/360-inch units (1-32767)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Validation(ValidationError::InvalidPageLength)` - Dots value is 0
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Sets page length in 1/360-inch units (ESC ( C command). Provides finer control than line-based method.

**Example**:
```rust
// Set page to 11 inches (11 * 360 = 3960 dots)
printer.set_page_length_dots(3960)?;
```

---

### Method: `form_feed`

**Signature**:
```rust
pub fn form_feed(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Advances paper to next page (FF command, byte 0x0C). Respects page length setting.

**Example**:
```rust
printer.write_text("Page 1 content")?;
printer.form_feed()?;
printer.write_text("Page 2 content")?;
```

---

### Method: `line_feed`

**Signature**:
```rust
pub fn line_feed(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Advances paper by one line (LF command, byte 0x0A). Respects line spacing setting.

**Example**:
```rust
printer.write_text("Line 1")?;
printer.line_feed()?;
printer.write_text("Line 2")?;
```

---

### Method: `carriage_return`

**Signature**:
```rust
pub fn carriage_return(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Returns print head to left margin (CR command, byte 0x0D).

**Example**:
```rust
printer.write_text("Some text")?;
printer.carriage_return()?;
printer.line_feed()?;  // CR + LF = newline
```

---

## Micro-Feed Methods

### Method: `micro_forward`

**Signature**:
```rust
pub fn micro_forward(&mut self, units: u8) -> Result<(), PrinterError>
```

**Parameters**:
- `units: u8` - Forward movement in 1/180-inch units (1-255)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Validation(ValidationError::MicroFeedZero)` - Units value is 0
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Advances paper forward (down) by micro-feed units (ESC J n command). Allows precise vertical positioning.

**Example**:
```rust
// Move forward 10/180 inch
printer.micro_forward(10)?;

// Validation error for zero
let err = printer.micro_forward(0).unwrap_err();
assert!(matches!(err, PrinterError::Validation(ValidationError::MicroFeedZero)));
```

---

### Method: `micro_reverse`

**Signature**:
```rust
pub fn micro_reverse(&mut self, units: u8) -> Result<(), PrinterError>
```

**Parameters**:
- `units: u8` - Reverse movement in 1/180-inch units (1-255)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Validation(ValidationError::MicroFeedZero)` - Units value is 0
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Moves paper backward (up) by micro-feed units (ESC j n command). Maximum reverse movement ~1.41 inches (254 units).

**Example**:
```rust
// Move backward 5/180 inch
printer.micro_reverse(5)?;
```

---

## Line Spacing Methods

### Method: `set_line_spacing`

**Signature**:
```rust
pub fn set_line_spacing(&mut self, dots: u8) -> Result<(), PrinterError>
```

**Parameters**:
- `dots: u8` - Line spacing in 1/180-inch units (0-255)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Sets custom line spacing (ESC 3 n command). Affects vertical spacing for subsequent line feeds.

**Example**:
```rust
// Set 8-dot line spacing
printer.set_line_spacing(8)?;
printer.write_text("Line 1")?;
printer.line_feed()?;
printer.write_text("Line 2 (8/180 inch below)")?;
```

---

### Method: `set_default_line_spacing`

**Signature**:
```rust
pub fn set_default_line_spacing(&mut self) -> Result<(), PrinterError>
```

**Parameters**: None

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Resets line spacing to default 1/6-inch (ESC 2 command).

**Example**:
```rust
printer.set_default_line_spacing()?;
```

---

## Horizontal Positioning Methods

### Method: `move_absolute_x`

**Signature**:
```rust
pub fn move_absolute_x(&mut self, position: u16) -> Result<(), PrinterError>
```

**Parameters**:
- `position: u16` - Absolute horizontal position in 1/60-inch units

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Moves print head to absolute horizontal position (ESC $ command).

**Example**:
```rust
// Move to position 120 (2 inches from left edge: 120/60 = 2.0")
printer.move_absolute_x(120)?;
printer.write_text("At 2-inch position")?;
```

---

### Method: `move_relative_x`

**Signature**:
```rust
pub fn move_relative_x(&mut self, offset: i16) -> Result<(), PrinterError>
```

**Parameters**:
- `offset: i16` - Relative horizontal offset in 1/120-inch units (positive = right, negative = left)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Moves print head relative to current position (ESC \ command).

**Example**:
```rust
// Move right 60/120 inch (0.5 inch)
printer.move_relative_x(60)?;

// Move left 30/120 inch (0.25 inch)
printer.move_relative_x(-30)?;
```

---

## Margin Methods

### Method: `set_left_margin`

**Signature**:
```rust
pub fn set_left_margin(&mut self, chars: u8) -> Result<(), PrinterError>
```

**Parameters**:
- `chars: u8` - Left margin in character positions (0-255)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Sets left margin (ESC l n command). Subsequent text starts at this position.

**Example**:
```rust
// Set 10-character left margin
printer.set_left_margin(10)?;
printer.write_text("Indented text")?;
```

---

### Method: `set_right_margin`

**Signature**:
```rust
pub fn set_right_margin(&mut self, chars: u8) -> Result<(), PrinterError>
```

**Parameters**:
- `chars: u8` - Right margin in character positions (0-255)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Sets right margin (ESC Q n command). Text wraps or truncates at this position.

**Example**:
```rust
// Set 80-character right margin
printer.set_right_margin(80)?;
```

---

## Text Output Method

### Method: `write_text`

**Signature**:
```rust
pub fn write_text(&mut self, text: &str) -> Result<(), PrinterError>
```

**Parameters**:
- `text: &str` - Text string to print

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Sends text to printer. Non-ASCII characters (> 127) are replaced with '?'. Formatting state (bold, underline, etc.) affects appearance.

**Example**:
```rust
printer.write_text("Hello, World!")?;

// With formatting
printer.bold_on()?;
printer.write_text("Bold text")?;
printer.bold_off()?;
```

---

## Graphics Method

### Method: `print_graphics`

**Signature**:
```rust
pub fn print_graphics(
    &mut self,
    mode: GraphicsMode,
    width: u16,
    data: &[u8]
) -> Result<(), PrinterError>
```

**Parameters**:
- `mode: GraphicsMode` - Graphics density mode (SingleDensity, DoubleDensity, HighDensity)
- `width: u16` - Width in dots (must match data length)
- `data: &[u8]` - Bitmap data (each byte represents 8 vertical dots)

**Returns**: `Result<(), PrinterError>`

**Errors**:
- `PrinterError::Validation(ValidationError::GraphicsWidthExceeded)` - Width exceeds max_graphics_width
- `PrinterError::Validation(ValidationError::GraphicsWidthMismatch)` - Width doesn't match data.len()
- `PrinterError::Io` - Communication failure
- `PrinterError::Disconnected` - Printer disconnected

**Description**: Prints bitmap graphics in specified density mode.

**Example**:
```rust
use escp_driver::GraphicsMode;

// Print 10-dot wide single-density bitmap
let bitmap_data = vec![0xFF; 10];  // 10 bytes (vertical stripes)
printer.print_graphics(GraphicsMode::SingleDensity, 10, &bitmap_data)?;

// Validation error if width exceeds maximum
let large_data = vec![0xFF; 2000];
let err = printer.print_graphics(GraphicsMode::SingleDensity, 2000, &large_data).unwrap_err();
assert!(matches!(err, PrinterError::Validation(ValidationError::GraphicsWidthExceeded { .. })));

// Validation error if width doesn't match data length
let err = printer.print_graphics(GraphicsMode::SingleDensity, 100, &bitmap_data).unwrap_err();
assert!(matches!(err, PrinterError::Validation(ValidationError::GraphicsWidthMismatch { .. })));
```

---

## Public Type Definitions

### Type: `PrinterStatus`

**Definition**:
```rust
pub struct PrinterStatus {
    pub online: bool,
    pub paper_out: bool,
    pub error: bool,
}
```

**Description**: Represents printer operational status at query time.

**Usage**:
```rust
let status = printer.query_status(Duration::from_secs(2))?;
if !status.online {
    println!("Printer is offline");
}
```

---

### Type: `Font`

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Font {
    Roman,
    SansSerif,
    Courier,
    Script,
    Prestige,
}
```

**Description**: Available font typefaces.

**Usage**: See `select_font` method.

---

### Type: `Pitch`

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pitch {
    Pica,       // 10 CPI
    Elite,      // 12 CPI
    Condensed,  // 15 CPI
}
```

**Description**: Character pitch settings (characters per inch).

**Note**: Individual `select_Ncpi()` methods are provided; this enum is for future extensions.

---

### Type: `GraphicsMode`

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsMode {
    SingleDensity,  // 60 DPI
    DoubleDensity,  // 120 DPI
    HighDensity,    // 180 DPI
}
```

**Description**: Graphics printing density modes.

**Usage**: See `print_graphics` method.

---

### Type: `PrinterError`

**Definition**:
```rust
#[derive(Debug, Error)]
pub enum PrinterError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Permission denied: {message}")]
    Permission { path: String, message: String },

    #[error("Printer device not found: {path}")]
    DeviceNotFound { path: String },

    #[error("Printer disconnected")]
    Disconnected,

    #[error("Timeout waiting for printer response after {timeout:?}")]
    Timeout { timeout: Duration },

    #[error("Printer buffer full, retry operation")]
    BufferFull,

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}
```

**Description**: Comprehensive error type for all printer operations.

**Trait Implementations**:
- `std::error::Error`
- `Debug`
- `Display`

---

### Type: `ValidationError`

**Definition**:
```rust
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Micro-feed value must be 1-255, got 0")]
    MicroFeedZero,

    #[error("Graphics width {width} exceeds maximum {max_width}")]
    GraphicsWidthExceeded { width: u16, max_width: u16 },

    #[error("Graphics data length {data_len} doesn't match width {width}")]
    GraphicsWidthMismatch { width: u16, data_len: usize },

    #[error("Page length must be at least 1, got {value}")]
    InvalidPageLength { value: u8 },
}
```

**Description**: Parameter validation errors (caught before I/O).

**Trait Implementations**:
- `std::error::Error`
- `Debug`
- `Display`

---

## Common Usage Patterns

### Pattern: Basic Receipt Printing

```rust
use escp_driver::Printer;
use std::time::Duration;

fn print_receipt(printer: &mut Printer<impl Write, impl Read>) -> Result<(), PrinterError> {
    // Initialize
    printer.reset()?;

    // Header
    printer.bold_on()?;
    printer.select_15cpi()?;
    printer.write_text("RECEIPT")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Items
    printer.select_10cpi()?;
    printer.write_text("Item 1: $10.00")?;
    printer.line_feed()?;
    printer.write_text("Item 2: $5.00")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Total
    printer.bold_on()?;
    printer.write_text("Total: $15.00")?;
    printer.bold_off()?;

    // Eject
    printer.form_feed()?;

    Ok(())
}
```

### Pattern: Error Handling

```rust
use escp_driver::{Printer, PrinterError};

fn safe_print(path: &str) {
    match Printer::open_device(path, 1440) {
        Ok(mut printer) => {
            if let Err(e) = printer.reset() {
                eprintln!("Reset failed: {}", e);
                return;
            }

            // Continue printing...
        }
        Err(PrinterError::Permission { message, .. }) => {
            eprintln!("Permission error:\n{}", message);
        }
        Err(PrinterError::DeviceNotFound { path }) => {
            eprintln!("Printer not found at: {}", path);
        }
        Err(e) => {
            eprintln!("Error opening printer: {}", e);
        }
    }
}
```

### Pattern: Status Checking

```rust
use std::time::Duration;

fn wait_for_ready(printer: &mut Printer<impl Write, impl Read>) -> Result<(), PrinterError> {
    loop {
        match printer.query_status(Duration::from_secs(2)) {
            Ok(status) if status.online && !status.paper_out && !status.error => {
                return Ok(());
            }
            Ok(status) => {
                eprintln!("Printer not ready (online={}, paper_out={}, error={})",
                         status.online, status.paper_out, status.error);
                std::thread::sleep(Duration::from_secs(1));
            }
            Err(PrinterError::Timeout { .. }) => {
                eprintln!("Printer not responding");
                return Err(PrinterError::Timeout { timeout: Duration::from_secs(2) });
            }
            Err(e) => return Err(e),
        }
    }
}
```

### Pattern: Mock Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};

    #[test]
    fn test_receipt_rendering() {
        let mut output = Vec::new();
        let input = Cursor::new(vec![]);
        let mut printer = Printer::new(&mut output, input, 1440);

        printer.bold_on().unwrap();
        printer.write_text("TEST").unwrap();
        printer.bold_off().unwrap();

        // Verify ESC/P2 bytes
        assert_eq!(&output[0..2], &[0x1B, 0x45]); // Bold on
        assert_eq!(&output[2..6], b"TEST");
        assert_eq!(&output[6..8], &[0x1B, 0x46]); // Bold off
    }
}
```

---

## Prelude Module

**Definition**:
```rust
pub mod prelude {
    pub use crate::printer::Printer;
    pub use crate::error::{PrinterError, ValidationError};
    pub use crate::types::{PrinterStatus, Font, Pitch, GraphicsMode};
}
```

**Usage**:
```rust
use escp_driver::prelude::*;

fn main() -> Result<(), PrinterError> {
    let mut printer = Printer::open_device("/dev/usb/lp0", 1440)?;
    printer.reset()?;
    Ok(())
}
```

---

## API Stability Guarantees

### V1.x.x Series
- All public APIs frozen (no breaking changes)
- New methods may be added (minor version bumps)
- Bug fixes do not change API signatures (patch version bumps)
- Deprecation warnings given for >= 1 minor version before removal

### SemVer Compliance
- MAJOR: Breaking API changes, removed methods
- MINOR: New methods, backward-compatible features
- PATCH: Bug fixes, documentation improvements

### Minimum Supported Rust Version (MSRV)
- Rust 1.91.1+ (stable channel, 2021 edition)
- MSRV updates are MINOR version bumps

---

## Optional Features

### Feature: `tracing`

**Description**: Enable structured tracing/logging using `tracing` crate.

**Cargo.toml**:
```toml
[dependencies]
escp-driver = { version = "1.0", features = ["tracing"] }
```

**Behavior**:
- Emits INFO-level spans for high-level operations (reset, status query, errors)
- Emits DEBUG-level events for detailed I/O (raw bytes, partial writes)
- Zero runtime overhead when disabled (feature not enabled)

**Usage**:
```rust
use tracing_subscriber;

fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    // All printer operations are automatically traced
    let mut printer = Printer::open_device("/dev/usb/lp0", 1440)?;
    printer.reset()?;  // Emits INFO span: "Resetting printer"
}
```

---

## Performance Characteristics

### Latency Targets
- Command execution: < 1 ms (excluding printer hardware latency)
- Status query: < 2 seconds (with timeout)
- Graphics printing: Best-effort (hardware-dependent)

### Memory Usage
- Printer struct: ~32 bytes (two trait objects + u16)
- Per-command overhead: Minimal stack allocations
- Graphics commands: Temporary heap allocation matching bitmap size

### Concurrency
- Not thread-safe (requires external synchronization via Arc<Mutex<>>)
- No internal locks or atomic operations
- Suitable for multi-threaded applications with proper synchronization

---

**Document Status**: Phase 1 Complete
**Next Steps**: Create quickstart guide (quickstart.md)

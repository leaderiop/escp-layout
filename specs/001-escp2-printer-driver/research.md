# ESC/P2 Printer Driver Research

**Feature**: 001-escp2-printer-driver
**Created**: 2025-11-20
**Status**: Completed

This document contains comprehensive research findings for implementing a type-safe, idiomatic Rust driver for ESC/P2 printers (specifically EPSON LQ-2090II).

---

## 1. ESC/P2 Command Set

**Decision**: Implement core ESC/P2 command set covering text formatting, page layout, basic graphics, and status queries (~30-40 commands).

**Rationale**: The spec requires coverage of all user stories without implementing the full ESC/P2 specification (which has 200+ commands). Focusing on the core commands needed for:
- Text formatting (bold, underline, double-strike, fonts, pitch)
- Page layout (margins, line spacing, page length, positioning)
- Basic graphics (bitmap printing in multiple density modes)
- Status queries (paper out, offline, error conditions, buffer status)
- Device control (reset, initialization, paper feed)

This approach balances functionality with implementation complexity while keeping the driver focused on practical use cases (POS systems, invoice printing, receipt printing).

**Alternatives considered**:
1. **Full ESC/P2 specification**: Rejected due to excessive scope (200+ commands), many of which are printer-specific extensions or rarely used features
2. **Text-only subset**: Rejected because graphics printing (logos, barcodes) is a key requirement (User Story 3)
3. **ESC/P (not ESC/P2)**: Rejected because ESC/P2 provides better graphics capabilities and more precise positioning

**Implementation notes**:

### Core Command Categories and Byte Sequences

#### 1.1 Initialization & Reset
```rust
// ESC @ - Reset printer to default state
// Hex: 1B 40
const CMD_RESET: &[u8] = &[0x1B, 0x40];
```

#### 1.2 Text Formatting
```rust
// Bold
// ESC E - Bold ON (Hex: 1B 45)
const CMD_BOLD_ON: &[u8] = &[0x1B, 0x45];
// ESC F - Bold OFF (Hex: 1B 46)
const CMD_BOLD_OFF: &[u8] = &[0x1B, 0x46];

// Underline
// ESC - n (n=1: on, n=0: off)
// Hex: 1B 2D nn
fn cmd_underline(enabled: bool) -> [u8; 3] {
    [0x1B, 0x2D, if enabled { 1 } else { 0 }]
}

// Double-strike
// ESC G - Double-strike ON (Hex: 1B 47)
const CMD_DOUBLE_STRIKE_ON: &[u8] = &[0x1B, 0x47];
// ESC H - Double-strike OFF (Hex: 1B 48)
const CMD_DOUBLE_STRIKE_OFF: &[u8] = &[0x1B, 0x48];
```

#### 1.3 Character Pitch (CPI - Characters Per Inch)
```rust
// ESC P - 10 CPI (Hex: 1B 50)
const CMD_PITCH_10CPI: &[u8] = &[0x1B, 0x50];
// ESC M - 12 CPI (Hex: 1B 4D)
const CMD_PITCH_12CPI: &[u8] = &[0x1B, 0x4D];
// ESC g - 15 CPI (Hex: 1B 67)
const CMD_PITCH_15CPI: &[u8] = &[0x1B, 0x67];
```

#### 1.4 Font Selection
```rust
// ESC k n - Select font type
// Hex: 1B 6B nn
// n values:
//   0 = Roman
//   1 = Sans Serif
//   2 = Courier
//   3 = Script
//   4 = Prestige
enum Font {
    Roman = 0,
    SansSerif = 1,
    Courier = 2,
    Script = 3,
    Prestige = 4,
}

fn cmd_select_font(font: Font) -> [u8; 3] {
    [0x1B, 0x6B, font as u8]
}
```

#### 1.5 Page Length Control
```rust
// ESC C n - Set page length in lines
// Hex: 1B 43 nn
// Valid range: 1-127 lines
fn cmd_page_length_lines(lines: u8) -> [u8; 3] {
    debug_assert!(lines >= 1, "Page length must be at least 1 line");
    [0x1B, 0x43, lines]
}

// ESC ( C 2 0 nL nH - Set page length in 1/360-inch units
// Hex: 1B 28 43 02 00 nL nH
// Length = nL + nH×256
// Valid range: 1-32767 units
fn cmd_page_length_dots(dots: u16) -> [u8; 7] {
    debug_assert!(dots >= 1, "Page length must be at least 1 dot");
    [
        0x1B, 0x28, 0x43, 0x02, 0x00,
        (dots & 0xFF) as u8,        // nL
        ((dots >> 8) & 0xFF) as u8, // nH
    ]
}
```

#### 1.6 Line Spacing
```rust
// ESC 3 n - Set line spacing in 1/180-inch units
// Hex: 1B 33 nn
// Valid range: 0-255
fn cmd_line_spacing(dots: u8) -> [u8; 3] {
    [0x1B, 0x33, dots]
}

// ESC 2 - Set default 1/6-inch line spacing
// Hex: 1B 32
const CMD_LINE_SPACING_DEFAULT: &[u8] = &[0x1B, 0x32];
```

#### 1.7 Paper Feed & Positioning
```rust
// FF - Form feed (advance to next page)
// Hex: 0C
const CMD_FORM_FEED: &[u8] = &[0x0C];

// ESC J n - Micro-forward feed (down) in 1/180-inch steps
// Hex: 1B 4A nn
// Valid range: 1-255
fn cmd_micro_forward(units: u8) -> Result<[u8; 3], ValidationError> {
    if units == 0 {
        return Err(ValidationError::MicroFeedZero);
    }
    Ok([0x1B, 0x4A, units])
}

// ESC j n - Micro-reverse feed (up) in 1/180-inch steps
// Hex: 1B 6A nn
// Valid range: 1-255
// Note: Maximum ~1.41 inches (254 units) reverse movement allowed
fn cmd_micro_reverse(units: u8) -> Result<[u8; 3], ValidationError> {
    if units == 0 {
        return Err(ValidationError::MicroFeedZero);
    }
    Ok([0x1B, 0x6A, units])
}

// LF - Line feed
// Hex: 0A
const CMD_LINE_FEED: &[u8] = &[0x0A];

// CR - Carriage return
// Hex: 0D
const CMD_CARRIAGE_RETURN: &[u8] = &[0x0D];
```

#### 1.8 Horizontal Positioning
```rust
// ESC $ nL nH - Absolute horizontal position in 1/60-inch units
// Hex: 1B 24 nL nH
// Position = nL + nH×256
fn cmd_absolute_horizontal_position(position: u16) -> [u8; 4] {
    [
        0x1B, 0x24,
        (position & 0xFF) as u8,        // nL
        ((position >> 8) & 0xFF) as u8, // nH
    ]
}

// ESC \ nL nH - Relative horizontal position in 1/120-inch units
// Hex: 1B 5C nL nH
// Offset = nL + nH×256 (signed 16-bit)
fn cmd_relative_horizontal_position(offset: i16) -> [u8; 4] {
    let unsigned = offset as u16;
    [
        0x1B, 0x5C,
        (unsigned & 0xFF) as u8,
        ((unsigned >> 8) & 0xFF) as u8,
    ]
}
```

#### 1.9 Margins
```rust
// ESC l n - Set left margin in characters
// Hex: 1B 6C nn
// Valid range: 0-255
fn cmd_left_margin(chars: u8) -> [u8; 3] {
    [0x1B, 0x6C, chars]
}

// ESC Q n - Set right margin in characters
// Hex: 1B 51 nn
// Valid range: 0-255
fn cmd_right_margin(chars: u8) -> [u8; 3] {
    [0x1B, 0x51, chars]
}
```

#### 1.10 Graphics Mode (Bitmap Printing)
```rust
// ESC K nL nH [data...] - Single-density graphics (60 DPI)
// Hex: 1B 4B nL nH [data bytes]
// Width = nL + nH×256 (number of data bytes/dots)

// ESC L nL nH [data...] - Double-density graphics (120 DPI)
// Hex: 1B 4C nL nH [data bytes]

// ESC Y nL nH [data...] - High-density graphics (180 DPI)
// Hex: 1B 59 nL nH [data bytes]

enum GraphicsMode {
    SingleDensity = 0x4B, // 60 DPI
    DoubleDensity = 0x4C, // 120 DPI
    HighDensity = 0x59,   // 180 DPI
}

fn cmd_graphics(mode: GraphicsMode, width: u16, data: &[u8]) -> Result<Vec<u8>, ValidationError> {
    // Validate width matches data length
    if width as usize != data.len() {
        return Err(ValidationError::GraphicsWidthMismatch);
    }

    let mut cmd = Vec::with_capacity(4 + data.len());
    cmd.push(0x1B);
    cmd.push(mode as u8);
    cmd.push((width & 0xFF) as u8);        // nL
    cmd.push(((width >> 8) & 0xFF) as u8); // nH
    cmd.extend_from_slice(data);
    Ok(cmd)
}
```

#### 1.11 Status Query Commands
```rust
// DLE EOT n - Request printer status (real-time)
// Hex: 10 04 nn
// nn values:
//   1 = printer status
//   2 = off-line cause
//   3 = error cause
//   4 = paper roll sensor status

enum StatusType {
    PrinterStatus = 1,
    OfflineCause = 2,
    ErrorCause = 3,
    PaperSensor = 4,
}

fn cmd_request_status(status_type: StatusType) -> [u8; 3] {
    [0x10, 0x04, status_type as u8]
}

// Status byte interpretation for PrinterStatus (n=1):
// Bit 3: 1 = offline, 0 = online
// Bit 5: 1 = paper out
// Bit 6: 1 = error occurred

struct PrinterStatus {
    pub online: bool,
    pub paper_out: bool,
    pub error: bool,
}

impl PrinterStatus {
    fn from_byte(byte: u8) -> Self {
        Self {
            online: (byte & 0b0000_1000) == 0,
            paper_out: (byte & 0b0010_0000) != 0,
            error: (byte & 0b0100_0000) != 0,
        }
    }
}
```

### Command Validation Rules

All commands must validate parameters before sending to printer:

1. **Micro-feed**: 1-255 range, return error if 0 or >255
2. **Graphics width**: Validate against maximum width specified at Printer construction
3. **Page length**: Must be at least 1 line/dot
4. **Margins**: 0-255 range
5. **Font/Pitch enums**: Type-safe via Rust enums, invalid values impossible at compile-time

---

## 2. Bidirectional Communication Pattern

**Decision**: Use `Write + Read` trait bounds with timeout-aware status query methods and explicit buffer management.

**Rationale**: ESC/P2 printers support bidirectional communication for status queries. The driver must:
1. Send commands via Write trait
2. Receive status responses via Read trait
3. Handle timeouts when printer doesn't respond
4. Manage partial reads and writes
5. Parse status bytes correctly

Rust's `std::io::{Write, Read}` traits provide the ideal abstraction, allowing the driver to work with file descriptors, serial ports, USB devices, or even mock implementations for testing.

**Alternatives considered**:
1. **Write-only trait**: Rejected because bidirectional communication is explicitly required (FR-021, FR-022)
2. **Custom I/O traits**: Rejected because `std::io` traits are idiomatic, well-understood, and already provide the needed functionality
3. **Async I/O (tokio)**: Rejected for V1 to maintain zero dependencies (Constitution Principle VII); can be added in V2 via feature flag

**Implementation notes**:

### 2.1 Printer Struct Design
```rust
use std::io::{self, Write, Read};
use std::time::Duration;

pub struct Printer<W: Write, R: Read> {
    writer: W,
    reader: R,
    max_graphics_width: u16,
}

impl<W: Write, R: Read> Printer<W, R> {
    /// Create a new Printer instance with custom Write and Read implementations
    pub fn new(writer: W, reader: R, max_graphics_width: u16) -> Self {
        Self {
            writer,
            reader,
            max_graphics_width,
        }
    }

    /// Send raw bytes to the printer
    pub fn send(&mut self, data: &[u8]) -> Result<(), PrinterError> {
        self.write_all_with_retry(data)?;
        Ok(())
    }

    /// Internal method: retry partial writes automatically (FR-029)
    fn write_all_with_retry(&mut self, mut data: &[u8]) -> Result<(), PrinterError> {
        while !data.is_empty() {
            match self.writer.write(data) {
                Ok(0) => {
                    return Err(PrinterError::Io(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    )));
                }
                Ok(n) => {
                    data = &data[n..];
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                    // Retry on interrupt
                    continue;
                }
                Err(e) => {
                    return Err(PrinterError::Io(e));
                }
            }
        }

        // Ensure bytes are flushed to device
        self.writer.flush()?;
        Ok(())
    }
}
```

### 2.2 Opening Device Files
```rust
use std::fs::OpenOptions;

impl Printer<File, File> {
    /// Open a printer device file (e.g., /dev/usb/lp0)
    pub fn open_device(path: &str, max_graphics_width: u16) -> Result<Self, PrinterError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .map_err(|e| {
                // Provide helpful error messages (FR-025)
                if e.kind() == io::ErrorKind::PermissionDenied {
                    PrinterError::Permission {
                        path: path.to_string(),
                        message: format!(
                            "Permission denied accessing '{}'. Try: 'sudo usermod -aG lp $USER' \
                             or run with appropriate permissions.",
                            path
                        ),
                    }
                } else if e.kind() == io::ErrorKind::NotFound {
                    PrinterError::DeviceNotFound {
                        path: path.to_string(),
                    }
                } else {
                    PrinterError::Io(e)
                }
            })?;

        // Use the same File handle for both read and write
        // (requires cloning the file descriptor internally)
        Ok(Self::new(file.try_clone()?, file, max_graphics_width))
    }
}
```

### 2.3 Status Query with Timeout
```rust
use std::time::{Duration, Instant};

impl<W: Write, R: Read> Printer<W, R> {
    /// Query printer status with timeout
    pub fn query_status(&mut self, timeout: Duration) -> Result<PrinterStatus, PrinterError> {
        // Send status query command
        self.send(&[0x10, 0x04, 0x01])?;

        // Read response with timeout
        let status_byte = self.read_byte_with_timeout(timeout)?;

        Ok(PrinterStatus::from_byte(status_byte))
    }

    /// Internal: read single byte with timeout
    fn read_byte_with_timeout(&mut self, timeout: Duration) -> Result<u8, PrinterError> {
        // Note: This requires platform-specific timeout configuration
        // On Unix: use `fcntl` to set O_NONBLOCK and poll/select
        // On Windows: use overlapped I/O
        // For now, we'll use a simplified approach with set_read_timeout

        // This is a simplified implementation - production code would need
        // platform-specific handling or use a crate like `mio` or `nix`

        let mut buf = [0u8; 1];
        let start = Instant::now();

        loop {
            match self.reader.read(&mut buf) {
                Ok(0) => {
                    // EOF - printer disconnected
                    return Err(PrinterError::Disconnected);
                }
                Ok(_) => {
                    return Ok(buf[0]);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock
                       || e.kind() == io::ErrorKind::TimedOut => {
                    if start.elapsed() >= timeout {
                        return Err(PrinterError::Timeout);
                    }
                    // Retry
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                    // Retry on interrupt
                    continue;
                }
                Err(e) => {
                    return Err(PrinterError::Io(e));
                }
            }
        }
    }
}
```

### 2.4 Platform-Specific Timeout Handling
For production-grade timeout handling, consider platform-specific approaches:

**Unix/Linux**:
```rust
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

#[cfg(unix)]
fn set_read_timeout(file: &File, timeout: Duration) -> io::Result<()> {
    use nix::sys::socket::{setsockopt, sockopt::ReceiveTimeout};
    use nix::sys::time::TimeVal;

    let fd = file.as_raw_fd();
    let timeval = TimeVal::new(
        timeout.as_secs() as i64,
        timeout.subsec_micros() as i64,
    );
    setsockopt(fd, ReceiveTimeout, &timeval)?;
    Ok(())
}
```

**Alternative**: Use `mio` crate (feature-gated) for non-blocking I/O with timeout support across platforms.

### 2.5 Buffer Management
```rust
// Printers have internal buffers that can fill up
// When buffer is full, return error and let caller retry

pub enum PrinterError {
    BufferFull,
    // ... other errors
}

// Buffer full detection: printer stops responding or returns specific status
// Implementation depends on printer model and communication protocol
```

---

## 3. Error Handling Architecture

**Decision**: Use thiserror for custom error types with descriptive messages and conversion traits.

**Rationale**: Idiomatic Rust error handling requires:
1. Custom error types that implement `std::error::Error`
2. Descriptive error messages for debugging (SC-005: enable diagnosis within 5 minutes)
3. Conversion from underlying I/O errors
4. Distinction between validation errors, I/O errors, and printer-specific errors
5. Clear permission errors with remediation instructions (FR-025)

The `thiserror` crate provides derive macros that eliminate boilerplate while maintaining idiomatic patterns.

**Alternatives considered**:
1. **Manual Error trait implementation**: Rejected due to excessive boilerplate and maintenance burden
2. **anyhow crate**: Rejected because it's better for applications than libraries; libraries should provide typed errors
3. **std::io::Error only**: Rejected because it cannot represent validation errors or printer-specific failures
4. **Result<T, Box<dyn Error>>**: Rejected because it loses type information and makes error handling awkward for users

**Implementation notes**:

### 3.1 Error Type Hierarchy
```rust
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrinterError {
    /// I/O error communicating with printer
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Permission denied accessing printer device
    #[error("Permission denied: {message}")]
    Permission { path: String, message: String },

    /// Printer device not found
    #[error("Printer device not found: {path}")]
    DeviceNotFound { path: String },

    /// Printer disconnected during operation
    #[error("Printer disconnected")]
    Disconnected,

    /// Timeout waiting for printer response
    #[error("Timeout waiting for printer response after {timeout:?}")]
    Timeout { timeout: std::time::Duration },

    /// Printer buffer full, retry needed
    #[error("Printer buffer full, retry operation")]
    BufferFull,

    /// Validation error (invalid parameters)
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    /// Micro-feed value must be 1-255
    #[error("Micro-feed value must be 1-255, got 0")]
    MicroFeedZero,

    /// Graphics width exceeds maximum
    #[error("Graphics width {width} exceeds maximum {max_width}")]
    GraphicsWidthExceeded { width: u16, max_width: u16 },

    /// Graphics data length doesn't match specified width
    #[error("Graphics data length {data_len} doesn't match width {width}")]
    GraphicsWidthMismatch { width: u16, data_len: usize },

    /// Invalid page length
    #[error("Page length must be at least 1, got {value}")]
    InvalidPageLength { value: u8 },
}
```

### 3.2 Error Conversion Pattern
```rust
// Automatic conversion from io::Error via #[from] attribute
impl From<io::Error> for PrinterError {
    fn from(e: io::Error) -> Self {
        PrinterError::Io(e)
    }
}

// Usage in driver code:
pub fn send(&mut self, data: &[u8]) -> Result<(), PrinterError> {
    self.writer.write_all(data)?; // Automatically converts io::Error
    Ok(())
}
```

### 3.3 Permission Error Handling
```rust
// Provide helpful error messages per FR-025
fn open_device(path: &str) -> Result<File, PrinterError> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|e| {
            if e.kind() == io::ErrorKind::PermissionDenied {
                PrinterError::Permission {
                    path: path.to_string(),
                    message: format!(
                        "Cannot access printer device '{}'. \n\
                         Solutions:\n\
                         - Add your user to 'lp' group: sudo usermod -aG lp $USER\n\
                         - Or run with sudo (not recommended)\n\
                         - Or adjust device permissions: sudo chmod 666 {}",
                        path, path
                    ),
                }
            } else {
                PrinterError::Io(e)
            }
        })
}
```

### 3.4 Validation Before I/O
```rust
// All validation happens BEFORE any I/O operation (FR-018, FR-006)
pub fn micro_forward(&mut self, units: u8) -> Result<(), PrinterError> {
    // Validate parameter first
    if units == 0 {
        return Err(ValidationError::MicroFeedZero.into());
    }

    // Only send to printer after validation succeeds
    self.send(&[0x1B, 0x4A, units])
}

pub fn print_graphics(
    &mut self,
    mode: GraphicsMode,
    width: u16,
    data: &[u8],
) -> Result<(), PrinterError> {
    // Validate width against maximum
    if width > self.max_graphics_width {
        return Err(ValidationError::GraphicsWidthExceeded {
            width,
            max_width: self.max_graphics_width,
        }.into());
    }

    // Validate data length
    if width as usize != data.len() {
        return Err(ValidationError::GraphicsWidthMismatch {
            width,
            data_len: data.len(),
        }.into());
    }

    // Send command after validation
    let mut cmd = vec![0x1B, mode as u8];
    cmd.push((width & 0xFF) as u8);
    cmd.push(((width >> 8) & 0xFF) as u8);
    cmd.extend_from_slice(data);
    self.send(&cmd)
}
```

### 3.5 Error Display for Users
```rust
// Example usage showing clear error messages:
match printer.micro_forward(0) {
    Err(PrinterError::Validation(ValidationError::MicroFeedZero)) => {
        eprintln!("Error: Micro-feed value must be 1-255, got 0");
    }
    Err(e) => eprintln!("Printer error: {}", e),
    Ok(_) => {}
}
```

---

## 4. Testing Strategy for I/O Devices

**Decision**: Use mock Write/Read implementations for unit tests, property-based testing for validation logic, and integration tests with simulated printer responses.

**Rationale**: Testing I/O code without physical hardware requires abstractions that:
1. Allow unit tests to verify command construction without actual I/O
2. Simulate printer responses for bidirectional communication tests
3. Test error conditions (disconnection, timeout, buffer full) that are hard to reproduce with real hardware
4. Enable property-based testing to verify validation logic with arbitrary inputs
5. Provide fast, deterministic tests for CI/CD pipelines

Rust's trait system makes this straightforward: implement `Write` and `Read` for test-specific types that capture commands and return predefined responses.

**Alternatives considered**:
1. **Hardware-only testing**: Rejected because it's slow, requires physical setup, and can't reliably test error conditions
2. **File-based testing**: Rejected because it doesn't simulate bidirectional communication or error conditions
3. **Docker containers with virtual printers**: Rejected for unit tests (too heavy), but could be useful for integration tests
4. **Mock crates (mockall, mockito)**: Rejected because manual mocks are simpler and don't add dependencies

**Implementation notes**:

### 4.1 Mock Write Implementation
```rust
#[cfg(test)]
pub struct MockWriter {
    buffer: Vec<u8>,
    fail_after_bytes: Option<usize>,
    write_count: usize,
}

#[cfg(test)]
impl MockWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            fail_after_bytes: None,
            write_count: 0,
        }
    }

    /// Configure writer to fail after N bytes written
    pub fn fail_after_bytes(mut self, n: usize) -> Self {
        self.fail_after_bytes = Some(n);
        self
    }

    /// Get all bytes written so far
    pub fn written(&self) -> &[u8] {
        &self.buffer
    }

    /// Get number of write calls
    pub fn write_count(&self) -> usize {
        self.write_count
    }
}

#[cfg(test)]
impl std::io::Write for MockWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_count += 1;

        if let Some(limit) = self.fail_after_bytes {
            if self.buffer.len() >= limit {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "simulated write failure",
                ));
            }
        }

        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
```

### 4.2 Mock Read Implementation
```rust
#[cfg(test)]
pub struct MockReader {
    data: Vec<u8>,
    position: usize,
    read_delay: Option<Duration>,
}

#[cfg(test)]
impl MockReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            position: 0,
            read_delay: None,
        }
    }

    /// Simulate slow reader (for timeout tests)
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.read_delay = Some(delay);
        self
    }
}

#[cfg(test)]
impl std::io::Read for MockReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(delay) = self.read_delay {
            std::thread::sleep(delay);
        }

        if self.position >= self.data.len() {
            return Ok(0); // EOF
        }

        let remaining = &self.data[self.position..];
        let to_read = std::cmp::min(buf.len(), remaining.len());
        buf[..to_read].copy_from_slice(&remaining[..to_read]);
        self.position += to_read;
        Ok(to_read)
    }
}
```

### 4.3 Unit Test Examples
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold_on_sends_correct_bytes() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1024);

        printer.bold_on().unwrap();

        assert_eq!(printer.writer.written(), &[0x1B, 0x45]);
    }

    #[test]
    fn test_micro_forward_validates_range() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1024);

        // Should reject 0
        let err = printer.micro_forward(0).unwrap_err();
        assert!(matches!(err, PrinterError::Validation(ValidationError::MicroFeedZero)));

        // Should accept 1-255
        assert!(printer.micro_forward(1).is_ok());
        assert!(printer.micro_forward(255).is_ok());
    }

    #[test]
    fn test_graphics_validates_width() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let max_width = 100;
        let mut printer = Printer::new(writer, reader, max_width);

        let data = vec![0xFF; 200];
        let err = printer.print_graphics(GraphicsMode::SingleDensity, 200, &data).unwrap_err();

        assert!(matches!(
            err,
            PrinterError::Validation(ValidationError::GraphicsWidthExceeded { width: 200, max_width: 100 })
        ));
    }

    #[test]
    fn test_status_query_parses_response() {
        let writer = MockWriter::new();
        // Simulate status response: online, no paper out, no error
        let reader = MockReader::new(vec![0b0000_0000]);
        let mut printer = Printer::new(writer, reader, 1024);

        let status = printer.query_status(Duration::from_secs(1)).unwrap();

        assert!(status.online);
        assert!(!status.paper_out);
        assert!(!status.error);
    }

    #[test]
    fn test_partial_write_retry() {
        // Simulate partial writes
        let writer = PartialMockWriter::new(vec![3, 2, 5]); // Write 3, then 2, then 5 bytes
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1024);

        let data = b"0123456789"; // 10 bytes
        printer.send(data).unwrap();

        assert_eq!(printer.writer.write_count(), 3); // Three write calls
        assert_eq!(printer.writer.written(), data);
    }
}
```

### 4.4 Property-Based Testing with Proptest
```rust
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_micro_forward_rejects_zero(value in 0u8..=0u8) {
            let writer = MockWriter::new();
            let reader = MockReader::new(vec![]);
            let mut printer = Printer::new(writer, reader, 1024);

            assert!(printer.micro_forward(value).is_err());
        }

        #[test]
        fn test_micro_forward_accepts_valid_range(value in 1u8..=255u8) {
            let writer = MockWriter::new();
            let reader = MockReader::new(vec![]);
            let mut printer = Printer::new(writer, reader, 1024);

            assert!(printer.micro_forward(value).is_ok());
        }

        #[test]
        fn test_graphics_width_validation(
            width in 0u16..=2000u16,
            max_width in 100u16..=1000u16,
        ) {
            let writer = MockWriter::new();
            let reader = MockReader::new(vec![]);
            let mut printer = Printer::new(writer, reader, max_width);

            let data = vec![0xFF; width as usize];
            let result = printer.print_graphics(GraphicsMode::SingleDensity, width, &data);

            if width > max_width {
                assert!(result.is_err());
            } else {
                assert!(result.is_ok());
            }
        }

        #[test]
        fn test_no_panic_on_arbitrary_text(text in "\\PC*") {
            let writer = MockWriter::new();
            let reader = MockReader::new(vec![]);
            let mut printer = Printer::new(writer, reader, 1024);

            // Should never panic, regardless of input
            let _ = printer.write_text(&text);
        }
    }
}
```

### 4.5 Integration Test Pattern
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_receipt_workflow() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1024);

        // Complete receipt printing workflow
        printer.reset().unwrap();
        printer.bold_on().unwrap();
        printer.write_text("RECEIPT").unwrap();
        printer.bold_off().unwrap();
        printer.line_feed().unwrap();
        printer.write_text("Item 1: $10.00").unwrap();
        printer.line_feed().unwrap();
        printer.write_text("Item 2: $5.00").unwrap();
        printer.line_feed().unwrap();
        printer.bold_on().unwrap();
        printer.write_text("Total: $15.00").unwrap();
        printer.bold_off().unwrap();
        printer.form_feed().unwrap();

        let output = printer.writer.written();

        // Verify command sequence
        assert!(output.starts_with(&[0x1B, 0x40])); // Reset
        // ... verify remaining commands
    }

    #[test]
    fn test_error_recovery_scenario() {
        let writer = MockWriter::new();
        // Simulate paper out status
        let reader = MockReader::new(vec![0b0010_0000]);
        let mut printer = Printer::new(writer, reader, 1024);

        let status = printer.query_status(Duration::from_secs(1)).unwrap();
        assert!(status.paper_out);

        // Printer should still accept commands even if paper out
        // (actual printing won't happen, but driver shouldn't crash)
        assert!(printer.write_text("test").is_ok());
    }
}
```

### 4.6 Testing Against Physical Hardware
While most tests use mocks, some integration tests should run against real hardware:

```rust
#[cfg(feature = "hardware-tests")]
#[test]
#[ignore] // Requires physical printer
fn test_physical_printer() {
    // This test requires a physical printer connected to /dev/usb/lp0
    let mut printer = Printer::open_device("/dev/usb/lp0", 1440).unwrap();

    printer.reset().unwrap();
    printer.write_text("Hardware test").unwrap();
    printer.form_feed().unwrap();

    // Visual verification required
}
```

### 4.7 CI Test Configuration
```yaml
# .github/workflows/test.yml
test:
  - name: Run unit tests
    run: cargo test --lib

  - name: Run integration tests
    run: cargo test --test '*'

  - name: Run property tests
    run: cargo test --release -- --ignored proptest

  # Hardware tests not run in CI (require physical printer)
```

---

## 5. Tracing Integration

**Decision**: Use `tracing` crate with feature-gated dependency, providing zero overhead when disabled and appropriate span/event hierarchy for I/O operations.

**Rationale**: The spec requires feature-gated observability (FR-032) that:
1. Adds zero overhead when disabled (matches existing `serde` feature pattern)
2. Uses `tracing` crate (tokio-rs ecosystem standard, per clarifications)
3. Emits spans for high-level operations (commands, status queries, errors)
4. Emits debug-level events for detailed I/O (raw bytes, partial writes)
5. Integrates seamlessly with application tracing infrastructure

The `tracing` crate is the de facto standard for structured logging in async Rust and provides zero-cost abstractions when the feature is disabled.

**Alternatives considered**:
1. **log crate**: Rejected because it's less powerful than tracing (no structured data, no spans) and less commonly used in modern Rust
2. **Custom logging**: Rejected due to reinventing the wheel and poor ecosystem integration
3. **println!/eprintln!**: Rejected because it's not structured, not configurable, and pollutes stdout/stderr
4. **Always-on logging**: Rejected because it violates zero-dependency principle and adds overhead

**Implementation notes**:

### 5.1 Cargo.toml Configuration
```toml
[dependencies]
# No required dependencies (Constitution Principle VII)

[dependencies.tracing]
version = "0.1"
optional = true
default-features = false

[features]
default = []
# Optional features (no runtime overhead when disabled)
serde = ["dep:serde"]
tracing = ["dep:tracing"]
```

### 5.2 Conditional Compilation Macros
```rust
// Create no-op macros when tracing is disabled

#[cfg(feature = "tracing")]
use tracing::{debug, error, info, instrument, span, warn, Level};

#[cfg(not(feature = "tracing"))]
macro_rules! instrument {
    (level = $level:expr, $($rest:tt)*) => {};
    ($($rest:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
macro_rules! info {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
macro_rules! warn {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
macro_rules! error {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
macro_rules! span {
    ($level:expr, $name:expr $(, $field:tt)*) => {
        // No-op
    };
}
```

### 5.3 Instrumented Command Methods
```rust
impl<W: Write, R: Read> Printer<W, R> {
    /// Send raw bytes to printer with tracing
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self, data), fields(bytes_len = data.len())))]
    pub fn send(&mut self, data: &[u8]) -> Result<(), PrinterError> {
        #[cfg(feature = "tracing")]
        debug!("Sending {} bytes to printer", data.len());

        #[cfg(feature = "tracing")]
        debug!(bytes = ?data, "Raw bytes");

        self.write_all_with_retry(data)?;

        #[cfg(feature = "tracing")]
        debug!("Successfully sent bytes");

        Ok(())
    }

    /// Reset printer with tracing
    #[cfg_attr(feature = "tracing", instrument(level = "info", skip(self)))]
    pub fn reset(&mut self) -> Result<(), PrinterError> {
        #[cfg(feature = "tracing")]
        info!("Resetting printer");

        self.send(&[0x1B, 0x40])
    }

    /// Enable bold mode with tracing
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self)))]
    pub fn bold_on(&mut self) -> Result<(), PrinterError> {
        self.send(&[0x1B, 0x45])
    }

    /// Query printer status with tracing
    #[cfg_attr(feature = "tracing", instrument(level = "info", skip(self), fields(timeout = ?timeout)))]
    pub fn query_status(&mut self, timeout: Duration) -> Result<PrinterStatus, PrinterError> {
        #[cfg(feature = "tracing")]
        info!("Querying printer status with timeout {:?}", timeout);

        // Send status request
        self.send(&[0x10, 0x04, 0x01])?;

        // Read response
        let status_byte = match self.read_byte_with_timeout(timeout) {
            Ok(byte) => {
                #[cfg(feature = "tracing")]
                debug!(status_byte = byte, "Received status byte");
                byte
            }
            Err(e) => {
                #[cfg(feature = "tracing")]
                error!("Failed to read status: {:?}", e);
                return Err(e);
            }
        };

        let status = PrinterStatus::from_byte(status_byte);

        #[cfg(feature = "tracing")]
        info!(
            online = status.online,
            paper_out = status.paper_out,
            error = status.error,
            "Printer status parsed"
        );

        Ok(status)
    }

    /// Internal write with retry and detailed tracing
    #[cfg_attr(feature = "tracing", instrument(level = "debug", skip(self, data), fields(bytes_len = data.len())))]
    fn write_all_with_retry(&mut self, mut data: &[u8]) -> Result<(), PrinterError> {
        let total_bytes = data.len();
        let mut bytes_written = 0;

        while !data.is_empty() {
            match self.writer.write(data) {
                Ok(0) => {
                    #[cfg(feature = "tracing")]
                    error!("Write returned 0 bytes (write zero error)");

                    return Err(PrinterError::Io(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "failed to write whole buffer",
                    )));
                }
                Ok(n) => {
                    #[cfg(feature = "tracing")]
                    debug!(
                        bytes_written = n,
                        bytes_remaining = data.len() - n,
                        progress = format!("{}/{}", bytes_written + n, total_bytes),
                        "Partial write"
                    );

                    bytes_written += n;
                    data = &data[n..];
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                    #[cfg(feature = "tracing")]
                    debug!("Write interrupted, retrying");
                    continue;
                }
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    error!(error = ?e, bytes_written, "Write error");

                    return Err(PrinterError::Io(e));
                }
            }
        }

        #[cfg(feature = "tracing")]
        debug!(bytes_written, "Flushing write buffer");

        self.writer.flush()?;

        #[cfg(feature = "tracing")]
        debug!("Write complete");

        Ok(())
    }
}
```

### 5.4 Span Hierarchy Design

```rust
// High-level operation span (INFO level)
#[instrument(level = "info")]
pub fn print_receipt(&mut self, items: &[Item]) -> Result<(), PrinterError> {
    // Command spans (DEBUG level)
    self.reset()?;
    self.bold_on()?;
    self.write_text("RECEIPT")?;
    self.bold_off()?;

    for item in items {
        self.write_text(&format!("{}: ${:.2}", item.name, item.price))?;
        self.line_feed()?;
    }

    self.form_feed()?;
    Ok(())
}

// This creates a span hierarchy:
// print_receipt (INFO)
//   └─ reset (INFO)
//      └─ send (DEBUG)
//         └─ write_all_with_retry (DEBUG)
//   └─ bold_on (DEBUG)
//      └─ send (DEBUG)
//   └─ write_text (DEBUG)
//      └─ send (DEBUG)
//   ...
```

### 5.5 Usage Example (Application Code)
```rust
// Application enables tracing subscriber
#[cfg(feature = "tracing")]
fn setup_tracing() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

fn main() {
    #[cfg(feature = "tracing")]
    setup_tracing();

    let mut printer = Printer::open_device("/dev/usb/lp0", 1440).unwrap();

    // All operations are automatically traced
    printer.reset().unwrap();
    printer.write_text("Hello").unwrap();
}
```

### 5.6 Output Examples

**With tracing enabled (RUST_LOG=debug)**:
```
2025-11-20T10:30:15.123Z DEBUG escp_driver::printer: Sending 2 bytes to printer
2025-11-20T10:30:15.124Z DEBUG escp_driver::printer: Raw bytes bytes=[0x1b, 0x40]
2025-11-20T10:30:15.125Z DEBUG escp_driver::printer: Write complete bytes_written=2
2025-11-20T10:30:15.126Z  INFO escp_driver::printer: Resetting printer
2025-11-20T10:30:15.127Z DEBUG escp_driver::printer: Sending 5 bytes to printer
2025-11-20T10:30:15.128Z DEBUG escp_driver::printer: Raw bytes bytes=[0x48, 0x65, 0x6c, 0x6c, 0x6f]
```

**With tracing disabled (no feature flag)**: Zero runtime overhead, no output.

### 5.7 Testing Tracing Integration
```rust
#[cfg(all(test, feature = "tracing"))]
mod tracing_tests {
    use super::*;
    use tracing_subscriber::{layer::SubscriberExt, Registry};
    use tracing_test::TracingTest;

    #[test]
    #[traced_test]
    fn test_tracing_emits_spans() {
        let writer = MockWriter::new();
        let reader = MockReader::new(vec![]);
        let mut printer = Printer::new(writer, reader, 1024);

        printer.reset().unwrap();

        // Verify spans were emitted (tracing-test captures them)
        assert!(logs_contain("Resetting printer"));
    }
}
```

### 5.8 Benchmark Verification (Zero Overhead)
```rust
// Verify tracing feature adds zero overhead when disabled
#[cfg(all(test, not(feature = "tracing")))]
mod bench_no_tracing {
    use super::*;
    use criterion::{black_box, Criterion};

    pub fn bench_send_without_tracing(c: &mut Criterion) {
        let mut printer = Printer::new(MockWriter::new(), MockReader::new(vec![]), 1024);

        c.bench_function("send_no_tracing", |b| {
            b.iter(|| {
                printer.send(black_box(&[0x1B, 0x40])).unwrap();
            });
        });
    }
}

#[cfg(all(test, feature = "tracing"))]
mod bench_with_tracing {
    // Same benchmark with tracing enabled
    // CI verifies overhead is <5% when spans are filtered out by subscriber
}
```

---

## Summary of Key Technical Decisions

### 1. Command Set Architecture
- **Core ESC/P2 subset**: ~30-40 commands covering all user stories
- **Type-safe enums**: Font, GraphicsMode, Pitch, StatusType prevent invalid values at compile-time
- **Byte-level precision**: All commands documented with exact hex sequences
- **Validation before I/O**: All parameters validated before sending to printer

### 2. Bidirectional Communication
- **Trait-based abstraction**: `Write + Read` trait bounds for flexibility
- **Timeout-aware status queries**: Prevent application hangs (SC-008: 2-second timeout)
- **Automatic retry**: Partial writes retried until complete (FR-029)
- **Platform-agnostic**: Works with files, serial ports, USB, mock implementations

### 3. Error Handling
- **thiserror for custom errors**: Eliminates boilerplate while maintaining idiomaticity
- **Typed error hierarchy**: `PrinterError` and `ValidationError` with descriptive messages
- **Permission errors with remediation**: Clear instructions (e.g., "Add user to lp group")
- **Validation-before-I/O pattern**: All errors caught before sending to device

### 4. Testing Strategy
- **Mock Write/Read implementations**: Enable unit tests without hardware
- **Property-based testing**: Verify validation logic with arbitrary inputs (proptest)
- **Integration tests**: Simulate complete workflows with mock I/O
- **Hardware tests**: Optional feature-gated tests for physical printer validation
- **Zero-panic guarantee**: Fuzzing and property tests verify no panics in release builds

### 5. Tracing Integration
- **Feature-gated tracing dependency**: Zero overhead when disabled
- **Structured span hierarchy**: INFO for operations, DEBUG for I/O details
- **tracing crate (tokio-rs standard)**: Ecosystem compatibility
- **Application-controlled subscriber**: Driver emits spans, application chooses output format
- **Benchmarked zero-cost**: CI verifies <5% overhead when tracing disabled

---

## Implementation Roadmap

Based on this research, the recommended implementation order is:

1. **Phase 1 - Foundation** (User Story 5 prerequisite):
   - Error types (`PrinterError`, `ValidationError`)
   - Basic `Printer` struct with `new()` constructor
   - `send()` method with retry logic
   - Mock implementations for testing

2. **Phase 2 - Basic Text Printing** (User Story 1):
   - Reset, initialization
   - Text formatting (bold, underline, double-strike)
   - Character pitch selection
   - Font selection
   - `write_text()` method

3. **Phase 3 - Page Layout** (User Story 2):
   - Page length (lines and dots)
   - Form feed
   - Micro-feed (forward/reverse with validation)
   - Margins
   - Line spacing
   - Horizontal positioning

4. **Phase 4 - Status & Error Recovery** (User Story 5):
   - Device file opening with permission errors
   - Status query with timeout
   - Disconnection handling
   - Buffer full detection

5. **Phase 5 - Graphics** (User Story 3):
   - Graphics mode enum
   - Graphics command construction
   - Width validation against max_graphics_width
   - Bitmap data handling

6. **Phase 6 - Advanced Text Formatting** (User Story 4):
   - Additional pitch modes
   - Additional font styles
   - Any remaining text effects

7. **Phase 7 - Observability** (Cross-cutting):
   - Feature-gated tracing integration
   - Span instrumentation on all public methods
   - Debug events for detailed I/O

8. **Phase 8 - Documentation & Examples**:
   - Rustdoc for all public APIs
   - README with quickstart
   - Example programs for each user story
   - Troubleshooting guide

This roadmap ensures each phase delivers testable, valuable functionality while building toward the complete driver specification.

---

## References

1. **ESC/P2 Reference Manual**: EPSON ESC/P2 Programming Manual (available from Epson)
2. **EPSON LQ-2090II User Manual**: Printer-specific command reference
3. **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
4. **thiserror documentation**: https://docs.rs/thiserror/
5. **tracing documentation**: https://docs.rs/tracing/
6. **proptest documentation**: https://docs.rs/proptest/
7. **Project Constitution**: `/Users/mohammadalmechkor/Projects/escp-layout/.specify/memory/constitution.md`

---

**Research completed**: 2025-11-20
**Next step**: Create implementation plan (plan.md) and task breakdown (tasks.md)

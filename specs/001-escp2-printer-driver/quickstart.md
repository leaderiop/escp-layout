# Quickstart Guide: ESC/P2 Printer Driver

**Feature**: 001-escp2-printer-driver
**Created**: 2025-11-20
**Target**: Get printing "Hello World" in under 5 minutes

This guide will help you start using the ESC/P2 printer driver to print formatted text to ESC/P2-compatible printers (EPSON LQ-2090II and similar).

---

## Prerequisites

### Required
- **Rust 1.91.1 or later** (stable channel)
  ```bash
  rustc --version
  # Should show: rustc 1.91.1 (stable) or later
  ```

### Optional
- **Physical ESC/P2 printer** (for hardware testing)
  - EPSON LQ-2090II or compatible dot-matrix printer
  - USB or serial connection to your computer
  - Loaded with paper

### System Requirements
- **Linux**: Printer device accessible at `/dev/usb/lp0` or `/dev/ttyUSB0`
- **macOS**: Printer device accessible at `/dev/cu.usbserial` or similar
- **Windows**: Serial port accessible at `COM1`, `COM2`, etc.

**Note**: You can develop and test without a physical printer using mock I/O (see Testing section).

---

## Installation

Add the ESC/P2 printer driver to your `Cargo.toml`:

```toml
[dependencies]
escp-driver = "1.0"  # Replace with actual version when published
```

**Optional features**:
```toml
[dependencies]
escp-driver = { version = "1.0", features = ["tracing"] }  # Enable logging
```

Then run:
```bash
cargo build
```

---

## Hello World Example (< 50 lines)

Create a new file `examples/hello_world.rs`:

```rust
use escp_driver::prelude::*;
use std::time::Duration;

fn main() -> Result<(), PrinterError> {
    // Open printer device
    // For Linux/macOS: "/dev/usb/lp0" or "/dev/ttyUSB0"
    // For Windows: "COM1" (requires additional handling)
    let mut printer = Printer::open_device("/dev/usb/lp0", 1440)?;

    // Reset printer to default state
    printer.reset()?;

    // Check printer status (optional)
    let status = printer.query_status(Duration::from_secs(2))?;
    if !status.online || status.paper_out || status.error {
        eprintln!("Printer not ready: online={}, paper_out={}, error={}",
                 status.online, status.paper_out, status.error);
        return Ok(());
    }

    // Print "Hello, World!" with formatting
    printer.bold_on()?;
    printer.write_text("Hello, World!")?;
    printer.bold_off()?;

    // Add newline
    printer.line_feed()?;
    printer.line_feed()?;

    // Print additional text with different formatting
    printer.underline_on()?;
    printer.write_text("Welcome to ESC/P2 printing!")?;
    printer.underline_off()?;

    // Eject page
    printer.form_feed()?;

    println!("Printed successfully!");
    Ok(())
}
```

**Run the example**:
```bash
cargo run --example hello_world
```

**Expected output on printer**:
```
Hello, World!  (in bold)

Welcome to ESC/P2 printing!  (underlined)
```

---

## Common Patterns

### Pattern 1: Opening a Printer Device

```rust
use escp_driver::prelude::*;

fn open_printer() -> Result<Printer<File, File>, PrinterError> {
    // Method 1: Simple open (most common)
    let printer = Printer::open_device("/dev/usb/lp0", 1440)?;
    Ok(printer)
}

// Handle common errors
fn safe_open() {
    match Printer::open_device("/dev/usb/lp0", 1440) {
        Ok(printer) => {
            println!("Printer opened successfully");
            // Use printer...
        }
        Err(PrinterError::Permission { message, .. }) => {
            eprintln!("Permission error:");
            eprintln!("{}", message);
            // Follow printed instructions (add user to 'lp' group)
        }
        Err(PrinterError::DeviceNotFound { path }) => {
            eprintln!("Printer not found at: {}", path);
            eprintln!("Check connection and path");
        }
        Err(e) => {
            eprintln!("Failed to open printer: {}", e);
        }
    }
}
```

**Common device paths**:
- **Linux USB**: `/dev/usb/lp0`, `/dev/usb/lp1`
- **Linux Serial**: `/dev/ttyUSB0`, `/dev/ttyS0`
- **macOS Serial**: `/dev/cu.usbserial`, `/dev/tty.usbserial`
- **Windows Serial**: Use `serialport` crate for COM port access

**Troubleshooting device access**:
```bash
# Linux: Check device permissions
ls -l /dev/usb/lp0

# Add your user to 'lp' group for printer access
sudo usermod -aG lp $USER
# Log out and back in for changes to take effect

# Check if device exists
ls /dev/usb/
```

---

### Pattern 2: Error Handling

```rust
use escp_driver::prelude::*;

fn print_with_error_handling(printer: &mut Printer<impl Write, impl Read>) {
    // Handle specific validation errors
    match printer.micro_forward(0) {
        Err(PrinterError::Validation(ValidationError::MicroFeedZero)) => {
            eprintln!("Error: Micro-feed value must be 1-255");
        }
        Err(e) => eprintln!("Unexpected error: {}", e),
        Ok(_) => {}
    }

    // Handle I/O errors
    match printer.write_text("Hello") {
        Err(PrinterError::Io(e)) if e.kind() == std::io::ErrorKind::BrokenPipe => {
            eprintln!("Printer disconnected");
        }
        Err(PrinterError::Disconnected) => {
            eprintln!("Printer connection lost");
        }
        Err(e) => eprintln!("Error: {}", e),
        Ok(_) => println!("Text printed successfully"),
    }
}

// Generic error handling with ?
fn print_receipt() -> Result<(), PrinterError> {
    let mut printer = Printer::open_device("/dev/usb/lp0", 1440)?;
    printer.reset()?;
    printer.write_text("Receipt content")?;
    printer.form_feed()?;
    Ok(())
}
```

---

### Pattern 3: Status Checking

```rust
use escp_driver::prelude::*;
use std::time::Duration;

fn check_printer_ready(printer: &mut Printer<impl Write, impl Read>) -> Result<bool, PrinterError> {
    let status = printer.query_status(Duration::from_secs(2))?;

    if status.online && !status.paper_out && !status.error {
        Ok(true)
    } else {
        eprintln!("Printer status:");
        eprintln!("  Online: {}", status.online);
        eprintln!("  Paper out: {}", status.paper_out);
        eprintln!("  Error: {}", status.error);
        Ok(false)
    }
}

// Wait for printer to become ready
fn wait_until_ready(printer: &mut Printer<impl Write, impl Read>) -> Result<(), PrinterError> {
    loop {
        match printer.query_status(Duration::from_secs(2)) {
            Ok(status) if status.online && !status.paper_out && !status.error => {
                return Ok(());
            }
            Ok(_) => {
                eprintln!("Waiting for printer to be ready...");
                std::thread::sleep(Duration::from_secs(1));
            }
            Err(PrinterError::Timeout { .. }) => {
                eprintln!("Printer not responding, retrying...");
                std::thread::sleep(Duration::from_secs(1));
            }
            Err(e) => return Err(e),
        }
    }
}
```

---

### Pattern 4: Formatted Text Printing

```rust
use escp_driver::prelude::*;

fn print_formatted_document(printer: &mut Printer<impl Write, impl Read>) -> Result<(), PrinterError> {
    // Initialize
    printer.reset()?;

    // Title (bold, condensed)
    printer.bold_on()?;
    printer.select_15cpi()?;
    printer.write_text("SALES REPORT - 2025-11-20")?;
    printer.bold_off()?;
    printer.select_10cpi()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Section header (underlined)
    printer.underline_on()?;
    printer.write_text("Summary")?;
    printer.underline_off()?;
    printer.line_feed()?;

    // Body text
    printer.write_text("Total Sales: $1,234.56")?;
    printer.line_feed()?;
    printer.write_text("Items Sold: 42")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Footer (double-strike for emphasis)
    printer.double_strike_on()?;
    printer.write_text("*** END OF REPORT ***")?;
    printer.double_strike_off()?;

    // Eject page
    printer.form_feed()?;

    Ok(())
}
```

---

## Complete Receipt Example

Here's a complete receipt printing example demonstrating multiple formatting techniques:

```rust
use escp_driver::prelude::*;
use std::time::Duration;

fn print_receipt(printer: &mut Printer<impl Write, impl Read>) -> Result<(), PrinterError> {
    // Check printer status first
    let status = printer.query_status(Duration::from_secs(2))?;
    if !status.online || status.paper_out {
        return Err(PrinterError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Printer not ready"
        )));
    }

    // Initialize printer
    printer.reset()?;

    // Set margins for receipt (80mm paper)
    printer.set_left_margin(2)?;

    // Header - Store name (bold, condensed)
    printer.bold_on()?;
    printer.select_15cpi()?;
    printer.write_text("ACME STORE")?;
    printer.bold_off()?;
    printer.select_10cpi()?;
    printer.line_feed()?;

    // Store details
    printer.write_text("123 Main Street")?;
    printer.line_feed()?;
    printer.write_text("Anytown, USA")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Receipt header
    printer.underline_on()?;
    printer.write_text("RECEIPT")?;
    printer.underline_off()?;
    printer.line_feed()?;
    printer.write_text("Date: 2025-11-20  Time: 14:30")?;
    printer.line_feed()?;
    printer.write_text("Receipt #: 12345")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Items
    printer.write_text("Item                    Qty   Price")?;
    printer.line_feed()?;
    printer.write_text("------------------------------------")?;
    printer.line_feed()?;
    printer.write_text("Widget A                 2    $10.00")?;
    printer.line_feed()?;
    printer.write_text("Widget B                 1    $15.00")?;
    printer.line_feed()?;
    printer.write_text("Widget C                 3     $5.00")?;
    printer.line_feed()?;
    printer.write_text("------------------------------------")?;
    printer.line_feed()?;

    // Subtotal
    printer.write_text("Subtotal:                      $40.00")?;
    printer.line_feed()?;
    printer.write_text("Tax (8%):                       $3.20")?;
    printer.line_feed()?;

    // Total (bold)
    printer.bold_on()?;
    printer.write_text("TOTAL:                         $43.20")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Footer
    printer.write_text("Thank you for your purchase!")?;
    printer.line_feed()?;

    // Cut paper (form feed)
    printer.form_feed()?;

    Ok(())
}

fn main() -> Result<(), PrinterError> {
    let mut printer = Printer::open_device("/dev/usb/lp0", 1440)?;
    print_receipt(&mut printer)?;
    println!("Receipt printed successfully!");
    Ok(())
}
```

---

## Testing Without a Physical Printer

You can develop and test your code without a physical printer using mock I/O:

```rust
use escp_driver::Printer;
use std::io::Cursor;

#[test]
fn test_receipt_commands() {
    // Create in-memory buffers
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);

    // Create printer with mock I/O
    let mut printer = Printer::new(&mut output, input, 1440);

    // Print receipt
    printer.reset().unwrap();
    printer.bold_on().unwrap();
    printer.write_text("RECEIPT").unwrap();
    printer.bold_off().unwrap();
    printer.line_feed().unwrap();

    // Verify ESC/P2 bytes were generated correctly
    assert_eq!(&output[0..2], &[0x1B, 0x40]);  // Reset command
    assert_eq!(&output[2..4], &[0x1B, 0x45]);  // Bold on
    assert_eq!(&output[4..11], b"RECEIPT");
    assert_eq!(&output[11..13], &[0x1B, 0x46]); // Bold off
    assert_eq!(&output[13], 0x0A);             // Line feed

    println!("Generated ESC/P2 bytes: {:02X?}", output);
}
```

---

## Troubleshooting

### Problem: "Permission denied" when opening device

**Solution**:
```bash
# Linux: Add user to 'lp' group
sudo usermod -aG lp $USER
# Log out and back in

# Or run with sudo (not recommended for production)
sudo cargo run

# Or change device permissions (temporary)
sudo chmod 666 /dev/usb/lp0
```

### Problem: "Device not found"

**Solution**:
```bash
# Check if printer is connected and powered on
lsusb  # Linux: List USB devices

# Check available printer devices
ls /dev/usb/
ls /dev/tty*

# Check dmesg for printer detection
dmesg | grep -i printer
dmesg | grep -i usb
```

### Problem: "Timeout waiting for printer response"

**Solution**:
- Check printer is powered on
- Verify printer is online (not in standby mode)
- Check cable connection
- Increase timeout duration: `query_status(Duration::from_secs(5))`

### Problem: Nothing prints, but no errors

**Solution**:
- Check printer has paper loaded
- Verify printer is online (press "Online" button if present)
- Check if printer head is locked or ribbon cartridge installed (dot-matrix)
- Call `form_feed()` to eject page
- Some printers buffer output until page is full

### Problem: Garbled output or wrong characters

**Solution**:
- Verify printer is in ESC/P2 mode (not PCL or PostScript)
- Call `reset()` before printing to ensure clean state
- Check that non-ASCII characters are being handled (driver replaces with '?')

---

## Next Steps

### Learn More
- **API Documentation**: See `contracts/printer-api.md` for complete method reference
- **Data Model**: See `data-model.md` for entity definitions and validation rules
- **Examples**: Check `examples/` directory for more complex scenarios

### Advanced Topics
- **Graphics Printing**: Print logos and barcodes using `print_graphics()`
- **Page Layout**: Control margins, positioning, and line spacing
- **Multi-threaded Access**: Wrap printer in `Arc<Mutex<>>` for thread safety
- **Custom I/O**: Implement `Write` and `Read` traits for network printers

### Feature Flags
```toml
[dependencies]
escp-driver = { version = "1.0", features = ["tracing"] }
```

**Enable tracing for debugging**:
```rust
use tracing_subscriber;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // All printer operations will be logged
    let mut printer = Printer::open_device("/dev/usb/lp0", 1440)?;
    printer.reset()?;  // Logs: "Resetting printer"
}
```

---

## Quick Reference Card

### Common Commands
```rust
// Initialization
printer.reset()?;

// Text formatting
printer.bold_on()?;
printer.bold_off()?;
printer.underline_on()?;
printer.underline_off()?;

// Character pitch
printer.select_10cpi()?;  // Normal
printer.select_12cpi()?;  // Elite
printer.select_15cpi()?;  // Condensed

// Fonts
printer.select_font(Font::Courier)?;

// Output
printer.write_text("Hello")?;
printer.line_feed()?;
printer.form_feed()?;  // Eject page

// Status
let status = printer.query_status(Duration::from_secs(2))?;
```

### Error Handling
```rust
match result {
    Ok(_) => { /* success */ }
    Err(PrinterError::Permission { message, .. }) => { /* add user to lp group */ }
    Err(PrinterError::DeviceNotFound { .. }) => { /* check connection */ }
    Err(PrinterError::Timeout { .. }) => { /* printer not responding */ }
    Err(PrinterError::Disconnected) => { /* reconnect */ }
    Err(e) => { /* other error */ }
}
```

---

## Support

### Resources
- **GitHub Issues**: Report bugs and request features
- **API Documentation**: Generated via `cargo doc --open`
- **Examples**: `examples/` directory in repository

### Common ESC/P2 Printers
- EPSON LQ-2090II (target printer for this driver)
- EPSON LQ-570e, LQ-870, LQ-1170
- Compatible dot-matrix printers with ESC/P2 support

---

**Document Status**: Phase 1 Complete
**Time to First Print**: < 5 minutes (with connected printer)
**Next**: Explore full API documentation in `contracts/printer-api.md`

//! Basic "Hello World" example demonstrating ESC/P2 printer driver usage
//!
//! This example shows how to:
//! - Open a printer device
//! - Send formatted text with bold and underline
//! - Handle basic errors
//!
//! Run with: `cargo run --example hello_world`

use escp_layout::prelude::*;
use std::io::Cursor;

fn main() -> Result<(), PrinterError> {
    // For this example, we use mock I/O instead of a physical printer
    // In production, use: Printer::open_device("/dev/usb/lp0", 1440)?
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);
    let mut printer = Printer::new(&mut output, input, 1440);

    // Reset printer to default state
    printer.reset()?;

    // Print "Hello, World!" with bold formatting
    printer.bold_on()?;
    printer.write_text("Hello, World!")?;
    printer.bold_off()?;

    // Add newline
    printer.line_feed()?;
    printer.line_feed()?;

    // Print additional text with underline
    printer.underline_on()?;
    printer.write_text("Welcome to ESC/P2 printing!")?;
    printer.underline_off()?;

    // Eject page
    printer.line_feed()?;
    printer.line_feed()?;
    printer.form_feed()?;

    // Show generated ESC/P2 bytes
    println!("Successfully generated ESC/P2 commands:");
    println!("  Total bytes: {}", output.len());
    println!("  Command sequence:");
    println!("    - Reset (ESC @)");
    println!("    - Bold on (ESC E)");
    println!("    - Text: 'Hello, World!'");
    println!("    - Bold off (ESC F)");
    println!("    - Line feeds");
    println!("    - Underline on (ESC - 1)");
    println!("    - Text: 'Welcome to ESC/P2 printing!'");
    println!("    - Underline off (ESC - 0)");
    println!("    - Form feed");

    // To send to a physical printer, use:
    // let mut printer = Printer::open_device("/dev/usb/lp0", 1440)?;
    // ... (same commands as above) ...

    Ok(())
}

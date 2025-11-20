//! Simple 1D barcode printing example using graphics mode
//!
//! This example demonstrates how to print a simple barcode using bitmap graphics.
//! It creates a Code 39 style barcode pattern as a bitmap and prints it using
//! the ESC/P2 graphics commands.
//!
//! Run with: `cargo run --example barcode`

use escp_layout::prelude::*;
use std::io::Cursor;

fn main() -> Result<(), PrinterError> {
    // For this example, we use mock I/O
    // In production, use: Printer::open_device("/dev/usb/lp0", 1440)?
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);
    let mut printer = Printer::new(&mut output, input, 1440);

    // Print barcode label
    print_barcode_label(&mut printer)?;

    println!("Barcode generated successfully!");
    println!("Total ESC/P2 bytes: {}", output.len());
    println!("\nThis example creates a simple 1D barcode pattern");
    println!("using ESC/P2 graphics commands.");

    Ok(())
}

fn print_barcode_label<W: std::io::Write, R: std::io::Read>(
    printer: &mut Printer<W, R>,
) -> Result<(), PrinterError> {
    // Initialize printer
    printer.reset()?;

    // Print title
    printer.bold_on()?;
    printer.write_text("Product Barcode")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Generate a simple barcode pattern (bars and spaces)
    // This creates a simplified Code 39 style pattern for "12345"
    // In Code 39, bars are encoded as thin (1 unit) or wide (3 units)
    let barcode_pattern = generate_barcode_pattern("12345");

    // Print the barcode using high-density graphics mode (180 DPI)
    printer.print_graphics(
        GraphicsMode::HighDensity,
        barcode_pattern.len() as u16,
        &barcode_pattern,
    )?;
    printer.line_feed()?;

    // Print a second line to make the barcode taller
    printer.print_graphics(
        GraphicsMode::HighDensity,
        barcode_pattern.len() as u16,
        &barcode_pattern,
    )?;
    printer.line_feed()?;
    printer.print_graphics(
        GraphicsMode::HighDensity,
        barcode_pattern.len() as u16,
        &barcode_pattern,
    )?;
    printer.line_feed()?;

    // Print human-readable text below barcode
    printer.line_feed()?;
    printer.write_text("     *12345*")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Print product details
    printer.write_text("Product: Widget A")?;
    printer.line_feed()?;
    printer.write_text("Price: $10.00")?;
    printer.line_feed()?;
    printer.write_text("SKU: 12345")?;
    printer.line_feed()?;

    // Eject
    printer.form_feed()?;

    Ok(())
}

/// Generate a simple barcode pattern as a bitmap
///
/// This is a simplified representation for demonstration purposes.
/// A real barcode would require proper encoding according to the
/// barcode standard (Code 39, Code 128, EAN, etc.)
fn generate_barcode_pattern(data: &str) -> Vec<u8> {
    let mut pattern = Vec::new();

    // Start guard pattern (thin bar, thin space, thin bar)
    pattern.extend_from_slice(&[0xFF, 0x00, 0xFF, 0x00]);

    // For each character, create a simple pattern
    for ch in data.chars() {
        // Convert character to digit (simplified)
        if let Some(digit) = ch.to_digit(10) {
            // Create pattern: alternate bars and spaces
            // Even digits: thick bar, thin space
            // Odd digits: thin bar, thick space
            if digit % 2 == 0 {
                pattern.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0x00]);
            } else {
                pattern.extend_from_slice(&[0xFF, 0x00, 0x00, 0x00]);
            }
        }
    }

    // End guard pattern
    pattern.extend_from_slice(&[0xFF, 0x00, 0xFF]);

    pattern
}

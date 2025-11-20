//! Graphics printing example demonstrating logo and barcode printing
//!
//! This example shows how to use the ESC/P2 printer driver to print bitmap graphics
//! including a simple logo and a 1D barcode in different density modes.

use escp_layout::{types::GraphicsMode, Printer};
use std::io::Cursor;

fn main() {
    println!("=== ESC/P2 Graphics Printing Example ===\n");

    // For demonstration, we use in-memory buffers (mock I/O)
    // In production, replace with: Printer::open_device("/dev/usb/lp0", 1440)
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);

    match print_graphics_demo(&mut output, input) {
        Ok(_) => {
            println!("\n Graphics printed successfully!");
            println!("Generated {} bytes of ESC/P2 commands", output.len());
        }
        Err(e) => {
            eprintln!(" Error printing graphics: {}", e);
        }
    }
}

fn print_graphics_demo(
    output: &mut Vec<u8>,
    input: Cursor<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut printer = Printer::new(output, input, 1440);

    // Reset printer
    printer.reset()?;

    // === Example 1: Simple Company Logo ===
    println!("Example 1: Printing a simple company logo (checkerboard pattern)");

    printer.bold_on()?;
    printer.write_text("COMPANY LOGO:")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Create a simple checkerboard pattern logo (16x8 dots)
    // Each byte represents 8 vertical dots
    let logo_data = create_checkerboard_logo();

    // Print logo in high density mode (180 DPI)
    printer.print_graphics(
        GraphicsMode::HighDensity,
        logo_data.len() as u16,
        &logo_data,
    )?;
    printer.line_feed()?;
    printer.line_feed()?;

    println!("   Logo printed ({} bytes, high density)", logo_data.len());

    // === Example 2: 1D Barcode (Code 39 style bars) ===
    println!("Example 2: Printing a simple 1D barcode");

    printer.bold_on()?;
    printer.write_text("BARCODE:")?;
    printer.bold_off()?;
    printer.line_feed()?;

    // Create a simple barcode pattern (wide and narrow bars)
    let barcode_data = create_simple_barcode();

    // Print barcode in double density mode (120 DPI)
    printer.print_graphics(
        GraphicsMode::DoubleDensity,
        barcode_data.len() as u16,
        &barcode_data,
    )?;
    printer.line_feed()?;

    // Print human-readable text below barcode
    printer.write_text("         *12345*")?;
    printer.line_feed()?;
    printer.line_feed()?;

    println!(
        "   Barcode printed ({} bytes, double density)",
        barcode_data.len()
    );

    // === Example 3: Comparing Density Modes ===
    println!("Example 3: Comparing different density modes");

    printer.bold_on()?;
    printer.write_text("DENSITY COMPARISON:")?;
    printer.bold_off()?;
    printer.line_feed()?;

    // Create a simple test pattern (vertical lines)
    let test_pattern = create_test_pattern();

    // Single density (60 DPI)
    printer.write_text("Single Density (60 DPI):")?;
    printer.line_feed()?;
    printer.print_graphics(
        GraphicsMode::SingleDensity,
        test_pattern.len() as u16,
        &test_pattern,
    )?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Double density (120 DPI)
    printer.write_text("Double Density (120 DPI):")?;
    printer.line_feed()?;
    printer.print_graphics(
        GraphicsMode::DoubleDensity,
        test_pattern.len() as u16,
        &test_pattern,
    )?;
    printer.line_feed()?;
    printer.line_feed()?;

    // High density (180 DPI)
    printer.write_text("High Density (180 DPI):")?;
    printer.line_feed()?;
    printer.print_graphics(
        GraphicsMode::HighDensity,
        test_pattern.len() as u16,
        &test_pattern,
    )?;
    printer.line_feed()?;
    printer.line_feed()?;

    println!("   Density comparison printed");

    // Eject page
    printer.form_feed()?;

    println!("\n Graphics demo complete!");

    Ok(())
}

/// Create a simple checkerboard pattern logo (16x8 dots)
///
/// Each byte represents 8 vertical dots, with LSB at top
fn create_checkerboard_logo() -> Vec<u8> {
    // Create a 16-byte wide checkerboard pattern
    // Alternating columns of 0xAA (10101010) and 0x55 (01010101)
    let mut logo = Vec::new();

    for i in 0..16 {
        if i % 2 == 0 {
            logo.push(0xAA); // 10101010
        } else {
            logo.push(0x55); // 01010101
        }
    }

    logo
}

/// Create a simple 1D barcode pattern
///
/// Simulates Code 39 style barcode with wide and narrow bars
/// Pattern: start marker + data bars + stop marker
fn create_simple_barcode() -> Vec<u8> {
    let mut barcode = Vec::new();

    // Start marker (wide bar)
    barcode.push(0xFF); // Full bar
    barcode.push(0xFF);
    barcode.push(0x00); // Space
    barcode.push(0x00);

    // Data bars (representing digits 1, 2, 3, 4, 5)
    // Alternating narrow and wide bars
    for i in 0..5 {
        // Narrow bar
        barcode.push(0xFF);
        barcode.push(0x00);

        // Space
        barcode.push(0x00);

        // Wide bar
        if i % 2 == 0 {
            barcode.push(0xFF);
            barcode.push(0xFF);
            barcode.push(0xFF);
        } else {
            barcode.push(0xFF);
            barcode.push(0xFF);
        }

        // Space
        barcode.push(0x00);
        barcode.push(0x00);
    }

    // Stop marker (wide bar)
    barcode.push(0xFF);
    barcode.push(0xFF);
    barcode.push(0x00);
    barcode.push(0x00);

    barcode
}

/// Create a test pattern with vertical lines
///
/// Useful for comparing different graphics modes
fn create_test_pattern() -> Vec<u8> {
    let mut pattern = Vec::new();

    // Create alternating full and empty columns
    for i in 0..32 {
        if i % 4 == 0 || i % 4 == 1 {
            pattern.push(0xFF); // Full vertical line
        } else {
            pattern.push(0x00); // Empty
        }
    }

    pattern
}

/// Example: Creating a custom bitmap from a 2D boolean array
///
/// This function demonstrates how to convert a 2D boolean array
/// (where true = black pixel, false = white pixel) into ESC/P2 bitmap data.
///
/// # Arguments
///
/// * `bitmap` - 2D array where bitmap[y][x] represents a pixel
/// * `width` - Width in pixels (must equal bitmap[0].len())
/// * `height` - Height in pixels (must be multiple of 8, will be padded)
#[allow(dead_code)]
fn bitmap_to_escp2(bitmap: &[Vec<bool>], width: usize, height: usize) -> Vec<u8> {
    let mut data = Vec::new();

    // ESC/P2 bitmap is organized in columns, each byte represents 8 vertical pixels
    for x in 0..width {
        // Process 8 vertical pixels at a time
        for y_start in (0..height).step_by(8) {
            let mut byte = 0u8;

            for bit in 0..8 {
                let y = y_start + bit;

                if y < height && x < bitmap.len() && y < bitmap[x].len()
                    && bitmap[x][y] {
                        byte |= 1 << bit; // Set bit (LSB is top pixel)
                    }
            }

            data.push(byte);
        }
    }

    data
}

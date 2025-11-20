//! Professional document layout example
//!
//! This example demonstrates advanced page layout features including:
//! - Custom margins and line spacing
//! - Precise horizontal positioning
//! - Multi-column layouts
//! - Mixed fonts and pitches for document structure
//!
//! Run with: `cargo run --example document_layout`

use escp_layout::prelude::*;
use std::io::Cursor;

fn main() -> Result<(), PrinterError> {
    // For this example, we use mock I/O
    // In production, use: Printer::open_device("/dev/usb/lp0", 1440)?
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);
    let mut printer = Printer::new(&mut output, input, 1440);

    // Print professional document
    print_professional_document(&mut printer)?;

    println!("Professional document generated successfully!");
    println!("Total ESC/P2 bytes: {}", output.len());
    println!("\nThis example demonstrates:");
    println!("  - Custom margins and line spacing");
    println!("  - Precise horizontal positioning");
    println!("  - Mixed fonts for document structure");
    println!("  - Professional document formatting");

    Ok(())
}

fn print_professional_document<W: std::io::Write, R: std::io::Read>(
    printer: &mut Printer<W, R>,
) -> Result<(), PrinterError> {
    // Initialize printer
    printer.reset()?;

    // Set document margins (left: 5 chars, right: 75 chars for 80-char line)
    printer.set_left_margin(5)?;
    printer.set_right_margin(75)?;

    // Set tighter line spacing for professional look (1/8 inch = 22.5/180)
    printer.set_line_spacing(23)?;

    // Document Header
    printer.bold_on()?;
    printer.select_15cpi()?;
    printer.write_text("TECHNICAL SPECIFICATION DOCUMENT")?;
    printer.select_10cpi()?;
    printer.bold_off()?;
    printer.line_feed()?;

    // Separator line
    printer.write_text("================================================================")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Document metadata (two columns using absolute positioning)
    printer.write_text("Document No: TSD-2025-001")?;
    printer.line_feed()?;
    printer.write_text("Date: November 20, 2025")?;
    printer.line_feed()?;
    printer.write_text("Author: Engineering Team")?;
    printer.line_feed()?;
    printer.write_text("Status: Draft v1.0")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Section 1
    printer.bold_on()?;
    printer.underline_on()?;
    printer.write_text("1. INTRODUCTION")?;
    printer.underline_off()?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Body text with slightly tighter spacing
    printer.write_text("This document outlines the technical specifications for the")?;
    printer.line_feed()?;
    printer.write_text("ESC/P2 printer driver implementation. The driver provides a")?;
    printer.line_feed()?;
    printer.write_text("type-safe Rust interface to ESC/P2 compatible printers.")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Section 2
    printer.bold_on()?;
    printer.underline_on()?;
    printer.write_text("2. KEY FEATURES")?;
    printer.underline_off()?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Bullet points with indent
    printer.write_text("  * Type-safe command API with validation")?;
    printer.line_feed()?;
    printer.write_text("  * Bidirectional communication support")?;
    printer.line_feed()?;
    printer.write_text("  * Comprehensive error handling")?;
    printer.line_feed()?;
    printer.write_text("  * Zero runtime dependencies")?;
    printer.line_feed()?;
    printer.write_text("  * Feature-gated tracing support")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Section 3: Table (using Courier for monospace alignment)
    printer.bold_on()?;
    printer.underline_on()?;
    printer.write_text("3. COMMAND SUMMARY")?;
    printer.underline_off()?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Table header with Courier font for alignment
    printer.select_font(Font::Courier)?;
    printer.underline_on()?;
    printer.write_text("Command          Function              Parameters")?;
    printer.underline_off()?;
    printer.line_feed()?;

    // Table rows
    printer.write_text("bold_on()        Enable bold           None")?;
    printer.line_feed()?;
    printer.write_text("underline_on()   Enable underline      None")?;
    printer.line_feed()?;
    printer.write_text("select_font()    Change font           Font enum")?;
    printer.line_feed()?;
    printer.write_text("set_margins()    Set margins           left, right")?;
    printer.line_feed()?;
    printer.write_text("print_graphics() Print bitmap          mode, data")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Back to normal font
    printer.select_font(Font::Roman)?;

    // Section 4: Code example
    printer.bold_on()?;
    printer.underline_on()?;
    printer.write_text("4. USAGE EXAMPLE")?;
    printer.underline_off()?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Code block with Courier font and 12 CPI for readability
    printer.select_font(Font::Courier)?;
    printer.select_12cpi()?;
    printer.write_text("let mut printer = Printer::open_device(")?;
    printer.line_feed()?;
    printer.write_text("    \"/dev/usb/lp0\",")?;
    printer.line_feed()?;
    printer.write_text("    1440")?;
    printer.line_feed()?;
    printer.write_text(")?;")?;
    printer.line_feed()?;
    printer.write_text("printer.reset()?;")?;
    printer.line_feed()?;
    printer.write_text("printer.bold_on()?;")?;
    printer.line_feed()?;
    printer.write_text("printer.write_text(\"Hello\")?;")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Reset formatting
    printer.select_font(Font::Roman)?;
    printer.select_10cpi()?;

    // Footer with different line spacing
    printer.set_default_line_spacing()?;
    printer.line_feed()?;
    printer.write_text("================================================================")?;
    printer.line_feed()?;
    printer.select_15cpi()?;
    printer.write_text("End of Document - Page 1 of 1 - Confidential")?;
    printer.select_10cpi()?;
    printer.line_feed()?;

    // Eject page
    printer.form_feed()?;

    Ok(())
}

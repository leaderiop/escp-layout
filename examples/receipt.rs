//! Complete receipt printing example
//!
//! This example demonstrates a full receipt printing workflow including:
//! - Store header with different formatting
//! - Item listing with proper spacing
//! - Subtotal and total calculations
//! - Footer with thank you message
//!
//! Run with: `cargo run --example receipt`

use escp_layout::prelude::*;
use std::io::Cursor;

fn main() -> Result<(), PrinterError> {
    // For this example, we use mock I/O
    // In production, use: Printer::open_device("/dev/usb/lp0", 1440)?
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);
    let mut printer = Printer::new(&mut output, input, 1440);

    // Print the receipt
    print_receipt(&mut printer)?;

    // Display summary
    println!("Receipt generated successfully!");
    println!("Total ESC/P2 bytes: {}", output.len());
    println!("\nTo print to a physical printer:");
    println!("  1. Update the code to use: Printer::open_device(\"/dev/usb/lp0\", 1440)?");
    println!("  2. Ensure your printer is connected and has paper");
    println!("  3. Run the example again");

    Ok(())
}

fn print_receipt<W: std::io::Write, R: std::io::Read>(
    printer: &mut Printer<W, R>,
) -> Result<(), PrinterError> {
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
    printer.write_text("Anytown, USA 12345")?;
    printer.line_feed()?;
    printer.write_text("Tel: (555) 123-4567")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Receipt header
    printer.underline_on()?;
    printer.write_text("RECEIPT")?;
    printer.underline_off()?;
    printer.line_feed()?;
    printer.write_text("Date: 2025-11-20  Time: 14:30")?;
    printer.line_feed()?;
    printer.write_text("Receipt #: 00012345")?;
    printer.line_feed()?;
    printer.write_text("Cashier: Alice")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Items header
    printer.write_text("Item                    Qty   Price")?;
    printer.line_feed()?;
    printer.write_text("------------------------------------")?;
    printer.line_feed()?;

    // Items
    printer.write_text("Widget A                 2    $10.00")?;
    printer.line_feed()?;
    printer.write_text("Widget B                 1    $15.00")?;
    printer.line_feed()?;
    printer.write_text("Widget C                 3     $5.00")?;
    printer.line_feed()?;
    printer.write_text("Gadget X                 1    $25.00")?;
    printer.line_feed()?;
    printer.write_text("Doohickey Y              2     $7.50")?;
    printer.line_feed()?;
    printer.write_text("------------------------------------")?;
    printer.line_feed()?;

    // Subtotal
    printer.write_text("Subtotal:                      $70.00")?;
    printer.line_feed()?;
    printer.write_text("Tax (8.5%):                     $5.95")?;
    printer.line_feed()?;

    // Total (bold and double-strike for emphasis)
    printer.bold_on()?;
    printer.double_strike_on()?;
    printer.write_text("TOTAL:                         $75.95")?;
    printer.double_strike_off()?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Payment info
    printer.write_text("Payment Method: Cash")?;
    printer.line_feed()?;
    printer.write_text("Amount Paid:                   $80.00")?;
    printer.line_feed()?;
    printer.write_text("Change:                         $4.05")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Footer
    printer.underline_on()?;
    printer.write_text("Thank you for your purchase!")?;
    printer.underline_off()?;
    printer.line_feed()?;
    printer.write_text("Please come again!")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Barcode placeholder (in real implementation, would use graphics)
    printer.write_text("Barcode: *00012345*")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Cut paper (form feed)
    printer.form_feed()?;

    Ok(())
}

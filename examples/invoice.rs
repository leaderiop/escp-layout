//! Invoice printing example demonstrating page layout control
//!
//! This example shows how to use the ESC/P2 printer driver to print a multi-page
//! invoice with:
//! - Custom page length and margins
//! - Line spacing control
//! - Absolute and relative positioning
//! - Micro-feeding for precise positioning

use escp_layout::Printer;
use std::io::Cursor;

fn main() {
    println!("=== ESC/P2 Invoice Printing Example ===\n");

    // For demonstration, we use in-memory buffers (mock I/O)
    // In production, replace with: Printer::open_device("/dev/usb/lp0", 1440)
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);

    match print_invoice(&mut output, input) {
        Ok(_) => {
            println!(" Invoice printed successfully!");
            println!("\nGenerated {} bytes of ESC/P2 commands:", output.len());
            println!(
                "First 100 bytes: {:02X?}",
                &output[..std::cmp::min(100, output.len())]
            );
        }
        Err(e) => {
            eprintln!(" Error printing invoice: {}", e);
        }
    }
}

fn print_invoice(
    output: &mut Vec<u8>,
    input: Cursor<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut printer = Printer::new(output, input, 1440);

    // Reset printer to default state
    printer.reset()?;

    // Set page layout
    printer.set_page_length_lines(66)?; // 66 lines at 1/6" spacing = 11 inches
    printer.set_left_margin(5)?; // 5 character left margin
    printer.set_right_margin(75)?; // 75 character right margin
    printer.set_default_line_spacing()?; // 1/6 inch line spacing

    // === Page 1: Invoice Header and Details ===
    println!("Printing page 1: Header and line items...");

    // Company header (bold, condensed)
    printer.bold_on()?;
    printer.select_15cpi()?;
    printer.write_text("ACME CORPORATION")?;
    printer.bold_off()?;
    printer.select_10cpi()?;
    printer.line_feed()?;

    printer.write_text("123 Business Street")?;
    printer.line_feed()?;
    printer.write_text("Cityville, ST 12345")?;
    printer.line_feed()?;
    printer.write_text("Phone: (555) 123-4567")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Invoice title
    printer.bold_on()?;
    printer.underline_on()?;
    printer.write_text("INVOICE")?;
    printer.underline_off()?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Invoice details (using positioning)
    printer.write_text("Invoice Number:")?;
    printer.move_relative_x(120)?; // Move right 1 inch (120/120)
    printer.write_text("INV-2025-001")?;
    printer.carriage_return()?;
    printer.line_feed()?;

    printer.write_text("Date:")?;
    printer.move_relative_x(160)?; // Move right ~1.33 inches
    printer.write_text("November 20, 2025")?;
    printer.carriage_return()?;
    printer.line_feed()?;

    printer.write_text("Customer ID:")?;
    printer.move_relative_x(120)?;
    printer.write_text("CUST-5678")?;
    printer.carriage_return()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Bill To section
    printer.bold_on()?;
    printer.write_text("BILL TO:")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.write_text("  John Smith")?;
    printer.line_feed()?;
    printer.write_text("  456 Customer Avenue")?;
    printer.line_feed()?;
    printer.write_text("  Townsburg, ST 67890")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Line items header
    printer.bold_on()?;
    printer.write_text("Item Description         Qty    Unit Price    Total")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.write_text("-----------------------------------------------------")?;
    printer.line_feed()?;

    // Line items
    print_line_item(&mut printer, "Widget A", "10", "$25.00", "$250.00")?;
    print_line_item(&mut printer, "Widget B", "5", "$50.00", "$250.00")?;
    print_line_item(&mut printer, "Service Contract", "1", "$100.00", "$100.00")?;
    print_line_item(&mut printer, "Shipping", "1", "$25.00", "$25.00")?;

    printer.line_feed()?;
    printer.write_text("-----------------------------------------------------")?;
    printer.line_feed()?;

    // Totals
    printer.write_text("                              Subtotal:     $625.00")?;
    printer.line_feed()?;
    printer.write_text("                              Tax (8%):      $50.00")?;
    printer.line_feed()?;

    printer.bold_on()?;
    printer.write_text("                              TOTAL:        $675.00")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // === Page 2: Terms and Conditions ===
    printer.form_feed()?; // Eject page 1
    println!("Printing page 2: Terms and conditions...");

    printer.bold_on()?;
    printer.underline_on()?;
    printer.write_text("TERMS AND CONDITIONS")?;
    printer.underline_off()?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Use tighter line spacing for terms section
    printer.set_line_spacing(25)?; // 25/180" = ~0.139"

    printer.write_text("1. Payment is due within 30 days of invoice date.")?;
    printer.line_feed()?;
    printer.write_text("2. Late payments subject to 1.5% monthly interest.")?;
    printer.line_feed()?;
    printer.write_text("3. All sales are final unless defective.")?;
    printer.line_feed()?;
    printer.write_text("4. Returns must be made within 14 days.")?;
    printer.line_feed()?;
    printer.write_text("5. Shipping costs are non-refundable.")?;
    printer.line_feed()?;

    // Reset to default spacing
    printer.set_default_line_spacing()?;
    printer.line_feed()?;

    // Signature section with precise positioning using micro-feed
    printer.bold_on()?;
    printer.write_text("AUTHORIZED SIGNATURE")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Use micro-forward to position signature line precisely
    printer.write_text("_____________________________")?;
    printer.line_feed()?;
    printer.micro_forward(5)?; // Fine-tune vertical spacing (5/180")
    printer.write_text("Signature")?;
    printer.line_feed()?;
    printer.line_feed()?;

    printer.write_text("Date: _____________________")?;
    printer.line_feed()?;

    // Footer at bottom of page
    printer.line_feed()?;
    printer.line_feed()?;
    printer.line_feed()?;

    printer.select_12cpi()?; // Smaller font for footer
    printer.write_text("Thank you for your business!")?;
    printer.line_feed()?;
    printer.write_text("Questions? Call (555) 123-4567 or email support@acme.com")?;
    printer.select_10cpi()?;

    printer.form_feed()?; // Eject final page

    println!(" Invoice printing complete (2 pages)");

    Ok(())
}

/// Helper function to print a line item with aligned columns
fn print_line_item(
    printer: &mut Printer<&mut Vec<u8>, Cursor<Vec<u8>>>,
    description: &str,
    qty: &str,
    unit_price: &str,
    total: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Format: "Item Description         Qty    Unit Price    Total"
    let line = format!(
        "{:<25}{:>6}{:>14}{:>10}",
        description, qty, unit_price, total
    );
    printer.write_text(&line)?;
    printer.line_feed()?;
    Ok(())
}

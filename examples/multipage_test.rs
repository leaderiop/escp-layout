//! Multi-Page Printer Test
//!
//! Generates a 3-page document with:
//! - Page number on the first line of each page
//! - Page number on the last line of each page
//! - Content in the middle of each page
//!
//! Usage:
//!   cargo run --example multipage_test

use escp_layout::widget::{label_new, rect_new};
use escp_layout::{Document, Page};
use std::fs::File;
use std::io::Write;
use std::process::Command;

const PRINTER_NAME: &str = "EPSON_LQ_2090II";
const PAGE_WIDTH: u16 = 80;
const PAGE_HEIGHT: u16 = 50; // EPSON LQ-2090II page height (line 50 reserved)

/// Send ESC/P bytes to printer via CUPS
fn send_to_printer(escp_bytes: &[u8], job_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Write to temporary file
    let temp_path = "/tmp/escp_multipage_test.prn".to_string();
    let mut file = File::create(&temp_path)?;
    file.write_all(escp_bytes)?;
    file.flush()?;
    drop(file);

    println!(
        "Sending {} bytes to printer {} (job: {})",
        escp_bytes.len(),
        PRINTER_NAME,
        job_name
    );

    // Send to printer using lpr
    let output = Command::new("lpr")
        .arg("-P")
        .arg(PRINTER_NAME)
        .arg("-o")
        .arg("raw")
        .arg("-T")
        .arg(job_name)
        .arg(&temp_path)
        .output()?;

    if output.status.success() {
        println!("✓ Job '{}' sent successfully!", job_name);
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to print: {}", error).into())
    }
}

/// Create a page with page number at top and bottom
fn create_page(page_number: u16) -> Result<Page, Box<dyn std::error::Error>> {
    let mut root = rect_new!(PAGE_WIDTH, PAGE_HEIGHT);

    // Page number at the top (line 0)
    let top_label = label_new!(60).add_text(format!("Page {}", page_number))?;
    root.add_child(top_label, (10, 0))?;

    // Content in the middle
    let title = label_new!(60).add_text(format!(
        "EPSON LQ-2090II Multi-Page Test - Page {}",
        page_number
    ))?;
    root.add_child(title, (10, 5))?;

    let description = label_new!(70).add_text("This is a test of multi-page ESC/P printing.")?;
    root.add_child(description, (5, 10))?;

    let line1 = label_new!(70).add_text(format!("Current page: {}/2", page_number))?;
    root.add_child(line1, (5, 15))?;

    let line2 = label_new!(70).add_text("Each page has page numbers at top and bottom.")?;
    root.add_child(line2, (5, 18))?;

    let line3 = label_new!(70).add_text("Printer resets after each form feed.")?;
    root.add_child(line3, (5, 20))?;

    let line4 = label_new!(70).add_text("Testing 50-line page height".to_string())?;
    root.add_child(line4, (5, 22))?;

    // Add some additional content unique to each page
    match page_number {
        1 => {
            let content =
                label_new!(70).add_text("Page 1: Testing basic layout and page breaks")?;
            root.add_child(content, (5, 25))?;
        }
        2 => {
            let content =
                label_new!(70).add_text("Page 2: Testing printer reset - End of document")?;
            root.add_child(content, (5, 25))?;
        }
        _ => {}
    }

    // Page number at the bottom (last line)
    let bottom_label = label_new!(60).add_text(format!("End of Page {}", page_number))?;
    root.add_child(bottom_label, (10, PAGE_HEIGHT - 1))?;

    // Render to page
    let mut page_builder = Page::builder();
    page_builder.render(&root)?;
    Ok(page_builder.build())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== EPSON LQ-2090II Multi-Page Test ===\n");

    // Create document with 2 pages
    let mut doc_builder = Document::builder();

    println!("Creating Page 1...");
    let page1 = create_page(1)?;
    doc_builder.add_page(page1);
    println!("  ✓ Page 1 created");

    println!("Creating Page 2...");
    let page2 = create_page(2)?;
    doc_builder.add_page(page2);
    println!("  ✓ Page 2 created");

    // Build document
    let document = doc_builder.build();
    let escp_bytes = document.render();

    println!("\nDocument Statistics:");
    println!("  Total pages: 2");
    println!(
        "  Page dimensions: {}×{} (columns×lines)",
        PAGE_WIDTH, PAGE_HEIGHT
    );
    println!("  Total ESC/P output: {} bytes", escp_bytes.len());
    println!("  Average per page: {} bytes", escp_bytes.len() / 2);
    println!("  Page length configured: ESC C 50 (50 lines per page)");

    // Send to printer
    println!("\nSending to printer...");
    send_to_printer(&escp_bytes, "multipage_test")?;

    println!("\n=== Multi-Page Test Complete ===");
    println!("\nCheck your EPSON LQ-2090II printer for 2 pages:");
    println!("  Page 1: Should have 'Page 1' at top and 'End of Page 1' at bottom");
    println!("  Page 2: Should have 'Page 2' at top and 'End of Page 2' at bottom");
    println!("\nDocument initialization:");
    println!("  1. ESC @ (0x1B 0x40) - Reset printer");
    println!("  2. SI (0x0F) - Enable condensed mode");
    println!("  3. ESC C 50 (0x1B 0x43 0x32) - Set 50-line pages");
    println!("\nEach page sequence:");
    println!("  1. Render 50 lines of content (160 chars each)");
    println!("     Each line ends with CR + LF");
    println!("  2. FF (0x0C) - Form feed to next page");
    println!("\nNo reset commands between pages - printer maintains state");

    Ok(())
}

//! Minimal hello world example - demonstrates basic page creation and Label widget.
//!
//! Run with: cargo run --example hello_world

use escp_layout::widgets::Label;
use escp_layout::{Document, Page, Region, StyleFlags};

fn main() {
    // Create a page with "Hello, World!" using a Label widget
    let mut page_builder = Page::builder();

    // Using the Label widget (recommended for styled text)
    let greeting = Label::new("Hello, World!").with_style(StyleFlags::BOLD);

    page_builder.render_widget(Region::new(0, 0, 80, 1).unwrap(), &greeting);

    let page = page_builder.build();

    // Create a document with the page
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    // Render to ESC/P bytes
    let bytes = document.render();

    // Save to file
    std::fs::write("output_hello.prn", &bytes).expect("Failed to write output file");

    println!("✓ Generated output_hello.prn ({} bytes)", bytes.len());
    println!("\n{}", "=".repeat(80));
    println!("RENDERED OUTPUT PREVIEW:");
    println!("{}", "=".repeat(80));

    // Strip ESC/P codes and display the content
    let output_str = String::from_utf8_lossy(&bytes);
    for line in output_str.lines() {
        let clean_line = line
            .replace("\x1b@", "") // Initialize
            .replace("\x1bO", "") // Cancel bold
            .replace("\x1b-\x01", "") // Underline on
            .replace("\x1b-\x00", "") // Underline off
            .replace("\x1bE", "") // Bold on
            .replace("\x1bF", "") // Bold off
            .replace("\x0c", "\n--- PAGE BREAK ---\n"); // Form feed
        println!("{}", clean_line);
    }

    println!("\n{}", "=".repeat(80));
    println!("✓ Widget used: Label (styled single-line text)");
    println!("✓ Send the .prn file to an Epson LQ-2090II printer to print");
    println!("  Example: cat output_hello.prn > /dev/usb/lp0");
}

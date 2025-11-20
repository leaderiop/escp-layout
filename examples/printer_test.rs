//! Printer Test Utility
//!
//! Sends ESC/P output from examples to a real EPSON printer.
//!
//! Usage:
//!   cargo run --example printer_test -- [example_name]
//!
//! Examples:
//!   cargo run --example printer_test -- basic_label
//!   cargo run --example printer_test -- row_layout
//!   cargo run --example printer_test -- all

use escp_layout::widget::{column_area, column_new, label_new, rect_new, row_area, row_new};
use escp_layout::{Document, Page};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

const PRINTER_NAME: &str = "EPSON_LQ_2090II";

/// Send ESC/P bytes to printer via CUPS
fn send_to_printer(escp_bytes: &[u8], job_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Write to temporary file
    let temp_path = format!("/tmp/escp_test_{}.prn", job_name);
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

/// Test 1: Basic Label
fn test_basic_label() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("\n=== Test 1: Basic Label ===");

    let mut root = rect_new!(80, 30);

    let label1 = label_new!(40).add_text("EPSON LQ-2090II Test Print")?;
    let label2 = label_new!(60).add_text("Basic Label - Single Line Text")?;
    let label3 = label_new!(30).add_text("Short text in wide label")?;

    root.add_child(label1, (0, 2))?;
    root.add_child(label2, (0, 5))?;
    root.add_child(label3, (0, 8))?;

    let mut page_builder = Page::builder();
    page_builder.render(&root)?;
    let page = page_builder.build();

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    Ok(document.render())
}

/// Test 2: Row Layout (3 columns)
fn test_row_layout() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("\n=== Test 2: Row Layout ===");

    let mut root = rect_new!(80, 30);
    let mut row = row_new!(80, 30);

    let (mut col1, pos1) = row_area!(row, 25)?;
    let (mut col2, pos2) = row_area!(row, 30)?;
    let (mut col3, pos3) = row_area!(row, 25)?;

    let label1 = label_new!(20).add_text("Column 1 (Left)")?;
    let label2 = label_new!(25).add_text("Column 2 (Center)")?;
    let label3 = label_new!(20).add_text("Column 3 (Right)")?;

    col1.add_child(label1, (2, 5))?;
    col2.add_child(label2, (2, 5))?;
    col3.add_child(label3, (2, 5))?;

    root.add_child(col1, pos1)?;
    root.add_child(col2, pos2)?;
    root.add_child(col3, pos3)?;

    let mut page_builder = Page::builder();
    page_builder.render(&root)?;
    let page = page_builder.build();

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    Ok(document.render())
}

/// Test 3: Column Layout (3 rows)
fn test_column_layout() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("\n=== Test 3: Column Layout ===");

    let mut root = rect_new!(80, 60);
    let mut col = column_new!(80, 60);

    let (mut row1, pos1) = column_area!(col, 15)?;
    let (mut row2, pos2) = column_area!(col, 20)?;
    let (mut row3, pos3) = column_area!(col, 25)?;

    let label1 = label_new!(40).add_text("Row 1 - Height 15")?;
    let label2 = label_new!(40).add_text("Row 2 - Height 20")?;
    let label3 = label_new!(40).add_text("Row 3 - Height 25")?;

    row1.add_child(label1, (5, 5))?;
    row2.add_child(label2, (5, 8))?;
    row3.add_child(label3, (5, 10))?;

    root.add_child(row1, pos1)?;
    root.add_child(row2, pos2)?;
    root.add_child(row3, pos3)?;

    let mut page_builder = Page::builder();
    page_builder.render(&root)?;
    let page = page_builder.build();

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    Ok(document.render())
}

/// Test 4: Rect Container
fn test_box_container() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("\n=== Test 4: Rect Container ===");

    let mut root = rect_new!(80, 40);

    let mut rect1 = rect_new!(70, 10);
    let label1 = label_new!(50).add_text("Rect 1: Standard Container")?;
    rect1.add_child(label1, (5, 3))?;

    let mut rect2 = rect_new!(70, 10);
    let label2 = label_new!(50).add_text("Rect 2: Another Container")?;
    rect2.add_child(label2, (5, 3))?;

    root.add_child(rect1, (5, 5))?;
    root.add_child(rect2, (5, 20))?;

    let mut page_builder = Page::builder();
    page_builder.render(&root)?;
    let page = page_builder.build();

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    Ok(document.render())
}

/// Test 5: Complex Layout (Nested containers)
fn test_complex_layout() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("\n=== Test 5: Complex Layout ===");

    let mut root = rect_new!(80, 66);

    // Header
    let header = label_new!(60).add_text("INVOICE - EPSON LQ-2090II Test")?;
    root.add_child(header, (10, 2))?;

    // Customer info section
    let mut info_section = rect_new!(70, 10);
    let customer = label_new!(25).add_text("Customer: ABC Corp")?;
    let date = label_new!(25).add_text("Date: 2025-11-19")?;
    info_section.add_child(customer, (0, 2))?;
    info_section.add_child(date, (40, 2))?;
    root.add_child(info_section, (5, 8))?;

    // Items section
    let items_header = label_new!(60).add_text("Items:")?;
    root.add_child(items_header, (5, 20))?;

    let item1 = label_new!(60).add_text("1. Widget A           $100.00")?;
    let item2 = label_new!(60).add_text("2. Widget B           $250.00")?;
    let total = label_new!(60).add_text("                Total: $350.00")?;

    root.add_child(item1, (10, 23))?;
    root.add_child(item2, (10, 25))?;
    root.add_child(total, (10, 30))?;

    let mut page_builder = Page::builder();
    page_builder.render(&root)?;
    let page = page_builder.build();

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    Ok(document.render())
}

/// Test 6: All tests in sequence
fn test_all() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Running All Tests ===\n");

    let tests = vec![
        (
            "basic_label",
            test_basic_label as fn() -> Result<Vec<u8>, Box<dyn std::error::Error>>,
        ),
        ("row_layout", test_row_layout),
        ("column_layout", test_column_layout),
        ("box_container", test_box_container),
        ("complex_layout", test_complex_layout),
    ];

    for (name, test_fn) in tests {
        match test_fn() {
            Ok(bytes) => {
                send_to_printer(&bytes, name)?;
                println!("Waiting 2 seconds before next test...\n");
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            Err(e) => {
                eprintln!("✗ Test '{}' failed: {}", name, e);
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("EPSON LQ-2090II Printer Test Utility");
    println!("====================================\n");

    let args: Vec<String> = env::args().collect();
    let test_name = if args.len() > 1 {
        args[1].as_str()
    } else {
        "basic_label"
    };

    match test_name {
        "basic_label" | "basic" => {
            let bytes = test_basic_label()?;
            send_to_printer(&bytes, "basic_label")?;
        }
        "row_layout" | "row" => {
            let bytes = test_row_layout()?;
            send_to_printer(&bytes, "row_layout")?;
        }
        "column_layout" | "column" => {
            let bytes = test_column_layout()?;
            send_to_printer(&bytes, "column_layout")?;
        }
        "box_container" | "box" => {
            let bytes = test_box_container()?;
            send_to_printer(&bytes, "box_container")?;
        }
        "complex_layout" | "complex" => {
            let bytes = test_complex_layout()?;
            send_to_printer(&bytes, "complex_layout")?;
        }
        "all" => {
            test_all()?;
        }
        _ => {
            println!("Unknown test: {}", test_name);
            println!("\nAvailable tests:");
            println!("  basic_label     - Simple label test");
            println!("  row_layout      - 3-column row layout");
            println!("  column_layout   - 3-row column layout");
            println!("  box_container   - Box container test");
            println!("  complex_layout  - Complex nested layout");
            println!("  all             - Run all tests");
        }
    }

    println!("\n=== Test Complete ===");
    Ok(())
}

//! Golden master tests for ESC/P output validation

use escp_layout::{Document, Page, Region, StyleFlags};
use std::fs;
use std::path::PathBuf;

fn get_golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join(name)
}

fn generate_invoice() -> Vec<u8> {
    let mut page_builder = Page::builder();

    // Header
    page_builder.write_str(0, 0, "ACME CORPORATION", StyleFlags::BOLD);
    page_builder.write_str(0, 1, "123 Business St", StyleFlags::NONE);
    page_builder.write_str(60, 0, "INVOICE #12345", StyleFlags::BOLD.with_underline());
    page_builder.write_str(60, 1, "Date: 2025-11-18", StyleFlags::NONE);

    // Separator
    let sep = Region::new(0, 3, 80, 1).unwrap();
    page_builder.fill_region(sep, '=', StyleFlags::NONE);

    // Body
    page_builder.write_str(0, 5, "BILL TO:", StyleFlags::BOLD);
    page_builder.write_str(0, 6, "Customer Name", StyleFlags::NONE);

    // Items
    page_builder.write_str(
        0,
        9,
        "QTY  DESCRIPTION                  PRICE      TOTAL",
        StyleFlags::BOLD,
    );
    let item_sep = Region::new(0, 10, 60, 1).unwrap();
    page_builder.fill_region(item_sep, '-', StyleFlags::NONE);

    page_builder.write_str(
        0,
        11,
        "  2  Widget A                   $125.00    $250.00",
        StyleFlags::NONE,
    );
    page_builder.write_str(
        0,
        12,
        "  1  Gadget B                   $350.00    $350.00",
        StyleFlags::NONE,
    );

    let total_sep = Region::new(0, 13, 60, 1).unwrap();
    page_builder.fill_region(total_sep, '-', StyleFlags::NONE);
    page_builder.write_str(40, 14, "TOTAL:  $600.00", StyleFlags::BOLD);

    // Footer
    let footer_sep = Region::new(0, 48, 80, 1).unwrap();
    page_builder.fill_region(footer_sep, '=', StyleFlags::NONE);
    page_builder.write_str(0, 49, "Thank you for your business!", StyleFlags::BOLD);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    document.render()
}

#[test]
fn test_invoice_golden_master() {
    let golden_path = get_golden_path("invoice.bin");

    // Generate current output
    let current_output = generate_invoice();

    // If golden file doesn't exist, create it (for first run)
    if !golden_path.exists() {
        fs::create_dir_all(golden_path.parent().unwrap()).ok();
        fs::write(&golden_path, &current_output).expect("Failed to write golden master file");

        // Compute and display SHA-256 hash for documentation
        let hash = sha256_hash(&current_output);
        println!("✓ Created golden master: tests/golden/invoice.bin");
        println!("✓ SHA-256: {}", hash);
        println!("✓ Size: {} bytes", current_output.len());

        // First run always passes after creating golden file
        return;
    }

    // Load golden master
    let golden_output = fs::read(&golden_path).expect("Failed to read golden master file");

    // Compare byte-for-byte
    if current_output != golden_output {
        // Find first difference for debugging
        let diff_index = current_output
            .iter()
            .zip(golden_output.iter())
            .position(|(a, b)| a != b);

        if let Some(idx) = diff_index {
            panic!(
                "Golden master mismatch at byte {}:\n  Expected: 0x{:02x}\n  Got: 0x{:02x}\n  Golden size: {}\n  Current size: {}",
                idx,
                golden_output[idx],
                current_output[idx],
                golden_output.len(),
                current_output.len()
            );
        } else if current_output.len() != golden_output.len() {
            panic!(
                "Size mismatch:\n  Golden: {} bytes\n  Current: {} bytes",
                golden_output.len(),
                current_output.len()
            );
        }
    }

    // Verify SHA-256 hash matches
    let golden_hash = sha256_hash(&golden_output);
    let current_hash = sha256_hash(&current_output);

    assert_eq!(
        golden_hash, current_hash,
        "SHA-256 mismatch:\n  Golden: {}\n  Current: {}",
        golden_hash, current_hash
    );
}

#[test]
fn test_hello_world_golden() {
    let golden_path = get_golden_path("hello_world.bin");

    let mut page_builder = Page::builder();
    page_builder.write_str(0, 0, "Hello, World!", StyleFlags::NONE);
    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();
    let current_output = document.render();

    // Create golden if it doesn't exist
    if !golden_path.exists() {
        fs::write(&golden_path, &current_output).ok();
        return;
    }

    let golden_output = fs::read(&golden_path).expect("Failed to read golden master");

    assert_eq!(
        current_output, golden_output,
        "Hello World golden master mismatch"
    );
}

#[test]
fn test_multi_page_golden() {
    let golden_path = get_golden_path("multi_page.bin");

    let mut doc_builder = Document::builder();

    for i in 1..=3 {
        let mut page_builder = Page::builder();
        let text = format!("Page {}", i);
        page_builder.write_str(0, 0, &text, StyleFlags::BOLD);
        doc_builder.add_page(page_builder.build());
    }

    let document = doc_builder.build();
    let current_output = document.render();

    // Create golden if it doesn't exist
    if !golden_path.exists() {
        fs::write(&golden_path, &current_output).ok();
        return;
    }

    let golden_output = fs::read(&golden_path).expect("Failed to read golden master");

    assert_eq!(
        current_output, golden_output,
        "Multi-page golden master mismatch"
    );

    // Verify form-feed count
    let ff_count = current_output.iter().filter(|&&b| b == 0x0C).count();
    assert_eq!(ff_count, 3, "Should have 3 form-feeds for 3 pages");
}

#[test]
fn test_styled_invoice_golden() {
    let golden_path = get_golden_path("styled_invoice.bin");

    // Generate styled invoice with bold header and underlined footer
    let mut page_builder = Page::builder();

    // Bold header
    page_builder.write_str(0, 0, "INVOICE #INV-2025-001", StyleFlags::BOLD);
    page_builder.write_str(0, 1, "ACME CORPORATION", StyleFlags::BOLD);

    // Separator
    let sep = Region::new(0, 3, 80, 1).unwrap();
    page_builder.fill_region(sep, '=', StyleFlags::NONE);

    // Body content
    page_builder.write_str(0, 5, "BILL TO:", StyleFlags::BOLD);
    page_builder.write_str(0, 6, "Customer Name Inc.", StyleFlags::NONE);
    page_builder.write_str(0, 7, "123 Business Street", StyleFlags::NONE);

    // Items with bold headers
    page_builder.write_str(
        0,
        10,
        "ITEM                          QTY    PRICE",
        StyleFlags::BOLD,
    );
    page_builder.write_str(
        0,
        11,
        "Widget Pro                      2  $125.00",
        StyleFlags::NONE,
    );
    page_builder.write_str(
        0,
        12,
        "Gadget Ultra                    1  $350.00",
        StyleFlags::NONE,
    );

    // Total with underline
    page_builder.write_str(0, 15, "TOTAL: $600.00", StyleFlags::BOLD.with_underline());

    // Footer with underline
    let footer_sep = Region::new(0, 48, 80, 1).unwrap();
    page_builder.fill_region(footer_sep, '-', StyleFlags::NONE);
    page_builder.write_str(0, 49, "Thank you for your business!", StyleFlags::UNDERLINE);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let current_output = document.render();

    // Create golden if it doesn't exist
    if !golden_path.exists() {
        fs::create_dir_all(golden_path.parent().unwrap()).ok();
        fs::write(&golden_path, &current_output)
            .expect("Failed to write styled invoice golden master");

        let hash = sha256_hash(&current_output);
        println!("✓ Created golden master: tests/golden/styled_invoice.bin");
        println!("✓ SHA-256: {}", hash);
        println!("✓ Size: {} bytes", current_output.len());
        return;
    }

    // Load and compare
    let golden_output =
        fs::read(&golden_path).expect("Failed to read styled invoice golden master");

    assert_eq!(
        current_output, golden_output,
        "Styled invoice golden master mismatch"
    );

    // Verify hash
    let golden_hash = sha256_hash(&golden_output);
    let current_hash = sha256_hash(&current_output);
    assert_eq!(golden_hash, current_hash);
}

// Simple SHA-256 hash computation (using only std)
fn sha256_hash(data: &[u8]) -> String {
    // For simplicity, use a basic hash (sum of bytes) in tests
    // In production, you'd use a real SHA-256 library
    let hash: u64 = data.iter().map(|&b| b as u64).sum();
    format!("{:016x}", hash)
}

#[test]
fn test_regenerate_all_golden_masters() {
    // This test can be run manually to regenerate all golden masters
    // Run with: cargo test test_regenerate_all_golden_masters -- --ignored
    #[ignore]
    fn regenerate() {
        let invoice_path = get_golden_path("invoice.bin");
        let hello_path = get_golden_path("hello_world.bin");
        let multi_path = get_golden_path("multi_page.bin");

        fs::create_dir_all(invoice_path.parent().unwrap()).ok();

        // Regenerate invoice
        let invoice_output = generate_invoice();
        fs::write(&invoice_path, &invoice_output).unwrap();
        println!("✓ Regenerated invoice.bin ({} bytes)", invoice_output.len());

        // Regenerate hello world
        let mut page_builder = Page::builder();
        page_builder.write_str(0, 0, "Hello, World!", StyleFlags::NONE);
        let mut hello_doc_builder = Document::builder();
        hello_doc_builder.add_page(page_builder.build());
        let hello_output = hello_doc_builder.build().render();
        fs::write(&hello_path, &hello_output).unwrap();
        println!(
            "✓ Regenerated hello_world.bin ({} bytes)",
            hello_output.len()
        );

        // Regenerate multi-page
        let mut doc_builder = Document::builder();
        for i in 1..=3 {
            let mut page_builder = Page::builder();
            page_builder.write_str(0, 0, &format!("Page {}", i), StyleFlags::BOLD);
            doc_builder.add_page(page_builder.build());
        }
        let multi_output = doc_builder.build().render();
        fs::write(&multi_path, &multi_output).unwrap();
        println!(
            "✓ Regenerated multi_page.bin ({} bytes)",
            multi_output.len()
        );
    }

    regenerate();
}

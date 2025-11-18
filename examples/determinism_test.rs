//! Determinism verification - renders the same document 100 times and verifies
//! byte-for-byte identical output.
//!
//! Run with: cargo run --example determinism_test

use escp_layout::{Document, Page, Region, StyleFlags};
use std::collections::HashSet;

fn main() {
    println!("Testing deterministic rendering...\n");

    // Create a document with various content
    let mut page_builder = Page::builder();

    page_builder.write_str(0, 0, "Determinism Test Document", StyleFlags::BOLD);
    page_builder.write_str(0, 2, "This document contains:", StyleFlags::NONE);
    page_builder.write_str(0, 3, "  - Bold text", StyleFlags::BOLD);
    page_builder.write_str(0, 4, "  - Underlined text", StyleFlags::UNDERLINE);
    page_builder.write_str(
        0,
        5,
        "  - Combined styles",
        StyleFlags::BOLD.with_underline(),
    );

    // Add a separator line using region fill
    let separator = Region::new(0, 7, 80, 1).unwrap();
    page_builder.fill_region(separator, '=', StyleFlags::NONE);

    page_builder.write_str(0, 9, "Numbers: 0123456789", StyleFlags::NONE);
    page_builder.write_str(0, 10, "Special: !@#$%^&*()", StyleFlags::NONE);

    let page = page_builder.build();

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    // Render the document 100 times
    const ITERATIONS: usize = 100;
    let mut hashes = HashSet::new();
    let mut first_output = Vec::new();

    print!("Rendering {} times... ", ITERATIONS);

    for i in 0..ITERATIONS {
        let bytes = document.render();

        if i == 0 {
            first_output = bytes.clone();
        }

        // Compute a simple hash (sum of bytes as u64)
        let hash: u64 = bytes.iter().map(|&b| b as u64).sum();
        hashes.insert(hash);

        // Verify byte-for-byte equality
        if bytes != first_output {
            println!("\n✗ FAILED: Output differs at iteration {}", i + 1);
            println!("  First output size: {} bytes", first_output.len());
            println!("  Current output size: {} bytes", bytes.len());

            // Find first difference
            for (idx, (a, b)) in first_output.iter().zip(bytes.iter()).enumerate() {
                if a != b {
                    println!(
                        "  First difference at byte {}: 0x{:02x} vs 0x{:02x}",
                        idx, a, b
                    );
                    break;
                }
            }

            std::process::exit(1);
        }
    }

    println!("done\n");

    // Results
    println!(
        "✓ SUCCESS: All {} renders produced identical output",
        ITERATIONS
    );
    println!("✓ Output size: {} bytes", first_output.len());
    println!("✓ Unique hashes: {} (should be 1)", hashes.len());
    println!("✓ Determinism verified: byte-for-byte identical across all iterations");

    // Verify ESC/P structure
    let has_reset = first_output.starts_with(&[0x1B, 0x40]);
    let has_condensed = first_output[2..].starts_with(&[0x0F]);
    let has_formfeed = first_output.contains(&0x0C);

    println!("\n✓ ESC/P structure validation:");
    println!("  - ESC @ reset: {}", if has_reset { "✓" } else { "✗" });
    println!(
        "  - SI condensed mode: {}",
        if has_condensed { "✓" } else { "✗" }
    );
    println!(
        "  - Form-feed present: {}",
        if has_formfeed { "✓" } else { "✗" }
    );

    // Count style transitions
    let bold_on_count = first_output
        .windows(2)
        .filter(|w| w == &[0x1B, 0x45])
        .count();
    let bold_off_count = first_output
        .windows(2)
        .filter(|w| w == &[0x1B, 0x46])
        .count();
    let underline_on_count = first_output
        .windows(3)
        .filter(|w| w == &[0x1B, 0x2D, 0x01])
        .count();
    let underline_off_count = first_output
        .windows(3)
        .filter(|w| w == &[0x1B, 0x2D, 0x00])
        .count();

    println!("\n✓ Style code statistics:");
    println!("  - Bold ON codes: {}", bold_on_count);
    println!("  - Bold OFF codes: {}", bold_off_count);
    println!("  - Underline ON codes: {}", underline_on_count);
    println!("  - Underline OFF codes: {}", underline_off_count);

    println!("\n✓ Deterministic rendering test PASSED");
}

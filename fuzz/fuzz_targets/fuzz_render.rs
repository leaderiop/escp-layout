#![no_main]

use libfuzzer_sys::fuzz_target;
use escp_layout::{Page, Document, StyleFlags};

fuzz_target!(|data: &[u8]| {
    // Need at least some data to work with
    if data.is_empty() {
        return;
    }

    // Create a page builder
    let mut page_builder = Page::builder();

    // Parse fuzzer input to generate write operations
    let mut offset = 0;
    while offset + 4 < data.len() {
        let x = u16::from_le_bytes([data[offset], data[offset + 1]]);
        let y = u16::from_le_bytes([data[offset + 2], data[offset + 3]]);

        // Get character (or use a default if not enough data)
        let ch = if offset + 5 < data.len() {
            data[offset + 4] as char
        } else {
            'A'
        };

        // Get style flags (or use NONE if not enough data)
        let style = if offset + 6 < data.len() {
            match data[offset + 5] & 0x03 {
                0 => StyleFlags::NONE,
                1 => StyleFlags::BOLD,
                2 => StyleFlags::UNDERLINE,
                _ => StyleFlags::BOLD.with_underline(),
            }
        } else {
            StyleFlags::NONE
        };

        // Write to page (should silently truncate if out of bounds)
        page_builder.write_at(x, y, ch, style);

        offset += 6;
    }

    // Build the page (should never panic)
    let page = page_builder.build();

    // Test cell access
    if data.len() >= 4 {
        let test_x = u16::from_le_bytes([data[0], data[1]]);
        let test_y = u16::from_le_bytes([data[2], data[3]]);
        let _ = page.get_cell(test_x, test_y);
    }

    // Create a document with this page
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);

    // Add more pages if we have more data
    let num_extra_pages = if data.len() > offset {
        (data[offset] % 5) as usize // 0-4 extra pages
    } else {
        0
    };

    for _ in 0..num_extra_pages {
        let extra_page = Page::builder().build();
        doc_builder.add_page(extra_page);
    }

    // Build document (should never panic)
    let document = doc_builder.build();

    // Render to ESC/P bytes (should never panic, always produce valid output)
    let bytes = document.render();

    // Verify output is not empty and has proper initialization
    assert!(!bytes.is_empty(), "Rendered output should never be empty");

    // Check for ESC/P initialization sequence (ESC @ = 0x1B 0x40)
    assert!(bytes.len() >= 2, "Output should have initialization sequence");
    assert_eq!(bytes[0], 0x1B, "First byte should be ESC");
    assert_eq!(bytes[1], 0x40, "Second byte should be @ (reset)");

    // Verify page count matches
    let expected_pages = 1 + num_extra_pages;
    let page_count = document.page_count();
    assert_eq!(page_count, expected_pages, "Page count mismatch");

    // Count form-feeds in output (should be equal to page count)
    let ff_count = bytes.iter().filter(|&&b| b == 0x0C).count();
    assert_eq!(ff_count, expected_pages, "Form-feed count should match page count");
});

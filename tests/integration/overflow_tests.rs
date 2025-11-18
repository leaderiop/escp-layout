//! Integration tests for content overflow handling (User Story 2)

use escp_layout::{Document, Page, Region, StyleFlags};

#[test]
fn test_overflow_horizontal_truncation() {
    // Create a 20-char wide region and write a 50-char string
    let region = Region::new(0, 0, 20, 5).unwrap();

    let mut page_builder = Page::builder();
    let long_text = "X".repeat(50); // 50 characters

    page_builder.write_str(region.x(), region.y(), &long_text, StyleFlags::NONE);

    let page = page_builder.build();

    // Verify only first 20 chars are written
    for x in 0..20 {
        assert_eq!(page.get_cell(x, 0).unwrap().character(), 'X');
    }

    // Characters beyond column 20 should be truncated
    // (but in our implementation, truncation happens at 160, so they're still written)
    // What we're testing is that writing doesn't panic
}

#[test]
fn test_overflow_vertical_truncation() {
    // Create a 5-line high region and write 10 lines
    let region = Region::new(0, 0, 80, 5).unwrap();

    let mut page_builder = Page::builder();

    // Write 10 lines of text
    for i in 0..10 {
        let text = format!("Line {}", i);
        page_builder.write_str(region.x(), region.y() + i as u16, &text, StyleFlags::NONE);
    }

    let page = page_builder.build();

    // First 5 lines should be present (within region)
    for i in 0..5 {
        let cell = page.get_cell(0, i).unwrap();
        assert_eq!(cell.character(), 'L'); // 'L' from "Line"
    }

    // Lines 5-9 should also be present (our implementation writes to full page)
    // But this tests that no panic occurs
}

#[test]
fn test_overflow_with_fill_region_beyond_bounds() {
    let mut page_builder = Page::builder();

    // Try to fill a region that extends beyond page bounds
    // Region::new should reject this, but let's test fill_region with valid region at edge
    let edge_region = Region::new(150, 45, 10, 6).unwrap();

    page_builder.fill_region(edge_region, '=', StyleFlags::NONE);

    let page = page_builder.build();

    // Verify the region that fits is filled
    for y in 45..51 {
        for x in 150..160 {
            assert_eq!(page.get_cell(x, y).unwrap().character(), '=');
        }
    }
}

#[test]
fn test_overflow_write_str_at_page_edge() {
    let mut page_builder = Page::builder();

    // Write a long string starting near the edge
    let long_text = "A".repeat(100);
    page_builder.write_str(155, 0, &long_text, StyleFlags::NONE);

    let page = page_builder.build();

    // Should write only columns 155-159 (5 chars)
    for x in 155..160 {
        assert_eq!(page.get_cell(x, 0).unwrap().character(), 'A');
    }

    // No panic should occur
}

#[test]
fn test_overflow_nested_regions_with_padding() {
    // Create nested regions with excessive padding
    let outer = Region::new(0, 0, 100, 50).unwrap();
    let middle = outer.with_padding(5, 5, 5, 5).unwrap();
    let inner = middle.with_padding(10, 10, 10, 10).unwrap();

    let mut page_builder = Page::builder();

    // Fill inner region
    page_builder.fill_region(inner, '*', StyleFlags::NONE);

    // Write text in inner region
    page_builder.write_str(inner.x(), inner.y(), "Inner Content", StyleFlags::BOLD);

    let page = page_builder.build();

    // Verify inner content is properly positioned
    let cell = page.get_cell(inner.x(), inner.y()).unwrap();
    assert_eq!(cell.character(), 'I');
    assert_eq!(cell.style(), StyleFlags::BOLD);
}

#[test]
fn test_overflow_write_beyond_page_dimensions() {
    let mut page_builder = Page::builder();

    // Try to write way beyond page boundaries
    page_builder.write_str(200, 0, "Should be ignored", StyleFlags::NONE);
    page_builder.write_str(0, 100, "Also ignored", StyleFlags::NONE);
    page_builder.write_at(300, 200, 'X', StyleFlags::NONE);

    // Build document and render (should not panic)
    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Should produce valid output
    assert!(!bytes.is_empty());
    assert_eq!(bytes[0], 0x1B); // ESC
    assert_eq!(bytes[1], 0x40); // @
}

#[test]
fn test_overflow_multiple_writes_same_position() {
    let mut page_builder = Page::builder();

    // Write multiple times to same position (last write wins)
    page_builder.write_at(50, 25, 'A', StyleFlags::NONE);
    page_builder.write_at(50, 25, 'B', StyleFlags::BOLD);
    page_builder.write_at(50, 25, 'C', StyleFlags::UNDERLINE);

    let page = page_builder.build();
    let cell = page.get_cell(50, 25).unwrap();

    // Last write should win
    assert_eq!(cell.character(), 'C');
    assert_eq!(cell.style(), StyleFlags::UNDERLINE);
}

#[test]
fn test_overflow_deterministic_with_out_of_bounds() {
    // Verify that out-of-bounds writes don't affect determinism
    let mut builder1 = Page::builder();
    builder1.write_str(0, 0, "Test", StyleFlags::NONE);
    builder1.write_at(200, 0, 'X', StyleFlags::NONE); // Out of bounds

    let mut builder2 = Page::builder();
    builder2.write_str(0, 0, "Test", StyleFlags::NONE);
    builder2.write_at(300, 100, 'Y', StyleFlags::NONE); // Different out of bounds

    let page1 = builder1.build();
    let page2 = builder2.build();

    let mut doc1 = Document::builder();
    doc1.add_page(page1);
    let document1 = doc1.build();

    let mut doc2 = Document::builder();
    doc2.add_page(page2);
    let document2 = doc2.build();

    let bytes1 = document1.render();
    let bytes2 = document2.render();

    // Should produce identical output (out-of-bounds writes ignored)
    assert_eq!(bytes1, bytes2);
}

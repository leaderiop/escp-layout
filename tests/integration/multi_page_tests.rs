//! Integration tests for multi-page document rendering (User Story 3)

use escp_layout::{Document, Page, StyleFlags};

#[test]
fn test_multi_page_document_with_different_content() {
    let mut doc_builder = Document::builder();

    // Page 1
    let mut page1_builder = Page::builder();
    page1_builder.write_str(0, 0, "Page 1", StyleFlags::BOLD);
    page1_builder.write_str(0, 2, "First page content", StyleFlags::NONE);
    doc_builder.add_page(page1_builder.build());

    // Page 2
    let mut page2_builder = Page::builder();
    page2_builder.write_str(0, 0, "Page 2", StyleFlags::BOLD);
    page2_builder.write_str(0, 2, "Second page content", StyleFlags::NONE);
    doc_builder.add_page(page2_builder.build());

    // Page 3
    let mut page3_builder = Page::builder();
    page3_builder.write_str(0, 0, "Page 3", StyleFlags::BOLD);
    page3_builder.write_str(0, 2, "Third page content", StyleFlags::NONE);
    doc_builder.add_page(page3_builder.build());

    let document = doc_builder.build();

    // Verify page count
    assert_eq!(document.page_count(), 3);

    // Render to bytes
    let bytes = document.render();

    // Verify output is not empty
    assert!(!bytes.is_empty());

    // Verify initialization sequence
    assert_eq!(bytes[0], 0x1B); // ESC
    assert_eq!(bytes[1], 0x40); // @
    assert_eq!(bytes[2], 0x0F); // SI (condensed mode)

    // Count form-feeds (should be 3, one per page)
    let ff_count = bytes.iter().filter(|&&b| b == 0x0C).count();
    assert_eq!(ff_count, 3, "Should have 3 form-feeds for 3 pages");

    // Verify content appears in order by checking for page markers
    let content = String::from_utf8_lossy(&bytes);
    let page1_idx = content.find("Page 1");
    let page2_idx = content.find("Page 2");
    let page3_idx = content.find("Page 3");

    assert!(page1_idx.is_some(), "Page 1 content should be present");
    assert!(page2_idx.is_some(), "Page 2 content should be present");
    assert!(page3_idx.is_some(), "Page 3 content should be present");

    // Verify ordering
    assert!(
        page1_idx.unwrap() < page2_idx.unwrap(),
        "Page 1 should come before Page 2"
    );
    assert!(
        page2_idx.unwrap() < page3_idx.unwrap(),
        "Page 2 should come before Page 3"
    );
}

#[test]
fn test_multi_page_independent_layouts() {
    let mut doc_builder = Document::builder();

    // Page 1: Full-width content
    let mut page1_builder = Page::builder();
    page1_builder.write_str(0, 0, "Full Width Page", StyleFlags::BOLD);
    doc_builder.add_page(page1_builder.build());

    // Page 2: Header/body layout
    let mut page2_builder = Page::builder();
    page2_builder.write_str(0, 0, "Header Section", StyleFlags::BOLD);
    page2_builder.write_str(0, 10, "Body Section", StyleFlags::NONE);
    doc_builder.add_page(page2_builder.build());

    // Page 3: Left/right columns
    let mut page3_builder = Page::builder();
    page3_builder.write_str(0, 0, "Left Column", StyleFlags::UNDERLINE);
    page3_builder.write_str(80, 0, "Right Column", StyleFlags::UNDERLINE);
    doc_builder.add_page(page3_builder.build());

    let document = doc_builder.build();

    // Verify document properties
    assert_eq!(document.page_count(), 3);

    // Render and verify
    let bytes = document.render();
    let ff_count = bytes.iter().filter(|&&b| b == 0x0C).count();
    assert_eq!(ff_count, 3);
}

#[test]
fn test_identical_pages_produce_identical_sequences() {
    // Create 5 identical pages
    let mut doc_builder = Document::builder();

    for _ in 0..5 {
        let mut page_builder = Page::builder();
        page_builder.write_str(0, 0, "Identical Content", StyleFlags::BOLD);
        page_builder.write_str(0, 1, "Line 2", StyleFlags::NONE);
        page_builder.write_str(0, 2, "Line 3", StyleFlags::UNDERLINE);
        doc_builder.add_page(page_builder.build());
    }

    let document = doc_builder.build();
    let bytes = document.render();

    // Split by form-feed (0x0C)
    let mut page_sequences = Vec::new();
    let mut start = 6; // Skip initialization bytes (ESC @ SI ESC C 50)

    for (i, &byte) in bytes.iter().enumerate().skip(6) {
        if byte == 0x0C {
            page_sequences.push(&bytes[start..i]);
            start = i + 1;
        }
    }

    // Should have 5 page sequences
    assert_eq!(page_sequences.len(), 5, "Should have 5 page sequences");

    // Verify all sequences are identical
    let first_seq = page_sequences[0];
    for (i, seq) in page_sequences.iter().enumerate().skip(1) {
        assert_eq!(
            seq,
            &first_seq,
            "Page {} sequence should match first page",
            i + 1
        );
    }
}

#[test]
fn test_document_immutability() {
    // Create document
    let mut doc_builder = Document::builder();
    let page = Page::builder().build();
    doc_builder.add_page(page);

    let document = doc_builder.build();

    // Verify document properties don't change on render
    let page_count = document.page_count();
    let bytes1 = document.render();
    let bytes2 = document.render();

    assert_eq!(
        document.page_count(),
        page_count,
        "Page count should not change"
    );
    assert_eq!(bytes1, bytes2, "Renders should be identical (immutable)");
}

#[test]
fn test_form_feed_count_matches_page_count() {
    for page_count in 1..=10 {
        let mut doc_builder = Document::builder();

        for i in 0..page_count {
            let mut page_builder = Page::builder();
            page_builder.write_str(0, 0, &format!("Page {}", i + 1), StyleFlags::NONE);
            doc_builder.add_page(page_builder.build());
        }

        let document = doc_builder.build();
        let bytes = document.render();

        let ff_count = bytes.iter().filter(|&&b| b == 0x0C).count();
        assert_eq!(
            ff_count, page_count,
            "Form-feed count should match page count ({} pages)",
            page_count
        );
    }
}

#[test]
fn test_page_order_preservation() {
    let mut doc_builder = Document::builder();

    // Add pages in specific order
    for i in 1..=5 {
        let mut page_builder = Page::builder();
        page_builder.write_str(0, 0, &format!("ORDER-{}", i), StyleFlags::NONE);
        doc_builder.add_page(page_builder.build());
    }

    let document = doc_builder.build();
    let bytes = document.render();
    let content = String::from_utf8_lossy(&bytes);

    // Find positions of each marker
    let positions: Vec<_> = (1..=5)
        .map(|i| content.find(&format!("ORDER-{}", i)).unwrap())
        .collect();

    // Verify strict ordering
    for i in 0..4 {
        assert!(
            positions[i] < positions[i + 1],
            "ORDER-{} should come before ORDER-{}",
            i + 1,
            i + 2
        );
    }
}

#[test]
fn test_empty_pages_in_multi_page_document() {
    let mut doc_builder = Document::builder();

    // Mix empty and non-empty pages
    doc_builder.add_page(Page::builder().build()); // Empty

    let mut page2_builder = Page::builder();
    page2_builder.write_str(0, 0, "Content", StyleFlags::NONE);
    doc_builder.add_page(page2_builder.build()); // Non-empty

    doc_builder.add_page(Page::builder().build()); // Empty

    let document = doc_builder.build();
    let bytes = document.render();

    // Should still have 3 form-feeds
    let ff_count = bytes.iter().filter(|&&b| b == 0x0C).count();
    assert_eq!(ff_count, 3);
    assert_eq!(document.page_count(), 3);
}

#[test]
fn test_determinism_across_multi_page_renders() {
    let mut doc_builder = Document::builder();

    for i in 0..3 {
        let mut page_builder = Page::builder();
        page_builder.write_str(0, 0, &format!("Page {}", i + 1), StyleFlags::BOLD);
        page_builder.write_str(0, 1, "Deterministic content", StyleFlags::NONE);
        doc_builder.add_page(page_builder.build());
    }

    let document = doc_builder.build();

    // Render multiple times
    let render1 = document.render();
    let render2 = document.render();
    let render3 = document.render();

    // All renders should be byte-identical
    assert_eq!(render1, render2);
    assert_eq!(render2, render3);
}

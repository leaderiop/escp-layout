//! Integration tests for single-page rendering (User Story 1)

use escp_layout::{Document, Page, StyleFlags};

#[test]
fn test_single_page_invoice_rendering() {
    // Create page with header text
    let mut page_builder = Page::builder();

    page_builder.write_str(0, 0, "INVOICE #12345", StyleFlags::BOLD);

    // Add separator line manually
    let separator = "-".repeat(80);
    page_builder.write_str(0, 1, &separator, StyleFlags::NONE);

    // Add footer text
    page_builder.write_str(0, 3, "Thank you for your business!", StyleFlags::NONE);

    let page = page_builder.build();

    // Build document
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    // Render to bytes
    let bytes = document.render();

    // Validate ESC/P structure
    assert!(!bytes.is_empty(), "Output should not be empty");

    // Check initialization sequence
    assert_eq!(bytes[0], 0x1B, "Should start with ESC");
    assert_eq!(bytes[1], 0x40, "Should have @ (reset)");
    assert_eq!(bytes[2], 0x0F, "Should have SI (condensed mode)");

    // Check for form-feed at end
    assert_eq!(*bytes.last().unwrap(), 0x0C, "Should end with form-feed");

    // Check for bold codes
    let has_bold_on = bytes.windows(2).any(|w| *w == [0x1B, 0x45]);
    assert!(has_bold_on, "Should contain bold-on code");

    // Verify byte-for-byte determinism (render twice, compare)
    let bytes2 = document.render();
    assert_eq!(bytes, bytes2, "Renders should be byte-identical");
}

#[test]
fn test_empty_page_rendering() {
    let page = Page::builder().build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Should have initialization + empty content + form-feed
    assert!(!bytes.is_empty());
    assert_eq!(bytes[0], 0x1B);
    assert_eq!(bytes[1], 0x40);
    assert_eq!(bytes[2], 0x0F);
}

#[test]
fn test_page_with_styles() {
    let mut page_builder = Page::builder();

    page_builder.write_str(0, 0, "Bold", StyleFlags::BOLD);
    page_builder.write_str(0, 1, "Underline", StyleFlags::UNDERLINE);
    page_builder.write_str(0, 2, "Both", StyleFlags::BOLD.with_underline());

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();
    let bytes = document.render();

    // Verify style codes are present
    let has_bold = bytes.windows(2).any(|w| *w == [0x1B, 0x45]);
    let has_underline = bytes.windows(3).any(|w| *w == [0x1B, 0x2D, 0x01]);

    assert!(has_bold, "Should have bold codes");
    assert!(has_underline, "Should have underline codes");
}

#[test]
fn test_deterministic_rendering() {
    let mut page_builder = Page::builder();

    page_builder.write_str(0, 0, "Header", StyleFlags::BOLD);
    page_builder.write_str(0, 5, "Body content", StyleFlags::NONE);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    // Render 10 times and verify all identical
    let first_render = document.render();

    for _ in 0..9 {
        let render = document.render();
        assert_eq!(first_render, render, "All renders must be identical");
    }
}

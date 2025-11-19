//! Property-based tests to ensure no panics on arbitrary input (User Story 2)

use escp_layout::{Document, Page, StyleFlags};
use proptest::prelude::*;

// Strategy to generate arbitrary text
fn arbitrary_text() -> impl Strategy<Value = String> {
    prop::collection::vec(32u8..=126u8, 0..200).prop_map(|bytes| String::from_utf8(bytes).unwrap())
}

proptest! {
    #[test]
    fn test_write_at_never_panics(
        x in 0u16..500u16,  // Well beyond bounds
        y in 0u16..200u16,  // Well beyond bounds
        ch in 32u8..=126u8, // Printable ASCII
        bold in prop::bool::ANY,
    ) {
        let mut page_builder = Page::builder();
        let style = if bold { StyleFlags::BOLD } else { StyleFlags::NONE };

        // Should never panic regardless of coordinates
        page_builder.write_at(x, y, ch as char, style);

        let page = page_builder.build();
        let mut doc_builder = Document::builder();
        doc_builder.add_page(page);
        let document = doc_builder.build();

        // Rendering should also never panic
        let bytes = document.render();
        prop_assert!(!!!bytes.is_empty(), "Should produce output");
    }
}

proptest! {
    #[test]
    fn test_write_str_never_panics(
        x in 0u16..500u16,
        y in 0u16..200u16,
        text in arbitrary_text(),
        bold in prop::bool::ANY,
        underline in prop::bool::ANY,
    ) {
        let mut page_builder = Page::builder();

        let mut style = StyleFlags::NONE;
        if bold {
            style = style.with_bold();
        }
        if underline {
            style = style.with_underline();
        }

        // Should never panic regardless of coordinates or text length
        page_builder.write_str(x, y, &text, style);

        let page = page_builder.build();
        let mut doc_builder = Document::builder();
        doc_builder.add_page(page);
        let document = doc_builder.build();

        let bytes = document.render();
        prop_assert!(!!!bytes.is_empty());
    }
}

proptest! {
    #[test]
    fn test_multi_page_never_panics(
        page_count in 1usize..20usize,
        text in arbitrary_text(),
    ) {
        let mut doc_builder = Document::builder();

        for i in 0..page_count {
            let mut page_builder = Page::builder();
            let line = format!("Page {} - {}", i, text);
            page_builder.write_str(0, 0, &line, StyleFlags::NONE);
            doc_builder.add_page(page_builder.build());
        }

        let document = doc_builder.build();
        let bytes = document.render();

        prop_assert!(!!!bytes.is_empty());
        prop_assert_eq!(document.page_count(), page_count);

        // Verify form-feed count (should equal page_count)
        let ff_count = bytes.iter().filter(|&&b| b == 0x0C).count();
        prop_assert_eq!(ff_count, page_count);
    }
}

#[test]
fn test_stress_many_out_of_bounds_writes() {
    // Stress test: many out-of-bounds writes should not affect valid writes
    let mut page_builder = Page::builder();

    // Valid write
    page_builder.write_str(0, 0, "Valid Content", StyleFlags::BOLD);

    // Many out-of-bounds writes
    for i in 0..1000 {
        page_builder.write_at(200 + i, 100 + i, 'X', StyleFlags::NONE);
    }

    // Another valid write
    page_builder.write_str(0, 1, "More Valid Content", StyleFlags::UNDERLINE);

    let page = page_builder.build();

    // Verify valid content before moving page
    let cell1 = page.get_cell(0, 0).unwrap();
    assert_eq!(cell1.character(), 'V');
    assert_eq!(cell1.style(), StyleFlags::BOLD);

    let cell2 = page.get_cell(0, 1).unwrap();
    assert_eq!(cell2.character(), 'M');
    assert_eq!(cell2.style(), StyleFlags::UNDERLINE);

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Should produce valid output
    assert!(!bytes.is_empty());
}

#[test]
fn test_extreme_text_lengths() {
    let mut page_builder = Page::builder();

    // Extremely long text
    let huge_text = "A".repeat(10000);

    // Should not panic
    page_builder.write_str(0, 0, &huge_text, StyleFlags::NONE);

    let page = page_builder.build();

    // Only first 160 chars should be written
    for x in 0..160 {
        assert_eq!(page.get_cell(x, 0).unwrap().character(), 'A');
    }

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();
    assert!(!bytes.is_empty());
}

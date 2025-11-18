//! Property-based tests for deterministic rendering using proptest

use escp_layout::{Document, Page, StyleFlags};
use proptest::prelude::*;
use std::collections::HashSet;

// Strategy to generate random page content
fn arbitrary_text() -> impl Strategy<Value = String> {
    // Generate ASCII-only text (32-126) to avoid conversion issues
    prop::collection::vec(32u8..=126u8, 0..100).prop_map(|bytes| String::from_utf8(bytes).unwrap())
}

proptest! {
    #[test]
    fn test_deterministic_rendering_arbitrary_content(
        text in arbitrary_text(),
        x in 0u16..160u16,
        y in 0u16..51u16,
        bold in prop::bool::ANY,
    ) {
        // Create page with arbitrary content
        let mut page_builder = Page::builder();
        let style = if bold {
            StyleFlags::BOLD
        } else {
            StyleFlags::NONE
        };
        page_builder.write_str(x, y, &text, style);
        let page = page_builder.build();

        let mut doc_builder = Document::builder();
        doc_builder.add_page(page);
        let document = doc_builder.build();

        // Render 10 times and verify all identical
        let first_render = document.render();

        for _ in 0..9 {
            let render = document.render();
            prop_assert_eq!(&first_render, &render, "All renders must be byte-identical");
        }

        // Verify output is non-empty
        prop_assert!(!first_render.is_empty(), "Output should not be empty");
    }
}

#[test]
fn test_determinism_with_hash_verification() {
    // Test a fixed document with 1000 renders
    let mut page_builder = Page::builder();
    page_builder.write_str(0, 0, "Determinism Test", StyleFlags::BOLD);
    page_builder.write_str(0, 1, "Testing hash consistency", StyleFlags::NONE);
    let page = page_builder.build();

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    // Render 1000 times and collect hashes
    let mut hashes = HashSet::new();
    let mut first_output = Vec::new();

    for i in 0..1000 {
        let bytes = document.render();

        if i == 0 {
            first_output = bytes.clone();
        }

        // Compute hash (sum of bytes)
        let hash: u64 = bytes.iter().map(|&b| b as u64).sum();
        hashes.insert(hash);

        // Verify byte-for-byte equality
        assert_eq!(
            bytes, first_output,
            "Iteration {} produced different output",
            i
        );
    }

    // All renders should produce the same hash
    assert_eq!(
        hashes.len(),
        1,
        "Should have exactly 1 unique hash, got {}",
        hashes.len()
    );
}

#[test]
fn test_determinism_across_multiple_pages() {
    let mut doc_builder = Document::builder();

    for page_num in 0..5 {
        let mut page_builder = Page::builder();
        let text = format!("Page {}", page_num);
        page_builder.write_str(0, 0, &text, StyleFlags::NONE);
        doc_builder.add_page(page_builder.build());
    }

    let document = doc_builder.build();

    // Render 100 times
    let first_render = document.render();

    for i in 1..100 {
        let render = document.render();
        assert_eq!(first_render, render, "Render {} differs from first", i);
    }
}

proptest! {
    #[test]
    fn test_no_panic_on_arbitrary_input(
        x in 0u16..300u16,  // Beyond bounds intentionally
        y in 0u16..100u16,  // Beyond bounds intentionally
        text in arbitrary_text(),
    ) {
        // Should not panic even with out-of-bounds coordinates
        let mut page_builder = Page::builder();
        page_builder.write_str(x, y, &text, StyleFlags::NONE);
        let page = page_builder.build();

        let mut doc_builder = Document::builder();
        doc_builder.add_page(page);
        let document = doc_builder.build();
        let bytes = document.render();

        // Should produce valid output without panicking
        prop_assert!(!bytes.is_empty());
    }
}

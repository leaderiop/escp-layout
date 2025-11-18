//! Integration tests for text styling (User Story 5)

use escp_layout::{Document, Page, StyleFlags};

#[test]
fn test_bold_text_esc_codes() {
    let mut page_builder = Page::builder();
    page_builder.write_str(0, 0, "Bold Text", StyleFlags::BOLD);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Find ESC E (bold on: 0x1B 0x45)
    let has_bold_on = bytes.windows(2).any(|w| *w == [0x1B, 0x45]);
    assert!(has_bold_on, "Should contain ESC E (bold on) code");

    // Find ESC F (bold off: 0x1B 0x46)
    let has_bold_off = bytes.windows(2).any(|w| *w == [0x1B, 0x46]);
    assert!(has_bold_off, "Should contain ESC F (bold off) code");

    // Verify the text content is present
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("Bold Text"));
}

#[test]
fn test_underline_text_esc_codes() {
    let mut page_builder = Page::builder();
    page_builder.write_str(0, 0, "Underlined Text", StyleFlags::UNDERLINE);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Find ESC - 1 (underline on: 0x1B 0x2D 0x01)
    let has_underline_on = bytes.windows(3).any(|w| *w == [0x1B, 0x2D, 0x01]);
    assert!(
        has_underline_on,
        "Should contain ESC - 1 (underline on) code"
    );

    // Find ESC - 0 (underline off: 0x1B 0x2D 0x00)
    let has_underline_off = bytes.windows(3).any(|w| *w == [0x1B, 0x2D, 0x00]);
    assert!(
        has_underline_off,
        "Should contain ESC - 0 (underline off) code"
    );

    // Verify the text content is present
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("Underlined Text"));
}

#[test]
fn test_combined_bold_underline_styles() {
    let mut page_builder = Page::builder();
    let combined_style = StyleFlags::BOLD.with_underline();
    page_builder.write_str(0, 0, "Bold+Underline", combined_style);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Should have both bold and underline codes
    let has_bold_on = bytes.windows(2).any(|w| *w == [0x1B, 0x45]);
    let has_underline_on = bytes.windows(3).any(|w| *w == [0x1B, 0x2D, 0x01]);

    assert!(has_bold_on, "Should contain bold on code");
    assert!(has_underline_on, "Should contain underline on code");

    // Should have off codes
    let has_bold_off = bytes.windows(2).any(|w| *w == [0x1B, 0x46]);
    let has_underline_off = bytes.windows(3).any(|w| *w == [0x1B, 0x2D, 0x00]);

    assert!(has_bold_off, "Should contain bold off code");
    assert!(has_underline_off, "Should contain underline off code");

    // Verify content
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("Bold+Underline"));
}

#[test]
fn test_style_optimization_adjacent_cells() {
    let mut page_builder = Page::builder();

    // Write 5 adjacent bold 'A's
    for x in 0..5 {
        page_builder.write_at(x, 0, 'A', StyleFlags::BOLD);
    }

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Count ESC E (bold on) occurrences
    let bold_on_count = bytes.windows(2).filter(|w| *w == [0x1B, 0x45]).count();

    // Count ESC F (bold off) occurrences
    let bold_off_count = bytes.windows(2).filter(|w| *w == [0x1B, 0x46]).count();

    // With optimization, should have only 1 bold on and 1 bold off
    assert_eq!(
        bold_on_count, 1,
        "Should have exactly 1 ESC E (optimization)"
    );
    assert_eq!(
        bold_off_count, 1,
        "Should have exactly 1 ESC F (optimization)"
    );

    // Verify all 'A's are present
    let text = String::from_utf8_lossy(&bytes);
    let a_count = text.chars().filter(|&c| c == 'A').count();
    assert_eq!(a_count, 5, "Should have 5 'A' characters");
}

#[test]
fn test_style_transitions_between_different_styles() {
    let mut page_builder = Page::builder();

    // Write text with different styles on same line
    page_builder.write_str(0, 0, "Normal ", StyleFlags::NONE);
    page_builder.write_str(7, 0, "Bold ", StyleFlags::BOLD);
    page_builder.write_str(12, 0, "Underline ", StyleFlags::UNDERLINE);
    page_builder.write_str(22, 0, "Both", StyleFlags::BOLD.with_underline());

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Should have style transitions
    let has_bold = bytes.windows(2).any(|w| *w == [0x1B, 0x45]);
    let has_underline = bytes.windows(3).any(|w| *w == [0x1B, 0x2D, 0x01]);

    assert!(has_bold, "Should have bold codes");
    assert!(has_underline, "Should have underline codes");

    // Verify content
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("Normal"));
    assert!(text.contains("Bold"));
    assert!(text.contains("Underline"));
    assert!(text.contains("Both"));
}

#[test]
fn test_style_reset_at_line_end() {
    let mut page_builder = Page::builder();

    // Write bold text on line 0
    page_builder.write_str(0, 0, "Bold Line 1", StyleFlags::BOLD);

    // Write normal text on line 1
    page_builder.write_str(0, 1, "Normal Line 2", StyleFlags::NONE);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Find CR+LF (0x0D 0x0A)
    let line_endings = bytes
        .windows(2)
        .enumerate()
        .filter(|(_, w)| *w == [0x0D, 0x0A])
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    assert!(!line_endings.is_empty(), "Should have line endings");

    // Verify bold off code appears before first line ending
    let first_line_end = line_endings[0];
    let before_line_end = &bytes[..first_line_end];
    let has_bold_off_before_newline = before_line_end.windows(2).any(|w| *w == [0x1B, 0x46]);

    assert!(
        has_bold_off_before_newline,
        "Bold should be turned off before line ending"
    );
}

#[test]
fn test_no_style_codes_for_none_style() {
    let mut page_builder = Page::builder();
    page_builder.write_str(0, 0, "Plain Text", StyleFlags::NONE);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Should not have any style codes (bold or underline)
    let has_bold_on = bytes.windows(2).any(|w| *w == [0x1B, 0x45]);
    let has_underline_on = bytes.windows(3).any(|w| *w == [0x1B, 0x2D, 0x01]);

    assert!(!has_bold_on, "Should not have bold codes for NONE style");
    assert!(
        !has_underline_on,
        "Should not have underline codes for NONE style"
    );

    // Verify text is present
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("Plain Text"));
}

#[test]
fn test_style_persistence_across_writes() {
    let mut page_builder = Page::builder();

    // Write bold text character by character
    page_builder.write_at(0, 0, 'B', StyleFlags::BOLD);
    page_builder.write_at(1, 0, 'O', StyleFlags::BOLD);
    page_builder.write_at(2, 0, 'L', StyleFlags::BOLD);
    page_builder.write_at(3, 0, 'D', StyleFlags::BOLD);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Count ESC E (should be optimized to 1)
    let bold_on_count = bytes.windows(2).filter(|w| *w == [0x1B, 0x45]).count();

    assert_eq!(
        bold_on_count, 1,
        "Should optimize consecutive bold characters to single ESC E"
    );
}

#[test]
fn test_alternating_styles_optimization() {
    let mut page_builder = Page::builder();

    // Write: BOLD BOLD NORMAL BOLD BOLD
    page_builder.write_at(0, 0, 'A', StyleFlags::BOLD);
    page_builder.write_at(1, 0, 'B', StyleFlags::BOLD);
    page_builder.write_at(2, 0, 'C', StyleFlags::NONE);
    page_builder.write_at(3, 0, 'D', StyleFlags::BOLD);
    page_builder.write_at(4, 0, 'E', StyleFlags::BOLD);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Should have 2 bold on codes (at positions 0 and 3)
    let bold_on_count = bytes.windows(2).filter(|w| *w == [0x1B, 0x45]).count();

    assert_eq!(
        bold_on_count, 2,
        "Should have 2 ESC E codes for alternating styles"
    );
}

#[test]
fn test_empty_cells_break_style_runs() {
    let mut page_builder = Page::builder();

    // Write: BOLD (empty) BOLD
    page_builder.write_at(0, 0, 'A', StyleFlags::BOLD);
    // Skip position 1 (empty cell)
    page_builder.write_at(2, 0, 'B', StyleFlags::BOLD);

    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();

    // Empty cells should break the style run
    // So we should have 2 bold on codes
    let bold_on_count = bytes.windows(2).filter(|w| *w == [0x1B, 0x45]).count();

    assert_eq!(
        bold_on_count, 2,
        "Empty cells should break style optimization"
    );
}

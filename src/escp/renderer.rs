//! Core ESC/P rendering logic.

use super::constants::*;
use super::state::RenderState;
use crate::{Cell, Document, Page};

/// Renders a complete document to an ESC/P byte stream.
///
/// Output format:
/// 1. ESC_RESET + SI_CONDENSED (initialization)
/// 2. Page content (one page at a time)
/// 3. Form-feed after each page
pub(crate) fn render_document(doc: &Document) -> Vec<u8> {
    let mut output = Vec::new();

    // Initialization sequence
    output.extend_from_slice(ESC_RESET);
    output.extend_from_slice(SI_CONDENSED);

    // Render each page
    for page in doc.pages() {
        render_page(page, &mut output);
        // Form-feed after each page
        output.push(FF);
    }

    output
}

/// Renders a single page to the output buffer.
fn render_page(page: &Page, output: &mut Vec<u8>) {
    let mut state = RenderState::new();

    // Render all 51 lines
    for y in 0..51 {
        render_line(&page.cells()[y as usize], &mut state, output);

        // Line termination
        output.push(CR);
        output.push(LF);

        // Reset styles at end of line
        state.reset(output);
    }
}

/// Renders a single line of 160 cells.
fn render_line(cells: &[Cell; 160], state: &mut RenderState, output: &mut Vec<u8>) {
    for cell in cells.iter() {
        // Transition to cell's style
        state.transition_to(cell.style(), output);

        // Emit character (empty cells → space)
        output.push(cell.character() as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::StyleFlags;

    #[test]
    fn test_render_document_empty() {
        let document = Document::builder().build();
        let bytes = render_document(&document);

        // Should still have initialization
        assert!(bytes.starts_with(ESC_RESET));
        assert_eq!(bytes[2], SI_CONDENSED[0]);

        // No form-feeds for 0 pages
        assert!(!bytes.contains(&FF));
    }

    #[test]
    fn test_render_document_single_page() {
        let page = Page::builder().build();
        let mut builder = Document::builder();
        builder.add_page(page);
        let document = builder.build();

        let bytes = render_document(&document);

        // Should have initialization
        assert!(bytes.starts_with(ESC_RESET));

        // Should have exactly 1 form-feed
        let ff_count = bytes.iter().filter(|&&b| b == FF).count();
        assert_eq!(ff_count, 1);
    }

    #[test]
    fn test_render_document_multi_page() {
        let mut builder = Document::builder();
        builder.add_page(Page::builder().build());
        builder.add_page(Page::builder().build());
        builder.add_page(Page::builder().build());
        let document = builder.build();

        let bytes = render_document(&document);

        // Should have 3 form-feeds
        let ff_count = bytes.iter().filter(|&&b| b == FF).count();
        assert_eq!(ff_count, 3);
    }

    #[test]
    fn test_render_page_with_text() {
        let mut page_builder = Page::builder();
        page_builder.write_str(0, 0, "Test", StyleFlags::NONE);
        let page = page_builder.build();

        let mut output = Vec::new();
        render_page(&page, &mut output);

        // Convert to string for easier verification
        let text = String::from_utf8_lossy(&output);
        assert!(text.contains("Test"));
    }

    #[test]
    fn test_render_line_with_styles() {
        let mut page_builder = Page::builder();
        page_builder.write_at(0, 0, 'B', StyleFlags::BOLD);
        page_builder.write_at(1, 0, 'U', StyleFlags::UNDERLINE);
        let page = page_builder.build();

        let mut state = RenderState::new();
        let mut output = Vec::new();

        render_line(&page.cells()[0], &mut state, &mut output);

        // Should contain bold codes
        assert!(output.windows(ESC_BOLD_ON.len()).any(|w| w == ESC_BOLD_ON));

        // Should contain underline codes
        assert!(output
            .windows(ESC_UNDERLINE_ON.len())
            .any(|w| w == ESC_UNDERLINE_ON));
    }

    #[test]
    fn test_deterministic_output() {
        let mut page_builder = Page::builder();
        page_builder.write_str(0, 0, "Deterministic Test", StyleFlags::NONE);
        let page = page_builder.build();

        let mut builder = Document::builder();
        builder.add_page(page.clone());
        let document = builder.build();

        let bytes1 = render_document(&document);
        let bytes2 = render_document(&document);

        // Byte-for-byte identical
        assert_eq!(bytes1, bytes2);
    }
}

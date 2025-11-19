//! Diagnostic tool to examine ESC/P output
//!
//! Shows the exact byte sequence being sent to printer

use escp_layout::widget::{label_new, rect_new};
use escp_layout::{Page, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== ESC/P Output Diagnostic ===\n");

    // Create a simple 2-page document
    let mut doc_builder = Document::builder();

    // Page 1
    let mut root1 = rect_new!(80, 49);
    let header1 = label_new!(20).add_text("Page 1")?;
    let footer1 = label_new!(20).add_text("End of Page 1")?;
    root1.add_child(header1, (10, 0))?;
    root1.add_child(footer1, (10, 48))?;

    let mut page_builder1 = Page::builder();
    page_builder1.render(&root1)?;
    let page1 = page_builder1.build();
    doc_builder.add_page(page1);

    // Page 2
    let mut root2 = rect_new!(80, 49);
    let header2 = label_new!(20).add_text("Page 2")?;
    let footer2 = label_new!(20).add_text("End of Page 2")?;
    root2.add_child(header2, (10, 0))?;
    root2.add_child(footer2, (10, 48))?;

    let mut page_builder2 = Page::builder();
    page_builder2.render(&root2)?;
    let page2 = page_builder2.build();
    doc_builder.add_page(page2);

    // Build and render
    let document = doc_builder.build();
    let escp_bytes = document.render();

    println!("Total bytes: {}", escp_bytes.len());
    println!("\nFirst 100 bytes (hex):");
    for (i, byte) in escp_bytes.iter().take(100).enumerate() {
        if i % 16 == 0 {
            print!("\n{:04x}:  ", i);
        }
        print!("{:02x} ", byte);
    }
    println!("\n");

    // Find key markers
    println!("Command markers:");

    // ESC @ (1B 40)
    let mut esc_reset_count = 0;
    for window in escp_bytes.windows(2) {
        if window == [0x1B, 0x40] {
            esc_reset_count += 1;
        }
    }
    println!("  ESC @ (reset) count: {}", esc_reset_count);

    // FF (0C)
    let ff_count = escp_bytes.iter().filter(|&&b| b == 0x0C).count();
    println!("  FF (form feed) count: {}", ff_count);

    // SI (0F)
    let si_count = escp_bytes.iter().filter(|&&b| b == 0x0F).count();
    println!("  SI (condensed) count: {}", si_count);

    // CR (0D)
    let cr_count = escp_bytes.iter().filter(|&&b| b == 0x0D).count();
    println!("  CR (carriage return) count: {}", cr_count);

    // LF (0A)
    let lf_count = escp_bytes.iter().filter(|&&b| b == 0x0A).count();
    println!("  LF (line feed) count: {}", lf_count);

    // ESC C (1B 43)
    let mut esc_c_count = 0;
    for window in escp_bytes.windows(2) {
        if window == [0x1B, 0x43] {
            esc_c_count += 1;
        }
    }
    println!("  ESC C (page length) count: {}", esc_c_count);

    println!("\nExpected for 2 pages (EPSON LQ-2090II with ESC C 50):");
    println!("  ESC @ should be: 1 (initial only, not between pages)");
    println!("  ESC C should be: 1 (set page length once)");
    println!("  FF should be: 2 (after each page)");
    println!("  SI should be: 1 (initial only)");
    println!("  CR should be: 100 (50 lines × 2 pages, no extra CR)");
    println!("  LF should be: 100 (50 lines × 2 pages)");

    // Check page structure
    println!("\nPage structure analysis:");
    let page = &document.pages()[0];
    let first_line = &page.cells()[0];
    let last_line = &page.cells()[50];

    println!("  First line (0): {} chars",
        first_line.iter().filter(|c| c.character() != ' ').count());
    println!("  Last line (50): {} chars",
        last_line.iter().filter(|c| c.character() != ' ').count());

    // Show where content is
    println!("\nContent positions on first page:");
    for y in 0..51 {
        let line = &page.cells()[y as usize];
        let content_count = line.iter().filter(|c| c.character() != ' ').count();
        if content_count > 0 {
            let first_char_x = line.iter().position(|c| c.character() != ' ').unwrap();
            let content: String = line[first_char_x..first_char_x + content_count]
                .iter()
                .map(|c| c.character())
                .collect();
            println!("  Line {:2}: col {:3}, {} chars: '{}'",
                y, first_char_x, content_count, content);
        }
    }

    Ok(())
}

//! Example 1: Basic Label Widget
//!
//! Demonstrates:
//! - Creating labels with different widths
//! - Adding text content
//! - Text validation (length constraints)
//! - Rendering labels to a page

use escp_layout::widget::{label_new, rect_new};
use escp_layout::{Document, Page};

fn print_page(page: &Page, width: u16, height: u16) {
    println!("┌{}┐", "─".repeat(width as usize));
    for y in 0..height {
        print!("│");
        for x in 0..width {
            if let Some(cell) = page.get_cell(x, y) {
                print!("{}", cell.character());
            } else {
                print!(" ");
            }
        }
        println!("│");
    }
    println!("└{}┘", "─".repeat(width as usize));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Example 1: Basic Label Widget ===\n");

    // Create a root container
    let mut root = rect_new!(80, 30);

    // Example 1.1: Simple label
    println!("1.1 Simple Label:");
    let simple_label = label_new!(20).add_text("Hello, World!")?;
    root.add_child(simple_label, (0, 0))?;
    println!("  ✓ Created label: 'Hello, World!' at (0, 0)\n");

    // Example 1.2: Label with maximum width text
    println!("1.2 Label with Maximum Width:");
    let max_width_label = label_new!(25).add_text("This is exactly 25 chars")?;
    root.add_child(max_width_label, (0, 2))?;
    println!("  ✓ Created 25-char label at (0, 2)\n");

    // Example 1.3: Short text in wide label
    println!("1.3 Short Text in Wide Label:");
    let short_label = label_new!(30).add_text("Short")?;
    root.add_child(short_label, (0, 4))?;
    println!("  ✓ Created wide label (30 cols) with short text at (0, 4)\n");

    // Example 1.4: Empty label (valid - renders nothing)
    println!("1.4 Empty Label:");
    let empty_label = label_new!(15);
    root.add_child(empty_label, (0, 6))?;
    println!("  ✓ Created empty label at (0, 6) (renders nothing)\n");

    // Example 1.5: Text validation - too long
    println!("1.5 Text Validation (Error Handling):");
    let result = label_new!(10).add_text("This text is way too long for the label");
    match result {
        Err(e) => println!("  ✓ Correctly rejected text that's too long: {}\n", e),
        Ok(_) => println!("  ✗ Should have rejected long text!\n"),
    }

    // Example 1.6: Newline validation
    println!("1.6 Newline Validation:");
    let result = label_new!(20).add_text("Line1\nLine2");
    match result {
        Err(e) => println!("  ✓ Correctly rejected text with newline: {}\n", e),
        Ok(_) => println!("  ✗ Should have rejected text with newline!\n"),
    }

    // Render to page
    let mut page_builder = Page::builder();
    page_builder.render(&root)?;
    let page = page_builder.build();

    // Verify rendering
    println!("Verification:");
    let cell = page.get_cell(0, 0).unwrap();
    println!("  Cell at (0, 0): '{}'", cell.character());
    assert_eq!(cell.character(), 'H');
    println!("  ✓ Label rendered correctly!\n");

    // Print the rendered output
    println!("Rendered Output (80×10):");
    print_page(&page, 80, 10);

    // Also show ESC/P output
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();
    let escp_bytes = document.render();
    println!("\nESC/P output: {} bytes", escp_bytes.len());

    println!("\n=== Label Widget Example Complete ===");
    Ok(())
}

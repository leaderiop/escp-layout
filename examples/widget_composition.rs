//! Comprehensive example demonstrating widget composability system.
//!
//! This example shows how to:
//! - Create nested widget hierarchies
//! - Use layout components (Column, Row, Stack)
//! - Apply styling to labels
//! - Render widget trees to pages

use escp_layout::widget::{column_area, column_new, label_new, rect_new, row_area, row_new};
use escp_layout::{Document, Page};

fn print_page(page: &Page, width: u16, height: u16) {
    println!("  ┌{}┐", "─".repeat(width as usize));
    for y in 0..height {
        print!("  │");
        for x in 0..width {
            if let Some(cell) = page.get_cell(x, y) {
                print!("{}", cell.character());
            } else {
                print!(" ");
            }
        }
        println!("│");
    }
    println!("  └{}┘", "─".repeat(width as usize));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Widget Composability System Example");
    println!("====================================\n");

    // Example 1: Simple label in a rect
    println!("Example 1: Simple Label");
    {
        let mut root = rect_new!(80, 30);
        let label = label_new!(20).add_text("Hello, World!")?;
        root.add_child(label, (10, 5))?;

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        // Verify rendering
        let cell = page.get_cell(10, 5).unwrap();
        println!("  Rendered 'H' at (10, 5): {}", cell.character());
        println!("\n  Visual Output (80×10):");
        print_page(&page, 80, 10);
    }

    // Example 2: Nested rectes
    println!("\nExample 2: Nested Rectes");
    {
        let mut root = rect_new!(80, 30);

        let mut child_rect = rect_new!(40, 15);
        let label = label_new!(10).add_text("Nested")?;
        child_rect.add_child(label, (5, 3))?;

        root.add_child(child_rect, (20, 10))?;

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        // Label is at (5, 3) within child_rect, which is at (20, 10) within root
        // Absolute position: (25, 13)
        let cell = page.get_cell(25, 13).unwrap();
        println!("  Rendered 'N' at absolute (25, 13): {}", cell.character());
        println!("\n  Visual Output (80×20):");
        print_page(&page, 80, 20);
    }

    // Example 3: Column layout
    println!("\nExample 3: Column Layout");
    {
        let mut root = rect_new!(80, 30);
        let mut column = column_new!(80, 30);

        let (mut row1, pos1) = column_area!(column, 10)?;
        let label1 = label_new!(15).add_text("Row 1")?;
        row1.add_child(label1, (0, 0))?;
        root.add_child(row1, pos1)?;

        let (mut row2, pos2) = column_area!(column, 10)?;
        let label2 = label_new!(15).add_text("Row 2")?;
        row2.add_child(label2, (0, 0))?;
        root.add_child(row2, pos2)?;

        let (mut row3, pos3) = column_area!(column, 10)?;
        let label3 = label_new!(15).add_text("Row 3")?;
        row3.add_child(label3, (0, 0))?;
        root.add_child(row3, pos3)?;

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        println!("  Created 3-row layout with Column");
        println!("  Row 1 at y=0, Row 2 at y=10, Row 3 at y=20");
        println!("\n  Visual Output (80×30):");
        print_page(&page, 80, 30);
    }

    // Example 4: Row layout
    println!("\nExample 4: Row Layout (3-column)");
    {
        let mut root = rect_new!(80, 30);
        let mut row = row_new!(80, 30);

        let (mut col1, pos1) = row_area!(row, 25)?;
        let label1 = label_new!(25).add_text("Column 1")?;
        col1.add_child(label1, (0, 0))?;
        root.add_child(col1, pos1)?;

        let (mut col2, pos2) = row_area!(row, 30)?;
        let label2 = label_new!(30).add_text("Column 2")?;
        col2.add_child(label2, (0, 0))?;
        root.add_child(col2, pos2)?;

        let (mut col3, pos3) = row_area!(row, 25)?;
        let label3 = label_new!(25).add_text("Column 3")?;
        col3.add_child(label3, (0, 0))?;
        root.add_child(col3, pos3)?;

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        println!("  Created 3-column layout with Row");
        println!("  Col 1 at x=0, Col 2 at x=25, Col 3 at x=55");
        println!("\n  Visual Output (80×10):");
        print_page(&page, 80, 10);
    }

    // Example 5: Styled labels
    println!("\nExample 5: Styled Labels");
    {
        let mut root = rect_new!(80, 30);

        let bold_label = label_new!(20).add_text("Bold Text")?.bold();

        let underlined_label = label_new!(20).add_text("Underlined Text")?.underline();

        let both_label = label_new!(20)
            .add_text("Bold & Underlined")?
            .bold()
            .underline();

        root.add_child(bold_label, (0, 0))?;
        root.add_child(underlined_label, (0, 2))?;
        root.add_child(both_label, (0, 4))?;

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        println!("  Created labels with bold, underline, and both styles");
        println!("  Note: Styles are encoded in ESC/P but shown as plain text here");
        println!("\n  Visual Output (80×10):");
        print_page(&page, 80, 10);
    }

    // Example 6: Complex nested layout (invoice-like)
    println!("\nExample 6: Complex Nested Layout (Invoice)");
    {
        let mut root = rect_new!(80, 40);
        let mut main_column = column_new!(80, 40);

        // Header row
        let (mut header, header_pos) = column_area!(main_column, 5)?;
        let header_label = label_new!(80).add_text("=== INVOICE ===")?.bold();
        header.add_child(header_label, (0, 1))?;
        root.add_child(header, header_pos)?;

        // Body with 3 columns
        let (mut body, body_pos) = column_area!(main_column, 25)?;
        let mut body_row = row_new!(80, 25);

        let (mut body_col1, pos1) = row_area!(body_row, 25)?;
        let item_label = label_new!(25).add_text("Item")?;
        body_col1.add_child(item_label, (0, 0))?;
        body.add_child(body_col1, pos1)?;

        let (mut body_col2, pos2) = row_area!(body_row, 30)?;
        let desc_label = label_new!(30).add_text("Description")?;
        body_col2.add_child(desc_label, (0, 0))?;
        body.add_child(body_col2, pos2)?;

        let (mut body_col3, pos3) = row_area!(body_row, 25)?;
        let price_label = label_new!(25).add_text("Price")?;
        body_col3.add_child(price_label, (0, 0))?;
        body.add_child(body_col3, pos3)?;

        root.add_child(body, body_pos)?;

        // Footer row
        let (mut footer, footer_pos) = column_area!(main_column, 10)?;
        let total_label = label_new!(40)
            .add_text("Total: $1,234.56")?
            .bold()
            .underline();
        footer.add_child(total_label, (0, 2))?;
        root.add_child(footer, footer_pos)?;

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        println!("  Created complex invoice layout:");
        println!("    - Header (5 rows)");
        println!("    - Body with 3 columns (25 rows)");
        println!("    - Footer (10 rows)");
        println!("\n  Visual Output (80×40):");
        print_page(&page, 80, 40);

        // Show ESC/P output size
        let mut doc_builder = Document::builder();
        doc_builder.add_page(page);
        let document = doc_builder.build();
        let escp_bytes = document.render();
        println!("\n  ESC/P output: {} bytes", escp_bytes.len());
    }

    println!("\n====================================");
    println!("All examples completed successfully!");
    println!("Widget composability system is working correctly.");

    Ok(())
}

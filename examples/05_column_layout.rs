//! Example 5: Column Layout
//!
//! Demonstrates:
//! - Creating column layouts for vertical division
//! - Allocating rows with different heights
//! - Automatic y-position tracking
//! - InsufficientSpace error handling

use escp_layout::widget::{box_new, column_area, column_new, label_new};
use escp_layout::Page;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Example 5: Column Layout ===\n");

    // Example 5.1: Simple 3-row column
    println!("5.1 Simple 3-Row Column:");
    {
        let mut root = box_new!(80, 30);
        let mut column = column_new!(80, 30);

        let (mut row1, pos1) = column_area!(column, 10)?;
        let (mut row2, pos2) = column_area!(column, 10)?;
        let (mut row3, pos3) = column_area!(column, 10)?;

        let label1 = label_new!(15).add_text("Row 1 (h=10)")?;
        let label2 = label_new!(15).add_text("Row 2 (h=10)")?;
        let label3 = label_new!(15).add_text("Row 3 (h=10)")?;

        row1.add_child(label1, (0, 0))?;
        row2.add_child(label2, (0, 0))?;
        row3.add_child(label3, (0, 0))?;

        root.add_child(row1, pos1)?;
        root.add_child(row2, pos2)?;
        root.add_child(row3, pos3)?;

        println!("  Row 1: height=10, position={:?}", pos1);
        println!("  Row 2: height=10, position={:?}", pos2);
        println!("  Row 3: height=10, position={:?}", pos3);
        println!("  ✓ Created equal-height rows\n");
    }

    // Example 5.2: Variable height rows
    println!("5.2 Variable Height Rows:");
    {
        let mut root = box_new!(80, 40);
        let mut column = column_new!(80, 40);

        let (mut header, header_pos) = column_area!(column, 5)?;
        let (mut body, body_pos) = column_area!(column, 30)?;
        let (mut footer, footer_pos) = column_area!(column, 5)?;

        let header_label = label_new!(30).add_text("Header (5 rows)")?;
        let body_label = label_new!(30).add_text("Body (30 rows)")?;
        let footer_label = label_new!(30).add_text("Footer (5 rows)")?;

        header.add_child(header_label, (0, 1))?;
        body.add_child(body_label, (0, 10))?;
        footer.add_child(footer_label, (0, 1))?;

        root.add_child(header, header_pos)?;
        root.add_child(body, body_pos)?;
        root.add_child(footer, footer_pos)?;

        println!("  Header: 5 rows at {:?}", header_pos);
        println!("  Body: 30 rows at {:?}", body_pos);
        println!("  Footer: 5 rows at {:?}", footer_pos);
        println!("  Total: 40 rows (exact fit)");
        println!("  ✓ Created variable-height layout\n");
    }

    // Example 5.3: Multiple labels per row
    println!("5.3 Multiple Labels Per Row:");
    {
        let mut root = box_new!(80, 30);
        let mut column = column_new!(80, 30);

        for i in 0..5 {
            let (mut row, pos) = column_area!(column, 5)?;

            let label1 = label_new!(20).add_text(&format!("Row {} Left", i))?;
            let label2 = label_new!(20).add_text(&format!("Row {} Right", i))?;

            row.add_child(label1, (0, 1))?;
            row.add_child(label2, (30, 1))?;

            root.add_child(row, pos)?;
        }

        println!("  Created 5 rows, each with 2 labels");
        println!("  ✓ Column with multiple children per row\n");
    }

    // Example 5.4: Nested content in rows
    println!("5.4 Nested Content in Rows:");
    {
        let mut root = box_new!(80, 30);
        let mut column = column_new!(80, 30);

        let (mut row1, pos1) = column_area!(column, 10)?;

        // Add nested box inside the row
        let mut nested = box_new!(60, 8);
        let label1 = label_new!(20).add_text("Nested Label 1")?;
        let label2 = label_new!(20).add_text("Nested Label 2")?;
        nested.add_child(label1, (0, 0))?;
        nested.add_child(label2, (0, 4))?;

        row1.add_child(nested, (10, 1))?;
        root.add_child(row1, pos1)?;

        println!("  Created row with nested box containing 2 labels");
        println!("  ✓ Nesting works inside column rows\n");
    }

    // Example 5.5: Error handling - insufficient space
    println!("5.5 Error Handling - Insufficient Space:");
    {
        let mut column = column_new!(80, 30);

        let _ = column_area!(column, 15)?;  // 15 rows used
        let _ = column_area!(column, 10)?;  // 25 rows used

        // Try to allocate 10 more rows (only 5 available)
        let result = column_area!(column, 10);

        match result {
            Err(e) => println!("  ✓ Correctly rejected oversized allocation: {}\n", e),
            Ok(_) => println!("  ✗ Should have rejected oversized allocation!\n"),
        }
    }

    // Example 5.6: Exact fit (use all space)
    println!("5.6 Exact Fit:");
    {
        let mut root = box_new!(80, 30);
        let mut column = column_new!(80, 30);

        let (row1, pos1) = column_area!(column, 10)?;
        let (row2, pos2) = column_area!(column, 10)?;
        let (row3, pos3) = column_area!(column, 10)?;  // Exactly 30 rows total

        root.add_child(row1, pos1)?;
        root.add_child(row2, pos2)?;
        root.add_child(row3, pos3)?;

        // Try to allocate one more - should fail
        let result = column_area!(column, 1);
        match result {
            Err(_) => println!("  ✓ All space used, further allocation rejected\n"),
            Ok(_) => println!("  ✗ Should have rejected allocation when full!\n"),
        }
    }

    // Example 5.7: Full render
    println!("5.7 Full Render:");
    {
        let mut root = box_new!(80, 30);
        let mut column = column_new!(80, 30);

        let (mut row1, pos1) = column_area!(column, 8)?;
        let (mut row2, pos2) = column_area!(column, 8)?;
        let (mut row3, pos3) = column_area!(column, 8)?;

        let label1 = label_new!(25).add_text("First Row")?;
        let label2 = label_new!(25).add_text("Second Row")?;
        let label3 = label_new!(25).add_text("Third Row")?;

        row1.add_child(label1, (5, 2))?;
        row2.add_child(label2, (5, 2))?;
        row3.add_child(label3, (5, 2))?;

        root.add_child(row1, pos1)?;
        root.add_child(row2, pos2)?;
        root.add_child(row3, pos3)?;

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        println!("  Row 1 label at absolute (5, 2)");
        println!("  Row 2 label at absolute (5, 10)");
        println!("  Row 3 label at absolute (5, 18)");

        let cell = page.get_cell(5, 2).unwrap();
        println!("  Cell at (5, 2): '{}'", cell.character());
        println!("  ✓ Column layout rendered correctly!\n");
    }

    println!("=== Column Layout Example Complete ===");
    Ok(())
}

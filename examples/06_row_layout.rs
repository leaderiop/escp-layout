//! Example 6: Row Layout
//!
//! Demonstrates:
//! - Creating row layouts for horizontal division
//! - Allocating columns with different widths
//! - Automatic x-position tracking
//! - Creating multi-column layouts

use escp_layout::widget::{box_new, label_new, row_area, row_new};
use escp_layout::Page;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Example 6: Row Layout ===\n");

    // Example 6.1: Simple 3-column row
    println!("6.1 Simple 3-Column Row:");
    {
        let mut root = box_new!(80, 30);
        let mut row = row_new!(80, 30);

        let (mut col1, pos1) = row_area!(row, 25)?;
        let (mut col2, pos2) = row_area!(row, 30)?;
        let (mut col3, pos3) = row_area!(row, 25)?;

        let label1 = label_new!(20).add_text("Column 1 (w=25)")?;
        let label2 = label_new!(25).add_text("Column 2 (w=30)")?;
        let label3 = label_new!(20).add_text("Column 3 (w=25)")?;

        col1.add_child(label1, (0, 0))?;
        col2.add_child(label2, (0, 0))?;
        col3.add_child(label3, (0, 0))?;

        root.add_child(col1, pos1)?;
        root.add_child(col2, pos2)?;
        root.add_child(col3, pos3)?;

        println!("  Col 1: width=25, position={:?}", pos1);
        println!("  Col 2: width=30, position={:?}", pos2);
        println!("  Col 3: width=25, position={:?}", pos3);
        println!("  Total: 80 columns (exact fit)");
        println!("  ✓ Created 3-column layout\n");
    }

    // Example 6.2: Variable width columns
    println!("6.2 Variable Width Columns:");
    {
        let mut root = box_new!(80, 30);
        let mut row = row_new!(80, 30);

        let (mut sidebar, sidebar_pos) = row_area!(row, 20)?;
        let (mut main, main_pos) = row_area!(row, 50)?;
        let (mut aside, aside_pos) = row_area!(row, 10)?;

        let sidebar_label = label_new!(15).add_text("Sidebar")?;
        let main_label = label_new!(30).add_text("Main Content Area")?;
        let aside_label = label_new!(8).add_text("Aside")?;

        sidebar.add_child(sidebar_label, (0, 5))?;
        main.add_child(main_label, (10, 5))?;
        aside.add_child(aside_label, (0, 5))?;

        root.add_child(sidebar, sidebar_pos)?;
        root.add_child(main, main_pos)?;
        root.add_child(aside, aside_pos)?;

        println!("  Sidebar: 20 cols at {:?}", sidebar_pos);
        println!("  Main: 50 cols at {:?}", main_pos);
        println!("  Aside: 10 cols at {:?}", aside_pos);
        println!("  ✓ Created sidebar/main/aside layout\n");
    }

    // Example 6.3: Multiple rows of labels per column
    println!("6.3 Multiple Rows Per Column:");
    {
        let mut root = box_new!(80, 30);
        let mut row = row_new!(80, 30);

        for i in 0..4 {
            let (mut col, pos) = row_area!(row, 20)?;

            let label1 = label_new!(18).add_text(&format!("Col {} Top", i))?;
            let label2 = label_new!(18).add_text(&format!("Col {} Mid", i))?;
            let label3 = label_new!(18).add_text(&format!("Col {} Bot", i))?;

            col.add_child(label1, (0, 0))?;
            col.add_child(label2, (0, 10))?;
            col.add_child(label3, (0, 20))?;

            root.add_child(col, pos)?;
        }

        println!("  Created 4 columns, each with 3 labels");
        println!("  ✓ Row with multiple children per column\n");
    }

    // Example 6.4: Narrow and wide columns
    println!("6.4 Narrow and Wide Columns:");
    {
        let mut root = box_new!(80, 30);
        let mut row = row_new!(80, 30);

        let (mut narrow1, pos1) = row_area!(row, 10)?;
        let (mut wide, pos2) = row_area!(row, 60)?;
        let (mut narrow2, pos3) = row_area!(row, 10)?;

        let n1_label = label_new!(8).add_text("N1")?;
        let wide_label = label_new!(55).add_text("Wide Content Area Here")?;
        let n2_label = label_new!(8).add_text("N2")?;

        narrow1.add_child(n1_label, (1, 1))?;
        wide.add_child(wide_label, (2, 1))?;
        narrow2.add_child(n2_label, (1, 1))?;

        root.add_child(narrow1, pos1)?;
        root.add_child(wide, pos2)?;
        root.add_child(narrow2, pos3)?;

        println!("  Narrow1: 10 cols");
        println!("  Wide: 60 cols");
        println!("  Narrow2: 10 cols");
        println!("  ✓ Mixed column widths\n");
    }

    // Example 6.5: Equal-width columns
    println!("6.5 Equal-Width Columns (5 cols):");
    {
        let mut root = box_new!(80, 30);
        let mut row = row_new!(80, 30);

        for i in 0..5 {
            let (mut col, pos) = row_area!(row, 16)?;
            let label = label_new!(12).add_text(&format!("Col {}", i + 1))?;
            col.add_child(label, (2, 5))?;
            root.add_child(col, pos)?;
        }

        println!("  Created 5 equal-width columns (16 cols each)");
        println!("  ✓ Equal division\n");
    }

    // Example 6.6: Error handling - insufficient space
    println!("6.6 Error Handling - Insufficient Space:");
    {
        let mut row = row_new!(80, 30);

        let _ = row_area!(row, 30)?;  // 30 cols used
        let _ = row_area!(row, 40)?;  // 70 cols used

        // Try to allocate 20 more cols (only 10 available)
        let result = row_area!(row, 20);

        match result {
            Err(e) => println!("  ✓ Correctly rejected oversized allocation: {}\n", e),
            Ok(_) => println!("  ✗ Should have rejected oversized allocation!\n"),
        }
    }

    // Example 6.7: Full render
    println!("6.7 Full Render:");
    {
        let mut root = box_new!(80, 30);
        let mut row = row_new!(80, 30);

        let (mut col1, pos1) = row_area!(row, 25)?;
        let (mut col2, pos2) = row_area!(row, 25)?;
        let (mut col3, pos3) = row_area!(row, 30)?;

        let label1 = label_new!(20).add_text("Left")?;
        let label2 = label_new!(20).add_text("Center")?;
        let label3 = label_new!(25).add_text("Right")?;

        col1.add_child(label1, (2, 10))?;
        col2.add_child(label2, (2, 10))?;
        col3.add_child(label3, (2, 10))?;

        root.add_child(col1, pos1)?;
        root.add_child(col2, pos2)?;
        root.add_child(col3, pos3)?;

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        println!("  Col 1 label at absolute (2, 10)");
        println!("  Col 2 label at absolute (27, 10)");
        println!("  Col 3 label at absolute (52, 10)");

        let cell1 = page.get_cell(2, 10).unwrap();
        let cell2 = page.get_cell(27, 10).unwrap();
        let cell3 = page.get_cell(52, 10).unwrap();
        println!("  Cell at (2, 10): '{}'", cell1.character());
        println!("  Cell at (27, 10): '{}'", cell2.character());
        println!("  Cell at (52, 10): '{}'", cell3.character());
        println!("  ✓ Row layout rendered correctly!\n");
    }

    println!("=== Row Layout Example Complete ===");
    Ok(())
}

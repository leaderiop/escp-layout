//! Example 3: Box Container Widget
//!
//! Demonstrates:
//! - Creating box containers with different sizes
//! - Adding multiple children to a box
//! - Positioning children with explicit coordinates
//! - Boundary validation and error handling

use escp_layout::widget::{box_new, label_new};
use escp_layout::Page;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Example 3: Box Container Widget ===\n");

    // Example 3.1: Simple box with one child
    println!("3.1 Simple Box with One Child:");
    {
        let mut container = box_new!(40, 20);
        let label = label_new!(15).add_text("Inside Box")?;
        container.add_child(label, (5, 3))?;

        println!("  ✓ Created 40×20 box with label at (5, 3)\n");
    }

    // Example 3.2: Box with multiple children at different positions
    println!("3.2 Multiple Children:");
    {
        let mut container = box_new!(60, 30);

        let label1 = label_new!(20).add_text("Top Left")?;
        let label2 = label_new!(20).add_text("Middle")?;
        let label3 = label_new!(20).add_text("Bottom Right")?;

        container.add_child(label1, (0, 0))?;
        container.add_child(label2, (20, 10))?;
        container.add_child(label3, (40, 25))?;

        println!("  ✓ Created box with 3 labels at different positions\n");
    }

    // Example 3.3: Adjacent children (touching edges - allowed)
    println!("3.3 Adjacent Children (Touching Edges):");
    {
        let mut container = box_new!(80, 30);

        let label1 = label_new!(20).add_text("Label 1")?;
        let label2 = label_new!(20).add_text("Label 2")?;
        let label3 = label_new!(20).add_text("Label 3")?;

        container.add_child(label1, (0, 0))?;
        container.add_child(label2, (20, 0))?;  // Touches right edge of label1
        container.add_child(label3, (40, 0))?;  // Touches right edge of label2

        println!("  ✓ Created 3 adjacent labels (edges touching)\n");
    }

    // Example 3.4: Grid of labels
    println!("3.4 Grid of Labels:");
    {
        let mut container = box_new!(80, 30);

        // Create a 4×3 grid
        for row in 0..3 {
            for col in 0..4 {
                let text = format!("R{}C{}", row, col);
                let label = label_new!(10).add_text(&text)?;
                container.add_child(label, (col * 20, row * 10))?;
            }
        }

        println!("  ✓ Created 4×3 grid of labels (12 children)\n");
    }

    // Example 3.5: Error handling - child exceeds parent
    println!("3.5 Error Handling - Child Too Large:");
    {
        let mut small_box = box_new!(20, 20);
        let big_label = label_new!(30).add_text("Too wide!")?;

        let result = small_box.add_child(big_label, (0, 0));
        match result {
            Err(e) => println!("  ✓ Correctly rejected oversized child: {}\n", e),
            Ok(_) => println!("  ✗ Should have rejected oversized child!\n"),
        }
    }

    // Example 3.6: Error handling - overlapping children
    println!("3.6 Error Handling - Overlapping Children:");
    {
        let mut container = box_new!(80, 30);

        let label1 = label_new!(20).add_text("Label 1")?;
        let label2 = label_new!(20).add_text("Label 2")?;

        container.add_child(label1, (0, 0))?;
        let result = container.add_child(label2, (10, 0));  // Overlaps with label1

        match result {
            Err(e) => println!("  ✓ Correctly rejected overlapping child: {}\n", e),
            Ok(_) => println!("  ✗ Should have rejected overlapping child!\n"),
        }
    }

    // Example 3.7: Full render
    println!("3.7 Full Render:");
    {
        let mut root = box_new!(80, 30);

        let label1 = label_new!(30).add_text("Box Container Example")?;
        let label2 = label_new!(40).add_text("Multiple children positioned correctly")?;

        root.add_child(label1, (10, 5))?;
        root.add_child(label2, (10, 10))?;

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        let cell = page.get_cell(10, 5).unwrap();
        println!("  ✓ Rendered successfully, cell at (10, 5): '{}'", cell.character());
    }

    println!("\n=== Box Container Example Complete ===");
    Ok(())
}

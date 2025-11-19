//! Example 4: Nested Boxes
//!
//! Demonstrates:
//! - Creating hierarchical widget structures
//! - Nesting boxes inside boxes
//! - Automatic cumulative coordinate calculation
//! - Deep nesting (multiple levels)

use escp_layout::widget::{box_new, label_new};
use escp_layout::Page;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Example 4: Nested Boxes ===\n");

    // Example 4.1: Two-level nesting
    println!("4.1 Two-Level Nesting:");
    {
        let mut root = box_new!(80, 30);

        let mut child_box = box_new!(40, 15);
        let label = label_new!(20).add_text("In Child Box")?;
        child_box.add_child(label, (5, 3))?;

        root.add_child(child_box, (20, 10))?;

        println!("  Root Box: 80×30");
        println!("  ├─ Child Box: 40×15 at (20, 10)");
        println!("  │  └─ Label: at (5, 3) within child");
        println!("  └─ Label absolute position: (25, 13)");
        println!("  ✓ Two-level hierarchy created\n");
    }

    // Example 4.2: Three-level nesting
    println!("4.2 Three-Level Nesting:");
    {
        let mut root = box_new!(80, 30);

        let mut level1 = box_new!(60, 25);
        let mut level2 = box_new!(40, 20);
        let label = label_new!(20).add_text("Deep Label")?;

        level2.add_child(label, (5, 5))?;
        level1.add_child(level2, (10, 3))?;
        root.add_child(level1, (10, 2))?;

        println!("  Root: 80×30 at (0, 0)");
        println!("  └─ Level1: 60×25 at (10, 2)");
        println!("     └─ Level2: 40×20 at (10, 3) → absolute (20, 5)");
        println!("        └─ Label at (5, 5) → absolute (25, 10)");
        println!("  ✓ Three-level hierarchy created\n");
    }

    // Example 4.3: Multiple children at each level
    println!("4.3 Multiple Children Per Level:");
    {
        let mut root = box_new!(80, 30);

        // First branch
        let mut branch1 = box_new!(35, 25);
        let label1a = label_new!(15).add_text("Branch 1 Top")?;
        let label1b = label_new!(15).add_text("Branch 1 Bot")?;
        branch1.add_child(label1a, (0, 0))?;
        branch1.add_child(label1b, (0, 20))?;

        // Second branch
        let mut branch2 = box_new!(35, 25);
        let label2a = label_new!(15).add_text("Branch 2 Top")?;
        let label2b = label_new!(15).add_text("Branch 2 Bot")?;
        branch2.add_child(label2a, (0, 0))?;
        branch2.add_child(label2b, (0, 20))?;

        root.add_child(branch1, (0, 0))?;
        root.add_child(branch2, (40, 0))?;

        println!("  Root with 2 branches, each with 2 labels");
        println!("  ✓ Multi-branch hierarchy created\n");
    }

    // Example 4.4: Deep nesting (5 levels)
    println!("4.4 Deep Nesting (5 Levels):");
    {
        let mut level0 = box_new!(80, 30);
        let mut level1 = box_new!(70, 25);
        let mut level2 = box_new!(60, 20);
        let mut level3 = box_new!(50, 15);
        let mut level4 = box_new!(40, 10);
        let label = label_new!(30).add_text("Deep Nested Label")?;

        level4.add_child(label, (5, 4))?;
        level3.add_child(level4, (5, 3))?;
        level2.add_child(level3, (5, 3))?;
        level1.add_child(level2, (5, 3))?;
        level0.add_child(level1, (5, 3))?;

        println!("  Level 0 (root): 80×30 at (0, 0)");
        println!("  └─ Level 1: 70×25 at (5, 3) → abs (5, 3)");
        println!("     └─ Level 2: 60×20 at (5, 3) → abs (10, 6)");
        println!("        └─ Level 3: 50×15 at (5, 3) → abs (15, 9)");
        println!("           └─ Level 4: 40×10 at (5, 3) → abs (20, 12)");
        println!("              └─ Label at (5, 4) → abs (25, 16)");
        println!("  ✓ 5-level deep hierarchy created\n");
    }

    // Example 4.5: Render and verify cumulative coordinates
    println!("4.5 Render with Cumulative Coordinates:");
    {
        let mut root = box_new!(80, 30);

        let mut outer = box_new!(60, 25);
        let mut inner = box_new!(40, 20);
        let label = label_new!(15).add_text("Test")?;

        inner.add_child(label, (10, 8))?;   // Position in inner
        outer.add_child(inner, (15, 5))?;   // Position in outer
        root.add_child(outer, (5, 2))?;     // Position in root

        // Expected absolute position: (5 + 15 + 10, 2 + 5 + 8) = (30, 15)

        let mut page_builder = Page::builder();
        page_builder.render(&root)?;
        let page = page_builder.build();

        let cell = page.get_cell(30, 15).unwrap();
        println!("  Label positioned at:");
        println!("    (10, 8) in inner box");
        println!("    + (15, 5) inner's position in outer");
        println!("    + (5, 2) outer's position in root");
        println!("    = (30, 15) absolute position");
        println!("  Cell at (30, 15): '{}'", cell.character());
        println!("  ✓ Cumulative coordinates calculated correctly!\n");
    }

    println!("=== Nested Boxes Example Complete ===");
    Ok(())
}

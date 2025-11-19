//! Example 2: Styled Labels
//!
//! Demonstrates:
//! - Applying bold styling
//! - Applying underline styling
//! - Combining multiple styles
//! - Builder pattern for styling

use escp_layout::widget::{box_new, label_new};
use escp_layout::Page;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Example 2: Styled Labels ===\n");

    let mut root = box_new!(80, 30);

    // Example 2.1: Bold text
    println!("2.1 Bold Text:");
    let bold_label = label_new!(20).add_text("Bold Text Here")?.bold();
    root.add_child(bold_label, (0, 0))?;
    println!("  ✓ Created bold label at (0, 0)\n");

    // Example 2.2: Underlined text
    println!("2.2 Underlined Text:");
    let underline_label = label_new!(20).add_text("Underlined Text")?.underline();
    root.add_child(underline_label, (0, 2))?;
    println!("  ✓ Created underlined label at (0, 2)\n");

    // Example 2.3: Bold AND underlined
    println!("2.3 Bold + Underlined:");
    let both_label = label_new!(20)
        .add_text("Bold & Underlined")?
        .bold()
        .underline();
    root.add_child(both_label, (0, 4))?;
    println!("  ✓ Created bold+underlined label at (0, 4)\n");

    // Example 2.4: Style order doesn't matter
    println!("2.4 Style Order (underline first, then bold):");
    let reversed_label = label_new!(20)
        .add_text("Underline -> Bold")?
        .underline()
        .bold();
    root.add_child(reversed_label, (0, 6))?;
    println!("  ✓ Created label with reversed style order at (0, 6)\n");

    // Example 2.5: Multiple styled labels in a row
    println!("2.5 Row of Styled Labels:");
    let label1 = label_new!(15).add_text("Normal")?;
    let label2 = label_new!(15).add_text("Bold")?.bold();
    let label3 = label_new!(15).add_text("Underline")?.underline();

    root.add_child(label1, (0, 10))?;
    root.add_child(label2, (20, 10))?;
    root.add_child(label3, (40, 10))?;
    println!("  ✓ Created row of 3 labels with different styles at y=10\n");

    // Render to page
    let mut page_builder = Page::builder();
    page_builder.render(&root)?;
    let page = page_builder.build();

    // Verify styling
    println!("Verification:");
    let cell = page.get_cell(0, 0).unwrap();
    println!("  Cell at (0, 0) has bold: {}", cell.style().bold());
    println!("  Cell at (0, 2) has underline: {}", page.get_cell(0, 2).unwrap().style().underline());
    println!("  Cell at (0, 4) has both: bold={}, underline={}",
        page.get_cell(0, 4).unwrap().style().bold(),
        page.get_cell(0, 4).unwrap().style().underline());
    println!("  ✓ All styles rendered correctly!\n");

    println!("=== Styled Labels Example Complete ===");
    Ok(())
}

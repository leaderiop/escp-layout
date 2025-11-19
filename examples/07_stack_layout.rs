//! Example 7: Stack Layout
//!
//! Demonstrates:
//! - Creating stack layouts for conceptual overlapping layers
//! - All areas positioned at (0, 0)
//! - How to use stack-allocated rectes as layer containers
//! - Rendering strategies for layered content

use escp_layout::widget::{label_new, stack_new};
use escp_layout::Page;

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
    println!("=== Example 7: Stack Layout ===\n");

    println!("IMPORTANT: Stack layout returns overlapping rectes at (0, 0).");
    println!("This enables conceptual layering where you:");
    println!("1. Build content in separate layer rectes");
    println!("2. Choose which layer to render (or render in sequence)\n");

    // Example 7.1: Simple 2-layer concept
    println!("7.1 Two-Layer Stack Concept:");
    {
        let stack = stack_new!(80, 30);

        let (mut background, bg_pos) = stack.area();
        let (mut foreground, fg_pos) = stack.area();

        println!("  Background rect at {:?}", bg_pos);
        println!("  Foreground rect at {:?}", fg_pos);
        println!("  Both at (0, 0) - conceptually overlapping");

        let bg_label = label_new!(30).add_text("Background Layer")?;
        let fg_label = label_new!(30).add_text("Foreground Layer")?;

        background.add_child(bg_label, (10, 10))?;
        foreground.add_child(fg_label, (15, 15))?;

        println!("  ✓ Two layer rectes created\n");
    }

    // Example 7.2: Rendering strategy - use top layer
    println!("7.2 Rendering Strategy - Top Layer Only:");
    {
        let stack = stack_new!(80, 30);

        let (mut background, _bg_pos) = stack.area();
        let (mut foreground, _fg_pos) = stack.area();

        // Build background content
        let bg_marker1 = label_new!(10).add_text("[BG-TL]")?;
        let bg_marker2 = label_new!(10).add_text("[BG-BR]")?;
        background.add_child(bg_marker1, (0, 0))?;
        background.add_child(bg_marker2, (70, 29))?;

        // Build foreground content
        let fg_content = label_new!(30).add_text("FOREGROUND CONTENT")?;
        foreground.add_child(fg_content, (25, 15))?;

        // Render only the foreground layer
        let mut page_builder = Page::builder();
        page_builder.render(&foreground)?;
        let page = page_builder.build();

        let cell = page.get_cell(25, 15).unwrap();
        println!("  Rendered foreground only");
        println!("  Cell at (25, 15): '{}'", cell.character());
        println!("  ✓ Single layer rendering\n");
    }

    // Example 7.3: Rendering strategy - background then overlay
    println!("7.3 Rendering Strategy - Background Then Overlay:");
    {
        let stack = stack_new!(80, 30);

        let (mut background, _) = stack.area();
        let (mut overlay, _) = stack.area();

        // Background with pattern
        let bg1 = label_new!(5).add_text("BG-1")?;
        let bg2 = label_new!(5).add_text("BG-2")?;
        let bg3 = label_new!(5).add_text("BG-3")?;
        background.add_child(bg1, (0, 0))?;
        background.add_child(bg2, (37, 14))?;
        background.add_child(bg3, (75, 29))?;

        // Overlay with main content
        let title = label_new!(40).add_text("MAIN CONTENT")?;
        overlay.add_child(title, (20, 10))?;

        println!("  Background has 3 markers");
        println!("  Overlay has centered title");
        println!("  ✓ Multi-step rendering strategy\n");
    }

    // Example 7.4: Three conceptual layers
    println!("7.4 Three Conceptual Layers:");
    {
        let stack = stack_new!(80, 30);

        let (mut layer1, pos1) = stack.area();
        let (mut layer2, pos2) = stack.area();
        let (mut layer3, pos3) = stack.area();

        println!("  Layer 1 at {:?}", pos1);
        println!("  Layer 2 at {:?}", pos2);
        println!("  Layer 3 at {:?}", pos3);

        let label1 = label_new!(20).add_text("Layer 1 (bottom)")?;
        let label2 = label_new!(20).add_text("Layer 2 (middle)")?;
        let label3 = label_new!(20).add_text("Layer 3 (top)")?;

        layer1.add_child(label1, (0, 0))?;
        layer2.add_child(label2, (5, 5))?;
        layer3.add_child(label3, (10, 10))?;

        println!("  ✓ Three layers prepared\n");
    }

    // Example 7.5: Using a layer as the root
    println!("7.5 Using Layer as Root Widget:");
    {
        let stack = stack_new!(80, 30);

        let (mut _background, _) = stack.area();
        let (mut main_layer, _) = stack.area();

        // Build content in main layer
        let header = label_new!(40).add_text("DOCUMENT HEADER")?;
        let body = label_new!(60).add_text("This is the main content of the document.")?;
        let footer = label_new!(40).add_text("Page 1")?;

        main_layer.add_child(header, (20, 5))?;
        main_layer.add_child(body, (10, 15))?;
        main_layer.add_child(footer, (35, 28))?;

        // Use main_layer directly as root for rendering
        let mut page_builder = Page::builder();
        page_builder.render(&main_layer)?;
        let page = page_builder.build();

        let cell_header = page.get_cell(20, 5).unwrap();
        let cell_body = page.get_cell(10, 15).unwrap();
        let cell_footer = page.get_cell(35, 28).unwrap();

        println!("  Header at (20, 5): '{}'", cell_header.character());
        println!("  Body at (10, 15): '{}'", cell_body.character());
        println!("  Footer at (35, 28): '{}'", cell_footer.character());
        println!("  ✓ Layer used as root widget");

        // Print visual output
        println!("\n  Rendered Output (80×30):");
        print_page(&page, 80, 30);
    }

    // Example 7.6: Stack with styled content
    println!("7.6 Styled Layers:");
    {
        let stack = stack_new!(80, 30);

        let (mut normal_layer, _) = stack.area();
        let (mut bold_layer, _) = stack.area();

        let normal = label_new!(25).add_text("Normal Text Layer")?;
        let bold = label_new!(25).add_text("Bold Text Layer")?.bold();

        normal_layer.add_child(normal, (10, 10))?;
        bold_layer.add_child(bold, (10, 15))?;

        println!("  Normal layer with regular text");
        println!("  Bold layer with emphasized text");
        println!("  ✓ Styling works in stack layers\n");
    }

    println!("=== Stack Layout Summary ===");
    println!("\nKey Points:");
    println!("1. Stack returns overlapping rectes at (0, 0)");
    println!("2. Cannot add overlapping children to same parent Rect");
    println!("3. Use cases:");
    println!("   - Build content in separate logical layers");
    println!("   - Choose which layer to render");
    println!("   - Render layers sequentially (background first)");
    println!("   - Use one layer as the final root widget");

    println!("\n✓ Stack Layout Example Complete");

    Ok(())
}

//! Widget showcase - demonstrates all 6 widget types in a single document.
//!
//! Run with: cargo run --example widgets_showcase

use escp_layout::widgets::{ASCIIBox, ColumnDef, KeyValueList, Label, Paragraph, Table, TextBlock};
use escp_layout::{Document, Page, Region, StyleFlags};

fn main() {
    let mut page_builder = Page::builder();

    // === TITLE ===
    let title = Label::new("WIDGET SHOWCASE - ESC/P Layout Engine")
        .with_style(StyleFlags::BOLD.with_underline());
    page_builder.render_widget(Region::new(0, 0, 160, 1).unwrap(), &title);

    // Separator
    page_builder.fill_region(Region::new(0, 1, 160, 1).unwrap(), '=', StyleFlags::NONE);

    // === 1. LABEL WIDGET ===
    let label_demo = Label::new("1. Label Widget - Single line text with optional styles")
        .with_style(StyleFlags::BOLD);
    page_builder.render_widget(Region::new(0, 3, 160, 1).unwrap(), &label_demo);

    // === 2. TEXTBLOCK WIDGET ===
    let textblock_title = Label::new("2. TextBlock Widget - Multi-line text without wrapping")
        .with_style(StyleFlags::BOLD);
    page_builder.render_widget(Region::new(0, 5, 160, 1).unwrap(), &textblock_title);

    let textblock_demo = TextBlock::from_text(
        "Line 1: Each line is rendered separately\nLine 2: No automatic word wrapping\nLine 3: Perfect for pre-formatted content"
    );
    page_builder.render_widget(Region::new(2, 6, 150, 5).unwrap(), &textblock_demo);

    // === 3. PARAGRAPH WIDGET ===
    let para_title = Label::new("3. Paragraph Widget - Multi-line text with word wrapping")
        .with_style(StyleFlags::BOLD);
    page_builder.render_widget(Region::new(0, 11, 160, 1).unwrap(), &para_title);

    let para_demo = Paragraph::new(
        "This is a paragraph widget that automatically wraps long text at word boundaries. \
        It intelligently breaks lines to fit within the specified region width, making it \
        perfect for longer prose content that needs to flow naturally.",
    );
    page_builder.render_widget(Region::new(2, 12, 70, 5).unwrap(), &para_demo);

    // === 4. KEYVALUELIST WIDGET ===
    let kv_title =
        Label::new("4. KeyValueList Widget - Aligned key-value pairs").with_style(StyleFlags::BOLD);
    page_builder.render_widget(Region::new(0, 18, 160, 1).unwrap(), &kv_title);

    let kv_demo = KeyValueList::new(vec![
        ("Name".into(), "John Doe".into()),
        ("Email".into(), "john@example.com".into()),
        ("Phone".into(), "(555) 123-4567".into()),
        ("Department".into(), "Engineering".into()),
    ])
    .with_separator(": ");
    page_builder.render_widget(Region::new(2, 19, 60, 6).unwrap(), &kv_demo);

    // === 5. TABLE WIDGET ===
    let table_title = Label::new("5. Table Widget - Fixed-column tables with bold headers")
        .with_style(StyleFlags::BOLD);
    page_builder.render_widget(Region::new(0, 26, 160, 1).unwrap(), &table_title);

    let table_demo = Table::new(
        vec![
            ColumnDef {
                name: "Product".into(),
                width: 40,
            },
            ColumnDef {
                name: "Category".into(),
                width: 25,
            },
            ColumnDef {
                name: "Price".into(),
                width: 15,
            },
            ColumnDef {
                name: "Stock".into(),
                width: 10,
            },
        ],
        vec![
            vec![
                "Widget A".into(),
                "Hardware".into(),
                "$29.99".into(),
                "150".into(),
            ],
            vec![
                "Gadget B".into(),
                "Electronics".into(),
                "$49.99".into(),
                "87".into(),
            ],
            vec![
                "Component C".into(),
                "Parts".into(),
                "$12.50".into(),
                "340".into(),
            ],
        ],
    );
    page_builder.render_widget(Region::new(2, 27, 95, 6).unwrap(), &table_demo);

    // === 6. ASCIIBOX WIDGET ===
    let box_title = Label::new("6. ASCIIBox Widget - Bordered containers with optional titles")
        .with_style(StyleFlags::BOLD);
    page_builder.render_widget(Region::new(0, 34, 160, 1).unwrap(), &box_title);

    // Create a nested widget structure: Box containing a KeyValueList
    let inner_content = KeyValueList::new(vec![
        ("Feature".into(), "ASCII borders".into()),
        ("Title".into(), "Optional".into()),
        ("Nesting".into(), "Supported".into()),
    ]);

    let box_demo = ASCIIBox::new(Box::new(inner_content)).with_title("Feature Highlight");
    page_builder.render_widget(Region::new(2, 35, 50, 7).unwrap(), &box_demo);

    // Create another box with a paragraph
    let usage_text = Paragraph::new(
        "Widgets can be nested inside ASCIIBox for emphasis. Perfect for highlighting \
        important information or grouping related content.",
    );
    let usage_box = ASCIIBox::new(Box::new(usage_text)).with_title("Usage Tip");
    page_builder.render_widget(Region::new(55, 35, 50, 7).unwrap(), &usage_box);

    // === FOOTER ===
    page_builder.fill_region(Region::new(0, 43, 160, 1).unwrap(), '=', StyleFlags::NONE);

    let footer_text = TextBlock::from_text(
        "Widget Benefits:\n  \
        * Reusable components for common layouts\n  \
        * Automatic truncation and boundary handling\n  \
        * Clean, declarative API\n  \
        * Type-safe and zero-panic guaranteed",
    );
    page_builder.render_widget(Region::new(0, 45, 160, 6).unwrap(), &footer_text);

    // Build and render
    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    let bytes = document.render();
    std::fs::write("output_widgets_showcase.prn", &bytes).expect("Failed to write output file");

    println!(
        "✓ Generated output_widgets_showcase.prn ({} bytes)",
        bytes.len()
    );
    println!("\n{}", "=".repeat(80));
    println!("RENDERED OUTPUT PREVIEW:");
    println!("{}", "=".repeat(80));

    // Strip ESC/P codes and display the content
    let output_str = String::from_utf8_lossy(&bytes);
    for line in output_str.lines() {
        // Simple ESC/P code removal (ESC followed by any character)
        let clean_line = line
            .replace("\x1b@", "") // Initialize
            .replace("\x1bO", "") // Cancel bold
            .replace("\x1b-\x01", "") // Underline on
            .replace("\x1b-\x00", "") // Underline off
            .replace("\x1bE", "") // Bold on
            .replace("\x1bF", "") // Bold off
            .replace("\x0c", "\n--- PAGE BREAK ---\n"); // Form feed
        println!("{}", clean_line);
    }

    println!("\n{}", "=".repeat(80));
    println!("✓ All 6 widgets demonstrated:");
    println!("  1. Label       - Styled single-line text");
    println!("  2. TextBlock   - Multi-line without wrapping");
    println!("  3. Paragraph   - Multi-line with word wrapping");
    println!("  4. KeyValueList - Aligned key-value pairs");
    println!("  5. Table       - Fixed-column tables");
    println!("  6. ASCIIBox    - Bordered containers");
}

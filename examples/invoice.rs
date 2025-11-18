//! Invoice example - demonstrates widgets including Table, KeyValueList, and ASCIIBox.
//!
//! Run with: cargo run --example invoice

use escp_layout::widgets::{ASCIIBox, ColumnDef, KeyValueList, Label, Table};
use escp_layout::{Document, Page, Region, StyleFlags};

fn main() {
    let mut page_builder = Page::builder();

    // Define regions for invoice layout
    let full_page = Region::full_page();

    // Split page into header (12 lines), body (33 lines), footer (6 lines)
    let (header, rest) = full_page.split_vertical(12).unwrap();
    let (body, footer) = rest.split_vertical(33).unwrap();

    // === HEADER SECTION ===
    // Split header into company info (left) and invoice details (right)
    let (company_region, invoice_region) = header.split_horizontal(80).unwrap();

    // Company info using KeyValueList inside an ASCIIBox
    let company_info = KeyValueList::new(vec![
        ("Company".into(), "ACME CORPORATION".into()),
        ("Address".into(), "123 Business St, Suite 100".into()),
        ("City".into(), "City, ST 12345".into()),
        ("Phone".into(), "(555) 123-4567".into()),
    ])
    .with_separator(": ");

    let company_box = ASCIIBox::new(Box::new(company_info)).with_title("Seller");

    page_builder.render_widget(company_region, &company_box);

    // Invoice details using KeyValueList
    let invoice_details = KeyValueList::new(vec![
        ("Invoice #".into(), "12345".into()),
        ("Date".into(), "2025-11-18".into()),
        ("Due Date".into(), "2025-12-18".into()),
    ])
    .with_separator(": ");

    let invoice_box = ASCIIBox::new(Box::new(invoice_details)).with_title("Invoice Info");

    page_builder.render_widget(invoice_region, &invoice_box);

    // === BODY SECTION ===
    // Bill To section
    let (bill_to_region, items_region) = body.split_vertical(10).unwrap();

    let bill_to_info = KeyValueList::new(vec![
        ("Bill To".into(), "Customer Name".into()),
        ("Address".into(), "456 Customer Ave".into()),
        ("City".into(), "Town, ST 67890".into()),
    ])
    .with_separator(": ");

    let bill_to_box = ASCIIBox::new(Box::new(bill_to_info)).with_title("Customer");

    page_builder.render_widget(
        Region::new(
            bill_to_region.x(),
            bill_to_region.y(),
            80,
            bill_to_region.height(),
        )
        .unwrap(),
        &bill_to_box,
    );

    // Items table using Table widget
    let items_table = Table::new(
        vec![
            ColumnDef {
                name: "QTY".into(),
                width: 5,
            },
            ColumnDef {
                name: "DESCRIPTION".into(),
                width: 70,
            },
            ColumnDef {
                name: "UNIT PRICE".into(),
                width: 15,
            },
            ColumnDef {
                name: "TOTAL".into(),
                width: 15,
            },
        ],
        vec![
            vec![
                "2".into(),
                "Widget Model A".into(),
                "$125.00".into(),
                "$250.00".into(),
            ],
            vec![
                "1".into(),
                "Gadget Model B".into(),
                "$350.00".into(),
                "$350.00".into(),
            ],
            vec![
                "5".into(),
                "Component Type C".into(),
                "$45.00".into(),
                "$225.00".into(),
            ],
            vec![
                "3".into(),
                "Service Package D".into(),
                "$200.00".into(),
                "$600.00".into(),
            ],
        ],
    );

    page_builder.render_widget(
        Region::new(items_region.x(), items_region.y(), 105, 15).unwrap(),
        &items_table,
    );

    // Totals section
    let totals_y = items_region.y() + 17;
    page_builder.fill_region(
        Region::new(0, totals_y - 1, 160, 1).unwrap(),
        '-',
        StyleFlags::NONE,
    );

    // Totals using KeyValueList
    let totals = KeyValueList::new(vec![
        ("SUBTOTAL".into(), "$1,425.00".into()),
        ("TAX (8%)".into(), "$114.00".into()),
        ("TOTAL".into(), "$1,539.00".into()),
    ])
    .with_separator(": ");

    page_builder.render_widget(Region::new(100, totals_y, 60, 5).unwrap(), &totals);

    // Make the total bold
    page_builder.write_str(100, totals_y + 2, "TOTAL", StyleFlags::BOLD);
    page_builder.write_str(107, totals_y + 2, ":", StyleFlags::BOLD);
    page_builder.write_str(109, totals_y + 2, "$1,539.00", StyleFlags::BOLD);

    // === FOOTER SECTION ===
    page_builder.fill_region(
        Region::new(0, footer.y(), 160, 1).unwrap(),
        '=',
        StyleFlags::NONE,
    );

    // Footer text using Labels
    let payment_terms = Label::new("Payment Terms: Net 30 days");
    page_builder.render_widget(
        Region::new(footer.x(), footer.y() + 2, 80, 1).unwrap(),
        &payment_terms,
    );

    let thank_you = Label::new("Thank you for your business!").with_style(StyleFlags::BOLD);
    page_builder.render_widget(
        Region::new(footer.x(), footer.y() + 3, 80, 1).unwrap(),
        &thank_you,
    );

    // Build document
    let page = page_builder.build();
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    // Render to ESC/P bytes
    let bytes = document.render();

    // Save to file
    std::fs::write("output_invoice.prn", &bytes).expect("Failed to write output file");

    println!("✓ Generated output_invoice.prn ({} bytes)", bytes.len());
    println!("\n{}", "=".repeat(80));
    println!("RENDERED OUTPUT PREVIEW:");
    println!("{}", "=".repeat(80));

    // Strip ESC/P codes and display the content
    let output_str = String::from_utf8_lossy(&bytes);
    for line in output_str.lines() {
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
    println!("✓ Invoice layout:");
    println!("  - Header: Company info and invoice details (ASCIIBox + KeyValueList)");
    println!("  - Body: Bill-to info and line items (Table widget)");
    println!("  - Footer: Payment terms (Label widgets)");
    println!("✓ Widgets demonstrated:");
    println!("  - ASCIIBox: Bordered sections with titles");
    println!("  - KeyValueList: Aligned key-value pairs");
    println!("  - Table: Multi-column data with bold headers");
    println!("  - Label: Styled single-line text");
}

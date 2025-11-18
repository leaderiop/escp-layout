//! Multi-page report example - demonstrates TextBlock, Paragraph, and ASCIIBox widgets.
//!
//! Run with: cargo run --example report

use escp_layout::widgets::{ASCIIBox, ColumnDef, Label, Table, TextBlock, Widget};
use escp_layout::{Document, Page, Region, StyleFlags};

fn create_page_with_header(
    page_num: u16,
    total_pages: u16,
    title: &str,
    content_widget: &dyn Widget,
) -> Page {
    let mut page_builder = Page::builder();

    // Header with title and page number
    let header_title = Label::new(title).with_style(StyleFlags::BOLD.with_underline());
    page_builder.render_widget(Region::new(0, 0, 120, 1).unwrap(), &header_title);

    let page_info = format!("Page {} of {}", page_num, total_pages);
    let page_label = Label::new(&page_info);
    page_builder.render_widget(Region::new(140, 0, 20, 1).unwrap(), &page_label);

    // Header separator
    page_builder.fill_region(Region::new(0, 1, 160, 1).unwrap(), '=', StyleFlags::NONE);

    // Content area (lines 3-48)
    let content_region = Region::new(0, 3, 160, 45).unwrap();
    page_builder.render_widget(content_region, content_widget);

    // Footer separator
    page_builder.fill_region(Region::new(0, 49, 160, 1).unwrap(), '=', StyleFlags::NONE);

    // Footer
    let footer_label = Label::new("Confidential - For Internal Use Only");
    page_builder.render_widget(Region::new(0, 50, 80, 1).unwrap(), &footer_label);

    page_builder.build()
}

fn main() {
    let mut doc_builder = Document::builder();

    // === PAGE 1: EXECUTIVE SUMMARY ===
    let page1_text = r#"EXECUTIVE SUMMARY

This quarterly report provides a comprehensive overview of our operations, financial performance, and strategic initiatives for Q4 2025.

KEY HIGHLIGHTS:

  * Revenue increased 15% year-over-year to $4.2M
  * Customer base grew to 1,250 active accounts
  * Launched three new product features
  * Employee headcount reached 45 team members
  * Customer satisfaction score: 4.7/5.0

STRATEGIC INITIATIVES:

1. Market Expansion
   Successfully entered two new geographic markets, expanding our reach to the Pacific Northwest and Mountain regions.

2. Product Development
   Released version 2.0 with enhanced analytics capabilities and improved user interface based on customer feedback.

3. Team Growth
   Hired 8 new team members across engineering, sales, and support to support our growth trajectory.

CHALLENGES:

  * Supply chain delays impacted Q4 delivery schedules
  * Increased competition in core markets
  * Talent acquisition in competitive tech labor market

OUTLOOK:

We remain optimistic about Q1 2026, with projected revenue growth of 20% and continued expansion into new markets. Our focus will be on customer retention, product innovation, and operational efficiency."#;

    let page1_content = TextBlock::from_text(page1_text);
    doc_builder.add_page(create_page_with_header(
        1,
        3,
        "QUARTERLY REPORT Q4 2025",
        &page1_content,
    ));

    // === PAGE 2: FINANCIAL SUMMARY ===
    // Create financial tables
    let mut page2_builder = Page::builder();

    // Header
    let header_title =
        Label::new("QUARTERLY REPORT Q4 2025").with_style(StyleFlags::BOLD.with_underline());
    page2_builder.render_widget(Region::new(0, 0, 120, 1).unwrap(), &header_title);

    let page_label = Label::new("Page 2 of 3");
    page2_builder.render_widget(Region::new(140, 0, 20, 1).unwrap(), &page_label);

    page2_builder.fill_region(Region::new(0, 1, 160, 1).unwrap(), '=', StyleFlags::NONE);

    // Section title
    let section_title = Label::new("FINANCIAL SUMMARY").with_style(StyleFlags::BOLD);
    page2_builder.render_widget(Region::new(0, 3, 80, 1).unwrap(), &section_title);

    // Revenue breakdown table
    let revenue_table = Table::new(
        vec![
            ColumnDef {
                name: "Category".into(),
                width: 60,
            },
            ColumnDef {
                name: "Amount".into(),
                width: 25,
            },
            ColumnDef {
                name: "%".into(),
                width: 10,
            },
        ],
        vec![
            vec!["Product Sales".into(), "$2,800,000".into(), "67%".into()],
            vec![
                "Professional Services".into(),
                "$980,000".into(),
                "23%".into(),
            ],
            vec![
                "Maintenance & Support".into(),
                "$420,000".into(),
                "10%".into(),
            ],
            vec!["TOTAL REVENUE".into(), "$4,200,000".into(), "100%".into()],
        ],
    );

    let revenue_box =
        ASCIIBox::new(Box::new(revenue_table)).with_title("Revenue Breakdown (Q4 2025)");
    page2_builder.render_widget(Region::new(0, 5, 100, 8).unwrap(), &revenue_box);

    // Expenses table
    let expenses_table = Table::new(
        vec![
            ColumnDef {
                name: "Category".into(),
                width: 60,
            },
            ColumnDef {
                name: "Amount".into(),
                width: 25,
            },
            ColumnDef {
                name: "%".into(),
                width: 10,
            },
        ],
        vec![
            vec![
                "Salaries & Benefits".into(),
                "$1,900,000".into(),
                "52%".into(),
            ],
            vec!["Marketing & Sales".into(), "$650,000".into(), "18%".into()],
            vec!["R&D".into(), "$520,000".into(), "14%".into()],
            vec!["Operations".into(), "$380,000".into(), "10%".into()],
            vec!["General & Admin".into(), "$220,000".into(), "6%".into()],
            vec!["TOTAL EXPENSES".into(), "$3,670,000".into(), "100%".into()],
        ],
    );

    let expenses_box = ASCIIBox::new(Box::new(expenses_table)).with_title("Expenses by Category");
    page2_builder.render_widget(Region::new(0, 14, 100, 10).unwrap(), &expenses_box);

    // Profitability summary
    let profitability_text = TextBlock::from_text(
        "PROFITABILITY:\n\n  Gross Profit:     $2,450,000  (58%)\n  Operating Income:   $530,000  (13%)\n  Net Income:         $445,000  (11%)"
    );
    page2_builder.render_widget(Region::new(0, 26, 80, 8).unwrap(), &profitability_text);

    // YoY comparison table
    let yoy_table = Table::new(
        vec![
            ColumnDef {
                name: "Metric".into(),
                width: 40,
            },
            ColumnDef {
                name: "Q4 2024".into(),
                width: 25,
            },
            ColumnDef {
                name: "Q4 2025".into(),
                width: 25,
            },
            ColumnDef {
                name: "Change".into(),
                width: 15,
            },
        ],
        vec![
            vec![
                "Revenue".into(),
                "$3,650,000".into(),
                "$4,200,000".into(),
                "+15%".into(),
            ],
            vec![
                "Operating Income".into(),
                "$438,000".into(),
                "$530,000".into(),
                "+21%".into(),
            ],
            vec![
                "Net Income".into(),
                "$365,000".into(),
                "$445,000".into(),
                "+22%".into(),
            ],
            vec![
                "Active Customers".into(),
                "1,050".into(),
                "1,250".into(),
                "+19%".into(),
            ],
        ],
    );

    let yoy_box = ASCIIBox::new(Box::new(yoy_table)).with_title("Year-over-Year Comparison");
    page2_builder.render_widget(Region::new(0, 36, 110, 8).unwrap(), &yoy_box);

    // Footer
    page2_builder.fill_region(Region::new(0, 49, 160, 1).unwrap(), '=', StyleFlags::NONE);
    let footer = Label::new("Confidential - For Internal Use Only");
    page2_builder.render_widget(Region::new(0, 50, 80, 1).unwrap(), &footer);

    doc_builder.add_page(page2_builder.build());

    // === PAGE 3: OPERATIONAL METRICS ===
    let page3_text = r#"OPERATIONAL METRICS

CUSTOMER METRICS:
  Total Active Customers:           1,250
  New Customers Acquired (Q4):        215
  Customer Churn Rate:               2.1%
  Average Customer Lifetime Value: $18,400
  Net Promoter Score (NPS):            64
  Customer Satisfaction Score:        4.7/5.0

PRODUCT USAGE:
  Daily Active Users (DAU):         8,500
  Monthly Active Users (MAU):      24,300
  Average Session Duration:      28 minutes
  Feature Adoption Rate:             78%
  Mobile App Downloads:            3,200

SUPPORT & SERVICE:
  Support Tickets Received:        1,240
  Average Response Time:        45 minutes
  First Contact Resolution:           82%
  Customer Support Satisfaction:     4.6/5.0

TEAM METRICS:
  Total Employees:                    45
  Engineering:                        22
  Sales & Marketing:                  12
  Customer Success:                    8
  Operations & Admin:                  3
  Employee Satisfaction:            4.5/5.0
  Voluntary Turnover Rate:           3.2%

RECOMMENDATIONS:

1. Continue investment in customer success to maintain low churn
2. Expand engineering team to accelerate product roadmap
3. Increase marketing budget to support new market entry
4. Implement advanced analytics to improve operational efficiency"#;

    let page3_content = TextBlock::from_text(page3_text);
    doc_builder.add_page(create_page_with_header(
        3,
        3,
        "QUARTERLY REPORT Q4 2025",
        &page3_content,
    ));

    // Build and render document
    let document = doc_builder.build();
    let bytes = document.render();

    // Save to file
    std::fs::write("output_report.prn", &bytes).expect("Failed to write output file");

    println!("✓ Generated output_report.prn ({} bytes)", bytes.len());
    println!("\n{}", "=".repeat(80));
    println!("RENDERED OUTPUT PREVIEW:");
    println!("{}", "=".repeat(80));

    // Strip ESC/P codes and display the content
    let output_str = String::from_utf8_lossy(&bytes);
    for line in output_str.lines() {
        // Simple ESC/P code removal
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
    println!("✓ Multi-page report with {} pages", document.page_count());
    println!("✓ Widgets demonstrated:");
    println!("  - TextBlock: Multi-line formatted text");
    println!("  - Table: Financial data with bold headers");
    println!("  - ASCIIBox: Bordered sections for emphasis");
    println!("  - Label: Headers and footers");
}

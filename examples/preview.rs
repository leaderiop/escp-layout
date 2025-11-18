//! Preview tool - displays page content as text (without ESC/P codes)
//!
//! Run with: cargo run --example preview

use escp_layout::{Document, Page, Region, StyleFlags};

fn print_page_preview(page: &Page, page_num: usize) {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!(
        "║ PAGE {} PREVIEW (first 60 cols, first 20 lines)             ║",
        page_num
    );
    println!("╠═══════════════════════════════════════════════════════════════╣");

    for y in 0..20.min(51) {
        print!("║ ");
        for x in 0..60.min(160) {
            if let Some(cell) = page.get_cell(x, y) {
                let ch = cell.character();
                print!("{}", ch);
            }
        }
        println!(" ║");
    }

    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn main() {
    println!("\n=== HELLO WORLD EXAMPLE ===");
    {
        let mut page_builder = Page::builder();
        page_builder.write_str(0, 0, "Hello, World!", StyleFlags::NONE);
        let page = page_builder.build();
        print_page_preview(&page, 1);
    }

    println!("\n=== INVOICE EXAMPLE (simplified) ===");
    {
        let mut page_builder = Page::builder();

        page_builder.write_str(0, 0, "ACME CORPORATION", StyleFlags::BOLD);
        page_builder.write_str(0, 1, "123 Business Street", StyleFlags::NONE);
        page_builder.write_str(60, 1, "INVOICE #12345", StyleFlags::BOLD);
        page_builder.write_str(60, 2, "Date: 2025-11-18", StyleFlags::NONE);

        page_builder.fill_region(Region::new(0, 3, 60, 1).unwrap(), '=', StyleFlags::NONE);

        page_builder.write_str(0, 5, "BILL TO: Customer Name", StyleFlags::NONE);
        page_builder.write_str(
            0,
            7,
            "QTY  DESCRIPTION                    PRICE      TOTAL",
            StyleFlags::BOLD,
        );
        page_builder.fill_region(Region::new(0, 8, 60, 1).unwrap(), '-', StyleFlags::NONE);
        page_builder.write_str(
            0,
            9,
            "  2  Widget A                     $125.00    $250.00",
            StyleFlags::NONE,
        );
        page_builder.write_str(
            0,
            10,
            "  1  Gadget B                     $350.00    $350.00",
            StyleFlags::NONE,
        );
        page_builder.fill_region(Region::new(0, 11, 60, 1).unwrap(), '-', StyleFlags::NONE);
        page_builder.write_str(40, 12, "TOTAL:  $600.00", StyleFlags::BOLD);

        let page = page_builder.build();
        print_page_preview(&page, 1);
    }

    println!("\n=== MULTI-PAGE DOCUMENT ===");
    {
        let mut doc_builder = Document::builder();

        for page_num in 1..=3 {
            let mut page_builder = Page::builder();
            let title = format!("REPORT - PAGE {}", page_num);
            page_builder.write_str(0, 0, &title, StyleFlags::BOLD);
            page_builder.fill_region(Region::new(0, 1, 60, 1).unwrap(), '=', StyleFlags::NONE);

            let content = format!("This is the content for page {}.", page_num);
            page_builder.write_str(0, 3, &content, StyleFlags::NONE);
            page_builder.write_str(0, 4, "Lorem ipsum dolor sit amet...", StyleFlags::NONE);

            page_builder.fill_region(Region::new(0, 18, 60, 1).unwrap(), '=', StyleFlags::NONE);
            page_builder.write_str(0, 19, "Confidential", StyleFlags::NONE);

            doc_builder.add_page(page_builder.build());
        }

        let document = doc_builder.build();
        println!("\n✓ Document created with {} pages", document.page_count());

        for (idx, page) in document.pages().iter().enumerate() {
            print_page_preview(page, idx + 1);
        }
    }

    println!("\n✓ Preview complete - all content rendered successfully");
}

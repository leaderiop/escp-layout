//! Integration tests for nested region layouts (User Story 4)

use escp_layout::{Page, Region, StyleFlags};

#[test]
fn test_nested_layout_header_body_footer() {
    let mut page_builder = Page::builder();

    // Full page split into header / body / footer
    let full_page = Region::full_page();
    let (header, rest1) = full_page.split_vertical(5).unwrap();
    let (body, footer) = rest1.split_vertical(41).unwrap();

    // Verify dimensions
    assert_eq!(header.height(), 5);
    assert_eq!(body.height(), 41);
    assert_eq!(footer.height(), 5);
    assert_eq!(
        header.height() + body.height() + footer.height(),
        full_page.height()
    );

    // Write content to each region
    page_builder.write_str(header.x(), header.y(), "HEADER", StyleFlags::BOLD);
    page_builder.write_str(body.x(), body.y(), "BODY", StyleFlags::NONE);
    page_builder.write_str(footer.x(), footer.y(), "FOOTER", StyleFlags::BOLD);

    let page = page_builder.build();

    // Verify header content at lines 0-4
    assert_eq!(
        page.get_cell(header.x(), header.y()).unwrap().character(),
        'H'
    );
    assert_eq!(
        page.get_cell(header.x(), header.y()).unwrap().style(),
        StyleFlags::BOLD
    );

    // Verify body content at line 5
    assert_eq!(page.get_cell(body.x(), body.y()).unwrap().character(), 'B');
    assert_eq!(
        page.get_cell(body.x(), body.y()).unwrap().style(),
        StyleFlags::NONE
    );

    // Verify footer content at line 46
    assert_eq!(
        page.get_cell(footer.x(), footer.y()).unwrap().character(),
        'F'
    );
    assert_eq!(
        page.get_cell(footer.x(), footer.y()).unwrap().style(),
        StyleFlags::BOLD
    );
}

#[test]
fn test_nested_layout_with_horizontal_split() {
    let mut page_builder = Page::builder();

    // Full page split vertically first, then body split horizontally
    let full_page = Region::full_page();
    let (header, rest) = full_page.split_vertical(5).unwrap();
    let (body, footer) = rest.split_vertical(41).unwrap();

    // Split body horizontally into sidebar and main
    let (sidebar, main) = body.split_horizontal(40).unwrap();

    // Verify dimensions sum correctly
    assert_eq!(sidebar.width() + main.width(), body.width());
    assert_eq!(sidebar.width(), 40);
    assert_eq!(main.width(), 120);

    // Write content to each region
    page_builder.write_str(header.x(), header.y(), "HEADER", StyleFlags::BOLD);
    page_builder.write_str(sidebar.x(), sidebar.y(), "SIDEBAR", StyleFlags::UNDERLINE);
    page_builder.write_str(main.x(), main.y(), "MAIN CONTENT", StyleFlags::NONE);
    page_builder.write_str(footer.x(), footer.y(), "FOOTER", StyleFlags::BOLD);

    let page = page_builder.build();

    // Verify header at line 0
    assert_eq!(page.get_cell(0, 0).unwrap().character(), 'H');

    // Verify sidebar at columns 0-39, starting at line 5
    assert_eq!(
        page.get_cell(sidebar.x(), sidebar.y()).unwrap().character(),
        'S'
    );
    assert!(sidebar.x() == 0);
    assert!(sidebar.y() == 5);

    // Verify main content at columns 40-159, starting at line 5
    assert_eq!(page.get_cell(main.x(), main.y()).unwrap().character(), 'M');
    assert_eq!(main.x(), 40);
    assert_eq!(main.y(), 5);

    // Verify footer at line 46
    assert_eq!(
        page.get_cell(footer.x(), footer.y()).unwrap().character(),
        'F'
    );
    assert_eq!(footer.y(), 46);
}

#[test]
fn test_no_content_bleeding_between_regions() {
    let mut page_builder = Page::builder();

    // Create two adjacent regions
    let full_page = Region::full_page();
    let (top, bottom) = full_page.split_vertical(25).unwrap();

    // Fill top region with 'A'
    page_builder.fill_region(top, 'A', StyleFlags::NONE);

    // Fill bottom region with 'B'
    page_builder.fill_region(bottom, 'B', StyleFlags::NONE);

    let page = page_builder.build();

    // Verify top region (lines 0-24) contains only 'A'
    for y in 0..25 {
        for x in 0..160 {
            assert_eq!(
                page.get_cell(x, y).unwrap().character(),
                'A',
                "Expected 'A' at ({}, {})",
                x,
                y
            );
        }
    }

    // Verify bottom region (lines 25-50) contains only 'B'
    for y in 25..51 {
        for x in 0..160 {
            assert_eq!(
                page.get_cell(x, y).unwrap().character(),
                'B',
                "Expected 'B' at ({}, {})",
                x,
                y
            );
        }
    }
}

#[test]
fn test_deeply_nested_regions_5_levels() {
    let mut page_builder = Page::builder();

    // Level 1: Full page
    let level1 = Region::full_page();
    assert_eq!(level1.width(), 160);
    assert_eq!(level1.height(), 51);

    // Level 2: Split vertically
    let (_, level2) = level1.split_vertical(5).unwrap();
    assert!(level2.height() < level1.height());

    // Level 3: Apply padding
    let level3 = level2.with_padding(2, 2, 5, 5).unwrap();
    assert!(level3.width() < level2.width());
    assert!(level3.height() < level2.height());

    // Level 4: Split horizontally
    let (_, level4) = level3.split_horizontal(60).unwrap();
    assert!(level4.width() < level3.width());

    // Level 5: Apply more padding
    let level5 = level4.with_padding(2, 2, 5, 5).unwrap();
    assert!(level5.width() < level4.width());
    assert!(level5.height() < level4.height());

    // Write content at deepest level
    page_builder.write_str(level5.x(), level5.y(), "DEEP", StyleFlags::BOLD);

    let page = page_builder.build();

    // Verify content is at the correct deeply nested position
    let cell = page.get_cell(level5.x(), level5.y()).unwrap();
    assert_eq!(cell.character(), 'D');
    assert_eq!(cell.style(), StyleFlags::BOLD);

    // Verify all ancestor boundaries respected
    // Content should be within level1, level2, level3, level4, and level5
    assert!(level5.x() >= level4.x());
    assert!(level5.y() >= level4.y());
    assert!(level5.x() + level5.width() <= level4.x() + level4.width());
    assert!(level5.y() + level5.height() <= level4.y() + level4.height());
}

#[test]
fn test_region_padding_dimensions() {
    let outer = Region::new(0, 0, 100, 50).unwrap();

    // Apply padding: top:2, right:5, bottom:2, left:5
    let inner = outer.with_padding(2, 5, 2, 5).unwrap();

    // Verify inner region dimensions
    assert_eq!(inner.x(), outer.x() + 5, "Left padding should offset x");
    assert_eq!(inner.y(), outer.y() + 2, "Top padding should offset y");
    assert_eq!(
        inner.width(),
        outer.width() - 10,
        "Width should be reduced by left+right padding"
    );
    assert_eq!(
        inner.height(),
        outer.height() - 4,
        "Height should be reduced by top+bottom padding"
    );

    // Create page and fill regions
    let mut page_builder = Page::builder();
    page_builder.fill_region(outer, '#', StyleFlags::NONE);
    page_builder.fill_region(inner, '@', StyleFlags::NONE);

    let page = page_builder.build();

    // Verify outer border contains '#'
    assert_eq!(page.get_cell(0, 0).unwrap().character(), '#');
    assert_eq!(page.get_cell(0, 1).unwrap().character(), '#');

    // Verify inner area contains '@'
    assert_eq!(
        page.get_cell(inner.x(), inner.y()).unwrap().character(),
        '@'
    );
}

#[test]
fn test_chained_splits_header_body_footer() {
    let full_page = Region::full_page();

    // Split into header/rest
    let (header, rest1) = full_page.split_vertical(10).unwrap();

    // Split rest into body/footer
    let (body, footer) = rest1.split_vertical(31).unwrap();

    // Verify dimensions
    assert_eq!(header.height(), 10);
    assert_eq!(body.height(), 31);
    assert_eq!(footer.height(), 10);

    // Verify they sum to original height
    assert_eq!(header.height() + body.height() + footer.height(), 51);

    // Verify positions
    assert_eq!(header.y(), 0);
    assert_eq!(body.y(), 10);
    assert_eq!(footer.y(), 41);
}

#[test]
fn test_combined_vertical_horizontal_splits() {
    let full_page = Region::full_page();

    // Vertical split first
    let (top, bottom) = full_page.split_vertical(25).unwrap();

    // Horizontal splits on each part
    let (top_left, top_right) = top.split_horizontal(80).unwrap();
    let (bottom_left, bottom_right) = bottom.split_horizontal(80).unwrap();

    // Verify dimensions
    assert_eq!(top_left.width() + top_right.width(), 160);
    assert_eq!(bottom_left.width() + bottom_right.width(), 160);

    assert_eq!(top_left.height(), 25);
    assert_eq!(bottom_left.height(), 26);

    // Verify positions
    assert_eq!(top_left.x(), 0);
    assert_eq!(top_left.y(), 0);

    assert_eq!(top_right.x(), 80);
    assert_eq!(top_right.y(), 0);

    assert_eq!(bottom_left.x(), 0);
    assert_eq!(bottom_left.y(), 25);

    assert_eq!(bottom_right.x(), 80);
    assert_eq!(bottom_right.y(), 25);

    // Create page and write to each quadrant
    let mut page_builder = Page::builder();
    page_builder.write_str(top_left.x(), top_left.y(), "TL", StyleFlags::NONE);
    page_builder.write_str(top_right.x(), top_right.y(), "TR", StyleFlags::NONE);
    page_builder.write_str(bottom_left.x(), bottom_left.y(), "BL", StyleFlags::NONE);
    page_builder.write_str(bottom_right.x(), bottom_right.y(), "BR", StyleFlags::NONE);

    let page = page_builder.build();

    assert_eq!(page.get_cell(0, 0).unwrap().character(), 'T');
    assert_eq!(page.get_cell(80, 0).unwrap().character(), 'T');
    assert_eq!(page.get_cell(0, 25).unwrap().character(), 'B');
    assert_eq!(page.get_cell(80, 25).unwrap().character(), 'B');
}

#[test]
fn test_render_in_region_callback() {
    let mut page_builder = Page::builder();
    let region = Region::new(10, 10, 50, 10).unwrap();

    // Render content directly in the region
    page_builder.write_str(region.x(), region.y(), "Widget Content", StyleFlags::BOLD);
    page_builder.fill_region(
        Region::new(region.x(), region.y() + 1, region.width(), 1).unwrap(),
        '-',
        StyleFlags::NONE,
    );

    let page = page_builder.build();

    // Verify content was written in the region
    assert_eq!(page.get_cell(10, 10).unwrap().character(), 'W');
    assert_eq!(page.get_cell(10, 11).unwrap().character(), '-');
}

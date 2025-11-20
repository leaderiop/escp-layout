//! Basic integration test for widget composability system.

use escp_layout::widget::{label_new, rect_new};
use escp_layout::Page;

#[test]
fn test_basic_widget_composition() {
    // Create a simple widget tree
    let mut root = rect_new!(80, 30);

    // Add a label
    let label = label_new!(20)
        .add_text("Hello, World!")
        .expect("Text should fit");
    root.add_child(label, (10, 5))
        .expect("Child should fit in parent");

    // Render to page
    let mut page_builder = Page::builder();
    page_builder
        .render(&root)
        .expect("Rendering should succeed");

    let page = page_builder.build();

    // Verify the text was rendered at position (10, 5)
    let cell = page.get_cell(10, 5).expect("Cell should exist");
    assert_eq!(cell.character(), 'H');

    let cell = page.get_cell(11, 5).expect("Cell should exist");
    assert_eq!(cell.character(), 'e');
}

#[test]
fn test_nested_rectes() {
    // Create a nested widget tree
    let mut root = rect_new!(80, 30);

    let mut child_rect = rect_new!(40, 15);
    let label = label_new!(10).add_text("Nested").expect("Text should fit");
    child_rect
        .add_child(label, (5, 3))
        .expect("Child should fit");

    // Add the child rect to the root at (20, 10)
    root.add_child(child_rect, (20, 10))
        .expect("Child should fit in parent");

    // Render to page
    let mut page_builder = Page::builder();
    page_builder
        .render(&root)
        .expect("Rendering should succeed");

    let page = page_builder.build();

    // The label is at (5, 3) within child_rect, which is at (20, 10) within root
    // So the absolute position is (25, 13)
    let cell = page.get_cell(25, 13).expect("Cell should exist");
    assert_eq!(cell.character(), 'N');
}

#[test]
fn test_child_exceeds_parent() {
    let mut parent = rect_new!(20, 20);
    let child = rect_new!(30, 10);

    // Child is too wide for parent
    let result = parent.add_child(child, (0, 0));
    assert!(result.is_err());
}

#[test]
fn test_overlapping_children() {
    let mut parent = rect_new!(80, 30);

    let label1 = label_new!(20).add_text("Label 1").expect("Text should fit");
    let label2 = label_new!(20).add_text("Label 2").expect("Text should fit");

    parent
        .add_child(label1, (0, 0))
        .expect("First child should fit");

    // Second label overlaps with first (0-20 intersects 10-30)
    let result = parent.add_child(label2, (10, 0));
    assert!(result.is_err());
}

#[test]
fn test_touching_edges_allowed() {
    let mut parent = rect_new!(80, 30);

    let label1 = label_new!(20).add_text("Label 1").expect("Text should fit");
    let label2 = label_new!(20).add_text("Label 2").expect("Text should fit");

    parent
        .add_child(label1, (0, 0))
        .expect("First child should fit");

    // Second label touches first label's right edge (edges touch at x=20)
    let result = parent.add_child(label2, (20, 0));
    assert!(result.is_ok(), "Touching edges should be allowed");
}

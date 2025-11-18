use criterion::{black_box, criterion_group, criterion_main, Criterion};
use escp_layout::widgets::*;
use escp_layout::{Page, Region, StyleFlags};

fn bench_label_widget(c: &mut Criterion) {
    let label = Label::new("Test Label").with_style(StyleFlags::BOLD);
    let region = Region::new(0, 0, 80, 1).unwrap();

    c.bench_function("widget_label", |b| {
        b.iter(|| {
            let mut page = Page::builder();
            black_box(page.render_widget(region, &label));
        });
    });
}

fn bench_text_block_widget(c: &mut Criterion) {
    let text = TextBlock::from_text("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");
    let region = Region::new(0, 0, 80, 10).unwrap();

    c.bench_function("widget_text_block", |b| {
        b.iter(|| {
            let mut page = Page::builder();
            black_box(page.render_widget(region, &text));
        });
    });
}

fn bench_paragraph_widget(c: &mut Criterion) {
    let para = Paragraph::new(
        "This is a longer paragraph that will require word wrapping when rendered into a region.",
    );
    let region = Region::new(0, 0, 40, 10).unwrap();

    c.bench_function("widget_paragraph", |b| {
        b.iter(|| {
            let mut page = Page::builder();
            black_box(page.render_widget(region, &para));
        });
    });
}

fn bench_table_widget(c: &mut Criterion) {
    let table = Table::new(
        vec![
            ColumnDef {
                name: "Column 1".into(),
                width: 30,
            },
            ColumnDef {
                name: "Column 2".into(),
                width: 30,
            },
            ColumnDef {
                name: "Column 3".into(),
                width: 30,
            },
        ],
        vec![
            vec!["Data 1".into(), "Data 2".into(), "Data 3".into()],
            vec!["Data 4".into(), "Data 5".into(), "Data 6".into()],
            vec!["Data 7".into(), "Data 8".into(), "Data 9".into()],
        ],
    );
    let region = Region::new(0, 0, 90, 10).unwrap();

    c.bench_function("widget_table", |b| {
        b.iter(|| {
            let mut page = Page::builder();
            black_box(page.render_widget(region, &table));
        });
    });
}

fn bench_ascii_box_widget(c: &mut Criterion) {
    let content = Label::new("Boxed content");
    let boxed = ASCIIBox::new(Box::new(content)).with_title("Title");
    let region = Region::new(0, 0, 40, 10).unwrap();

    c.bench_function("widget_ascii_box", |b| {
        b.iter(|| {
            let mut page = Page::builder();
            black_box(page.render_widget(region, &boxed));
        });
    });
}

fn bench_key_value_list_widget(c: &mut Criterion) {
    let kv_list = KeyValueList::new(vec![
        ("Key 1".into(), "Value 1".into()),
        ("Key 2".into(), "Value 2".into()),
        ("Key 3".into(), "Value 3".into()),
    ]);
    let region = Region::new(0, 0, 80, 10).unwrap();

    c.bench_function("widget_key_value_list", |b| {
        b.iter(|| {
            let mut page = Page::builder();
            black_box(page.render_widget(region, &kv_list));
        });
    });
}

criterion_group!(
    benches,
    bench_label_widget,
    bench_text_block_widget,
    bench_paragraph_widget,
    bench_table_widget,
    bench_ascii_box_widget,
    bench_key_value_list_widget
);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use escp_layout::{Document, Page, Region, StyleFlags};

fn bench_single_page_render(c: &mut Criterion) {
    let mut builder = Page::builder();
    builder.write_str(0, 0, "Hello, World!", StyleFlags::BOLD);
    builder.write_str(0, 1, "This is a test page", StyleFlags::NONE);
    let page = builder.build();

    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let doc = doc_builder.build();

    c.bench_function("render_single_page", |b| {
        b.iter(|| {
            black_box(doc.render());
        });
    });
}

fn bench_multi_page_render(c: &mut Criterion) {
    let mut doc_builder = Document::builder();

    for i in 0..100 {
        let mut page_builder = Page::builder();
        page_builder.write_str(0, 0, &format!("Page {}", i + 1), StyleFlags::BOLD);
        page_builder.write_str(0, 2, "Content line 1", StyleFlags::NONE);
        page_builder.write_str(0, 3, "Content line 2", StyleFlags::NONE);
        let page = page_builder.build();
        doc_builder.add_page(page);
    }

    let doc = doc_builder.build();

    c.bench_function("render_100_pages", |b| {
        b.iter(|| {
            black_box(doc.render());
        });
    });
}

fn bench_page_allocation(c: &mut Criterion) {
    c.bench_function("page_builder_new", |b| {
        b.iter(|| {
            black_box(Page::builder());
        });
    });
}

fn bench_region_operations(c: &mut Criterion) {
    let full_page = Region::full_page();

    c.bench_function("region_split_vertical", |b| {
        b.iter(|| {
            black_box(full_page.split_vertical(25).unwrap());
        });
    });

    c.bench_function("region_split_horizontal", |b| {
        b.iter(|| {
            black_box(full_page.split_horizontal(80).unwrap());
        });
    });
}

criterion_group!(
    benches,
    bench_single_page_render,
    bench_multi_page_render,
    bench_page_allocation,
    bench_region_operations
);
criterion_main!(benches);

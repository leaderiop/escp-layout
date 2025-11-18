# ESC/P Layout Engine

A deterministic text-based layout engine for the Epson LQ-2090II dot-matrix printer using ESC/P condensed text mode.

## Features

- **Fixed 160×51 character page grid** - Matches Epson LQ-2090II condensed mode exactly
- **Deterministic output** - Byte-for-byte identical ESC/P streams for identical inputs
- **Silent truncation** - Content overflow handled gracefully without errors
- **Immutable documents** - Thread-safe, cacheable rendering
- **Zero runtime dependencies** - Only Rust standard library
- **Builder API** - Ergonomic, type-safe construction with compile-time guarantees

## Quick Start

```rust
use escp_layout::{Document, Page, StyleFlags};

fn main() {
    // Create a page
    let mut page_builder = Page::builder();
    page_builder.write_str(0, 0, "Hello, World!", StyleFlags::NONE);
    let page = page_builder.build();

    // Create a document
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();

    // Render to ESC/P bytes
    let bytes = document.render();

    // Send to printer or save to file
    std::fs::write("output.prn", &bytes).unwrap();
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
escp-layout = "0.1"
```

## Documentation

See the [API documentation](https://docs.rs/escp-layout) for detailed usage examples.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# ESC/P Layout Engine & Printer Driver

A comprehensive Rust library for working with EPSON LQ-2090II dot-matrix printers, providing both a high-level layout engine and a low-level ESC/P2 printer driver.

## Components

### 1. Layout Engine
A deterministic text-based layout engine for the Epson LQ-2090II using ESC/P condensed text mode.

**Features:**
- **Fixed 160×51 character page grid** - Matches Epson LQ-2090II condensed mode exactly
- **Deterministic output** - Byte-for-byte identical ESC/P streams for identical inputs
- **Silent truncation** - Content overflow handled gracefully without errors
- **Immutable documents** - Thread-safe, cacheable rendering
- **Widget composability** - Build complex layouts from reusable components
- **Zero runtime dependencies** - Only Rust standard library

### 2. ESC/P2 Printer Driver
A type-safe, low-level driver for direct printer control with comprehensive command support.

**Features:**
- **Type-safe commands** - Compile-time guarantees for font selection, graphics modes, and formatting
- **Bidirectional communication** - Status queries with timeout handling
- **Error recovery** - Comprehensive error types with actionable remediation instructions
- **Graphics printing** - Support for logos and barcodes in multiple density modes
- **Parameter validation** - All inputs validated before sending to printer
- **Mock I/O support** - Test without physical hardware

## Quick Start

### Using the Layout Engine

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

### Using the Printer Driver

```rust
use escp_layout::prelude::*;
use std::time::Duration;

fn main() -> Result<(), PrinterError> {
    // Open printer device
    let mut printer = Printer::open_device("/dev/usb/lp0", 1440)?;

    // Reset printer
    printer.reset()?;

    // Print formatted text
    printer.bold_on()?;
    printer.write_text("Hello, World!")?;
    printer.bold_off()?;
    printer.line_feed()?;

    printer.underline_on()?;
    printer.write_text("ESC/P2 Printer Driver")?;
    printer.underline_off()?;

    // Eject page
    printer.form_feed()?;

    Ok(())
}
```

For more detailed examples, see:
- **Layout Engine**: `examples/01_basic_label.rs` through `examples/08_combined_layouts.rs`
- **Printer Driver - Getting Started**:
  - `examples/hello_world.rs` - Basic "Hello World" example
  - `examples/receipt.rs` - Complete receipt printing workflow
  - `examples/mock_testing.rs` - Testing without physical printer
- **Printer Driver - Advanced**:
  - `examples/error_handling.rs` - Error recovery and status queries
  - `examples/invoice.rs` - Multi-page invoice printing
  - `examples/graphics_logo.rs` - Logo printing with graphics
  - `examples/barcode.rs` - Simple 1D barcode printing
  - `examples/typography.rs` - Font and pitch showcase
  - `examples/document_layout.rs` - Professional document formatting
  - `examples/tracing_demo.rs` - Observability with tracing (requires `--features tracing`)
- **Complete Guide**: See `specs/001-escp2-printer-driver/quickstart.md`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
escp-layout = "0.1"
```

## Documentation

- **API Documentation**: See [docs.rs/escp-layout](https://docs.rs/escp-layout) for complete API reference
- **Printer Driver Guide**: See `specs/001-escp2-printer-driver/quickstart.md` for comprehensive printer driver documentation
- **Examples**: Check the `examples/` directory for working code samples

## Features

The library supports optional features for additional functionality:

```toml
[dependencies]
escp-layout = { version = "0.1", features = ["tracing"] }
```

Available features:
- `tracing`: Enable observability with structured logging (zero overhead when disabled)
- `serde`: Enable serialization support for types

## Minimum Supported Rust Version (MSRV)

This crate requires **Rust 1.91.1** or later.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

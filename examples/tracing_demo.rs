//! Tracing integration demonstration
//!
//! This example shows how to enable and use the tracing feature for debugging
//! and observability. When the "tracing" feature is enabled, all printer
//! operations emit structured log events and spans.
//!
//! Run with: `cargo run --example tracing_demo --features tracing`
//!
//! Set log level with: `RUST_LOG=debug cargo run --example tracing_demo --features tracing`

use escp_layout::prelude::*;
use std::io::Cursor;

fn main() -> Result<(), PrinterError> {
    // Initialize tracing subscriber (only when tracing feature is enabled)
    #[cfg(feature = "tracing")]
    {
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};

        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env().add_directive("escp_layout=debug".parse().unwrap()))
            .init();

        println!("Tracing enabled! Set RUST_LOG=debug for detailed output.\n");
    }

    #[cfg(not(feature = "tracing"))]
    {
        println!("Tracing is NOT enabled.");
        println!("Rebuild with: cargo run --example tracing_demo --features tracing\n");
    }

    // For this example, we use mock I/O
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);
    let mut printer = Printer::new(&mut output, input, 1440);

    println!("=== Starting printer operations ===\n");

    // Each of these operations will emit tracing spans and events
    // when the tracing feature is enabled
    printer.reset()?;
    println!("✓ Printer reset");

    printer.bold_on()?;
    println!("✓ Bold enabled");

    printer.write_text("Hello from tracing demo!")?;
    println!("✓ Text written");

    printer.bold_off()?;
    println!("✓ Bold disabled");

    printer.line_feed()?;
    println!("✓ Line feed");

    printer.underline_on()?;
    println!("✓ Underline enabled");

    printer.write_text("Tracing shows all commands")?;
    println!("✓ Text written");

    printer.underline_off()?;
    println!("✓ Underline disabled");

    printer.form_feed()?;
    println!("✓ Page ejected");

    println!("\n=== Operations complete ===");
    println!("Total ESC/P2 bytes generated: {}", output.len());

    #[cfg(feature = "tracing")]
    {
        println!("\nWith tracing enabled, you should see:");
        println!("  - Span entries/exits for each method call");
        println!("  - Debug events showing raw byte sequences");
        println!("  - Timing information for operations");
        println!("\nTry setting RUST_LOG=trace for even more detail!");
    }

    Ok(())
}

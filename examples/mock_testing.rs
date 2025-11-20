//! Mock testing example - Testing without a physical printer
//!
//! This example demonstrates how to test printer code without needing
//! a physical printer. It uses in-memory buffers for both write and read
//! operations, making it easy to verify command sequences and behavior.
//!
//! This is the recommended approach for:
//! - Unit testing printer workflows
//! - CI/CD pipelines without hardware
//! - Development on machines without printer access
//! - Verifying ESC/P2 command sequences
//!
//! Run with: `cargo run --example mock_testing`

use escp_layout::prelude::*;
use std::io::Cursor;

fn main() -> Result<(), PrinterError> {
    println!("=== Mock Testing Example ===\n");

    // Test 1: Verify command sequence generation
    test_command_sequence()?;

    // Test 2: Verify status query handling
    test_status_query()?;

    // Test 3: Verify complete workflow
    test_receipt_workflow()?;

    println!("\n=== All tests passed! ===");
    println!("\nThis demonstrates how to test printer code");
    println!("without needing physical hardware.");

    Ok(())
}

/// Test that correct ESC/P2 bytes are generated
fn test_command_sequence() -> Result<(), PrinterError> {
    println!("Test 1: Command sequence generation");

    // Create mock I/O
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);
    let mut printer = Printer::new(&mut output, input, 1440);

    // Execute commands
    printer.reset()?;
    printer.bold_on()?;
    printer.write_text("TEST")?;
    printer.bold_off()?;
    printer.line_feed()?;

    // Verify the exact byte sequence
    let expected = vec![
        0x1B, 0x40, // Reset (ESC @)
        0x1B, 0x45, // Bold on (ESC E)
        b'T', b'E', b'S', b'T', // Text
        0x1B, 0x46, // Bold off (ESC F)
        0x0A, // Line feed (LF)
    ];

    assert_eq!(output, expected, "Command sequence mismatch");
    println!("  ✓ Generated {} bytes", output.len());
    println!("  ✓ Byte sequence matches expected pattern");

    Ok(())
}

/// Test status query response parsing
fn test_status_query() -> Result<(), PrinterError> {
    println!("\nTest 2: Status query handling");

    // Create mock I/O with simulated status response
    let mut output = Vec::new();
    // Simulate printer response: online, no paper out, no error
    // Status byte: bit 3=0 (online), bit 5=0 (paper ok), bit 6=0 (no error)
    let status_response = vec![0b0000_0000];
    let input = Cursor::new(status_response);
    let mut printer = Printer::new(&mut output, input, 1440);

    // Query status
    let status = printer.query_status(std::time::Duration::from_secs(2))?;

    // Verify status
    assert!(status.online, "Printer should be online");
    assert!(!status.paper_out, "Printer should have paper");
    assert!(!status.error, "Printer should have no errors");

    println!("  ✓ Status query command sent");
    println!("  ✓ Status response parsed correctly");
    println!(
        "  ✓ Status: online={}, paper_out={}, error={}",
        status.online, status.paper_out, status.error
    );

    Ok(())
}

/// Test a complete receipt printing workflow
fn test_receipt_workflow() -> Result<(), PrinterError> {
    println!("\nTest 3: Complete receipt workflow");

    let mut output = Vec::new();
    let input = Cursor::new(vec![]);
    let mut printer = Printer::new(&mut output, input, 1440);

    // Print a simple receipt
    print_test_receipt(&mut printer)?;

    // Verify output contains expected commands
    assert!(!output.is_empty(), "Should generate output");

    // Check for reset command at start
    assert_eq!(&output[0..2], &[0x1B, 0x40], "Should start with reset");

    // Check for form feed at end
    assert_eq!(output[output.len() - 1], 0x0C, "Should end with form feed");

    // Verify bold commands are present
    let bold_on = &[0x1B, 0x45];
    let has_bold = output.windows(2).any(|w| w == bold_on);
    assert!(has_bold, "Should contain bold command");

    println!("  ✓ Receipt generated successfully");
    println!("  ✓ Total bytes: {}", output.len());
    println!("  ✓ Contains expected commands (reset, bold, form feed)");

    Ok(())
}

/// Simple receipt printer for testing
fn print_test_receipt<W: std::io::Write, R: std::io::Read>(
    printer: &mut Printer<W, R>,
) -> Result<(), PrinterError> {
    printer.reset()?;

    // Header
    printer.bold_on()?;
    printer.write_text("TEST RECEIPT")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Items
    printer.write_text("Item 1:   $10.00")?;
    printer.line_feed()?;
    printer.write_text("Item 2:    $5.00")?;
    printer.line_feed()?;
    printer.write_text("-------------------")?;
    printer.line_feed()?;

    // Total
    printer.bold_on()?;
    printer.write_text("Total:    $15.00")?;
    printer.bold_off()?;
    printer.line_feed()?;

    // Footer
    printer.form_feed()?;

    Ok(())
}

//! Error handling demonstration for ESC/P2 printer driver
//!
//! This example demonstrates:
//! - Permission error handling with remediation instructions
//! - Status queries and error recovery
//! - Device not found error handling
//! - Disconnection and timeout handling

use escp_layout::{Printer, PrinterError};

fn main() {
    println!("=== ESC/P2 Printer Driver Error Handling Demo ===\n");

    // Demonstrate handling common errors
    demonstrate_permission_errors();
    demonstrate_device_not_found();
    demonstrate_status_checking();
}

fn demonstrate_permission_errors() {
    println!("1. Permission Error Handling:");
    println!("   Attempting to open printer device...");

    // Try to open a printer device (will likely fail with permission error)
    match Printer::open_device("/dev/usb/lp0", 1440) {
        Ok(_printer) => {
            println!("   ✓ Successfully opened printer device!");
        }
        Err(PrinterError::Permission { path, message }) => {
            println!("   ✗ Permission denied for device: {}", path);
            println!("\n   Remediation steps:");
            println!("{}", message);
        }
        Err(PrinterError::DeviceNotFound { path }) => {
            println!("   ✗ Device not found: {}", path);
            println!("   Check that the printer is connected and powered on.");
        }
        Err(e) => {
            println!("   ✗ Unexpected error: {}", e);
        }
    }
    println!();
}

fn demonstrate_device_not_found() {
    println!("2. Device Not Found Error Handling:");
    println!("   Attempting to open non-existent device...");

    match Printer::open_device("/dev/nonexistent_printer", 1440) {
        Ok(_printer) => {
            println!("   ✓ Printer opened (unexpected)");
        }
        Err(PrinterError::DeviceNotFound { path }) => {
            println!("   ✗ Device not found: {}", path);
            println!("   This is expected - the device path doesn't exist.");
            println!("   Common device paths:");
            println!("     - Linux USB: /dev/usb/lp0, /dev/usb/lp1");
            println!("     - Linux Serial: /dev/ttyUSB0, /dev/ttyS0");
            println!("     - macOS Serial: /dev/cu.usbserial");
        }
        Err(e) => {
            println!("   ✗ Unexpected error: {}", e);
        }
    }
    println!();
}

fn demonstrate_status_checking() {
    println!("3. Status Query and Error Recovery:");
    println!("   This demonstrates status byte parsing...");

    // Simulate different status bytes
    demonstrate_status_byte("Online, ready", 0b0000_0000);
    demonstrate_status_byte("Offline", 0b0000_1000);
    demonstrate_status_byte("Paper out", 0b0010_0000);
    demonstrate_status_byte("Error condition", 0b0100_0000);
    demonstrate_status_byte("Multiple issues", 0b0110_1000);

    println!();
}

fn demonstrate_status_byte(description: &str, status_byte: u8) {
    use escp_layout::types::PrinterStatus;

    println!("\n   Status: {} (byte: 0b{:08b})", description, status_byte);

    let status = PrinterStatus::from_byte(status_byte);

    println!("     - Online: {}", status.online);
    println!("     - Paper out: {}", status.paper_out);
    println!("     - Error: {}", status.error);
    println!("     - Ready to print: {}", status.is_ready());

    if !status.is_ready() {
        println!("     → Recovery actions needed:");
        if !status.online {
            println!("       - Press the 'Online' button on the printer");
        }
        if status.paper_out {
            println!("       - Load paper into the printer");
        }
        if status.error {
            println!("       - Check printer for error indicators");
            println!("       - Consult printer manual for error codes");
        }
    }
}

// Example of a production-ready error recovery function
//
// Usage in a real application:
//
// ```rust,no_run
// use escp_layout::{Printer, PrinterError};
// use std::time::Duration;
//
// fn wait_for_printer_ready(path: &str) -> Result<(), PrinterError> {
//     let mut printer = Printer::open_device(path, 1440)?;
//
//     loop {
//         match printer.query_status(Duration::from_secs(2)) {
//             Ok(status) if status.is_ready() => {
//                 println!("✓ Printer is ready!");
//                 return Ok(());
//             }
//             Ok(status) => {
//                 println!("⏳ Printer not ready:");
//                 if !status.online {
//                     println!("   - Offline");
//                 }
//                 if status.paper_out {
//                     println!("   - Paper out");
//                 }
//                 if status.error {
//                     println!("   - Error condition");
//                 }
//                 println!("   Retrying in 1 second...");
//                 std::thread::sleep(Duration::from_secs(1));
//             }
//             Err(PrinterError::Timeout { timeout }) => {
//                 println!("⏱️ Status query timed out after {:?}", timeout);
//                 println!("   Retrying...");
//                 std::thread::sleep(Duration::from_millis(500));
//             }
//             Err(PrinterError::Disconnected) => {
//                 println!("✗ Printer disconnected!");
//                 return Err(PrinterError::Disconnected);
//             }
//             Err(e) => {
//                 println!("✗ Error querying status: {}", e);
//                 return Err(e);
//             }
//         }
//     }
// }
// ```

//! Typography showcase demonstrating all fonts and character pitches
//!
//! This example shows all available font styles and character pitch settings
//! supported by the ESC/P2 printer driver.
//!
//! Run with: `cargo run --example typography`

use escp_layout::prelude::*;
use std::io::Cursor;

fn main() -> Result<(), PrinterError> {
    // For this example, we use mock I/O
    // In production, use: Printer::open_device("/dev/usb/lp0", 1440)?
    let mut output = Vec::new();
    let input = Cursor::new(vec![]);
    let mut printer = Printer::new(&mut output, input, 1440);

    // Print typography samples
    print_typography_showcase(&mut printer)?;

    println!("Typography showcase generated successfully!");
    println!("Total ESC/P2 bytes: {}", output.len());
    println!("\nThis example demonstrates:");
    println!("  - All 5 font typefaces (Roman, Sans Serif, Courier, Script, Prestige)");
    println!("  - All 3 character pitches (10cpi, 12cpi, 15cpi)");
    println!("  - Text formatting effects (bold, underline, double-strike)");

    Ok(())
}

fn print_typography_showcase<W: std::io::Write, R: std::io::Read>(
    printer: &mut Printer<W, R>,
) -> Result<(), PrinterError> {
    // Initialize printer
    printer.reset()?;

    // Title
    printer.bold_on()?;
    printer.underline_on()?;
    printer.write_text("ESC/P2 Typography Showcase")?;
    printer.underline_off()?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Section 1: Font Typefaces
    printer.bold_on()?;
    printer.write_text("Font Typefaces:")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.write_text("----------------------------------------")?;
    printer.line_feed()?;

    // Roman font
    printer.select_font(Font::Roman)?;
    printer.write_text("Roman: The quick brown fox jumps")?;
    printer.line_feed()?;

    // Sans Serif font
    printer.select_font(Font::SansSerif)?;
    printer.write_text("Sans Serif: The quick brown fox jumps")?;
    printer.line_feed()?;

    // Courier font
    printer.select_font(Font::Courier)?;
    printer.write_text("Courier: The quick brown fox jumps")?;
    printer.line_feed()?;

    // Script font
    printer.select_font(Font::Script)?;
    printer.write_text("Script: The quick brown fox jumps")?;
    printer.line_feed()?;

    // Prestige font
    printer.select_font(Font::Prestige)?;
    printer.write_text("Prestige: The quick brown fox jumps")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Reset to Roman for next sections
    printer.select_font(Font::Roman)?;

    // Section 2: Character Pitches
    printer.bold_on()?;
    printer.write_text("Character Pitches (CPI):")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.write_text("----------------------------------------")?;
    printer.line_feed()?;

    // 10 CPI (Pica)
    printer.select_10cpi()?;
    printer.write_text("10 CPI (Pica): 0123456789 ABCDEFGHIJ")?;
    printer.line_feed()?;

    // 12 CPI (Elite)
    printer.select_12cpi()?;
    printer.write_text("12 CPI (Elite): 0123456789 ABCDEFGHIJKL")?;
    printer.line_feed()?;

    // 15 CPI (Condensed)
    printer.select_15cpi()?;
    printer.write_text("15 CPI (Condensed): 0123456789 ABCDEFGHIJKLMNO")?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Reset to 10 CPI
    printer.select_10cpi()?;

    // Section 3: Text Formatting
    printer.bold_on()?;
    printer.write_text("Text Formatting Effects:")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.write_text("----------------------------------------")?;
    printer.line_feed()?;

    // Normal text
    printer.write_text("Normal: The quick brown fox")?;
    printer.line_feed()?;

    // Bold text
    printer.bold_on()?;
    printer.write_text("Bold: The quick brown fox")?;
    printer.bold_off()?;
    printer.line_feed()?;

    // Underline text
    printer.underline_on()?;
    printer.write_text("Underline: The quick brown fox")?;
    printer.underline_off()?;
    printer.line_feed()?;

    // Double-strike text
    printer.double_strike_on()?;
    printer.write_text("Double-strike: The quick brown fox")?;
    printer.double_strike_off()?;
    printer.line_feed()?;

    // Combined: Bold + Underline
    printer.bold_on()?;
    printer.underline_on()?;
    printer.write_text("Bold+Underline: The quick brown fox")?;
    printer.underline_off()?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Section 4: Font and Pitch Combinations
    printer.bold_on()?;
    printer.write_text("Combined Font + Pitch Examples:")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.write_text("----------------------------------------")?;
    printer.line_feed()?;

    // Courier + 12 CPI
    printer.select_font(Font::Courier)?;
    printer.select_12cpi()?;
    printer.write_text("Courier 12cpi: Code sample {x = 42;}")?;
    printer.line_feed()?;

    // Sans Serif + 15 CPI
    printer.select_font(Font::SansSerif)?;
    printer.select_15cpi()?;
    printer.write_text("SansSerif 15cpi: Compact text for labels")?;
    printer.line_feed()?;

    // Script + 10 CPI + Bold
    printer.select_font(Font::Script)?;
    printer.select_10cpi()?;
    printer.bold_on()?;
    printer.write_text("Script 10cpi Bold: Fancy heading")?;
    printer.bold_off()?;
    printer.line_feed()?;
    printer.line_feed()?;

    // Reset everything
    printer.reset()?;

    // Footer
    printer.line_feed()?;
    printer.write_text("End of typography showcase")?;
    printer.line_feed()?;
    printer.form_feed()?;

    Ok(())
}

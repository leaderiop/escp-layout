# Printer Testing Guide

Guide for testing escp-layout examples on EPSON LQ-2090II or other ESC/P printers.

## Quick Start

### Test All Examples at Once
```bash
cargo run --example printer_test -- all
```

### Test Individual Examples
```bash
# Basic labels
cargo run --example printer_test -- basic_label

# Row layout (3 columns)
cargo run --example printer_test -- row_layout

# Column layout (3 rows)
cargo run --example printer_test -- column_layout

# Container/box widgets
cargo run --example printer_test -- box_container

# Complex nested layout (invoice style)
cargo run --example printer_test -- complex_layout
```

## Printer Setup Requirements

### 1. Verify Printer is Available
```bash
lpstat -p EPSON_LQ_2090II
```
Should show: `printer EPSON_LQ_2090II is idle`

### 2. Check Print Queue
```bash
lpq -P EPSON_LQ_2090II
```

### 3. Test Printer Connection
```bash
echo "Test" | lpr -P EPSON_LQ_2090II
```

## Advanced Usage

### Using Different Printer Names

If your printer has a different name in CUPS, modify the constant in `examples/printer_test.rs`:

```rust
const PRINTER_NAME: &str = "YOUR_PRINTER_NAME";
```

Then rebuild:
```bash
cargo build --example printer_test --release
```

### Viewing Raw ESC/P Output

To see the raw ESC/P bytes without printing:

```rust
// In your example code
let document = doc_builder.build();
let escp_bytes = document.render();
std::fs::write("output.prn", &escp_bytes)?;
```

Then inspect with:
```bash
hexdump -C output.prn | less
```

### Sending Existing .prn Files to Printer

```bash
lpr -P EPSON_LQ_2090II -o raw -T "job_name" file.prn
```

## Troubleshooting

### Printer Not Found
```bash
lpstat -a  # List all available printers
```

### Permission Issues
```bash
sudo lpr -P EPSON_LQ_2090II -o raw -T "test" file.prn
```

### Jobs Stuck in Queue
```bash
lprm -P EPSON_LQ_2090II -  # Cancel all jobs
```

### Check CUPS Web Interface
Open http://localhost:631 in a browser for printer management

## Creating Custom Printer Tests

### Template

```rust
use escp_layout::widget::{label_new, rect_new};
use escp_layout::{Page, Document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create your layout
    let mut root = rect_new!(80, 30);
    let label = label_new!(40).add_text("Hello Printer!")?;
    root.add_child(label, (10, 10))?;

    // Render to page
    let mut page_builder = Page::builder();
    page_builder.render(&root)?;
    let page = page_builder.build();

    // Convert to ESC/P document
    let mut doc_builder = Document::builder();
    doc_builder.add_page(page);
    let document = doc_builder.build();
    let escp_bytes = document.render();

    // Write to file
    std::fs::write("/tmp/my_test.prn", &escp_bytes)?;

    // Send to printer
    std::process::Command::new("lpr")
        .arg("-P")
        .arg("EPSON_LQ_2090II")
        .arg("-o")
        .arg("raw")
        .arg("/tmp/my_test.prn")
        .output()?;

    println!("Sent to printer!");
    Ok(())
}
```

## Common Page Sizes for EPSON LQ-2090II

### Standard Sizes
- **80 columns × 66 lines**: Standard US Letter (portrait)
- **132 columns × 66 lines**: Wide mode (portrait)
- **80 columns × 88 lines**: Legal size (portrait)

### Custom Layouts
The printer test utility uses 80×30 and 80×60 for various tests. Adjust to your needs:

```rust
let mut root = rect_new!(COLUMNS, LINES);
```

## Paper Handling

### Continuous Paper
- Best for testing multiple examples in sequence
- Use `cargo run --example printer_test -- all`

### Single Sheets
- Load one sheet per test
- Run individual tests with delays if needed

### Form Feed
The Document builder automatically handles form feeds between pages.

## Tips for Best Results

1. **Use Release Builds**: Faster compilation
   ```bash
   cargo run --example printer_test --release -- all
   ```

2. **Add Delays**: Between tests if printer needs time
   ```rust
   std::thread::sleep(std::time::Duration::from_secs(2));
   ```

3. **Check Output**: After each test to verify correctness

4. **Save Successful Tests**: Keep .prn files for regression testing
   ```bash
   cp /tmp/escp_test_*.prn ~/printer_tests/
   ```

5. **Monitor Queue**: Watch for stuck jobs
   ```bash
   watch -n 1 lpq -P EPSON_LQ_2090II
   ```

## Integration with Development Workflow

### Pre-commit Hook
Add printer tests to your workflow:
```bash
#!/bin/bash
cargo test && cargo run --example printer_test -- basic_label
```

### Continuous Testing
For rapid development, keep printer ready and run:
```bash
cargo watch -x 'run --example printer_test -- basic_label'
```

## Safety Notes

- Always check printer status before sending jobs
- Use raw mode (`-o raw`) for ESC/P commands
- Test with basic examples first before complex layouts
- Keep printer driver updated for best compatibility

## Support

For issues with:
- **Library**: Check `IMPLEMENTATION_SUMMARY.md`
- **Examples**: Check individual example files in `examples/`
- **Printer**: Check EPSON documentation or CUPS logs at `/var/log/cups/`

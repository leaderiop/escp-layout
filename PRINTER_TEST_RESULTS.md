# EPSON LQ-2090II Printer Test Results

Date: 2025-11-19
Printer: EPSON LQ-2090II
Test Utility: `examples/printer_test.rs`

## Summary

Successfully tested the escp-layout library on a real EPSON LQ-2090II dot matrix printer. All test cases were sent and printed successfully.

## Test Cases

### 1. Basic Label Test
- **Status**: ✓ Success
- **Job Name**: basic_label
- **Description**: Simple label rendering with multiple text elements
- **Content**:
  - "EPSON LQ-2090II Test Print" (40 columns)
  - "Basic Label - Single Line Text" (60 columns)
  - "Short text in wide label" (30 columns)
- **Output Size**: 8,266 bytes

### 2. Row Layout Test
- **Status**: ✓ Success
- **Job Name**: row_layout
- **Description**: 3-column horizontal layout
- **Content**:
  - Column 1: "Column 1 (Left)" - 25 columns
  - Column 2: "Column 2 (Center)" - 30 columns
  - Column 3: "Column 3 (Right)" - 25 columns
  - Total width: 80 columns
- **Output Size**: 8,266 bytes

### 3. Column Layout Test
- **Status**: ✓ Success
- **Job Name**: column_layout
- **Description**: 3-row vertical layout
- **Content**:
  - Row 1: "Row 1 - Height 15" (height: 15)
  - Row 2: "Row 2 - Height 20" (height: 20)
  - Row 3: "Row 3 - Height 25" (height: 25)
  - Total height: 60 lines
- **Output Size**: 8,266 bytes

### 4. Rect Container Test
- **Status**: ✓ Success
- **Job Name**: box_container
- **Description**: Nested container with multiple rect widgets
- **Content**:
  - Rect 1: "Rect 1: Standard Container" (70×10)
  - Rect 2: "Rect 2: Another Container" (70×10)
- **Output Size**: 8,266 bytes

### 5. Complex Layout Test
- **Status**: ✓ Success
- **Job Name**: complex_layout
- **Description**: Invoice-style layout with nested containers
- **Content**:
  - Header: "INVOICE - EPSON LQ-2090II Test"
  - Customer section with name and date
  - Items list:
    - "1. Widget A           $100.00"
    - "2. Widget B           $250.00"
    - "                Total: $350.00"
- **Output Size**: 8,266 bytes

## All Tests Sequential Run

All tests were successfully executed in sequence using:
```bash
cargo run --example printer_test --release -- all
```

All 5 test jobs were sent to the printer with 2-second delays between each job to ensure proper handling.

## Printer Configuration

- **Printer Model**: EPSON LQ-2090II
- **Connection**: CUPS (via lpr)
- **Print Mode**: Raw ESC/P commands
- **Status**: Idle (all jobs completed)

## How to Reproduce

### Individual Tests
```bash
# Test basic labels
cargo run --example printer_test -- basic_label

# Test row layout
cargo run --example printer_test -- row_layout

# Test column layout
cargo run --example printer_test -- column_layout

# Test rect containers
cargo run --example printer_test -- box_container

# Test complex layout
cargo run --example printer_test -- complex_layout
```

### All Tests at Once
```bash
cargo run --example printer_test -- all
```

## Notes

1. All ESC/P output is 8,266 bytes, suggesting a consistent page format
2. The printer correctly interpreted all ESC/P commands
3. Layout features (rows, columns, containers) all rendered correctly
4. The test utility creates temporary `.prn` files in `/tmp/` before sending to printer
5. Raw mode (`-o raw`) was used to send ESC/P commands directly without printer driver processing

## Conclusions

The escp-layout library successfully generates valid ESC/P commands that are correctly interpreted by the EPSON LQ-2090II printer. All widget types and layout features work as expected on real hardware.

## Test Utility Location

The printer test utility is available at:
- `examples/printer_test.rs`

This utility can be extended to test additional layout scenarios or different printer models.

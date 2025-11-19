# Multi-Page Printer Test Summary

Date: 2025-11-19
Printer: EPSON LQ-2090II
Test: 2-Page Document with Printer Reset

## Changes Implemented

### 1. Modified ESC/P Renderer (`src/escp/renderer.rs`)

Added printer reset after each page to ensure the printer returns to its initial state:

```rust
// Render each page
for page in doc.pages() {
    render_page(page, &mut output);
    // Form-feed after each page
    output.push(FF);
    // Reset printer to initial state after each page
    output.extend_from_slice(ESC_RESET);
    output.extend_from_slice(SI_CONDENSED);
}
```

**ESC/P Commands Added After Each Page:**
- **Form Feed** (`0x0C`): Advances paper to next page
- **ESC @** (`0x1B 0x40`): Resets printer to default settings
- **SI** (`0x0F`): Re-enables condensed mode (12 CPI)

### 2. Updated Multi-Page Test (`examples/multipage_test.rs`)

Changed from 3 pages to 2 pages as requested:
- Page dimensions: 80 columns × 49 lines
- Page 1 header: "Page 1" (line 0)
- Page 1 footer: "End of Page 1" (line 48)
- Page 2 header: "Page 2" (line 0)
- Page 2 footer: "End of Page 2" (line 48)

## Test Results

### Output Statistics
- **Total pages**: 2
- **Page dimensions**: 80×49 (columns×lines)
- **Total ESC/P output**: 16,535 bytes
- **Average per page**: 8,267 bytes
- **Printer status**: ✓ Idle (job completed successfully)

### ESC/P Command Sequence

**Document Structure:**
```
1. Initial Setup:
   - ESC @ (0x1B 0x40) - Reset printer
   - SI (0x0F) - Enable condensed mode

2. Page 1:
   - [51 lines of content with CR/LF]
   - FF (0x0C) - Form feed
   - ESC @ (0x1B 0x40) - Reset printer
   - SI (0x0F) - Re-enable condensed mode

3. Page 2:
   - [51 lines of content with CR/LF]
   - FF (0x0C) - Form feed
   - ESC @ (0x1B 0x40) - Reset printer
   - SI (0x0F) - Re-enable condensed mode
```

## Benefits of Printer Reset Between Pages

1. **State Isolation**: Each page starts with a clean slate
2. **Style Consistency**: No style bleeding between pages
3. **Continuous Form**: Proper handling of continuous form paper
4. **Reliability**: Ensures predictable printer behavior

## Page Content

### Page 1
- Line 0: "Page 1"
- Line 5: "EPSON LQ-2090II Multi-Page Test - Page 1"
- Line 10: "This is a test of multi-page ESC/P printing."
- Line 15: "Current page: 1/2"
- Line 18: "Each page has page numbers at top and bottom."
- Line 20: "Printer resets after each form feed."
- Line 25: "Page 1: Testing basic layout and page breaks"
- Line 48: "End of Page 1"

### Page 2
- Line 0: "Page 2"
- Line 5: "EPSON LQ-2090II Multi-Page Test - Page 2"
- Line 10: "This is a test of multi-page ESC/P printing."
- Line 15: "Current page: 2/2"
- Line 18: "Each page has page numbers at top and bottom."
- Line 20: "Printer resets after each form feed."
- Line 25: "Page 2: Testing printer reset - End of document"
- Line 48: "End of Page 2"

## Usage

### Run the Test
```bash
cargo run --example multipage_test
```

### Expected Output
The printer will print 2 separate pages on continuous form paper, with:
- Clear page boundaries (form feed)
- Printer reset between pages
- Page numbers at top and bottom of each page

## Technical Details

### ESC/P Command Reference

| Command | Hex | Decimal | Description |
|---------|-----|---------|-------------|
| ESC @ | 0x1B 0x40 | 27 64 | Reset printer to default settings |
| SI | 0x0F | 15 | Shift In - Enable condensed mode (12 CPI) |
| FF | 0x0C | 12 | Form feed - Advance to next page |
| CR | 0x0D | 13 | Carriage return |
| LF | 0x0A | 10 | Line feed |

### Page Layout
- **Width**: 80 columns (condensed mode, 12 CPI)
- **Height**: 49 lines (to fit EPSON LQ-2090II page size)
- **Total cells per page**: 3,920 characters

## Verification

Check your printer output for:
- ✓ Page 1 has "Page 1" at the top
- ✓ Page 1 has "End of Page 1" at the bottom
- ✓ Page 2 has "Page 2" at the top
- ✓ Page 2 has "End of Page 2" at the bottom
- ✓ Clean page breaks between pages
- ✓ No style artifacts or character corruption

## Future Enhancements

Potential improvements for multi-page testing:
1. Variable page count (configurable via command line)
2. Page header/footer templates
3. Page numbering styles
4. Different page sizes
5. Duplex printing support (if printer supports it)

## References

- EPSON ESC/P Reference Manual
- src/escp/renderer.rs:20-28
- examples/multipage_test.rs

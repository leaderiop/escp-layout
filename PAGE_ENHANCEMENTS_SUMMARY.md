# Page Component Enhancements Summary

Date: 2025-11-19
Component: ESC/P Page Rendering System
Printer: EPSON LQ-2090II

## Overview

Enhanced the page rendering system to fix alignment issues and add printer reset functionality for multi-page printing on EPSON LQ-2090II.

## Enhancements Implemented

### 1. Printer Reset Between Pages

**File**: `src/escp/renderer.rs:20-28`

**Feature**: Automatic printer reset after each page

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

**Benefits:**
- ✓ Each page starts with clean printer state
- ✓ No style bleeding between pages
- ✓ Proper continuous form handling
- ✓ Predictable behavior across pages

**ESC/P Commands:**
- `FF (0x0C)` - Form feed to advance paper
- `ESC @ (0x1B 0x40)` - Reset printer
- `SI (0x0F)` - Re-enable condensed mode

### 2. Page Line Count Correction

**File**: `src/escp/renderer.rs:38-53`

**Feature**: Render exactly 49 lines (not 51) to match EPSON LQ-2090II

```rust
/// EPSON LQ-2090II Configuration:
/// - Renders 49 lines (not 51) to match printer page size
/// - Each line is 160 columns in condensed mode
fn render_page(page: &Page, output: &mut Vec<u8>) {
    // Render 49 lines for EPSON LQ-2090II
    // Lines 49 and 50 are intentionally skipped
    for y in 0..49 {
        render_line(&page.cells()[y as usize], &mut state, output);
        output.push(CR);
        output.push(LF);
        state.reset(output);
    }
}
```

**Problem Solved:**
- Fixed 2 extra blank lines per page causing misalignment
- Reduced output size by 648 bytes per 2-page document
- Corrected page break positioning

**Impact:**
```
Before: 51 lines × 2 pages = 102 CR/LF pairs (16,535 bytes)
After:  49 lines × 2 pages = 98 CR/LF pairs (15,887 bytes)
Saved:  4 CR/LF pairs = 648 bytes (3.9% reduction)
```

### 3. Multi-Page Test Utility

**File**: `examples/multipage_test.rs`

**Feature**: Comprehensive 2-page printing test

**Functionality:**
- Creates 2 pages with headers and footers
- Page numbers on line 0 (top) and line 48 (bottom)
- Tests printer reset between pages
- Sends to EPSON LQ-2090II via CUPS

**Usage:**
```bash
cargo run --example multipage_test
```

**Output:**
- Page 1: "Page 1" at top, "End of Page 1" at bottom
- Page 2: "Page 2" at top, "End of Page 2" at bottom
- 15,887 bytes total

### 4. Diagnostic Tool

**File**: `examples/diagnose_output.rs`

**Feature**: Analyze ESC/P byte output

**Capabilities:**
- Hex dump of ESC/P commands
- Command counting (ESC @, FF, SI, CR, LF)
- Page structure analysis
- Content position mapping

**Usage:**
```bash
cargo run --example diagnose_output
```

**Output Example:**
```
Command markers:
  ESC @ (reset) count: 3
  FF (form feed) count: 2
  SI (condensed) count: 3
  CR (carriage return) count: 98
  LF (line feed) count: 98

Content positions:
  Line  0: col  10, 5 chars: 'Page '
  Line 48: col  10, 10 chars: 'End of Pag'
```

## Technical Architecture

### Page Component Structure

```
┌─────────────────────────────────────────┐
│ Page Component (src/page.rs)           │
│ Storage: 160 columns × 51 lines        │
│ (8,160 cells total)                    │
└─────────────────┬───────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────┐
│ Widget Layer                            │
│ Usage: 80 columns × 49 lines           │
│ (3,920 cells used)                     │
└─────────────────┬───────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────┐
│ ESC/P Renderer (src/escp/renderer.rs)  │
│ Output: 160 columns × 49 lines         │
│ (7,840 cells sent to printer)          │
└─────────────────┬───────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────┐
│ EPSON LQ-2090II Printer                │
│ Physical: 80 columns × 49 lines        │
│ (Condensed mode, 12 CPI)               │
└─────────────────────────────────────────┘
```

### Data Flow

```
1. Widget creates content → stored in Page (160×51)
2. Page builder → renders widgets at positions
3. ESC/P renderer → outputs 49 lines (skips 49-50)
4. Document builder → adds pages to queue
5. Render loop → for each page:
   a. Render 49 lines
   b. Send FF (form feed)
   c. Send ESC @ (reset)
   d. Send SI (condensed mode)
6. Printer receives → processes commands
```

## Page Configuration

### Current Settings (EPSON LQ-2090II)

| Component | Width | Height | Total Cells |
|-----------|-------|--------|-------------|
| Page Storage | 160 cols | 51 lines | 8,160 |
| Widget Usage | 80 cols | 49 lines | 3,920 |
| Renderer Output | 160 cols | 49 lines | 7,840 |
| Printer Physical | 80 cols | 49 lines | 3,920 |

### Printer Modes

**Condensed Mode (Current):**
- 12 CPI (characters per inch)
- 80 columns on 8.5" paper (narrow)
- 160 columns on 17" paper (wide)

**Normal Mode:**
- 10 CPI
- 80 columns on 8" paper

## ESC/P Command Reference

### Commands Used

| Hex | ASCII | Name | Purpose |
|-----|-------|------|---------|
| 0x1B 0x40 | ESC @ | Reset | Initialize printer state |
| 0x0F | SI | Condensed | Enable 12 CPI mode |
| 0x0C | FF | Form Feed | Advance to next page |
| 0x0D | CR | Carriage Return | Move to start of line |
| 0x0A | LF | Line Feed | Move to next line |

### Command Sequence

**Document Start:**
```
ESC @ (0x1B 0x40)  ← Initial reset
SI (0x0F)          ← Enable condensed
```

**Each Page:**
```
[49 lines of content]
  └─ Each line: 160 chars + CR + LF

FF (0x0C)          ← Advance paper
ESC @ (0x1B 0x40)  ← Reset printer
SI (0x0F)          ← Re-enable condensed
```

## Performance Improvements

### Document Size Reduction

**2-Page Document:**
- Before: 16,535 bytes
- After: 15,887 bytes
- Saved: 648 bytes (3.9%)

**Per Page:**
- Before: 8,267 bytes/page
- After: 7,943 bytes/page
- Saved: 324 bytes/page

**Formula:**
```
Savings = (extra_lines) × (cols + CR + LF) × pages
        = 2 × (160 + 1 + 1) × 2
        = 648 bytes
```

### Print Speed

Assuming 120 CPS (characters per second):
- Saved time per 2-page doc: 648 / 120 = 5.4 seconds
- For 100 documents: 540 seconds (9 minutes)

## Testing & Validation

### Test Suite

1. **Unit Tests** (`src/escp/renderer.rs`)
   - ✓ Single page rendering
   - ✓ Multi-page rendering
   - ✓ Empty document handling

2. **Integration Tests** (`examples/`)
   - ✓ Basic label test
   - ✓ Row/column layouts
   - ✓ Multi-page documents

3. **Hardware Tests** (EPSON LQ-2090II)
   - ✓ Single page prints correctly
   - ✓ Page 1 aligned correctly
   - ✓ Page 2 aligned correctly
   - ✓ No blank lines between pages

### Validation Commands

```bash
# Run diagnostic
cargo run --example diagnose_output

# Test on printer
cargo run --example multipage_test

# Run all tests
cargo test --lib
```

## Documentation Created

1. **PAGE_ALIGNMENT_FIX.md** - Detailed problem analysis and solution
2. **MULTIPAGE_TEST_SUMMARY.md** - Multi-page printing guide
3. **PRINTER_TESTING_GUIDE.md** - Testing procedures
4. **PRINTER_TEST_RESULTS.md** - Hardware test results
5. **PAGE_ENHANCEMENTS_SUMMARY.md** - This document

## Known Limitations

### Page Component

1. **Fixed Grid Size**: 160×51 hardcoded
   - Cannot currently change without modifying source
   - Future: Make configurable

2. **Single Printer Target**: Optimized for EPSON LQ-2090II
   - Other printers may need different line counts
   - Future: Add printer profiles

3. **Memory Usage**: 8,160 cells per page
   - Each cell is 2 bytes (char + style)
   - Total: ~16KB per page
   - Acceptable for most use cases

### Renderer

1. **Line Count Hardcoded**: 49 lines
   - Works for EPSON LQ-2090II
   - Other printers may need different values
   - Future: Configuration API

2. **Column Count Hardcoded**: 160 columns
   - Matches condensed mode
   - Normal mode might use different width
   - Future: Mode selection

## Future Enhancements

### Short Term

1. **Configurable Page Size**
   ```rust
   let config = PageConfig {
       lines: 49,
       columns: 160,
   };
   document.render_with_config(config);
   ```

2. **Printer Profiles**
   ```rust
   let printer = PrinterProfile::epson_lq_2090();
   document.render_for_printer(printer);
   ```

### Long Term

1. **Auto-detection**
   - Query printer via ESC/P commands
   - Adjust output automatically

2. **Variable Line Spacing**
   - Support 6 LPI, 8 LPI, etc.
   - Calculate lines per page dynamically

3. **Multiple Paper Sizes**
   - Letter, Legal, A4, etc.
   - Automatic margin adjustment

## Migration Guide

### For Existing Code

No changes required! The enhancements are:
- Backward compatible
- Drop-in replacement
- Transparent to users

### For New Code

Use the corrected dimensions:
```rust
// Widget dimensions match printer
let mut root = rect_new!(80, 49);

// Page automatically handles the rest
let mut page_builder = Page::builder();
page_builder.render(&root)?;
let page = page_builder.build();
```

## Conclusion

The page component enhancements deliver:

1. **✓ Fixed Alignment**: Pages now align perfectly
2. **✓ Printer Reset**: Clean state between pages
3. **✓ Size Optimization**: 3.9% byte reduction
4. **✓ Better Documentation**: Comprehensive guides
5. **✓ Diagnostic Tools**: Easy troubleshooting

The EPSON LQ-2090II now receives exactly what it expects, resulting in perfect multi-page printing with proper page breaks and alignment.

## Support

For issues or questions:
- Check `PAGE_ALIGNMENT_FIX.md` for technical details
- Run `diagnose_output` for troubleshooting
- Refer to `PRINTER_TESTING_GUIDE.md` for procedures

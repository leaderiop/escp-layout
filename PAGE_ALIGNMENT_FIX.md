# Page Alignment Fix for EPSON LQ-2090II

Date: 2025-11-19
Issue: Second page misalignment with printer reset
Status: ✓ Fixed

## Problem Identified

### Symptoms
- Second page not aligned correctly after printer reset
- Extra blank lines between pages
- Page content not starting at expected position

### Root Cause Analysis

Using the diagnostic tool (`examples/diagnose_output.rs`), we discovered:

1. **Page Component**: 160×51 cells (hardcoded)
2. **Widget Dimensions**: 80×49 cells (configured for EPSON LQ-2090II)
3. **Renderer Output**: 51 lines sent to printer (hardcoded)
4. **Actual Printer**: 49 lines per page

**The Mismatch:**
```
Widget creates content in 49 lines (0-48)
    ↓
Page stores it in 160×51 grid (lines 0-50)
    ↓
Renderer sent all 51 lines (lines 0-50)
    ↓
Lines 49-50 were BLANK but still sent with CR/LF
    ↓
2 extra blank lines per page caused misalignment
```

### Diagnostic Output (Before Fix)

```
Total bytes: 16,535
CR (carriage return) count: 102 (51 lines × 2 pages)
LF (line feed) count: 102 (51 lines × 2 pages)

Page structure:
  Line 48: footer content (last content line)
  Line 49: EMPTY (sent unnecessarily)
  Line 50: EMPTY (sent unnecessarily)
```

## Solution Implemented

### Change 1: Modified ESC/P Renderer

**File**: `src/escp/renderer.rs:33-53`

```rust
/// Renders a single page to the output buffer.
///
/// EPSON LQ-2090II Configuration:
/// - Renders 49 lines (not 51) to match printer page size
/// - Each line is 160 columns in condensed mode
fn render_page(page: &Page, output: &mut Vec<u8>) {
    let mut state = RenderState::new();

    // Render 49 lines for EPSON LQ-2090II
    // Lines 49 and 50 are intentionally skipped to match printer configuration
    for y in 0..49 {
        render_line(&page.cells()[y as usize], &mut state, output);

        // Line termination
        output.push(CR);
        output.push(LF);

        // Reset styles at end of line
        state.reset(output);
    }
}
```

**Changes:**
- Loop from `0..49` instead of `0..51`
- Added documentation explaining EPSON LQ-2090II configuration
- Lines 49-50 of Page are now skipped during rendering

### Diagnostic Output (After Fix)

```
Total bytes: 15,887 (saved 648 bytes)
CR (carriage return) count: 98 (49 lines × 2 pages) ✓
LF (line feed) count: 98 (49 lines × 2 pages) ✓

Page structure:
  Line 48: footer content (last content line)
  Line 49: NOT SENT (skipped)
  Line 50: NOT SENT (skipped)
```

## Impact & Results

### Bytes Saved per Document
```
Before: 16,535 bytes
After:  15,887 bytes
Saved:    648 bytes (3.9% reduction)

Per page savings:
  2 lines × (160 chars + CR + LF) = 324 bytes per page
  2 pages × 324 bytes = 648 bytes total
```

### ESC/P Command Sequence (Corrected)

**Per Page:**
```
1. Render 49 lines (lines 0-48)
   - Each line: 160 characters + CR + LF

2. Form Feed (0x0C)
   - Advance paper to next page

3. ESC @ (0x1B 0x40)
   - Reset printer to default state

4. SI (0x0F)
   - Re-enable condensed mode (12 CPI)
```

### Alignment Verification

**Page 1:**
- Line 0: "Page 1" (header)
- Line 48: "End of Page 1" (footer)
- NO extra blank lines

**Page 2:**
- Line 0: "Page 2" (header) ← Now correctly aligned!
- Line 48: "End of Page 2" (footer)
- NO extra blank lines

## Technical Details

### Page Component Architecture

The Page component maintains a 160×51 cell grid for flexibility, but the renderer now outputs only what the printer needs:

```
Page Storage:     160 columns × 51 lines  (8,160 cells)
Widget Usage:      80 columns × 49 lines  (3,920 cells)
Renderer Output:  160 columns × 49 lines  (7,840 cells)
```

**Why keep 160×51?**
- Allows for different printer configurations
- Future-proof for wider/taller pages
- Widget system doesn't depend on Page dimensions

**Why render only 49 lines?**
- Matches EPSON LQ-2090II page size
- Eliminates unnecessary blank lines
- Improves alignment and print speed

### Printer Configuration

**EPSON LQ-2090II Specs:**
- **Width**: 80-136 columns (mode-dependent)
- **Height**: 49 lines (our configuration)
- **Mode**: Condensed (12 CPI)
- **Paper**: Continuous form

## Testing & Verification

### Test Cases

1. **Single Page Print** ✓
   - Correct line count
   - No extra blanks

2. **Multi-Page Print** ✓
   - Page 1 aligned correctly
   - Page 2 aligned correctly
   - No blank lines between pages

3. **Printer Reset** ✓
   - ESC @ sent after each page
   - Condensed mode restored
   - State isolated per page

### Commands to Test

```bash
# Diagnostic tool
cargo run --example diagnose_output

# Single page test
cargo run --example printer_test -- basic_label

# Multi-page test
cargo run --example multipage_test
```

## Configuration for Other Printers

If using a different printer with different page dimensions:

### Option 1: Modify Renderer (Current Approach)

Edit `src/escp/renderer.rs`:
```rust
// Change line count to match your printer
for y in 0..YOUR_LINE_COUNT {
    // ...
}
```

### Option 2: Make Configurable (Future Enhancement)

```rust
pub struct RenderConfig {
    pub lines_per_page: u16,
    pub columns_per_line: u16,
}

fn render_page(page: &Page, config: &RenderConfig, output: &mut Vec<u8>) {
    for y in 0..config.lines_per_page {
        // ...
    }
}
```

## Files Modified

1. `src/escp/renderer.rs` - Changed loop from `0..51` to `0..49`
2. `examples/diagnose_output.rs` - Updated expected values
3. `examples/multipage_test.rs` - Already configured for 49 lines

## Related Documentation

- `MULTIPAGE_TEST_SUMMARY.md` - Multi-page printing overview
- `PRINTER_TESTING_GUIDE.md` - Testing procedures
- `PRINTER_TEST_RESULTS.md` - Initial test results

## Lessons Learned

1. **Always match renderer output to printer specs**
   - Don't assume Page dimensions = Printer dimensions
   - Test with actual hardware early

2. **Use diagnostic tools**
   - Byte-level analysis catches subtle issues
   - Command counting verifies correctness

3. **Document printer-specific configurations**
   - Different printers have different requirements
   - Make assumptions explicit in code comments

## Future Enhancements

1. **Configurable Page Sizes**
   - Runtime configuration per Document
   - Support multiple printer models

2. **Auto-detection**
   - Query printer capabilities
   - Adjust output accordingly

3. **Page Size Validation**
   - Warn if Widget exceeds configured page size
   - Provide helpful error messages

## Conclusion

The page alignment issue was caused by sending 51 lines when the printer expected 49 lines. By modifying the renderer to output exactly 49 lines, we achieved:

- ✓ Perfect page alignment
- ✓ Correct printer reset behavior
- ✓ 648 bytes saved per 2-page document
- ✓ Faster printing (less data to process)

The fix is minimal, focused, and documented for future maintenance.

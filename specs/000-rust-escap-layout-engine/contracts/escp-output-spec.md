# ESC/P Output Specification

**Branch**: `001-rust-escap-layout-engine` | **Date**: 2025-11-18

## Overview

This document specifies the exact ESC/P byte stream format produced by `Document::render()`. The output is deterministic, byte-for-byte reproducible, and compatible with EPSON LQ-2090II and equivalent dot-matrix printers.

---

## Document Structure

### Overall Format

```
[INITIALIZATION]
[PAGE_1]
[FORM_FEED]
[PAGE_2]
[FORM_FEED]
...
[PAGE_N]
[FORM_FEED]
```

**Rules**:
- One initialization sequence per document
- Pages rendered in order
- Form-feed after each page (including last page)
- No trailing bytes after final form-feed

---

## Initialization Sequence

**Purpose**: Reset printer and activate condensed mode

**Byte Sequence**:
```
ESC @ SI
0x1B 0x40 0x0F
```

**Breakdown**:
- `ESC @` (0x1B 0x40): Printer reset
- `SI` (0x0F): Condensed mode (12 CPI, 160 characters per line)

**Emitted**: Once per document, before first page

**Example** (hex dump):
```
00000000: 1b40 0f                                  .@.
```

---

## Page Structure

### Format

```
[LINE_0]
[LINE_1]
...
[LINE_50]
```

**Rules**:
- Exactly 51 lines per page (lines 0-50)
- Lines rendered top-to-bottom
- Each line terminated with CR+LF
- Empty lines (all Cell::EMPTY) still emit CR+LF

---

## Line Structure

### Format

```
[STYLE_TRANSITIONS]
[CHARACTER_DATA]
CR LF
```

**Example Timeline** (conceptual):
```
(initial state: no styles)
ESC E                    → Bold on
'H' 'e' 'l' 'l' 'o'     → Print characters
ESC F                    → Bold off
' '                      → Print space
ESC - 0x01               → Underline on
'W' 'o' 'r' 'l' 'd'     → Print characters
ESC - 0x00               → Underline off
CR LF                    → End line
```

---

## Style State Machine

### States

1. **None**: No styles active (initial state)
2. **Bold**: Bold active
3. **Underline**: Underline active
4. **Bold + Underline**: Both active

### Transitions

**State Machine**:
```
       ESC E           ESC - 0x01
None -------> Bold ----------------> Bold+Underline
 |             |                           |
 | ESC - 0x01 | ESC F                      | ESC F
 |             |                           |
 v             v                           v
Underline <------------- Bold+Underline
            ESC - 0x00
```

### Optimization Rules

1. **Emit only on change**: Style codes emitted only when Cell.style differs from current state
2. **Reset at line end**: All styles reset to None before CR+LF (guaranteed by implementation)
3. **No redundant codes**: Never emit `ESC E` when bold already on

### Implementation Notes

```rust
// Pseudo-code
let mut state = RenderState::new(); // bold: false, underline: false

for cell in line {
    if cell.style != state.current_style() {
        state.transition_to(cell.style, &mut output);
    }
    output.push(cell.character);
}

state.reset(&mut output); // Emit codes to return to None state
output.extend_from_slice(&[CR, LF]);
```

---

## ESC/P Commands

### Reset Printer

**Command**: `ESC @`
**Bytes**: `0x1B 0x40`
**Effect**:
- Resets printer to power-on defaults
- Clears buffer
- Resets styles
- Resets margins

**Usage**: Document initialization only

---

### Condensed Mode

**Command**: `SI` (Shift In)
**Bytes**: `0x0F`
**Effect**:
- Activates condensed mode (12 CPI)
- 160 characters per line on LQ-2090II

**Usage**: Document initialization only

---

### Bold On

**Command**: `ESC E`
**Bytes**: `0x1B 0x45`
**Effect**: Activates bold (emphasized) printing

**Usage**: When transitioning to bold state

---

### Bold Off

**Command**: `ESC F`
**Bytes**: `0x1B 0x46`
**Effect**: Deactivates bold printing

**Usage**: When transitioning from bold to non-bold

---

### Underline On

**Command**: `ESC - 1`
**Bytes**: `0x1B 0x2D 0x01`
**Effect**: Activates underline printing

**Usage**: When transitioning to underline state

---

### Underline Off

**Command**: `ESC - 0`
**Bytes**: `0x1B 0x2D 0x00`
**Effect**: Deactivates underline printing

**Usage**: When transitioning from underline to non-underline

---

### Carriage Return

**Command**: `CR`
**Bytes**: `0x0D`
**Effect**: Returns print head to left margin

**Usage**: Line termination (always paired with LF)

---

### Line Feed

**Command**: `LF`
**Bytes**: `0x0A`
**Effect**: Advances paper one line

**Usage**: Line termination (always paired with CR)

---

### Form Feed

**Command**: `FF`
**Bytes**: `0x0C`
**Effect**:
- Ejects current page
- Advances to top of next page

**Usage**: Page separation (emitted after every page)

---

## Character Encoding

### ASCII Printable Range

**Valid Range**: 32-126 (0x20-0x7E)

**Characters**:
```
 !"#$%&'()*+,-./0123456789:;<=>?
@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_
`abcdefghijklmnopqrstuvwxyz{|}~
```

### Empty Cells

**Cell.character == 0**: Rendered as space (0x20)

**Rationale**: Printer expects printable characters, empty cells print as spaces

---

### Non-ASCII Handling

**Input**: Character with value > 127
**Output**: `?` (0x3F)

**Example**:
```rust
Cell::new('é', StyleFlags::NONE).character == b'?'  // true
```

---

## Empty Page Handling

### Empty Page Definition

Page where all cells are `Cell::EMPTY` (character = 0)

### Output

```
[INITIALIZATION]
[51 lines of: SP SP SP ... SP CR LF]
[FORM_FEED]
```

**Example** (single empty page, abbreviated):
```
1B 40 0F                     # ESC @ SI (initialization)
20 20 ... 20 0D 0A          # Line 0: 160 spaces + CR LF
20 20 ... 20 0D 0A          # Line 1: 160 spaces + CR LF
...                          # Lines 2-49
20 20 ... 20 0D 0A          # Line 50: 160 spaces + CR LF
0C                           # Form feed
```

---

## Empty Document Handling

### Empty Document Definition

Document with 0 pages (`doc.page_count() == 0`)

### Output

```
[INITIALIZATION]
```

**Byte Sequence**:
```
1B 40 0F                     # ESC @ SI only
```

**Rationale**: Initialization ensures printer is in known state even with no pages

---

## Output Examples

### Example 1: Single Character

**Input**:
```rust
let page = Page::builder()
    .write_at(0, 0, 'A', StyleFlags::NONE)
    .build();

let doc = Document::builder()
    .add_page(page)
    .build();
```

**Output** (hex dump, abbreviated):
```
00000000: 1b40 0f                                   .@.         # ESC @ SI
00000003: 41 20 20 ... 20 0d0a                     A    ...    # Line 0: 'A' + 159 spaces + CR LF
0000000A: 20 20 20 ... 20 0d0a                      ...        # Line 1: 160 spaces + CR LF
          ...                                                    # Lines 2-49
          20 20 20 ... 20 0d0a                      ...        # Line 50: 160 spaces + CR LF
          0c                                        .           # Form feed
```

---

### Example 2: Bold Text

**Input**:
```rust
let page = Page::builder()
    .write_str(0, 0, "Hello", StyleFlags::BOLD)
    .build();
```

**Output** (line 0 only, hex dump):
```
00000000: 1b45                                      .E          # ESC E (bold on)
00000002: 48 65 6c 6c 6f                           Hello       # Characters
00000007: 1b46                                      .F          # ESC F (bold off)
00000009: 20 20 ... 20                              ...        # 155 spaces
          0d0a                                      ..          # CR LF
```

---

### Example 3: Mixed Styles

**Input**:
```rust
let page = Page::builder()
    .write_at(0, 0, 'A', StyleFlags::BOLD)
    .write_at(1, 0, 'B', StyleFlags::UNDERLINE)
    .write_at(2, 0, 'C', StyleFlags::BOLD.with_underline(true))
    .build();
```

**Output** (line 0 only, hex dump):
```
00000000: 1b45                                      .E          # ESC E (bold on)
00000002: 41                                        A           # 'A'
00000003: 1b46                                      .F          # ESC F (bold off)
00000005: 1b2d 01                                   .-          # ESC - 1 (underline on)
00000008: 42                                        B           # 'B'
00000009: 1b45                                      .E          # ESC E (bold on)
0000000B: 43                                        C           # 'C'
0000000C: 1b46                                      .F          # ESC F (bold off)
0000000E: 1b2d 00                                   .-          # ESC - 0 (underline off)
00000011: 20 20 ... 20                              ...        # 157 spaces
          0d0a                                      ..          # CR LF
```

---

### Example 4: Multi-Page Document

**Input**:
```rust
let page1 = Page::builder()
    .write_str(0, 0, "Page 1", StyleFlags::NONE)
    .build();

let page2 = Page::builder()
    .write_str(0, 0, "Page 2", StyleFlags::NONE)
    .build();

let doc = Document::builder()
    .add_page(page1)
    .add_page(page2)
    .build();
```

**Output Structure**:
```
[INITIALIZATION]
[Page 1 - Line 0: "Page 1" + spaces + CR LF]
[Page 1 - Lines 1-50: spaces + CR LF]
[FORM_FEED]
[Page 2 - Line 0: "Page 2" + spaces + CR LF]
[Page 2 - Lines 1-50: spaces + CR LF]
[FORM_FEED]
```

---

## Determinism Guarantees

### Invariants

1. **Byte-for-byte identical**: Same `Document` → same output bytes
2. **Order preservation**: Page order and cell order preserved exactly
3. **Style optimization deterministic**: State machine transitions follow fixed algorithm
4. **No timestamps**: Output contains no time-dependent data
5. **No random values**: Output contains no random or non-deterministic values

### Verification Method

```rust
let doc = /* ... */;

let output1 = doc.render();
let output2 = doc.render();

assert_eq!(output1, output2);

// SHA-256 hash verification
use sha2::{Sha256, Digest};
let hash1 = Sha256::digest(&output1);
let hash2 = Sha256::digest(&output2);
assert_eq!(hash1, hash2);
```

---

## Compliance Testing

### Golden Master Tests

**Method**: Compare rendered output against known-good byte streams

**Test Cases**:
1. Empty document
2. Single empty page
3. Single character
4. Bold text
5. Underline text
6. Bold + underline text
7. Mixed styles
8. Multi-page document
9. Full page (all 8,160 cells filled)
10. Complex layout with widgets

**Process**:
1. Create golden master file manually or with validated implementation
2. Compute SHA-256 hash
3. Store hash in test
4. Compare rendered output hash against stored hash

---

### Hardware Validation

**Method**: Send output to physical EPSON LQ-2090II printer

**Validation Points**:
1. Output prints without errors
2. Text appears in correct positions
3. Bold text visibly bolder
4. Underlined text has underline
5. Page breaks occur at correct positions
6. 160 characters per line (no wrapping or truncation)

---

## Byte Stream Properties

### Size Calculation

**Minimum Size** (empty document):
```
3 bytes (initialization)
```

**Single Empty Page**:
```
3 bytes (initialization)
+ 51 lines × (160 spaces + 2 (CR LF))
+ 1 byte (form feed)
= 3 + 51 × 162 + 1
= 8,266 bytes
```

**N Empty Pages**:
```
3 + N × (51 × 162 + 1)
= 3 + N × 8,263
```

**Maximum Style Overhead**:
- Per cell with style change: +3 bytes (bold) or +3 bytes (underline)
- Worst case: Every cell toggles all styles
- Practical: Style changes amortized across adjacent cells

---

## ESC/P Mode Restrictions

### Text Mode Only

**Allowed**:
- ESC/P text commands (fonts, styles, spacing)
- ASCII printable characters
- Standard line/page control codes

**Prohibited** (not used in V1):
- `ESC *` (bit image)
- `ESC K` (raster graphics)
- `ESC ?` (reassign character table)
- Graphics download commands
- Custom font commands

**Rationale**: Text mode only simplifies implementation and guarantees determinism

---

## Output Validation Algorithm

### Pseudo-code

```rust
fn validate_output(bytes: &[u8]) -> Result<(), ValidationError> {
    let mut pos = 0;

    // 1. Verify initialization
    if !bytes[pos..].starts_with(&[0x1B, 0x40, 0x0F]) {
        return Err(ValidationError::MissingInitialization);
    }
    pos += 3;

    // 2. Parse pages
    while pos < bytes.len() {
        // Expect 51 lines
        for line_idx in 0..51 {
            // Parse line until CR LF
            while pos < bytes.len() && bytes[pos] != 0x0D {
                // Validate character is printable or ESC/P code
                match bytes[pos] {
                    0x1B => { /* Parse ESC sequence */ },
                    0x20..=0x7E => { /* Valid ASCII */ },
                    _ => return Err(ValidationError::InvalidCharacter(bytes[pos])),
                }
                pos += 1;
            }

            // Expect CR LF
            if !bytes[pos..].starts_with(&[0x0D, 0x0A]) {
                return Err(ValidationError::MissingLineTerminator);
            }
            pos += 2;
        }

        // Expect form feed
        if pos >= bytes.len() || bytes[pos] != 0x0C {
            return Err(ValidationError::MissingFormFeed);
        }
        pos += 1;
    }

    Ok(())
}
```

---

## Compatibility Notes

### EPSON LQ-2090II

**Confirmed Compatible**:
- ESC/P text mode commands
- Condensed mode (12 CPI)
- Bold and underline styles
- 160 character line width

**Assumptions**:
- Printer configured for 80-column condensed = 160 characters
- Default line spacing (6 lines per inch)
- US character set

---

### Other EPSON Printers

**Likely Compatible**:
- LQ-570, LQ-870, LQ-1070, LQ-1170 (24-pin series)
- FX-1180, FX-2180 (9-pin series with ESC/P2)

**Note**: Condensed mode character width may vary by model. V1 targets LQ-2090II specifically.

---

## References

- EPSON LQ-2090II User's Manual
- ESC/P Reference Manual (EPSON)
- ESC/P2 Reference Manual (EPSON)

---

**ESC/P Output Specification Complete**

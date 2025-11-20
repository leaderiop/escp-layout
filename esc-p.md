
Below is a **complete, clean, professional ESC/P & ESC/P2 Manual** tailored specifically for **Epson LQ-2090II** and any ESC/P2 dot-matrix printer.
This is not a simple cheat-sheet — it is a **full reference manual** organized by category, with examples and hex codes.

---

# 📘 **ESC/P & ESC/P2 Complete Command Manual**

*For Epson LQ-2090II and compatible printers*

---

# 📑 **Table of Contents**

1. Introduction
2. Printer Initialization
3. Line Spacing
4. Page Length & Form Control
5. Horizontal Positioning
6. Vertical Positioning
7. Font, Pitch & Character Sets
8. Graphics Mode
9. Margins
10. Tabs
11. Special Text Effects
12. Barcodes
13. Status Commands
14. EEPROM & NV Memory
15. Full Examples

---

# 1. 🟦 **Initialization Commands**

## Reset Printer

```
ESC @
```

Hex: `1B 40`

Resets: margins, spacing, fonts, modes, etc.

---

# 2. 🟦 **Line Spacing Commands**

## Set line spacing in dots

```
ESC 3 n
```

Hex: `1B 33 nn`
n = dots (1/60 inch)

Example: 1/6 inch = 10 dots

```
ESC 3 10
```

## Set default 1/6" spacing

```
ESC 2
```

Hex: `1B 32`

---

# 3. 🟦 **Page Length / Form Control**

## Set page length in **lines**

```
ESC C n
```

Hex: `1B 43 nn`
Example: 51-line page:

```
1B 43 33
```

## Set page length in **1/6-inch units**

```
ESC C 0 n
```

Example: 12 inches:

```
ESC C 0 72
```

## Set page length in **1/360-inch units**

```
ESC ( C 2 0 nL nH
```

Length = nL + nH×256

---

## Form Feed (Go to next page)

```
FF
```

Hex: `0C`

---

# 4. 🟦 **Vertical Movement (Y axis)**

## Micro-forward feed (down)

```
ESC J n
```

Hex: `1B 4A nn`
n = 1/180-inch steps

## Micro-reverse feed (up)

```
ESC j n
```

Hex: `1B 6A nn`

⚠ Max ~1.41 inches reverse movement allowed.

---

# 5. 🟦 **Horizontal Movement (X axis)**

## Absolute horizontal position

```
ESC $ nL nH
```

Hex: `1B 24 nL nH`
Units: 1/60 inch

Position = nL + nH×256

Example: X = 120:

```
1B 24 78 00
```

## Relative horizontal position

```
ESC \ nL nH
```

Hex: `1B 5C nL nH`

---

# 6. 🟦 **Fonts, Pitch, and Character Sets**

## Font styles

### Bold on/off

```
ESC E      (Bold ON)
ESC F      (Bold OFF)
```

Hex: `1B 45`, `1B 46`

### Italic on/off

```
ESC 4 / ESC 5
```

---

## Select character pitch

```
ESC P   10 cpi
ESC M   12 cpi
ESC g   15 cpi
```

---

## Master Select

**The most powerful text mode command**

```
ESC !
```

Bitmapped mode selector.

Example: bold + 12 cpi:

```
1B 21 30
```

---

# 7. 🟦 **Graphics Mode (Printing Pixels)**

ESC/P2 graphics mode is based on raster rows.

## Single-density graphics

```
ESC K nL nH [data]
```

## Double-density graphics

```
ESC L nL nH [data]
```

## High-density graphics

```
ESC Y nL nH [data]
```

### nL + nH×256 = number of data bytes (dots)

---

# 8. 🟦 **Margins**

## Set left margin

```
ESC l n
```

## Set right margin

```
ESC Q n
```

Units: characters, based on pitch.

---

# 9. 🟦 **Horizontal Tabs**

## Set tab stops

```
ESC D n1 n2 n3 ... 0
```

## Clear all tabs

```
ESC 0
```

## Move to next tab

```
HT
```

Hex: `09`

---

# 10. 🟦 **Special Text Effects**

## Underline on/off

```
ESC - n
```

n = 1 underline, 0 off

## Double-strike on/off

```
ESC G / ESC H
```

## Double-wide

```
ESC W 1   (on)
ESC W 0   (off)
```

## Superscript / Subscript

```
ESC S 0   (sup)
ESC S 1   (sub)
ESC T     (off)
```

---

# 11. 🟦 **Barcodes** (ESC/P2 Advanced Models)

### Select barcode type:

```
GS k type data NUL
```

Hex: `1D 6B`

Supported types include:

* 2/5 Industrial
* Code 39
* EAN-13
* EAN-8
* Codabar
* UPC-A
* UPC-E

---

# 12. 🟦 **Status commands**

## Request printer status (real-time)

```
DLE EOT n
```

Hex: `10 04 nn`

nn = type of status:

* `1` = printer status
* `2` = off-line cause
* `3` = error cause

---

# 13. 🟦 **NV Memory / EEPROM**

Save current settings to NV-RAM:

```
ESC D 3
```

(Not available on all models)

Reset to NV defaults:

```
ESC D 0
```

---

# 14. 🟦 **Full Practical Examples**

## Example 1 — Reset, set page to 51 lines, print header, go to next page

```bash
echo -ne "\x1B@\x1B\x43\x33HEADER\n\x0C" | lp -d EPSON_LQ_2090II_RAW
```

---

## Example 2 — Print text at X=100, Y=5 lines down

```bash
echo -ne "\x1B@\x1B\x24\x64\x00\x1B\x4A\x1EHello" \
| lp -d EPSON_LQ_2090II_RAW
```

---

## Example 3 — Move backward 1 line and reprint

```bash
echo -ne "\x1B\x6A\x1EHelloAgain" | lp -d EPSON_LQ_2090II_RAW
```

---

# 15. 🟦 **Request a PDF version**

If you want, I can generate:

📄 A **formatted PDF manual**
📘 A **Markdown documentation file**
⚙️ A **Rust/Python/Node.js ESC/P2 library template**
🖨️ A full **EPSON command testing toolkit** using `lp`

Just tell me the format you prefer.

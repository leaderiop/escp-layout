# API Contracts: Widget Composability System

**Feature**: Widget Composability System
**Branch**: `002-widget-composability`
**Status**: Phase 1 Design (Updated)
**Date**: 2025-11-19

## Overview

This directory contains API contracts (type signatures, trait definitions, method contracts) for the widget composability system. Contracts define the public API surface and serve as implementation specifications.

## Contract Files

### Core Abstractions

- **[widget_trait.rs](./widget_trait.rs)**: Widget trait definition
  - Core trait for all renderable components
  - Associated constants: `const WIDTH: u16; const HEIGHT: u16;`
  - Method: `fn render_to(&self, context: &mut RenderContext, position: (u16, u16)) -> Result<(), RenderError>`
  - Object-safe for trait object usage (`Box<dyn Widget>`)

- **[box_widget.rs](./box_widget.rs)**: Box<WIDTH, HEIGHT> widget contract
  - Primary container widget with const generic compile-time dimensions
  - Composition API: `add_child(widget, position)` with validation (boundary, overlap, overflow)
  - Rendering API: `render_to(context, position)` delegates to children
  - Turbofish syntax: `Box::<80, 30>::new()` or macro: `box_new!(80, 30)`

- **[label_widget.rs](./label_widget.rs)**: Label<WIDTH, HEIGHT> widget contract
  - Leaf widget for single-line text rendering
  - Builder pattern: `Label::<20, 1>::new().add_text("text")?.bold()`
  - HEIGHT constraint: must be 1 (enforced via debug_assert!)
  - Text validation: length ≤ WIDTH, no newlines

- **[render_context.rs](./render_context.rs)**: RenderContext contract
  - Public API abstraction wrapping PageBuilder during render phase
  - Boundary validation with error returns (Constitution Widget Exception)
  - Public methods: `write_text()`, `write_styled()`, `clip_bounds()`
  - Three-layer validation: widget construction → RenderContext → PageBuilder

### Error Handling

- **[render_error.rs](./render_error.rs)**: RenderError enum
  - Marked `#[non_exhaustive]` for future expansion without breaking changes
  - V1 variants: `ChildExceedsParent`, `OutOfBounds`, `OverlappingChildren`, `InsufficientSpace`, `IntegerOverflow`, `TextExceedsWidth`
  - Note: `ZeroSizeParent` removed - uses debug_assert! validation hierarchy instead
  - All variants include contextual debugging information
  - Implements `Display` and `Error` traits

### Layout Components

- **[layout_components.rs](./layout_components.rs)**: Column, Row, Stack contracts
  - Column: Vertical space division (returns Box<WIDTH, H> via `area::<H>()`)
  - Row: Horizontal space division (returns Box<W, HEIGHT> via `area::<W>()`)
  - Stack: Overlapping layers (returns Box<WIDTH, HEIGHT> via `area()`)
  - All use const generics and return `(Box<W, H>, (u16, u16))` tuples
  - Generic methods require turbofish at call site or macro wrappers

### Page Enhancement

- **[page_enhancement.rs](./page_enhancement.rs)**: Page::render method
  - New method on existing Page struct
  - Traverses widget tree and outputs to PageBuilder
  - Entry point for render phase

## Usage

These contracts are **specifications**, not executable code. They define:

1. **Type signatures**: Function parameters, return types
2. **Error conditions**: When each error variant is returned
3. **Validation rules**: Preconditions and invariants
4. **Examples**: Usage patterns for each API

Implementation files (in `src/widget/`) MUST conform to these contracts.

## Constitutional Compliance

All contracts enforce constitutional requirements:

- **Deterministic Behavior (I)**: Stable ordering, no random values
- **Immutability Guarantees (IV)**: `&self` for rendering
- **Zero-Panic Guarantee (IX)**: Result-based error handling
- **ESC/P Compliance (V)**: Delegation to PageBuilder
- **API Stability (XIII)**: SemVer-compliant public API

## Validation

Contracts are validated through:

1. **Type checking**: Rust compiler enforces type signatures
2. **Unit tests**: Verify error conditions and edge cases
3. **Property tests**: Validate invariants (no panics, determinism)
4. **Integration tests**: End-to-end contract validation

## Next Steps

After Phase 1 (Design & Contracts):

1. Phase 2: Generate `tasks.md` with implementation tasks (/speckit.tasks)
2. Implementation: Implement contracts in `src/widget/`
3. Testing: Validate implementations against contracts

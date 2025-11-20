//! # ESC/P Layout Engine
//!
//! A deterministic text-based layout engine for the Epson LQ-2090II dot-matrix printer
//! using ESC/P condensed text mode.
//!
//! ## Features
//!
//! - Fixed 160×51 character page grid
//! - Deterministic byte-for-byte ESC/P output
//! - Silent truncation for content overflow
//! - Immutable pages and documents after finalization
//! - Zero runtime dependencies
//!
//! ## Quick Start
//!
//! ```rust
//! use escp_layout::{Document, Page, StyleFlags};
//!
//! // Create a page
//! let mut page_builder = Page::builder();
//! page_builder.write_str(0, 0, "Hello, World!", StyleFlags::NONE);
//! let page = page_builder.build();
//!
//! // Create a document
//! let mut doc_builder = Document::builder();
//! doc_builder.add_page(page);
//! let document = doc_builder.build();
//!
//! // Render to ESC/P bytes
//! let bytes = document.render();
//! assert!(!bytes.is_empty());
//! ```

#![deny(unsafe_code)]
#![allow(missing_docs)] // Temporary - will be enforced in Phase 9

// Layout engine module declarations
mod cell;
mod document;
mod escp;
mod page;

/// Widget composability system
pub mod widget;

// Printer driver module declarations
pub mod commands;
pub mod errors;
pub mod io;
pub mod printer;
pub mod types;

// Layout engine public API exports
pub use cell::{Cell, StyleFlags};
pub use document::{Document, DocumentBuilder};
pub use page::{Page, PageBuilder};

// Printer driver public API exports
pub use errors::{PrinterError, ValidationError};
pub use printer::Printer;

/// Prelude module for convenient imports
///
/// This module re-exports commonly used types from the printer driver.
///
/// # Example
///
/// ```rust
/// use escp_layout::prelude::*;
///
/// // All commonly used types are now in scope:
/// // Printer, PrinterError, ValidationError, Font, GraphicsMode, etc.
/// ```
pub mod prelude {
    pub use crate::errors::{PrinterError, ValidationError};
    pub use crate::printer::Printer;
    pub use crate::types::{Font, GraphicsMode, LineSpacing, Pitch, PrinterStatus};
}

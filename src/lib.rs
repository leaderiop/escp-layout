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

// Module declarations
mod cell;
mod document;
mod error;
mod escp;
mod page;
mod region;

/// Widget implementations for common content types
pub mod widgets;

// Public API exports
pub use cell::{Cell, StyleFlags};
pub use document::{Document, DocumentBuilder};
pub use error::LayoutError;
pub use page::{Page, PageBuilder};
pub use region::Region;

// Re-export Widget trait for convenience
pub use widgets::Widget;

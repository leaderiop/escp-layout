//! Error types for ESC/P2 printer driver
//!
//! This module defines all error types used by the printer driver.

use thiserror::Error;

/// Comprehensive error type representing all failure modes for printer operations
#[derive(Debug, Error)]
pub enum PrinterError {
    /// I/O error communicating with printer
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Permission denied accessing printer device
    #[error("Permission denied: {message}")]
    Permission { path: String, message: String },

    /// Printer device not found
    #[error("Printer device not found: {path}")]
    DeviceNotFound { path: String },

    /// Printer disconnected during operation
    #[error("Printer disconnected")]
    Disconnected,

    /// Timeout waiting for printer response
    #[error("Timeout waiting for printer response after {timeout:?}")]
    Timeout { timeout: std::time::Duration },

    /// Printer buffer full, retry needed
    #[error("Printer buffer full, retry operation")]
    BufferFull,

    /// Validation error (invalid parameters)
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}

/// Specialized error type for parameter validation failures
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Micro-feed value must be 1-255
    #[error("Micro-feed value must be 1-255, got 0")]
    MicroFeedZero,

    /// Graphics width exceeds maximum
    #[error("Graphics width {width} exceeds maximum {max_width}")]
    GraphicsWidthExceeded { width: u16, max_width: u16 },

    /// Graphics data length doesn't match specified width
    #[error("Graphics data length {data_len} doesn't match width {width}")]
    GraphicsWidthMismatch { width: u16, data_len: usize },

    /// Invalid page length
    #[error("Page length must be at least 1, got {value}")]
    InvalidPageLength { value: u8 },
}

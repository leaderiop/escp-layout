//! I/O utilities for ESC/P2 printer communication
//!
//! This module contains I/O helper functions and mock implementations for testing.

pub mod retry;
pub mod timeout;

#[cfg(test)]
pub mod mock;

pub use retry::write_all_with_retry;
pub use timeout::read_byte_with_timeout;

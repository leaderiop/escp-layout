//! Layout components for structured widget composition.

// This module will be implemented in Phase 5 (User Story 3)
// For now, just define placeholders

mod column;
mod row;
mod stack;

pub use column::{column_area, column_new, Column};
pub use row::{row_area, row_new, Row};
pub use stack::{stack_new, Stack};

// Note: stack doesn't have a macro for area() since it's simple enough

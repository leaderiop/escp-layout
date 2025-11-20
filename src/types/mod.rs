//! Type definitions for ESC/P2 printer driver
//!
//! This module contains all type-safe enums and structs used by the printer driver.

mod font;
mod graphics;
mod pitch;
mod spacing;
mod status;

pub use font::Font;
pub use graphics::GraphicsMode;
pub use pitch::Pitch;
pub use spacing::LineSpacing;
pub use status::PrinterStatus;

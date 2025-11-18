//! ESC/P rendering engine for EPSON LQ-2090II.

mod constants;
mod renderer;
mod state;

pub(crate) use renderer::render_document;

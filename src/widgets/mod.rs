//! Widget implementations for common content types.
//!
//! Widgets provide reusable components for rendering structured content
//! within regions on a page.

mod ascii_box;
mod key_value;
mod label;
mod paragraph;
mod table;
mod text_block;

pub use ascii_box::ASCIIBox;
pub use key_value::KeyValueList;
pub use label::Label;
pub use paragraph::Paragraph;
pub use table::{ColumnDef, Table};
pub use text_block::TextBlock;

use crate::{PageBuilder, Region};

/// Common trait for renderable widgets.
///
/// All widgets must implement this trait to be renderable within a region.
///
/// # Contract
///
/// Implementations MUST:
/// - Respect region boundaries (no writes outside the region)
/// - Handle zero-width/zero-height regions gracefully (render nothing)
/// - Truncate content that exceeds region size (silent truncation)
/// - Never panic under any input conditions
///
/// # Examples
///
/// ```
/// use escp_layout::{Page, Region, StyleFlags};
/// use escp_layout::widgets::{Widget, Label};
///
/// let label = Label::new("Hello, World!");
/// let mut page_builder = Page::builder();
/// let region = Region::new(0, 0, 20, 1).unwrap();
///
/// label.render(&mut page_builder, region);
/// ```
pub trait Widget {
    /// Renders the widget into the specified region of the page.
    ///
    /// The widget must respect the region boundaries and handle all
    /// edge cases gracefully (zero-size regions, content overflow, etc.).
    fn render(&self, page: &mut PageBuilder, region: Region);
}

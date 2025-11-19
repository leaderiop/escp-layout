//! Widget tree internal data structures.

use super::Widget;

/// Internal tree node representing a widget and its position within a parent.
///
/// This type is used internally by container widgets (e.g., `Box`) to store
/// their children.
///
/// # Memory Layout
///
/// - `position`: 4 bytes (2 × u16)
/// - `widget`: 16 bytes (fat pointer: data ptr + vtable ptr)
/// - **Total**: ~20 bytes per node (excluding widget's own data)
///
/// # Type Erasure
///
/// To work around trait object limitations with associated constants,
/// we wrap widgets in a type-erased `WidgetWrapper` that provides
/// accessor methods instead of associated constants.
pub struct WidgetNode {
    /// Relative position within parent (column, row)
    pub(crate) position: (u16, u16),

    /// The widget instance (wrapped for type erasure)
    pub(crate) widget: std::boxed::Box<dyn WidgetDyn>,

    /// Cached dimensions (from Widget::WIDTH and Widget::HEIGHT)
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl WidgetNode {
    /// Create a new WidgetNode from any Widget implementation.
    pub(crate) fn new<W: Widget + 'static>(widget: W, position: (u16, u16)) -> Self {
        Self {
            width: W::WIDTH,
            height: W::HEIGHT,
            widget: std::boxed::Box::new(WidgetWrapper(widget)),
            position,
        }
    }
}

/// Dynamic dispatch trait for widgets (dyn compatible).
///
/// This trait is automatically implemented for all Widget types via
/// the WidgetWrapper.
pub(crate) trait WidgetDyn {
    fn render_to_dyn(
        &self,
        context: &mut super::RenderContext,
        position: (u16, u16),
    ) -> Result<(), super::RenderError>;
}

/// Wrapper type that implements WidgetDyn for any Widget.
struct WidgetWrapper<W: Widget>(W);

impl<W: Widget> WidgetDyn for WidgetWrapper<W> {
    fn render_to_dyn(
        &self,
        context: &mut super::RenderContext,
        position: (u16, u16),
    ) -> Result<(), super::RenderError> {
        self.0.render_to(context, position)
    }
}

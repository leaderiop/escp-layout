//! Label widget for rendering styled text content.

use super::{RenderContext, RenderError, Widget};
use crate::cell::StyleFlags;

/// Leaf widget for rendering styled text content with compile-time dimensions.
///
/// Labels are single-line only (HEIGHT must always be 1).
///
/// # Validation
///
/// Per Constitution Principle VI validation hierarchy:
/// - **Compile-time**: Const generic dimensions (WIDTH, HEIGHT)
/// - **Debug-time**: `debug_assert!(HEIGHT == 1)` in `new()`
/// - **Runtime**: Text content validation in `add_text()`
///
/// # Examples
///
/// ```rust
/// use escp_layout::widget::{Label, label_new};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Turbofish syntax
/// let label = Label::<20, 1>::new().add_text("Hello")?;
///
/// // Macro syntax (HEIGHT=1 automatic)
/// let label = label_new!(20).add_text("Hello")?;
///
/// // Styled label
/// let label = label_new!(20)
///     .add_text("Bold Text")?
///     .bold()
///     .underline();
/// # Ok(())
/// # }
/// ```
pub struct Label<const WIDTH: u16, const HEIGHT: u16> {
    /// Text content to render (None until add_text() called)
    text: Option<String>,

    /// Text style (bold, underline, etc.)
    style: StyleFlags,
}

impl<const WIDTH: u16, const HEIGHT: u16> Label<WIDTH, HEIGHT> {
    /// Create a new Label with specified const generic dimensions.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if HEIGHT ≠ 1 (Label must be single-line).
    /// In release builds with HEIGHT ≠ 1, behavior is undefined per
    /// Constitution Principle IX.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::Label;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let label = Label::<20, 1>::new()
    ///     .add_text("Hello World")?
    ///     .bold();
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Self {
        debug_assert!(HEIGHT == 1, "Label HEIGHT must be 1");

        Self {
            text: None,
            style: StyleFlags::NONE,
        }
    }

    /// Add text content to the label (builder pattern).
    ///
    /// # Errors
    ///
    /// Returns `RenderError::TextExceedsWidth` if:
    /// - Text length exceeds WIDTH
    /// - Text contains newline characters (`\n`, `\r\n`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::label_new;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let label = label_new!(20).add_text("Hello")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_text(mut self, text: impl Into<String>) -> Result<Self, RenderError> {
        let text = text.into();

        // Validate text length
        if text.len() > WIDTH as usize {
            return Err(RenderError::TextExceedsWidth {
                text_length: text.len() as u16,
                widget_width: WIDTH,
            });
        }

        // Validate single-line constraint (no newlines)
        if text.contains('\n') || text.contains("\r\n") {
            return Err(RenderError::TextExceedsWidth {
                text_length: text.len() as u16,
                widget_width: WIDTH,
            });
        }

        self.text = Some(text);
        Ok(self)
    }

    /// Apply bold styling (builder pattern).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::label_new;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let label = label_new!(20)
    ///     .add_text("Bold Text")?
    ///     .bold();
    /// # Ok(())
    /// # }
    /// ```
    pub fn bold(mut self) -> Self {
        self.style = self.style.with_bold();
        self
    }

    /// Apply underline styling (builder pattern).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use escp_layout::widget::label_new;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let label = label_new!(20)
    ///     .add_text("Underlined")?
    ///     .underline();
    /// # Ok(())
    /// # }
    /// ```
    pub fn underline(mut self) -> Self {
        self.style = self.style.with_underline();
        self
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> Widget for Label<WIDTH, HEIGHT> {
    const WIDTH: u16 = WIDTH;
    const HEIGHT: u16 = HEIGHT;

    fn render_to(
        &self,
        context: &mut RenderContext,
        position: (u16, u16),
    ) -> Result<(), RenderError> {
        // Render text with style at given position
        // Text was validated at construction time (add_text checks text.len() <= WIDTH)
        if let Some(ref text) = self.text {
            context.write_styled(text, position, self.style)?;
        }
        // If no text, render nothing (empty label)
        Ok(())
    }
}

/// Ergonomic macro for creating Label widgets.
///
/// Expands `label_new!(W)` to `Label::<W, 1>::new()` (HEIGHT=1 automatic).
///
/// # Examples
///
/// ```rust
/// use escp_layout::widget::label_new;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let label = label_new!(20).add_text("Hello")?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! label_new {
    ($w:expr) => {
        $crate::widget::Label::<$w, 1>::new()
    };
}

pub use label_new;

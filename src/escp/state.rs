//! Rendering state machine for tracking style transitions.

use super::constants::*;
use crate::cell::StyleFlags;

/// Tracks the current style state during rendering.
///
/// Minimizes ESC/P code emission by only outputting state changes.
pub(crate) struct RenderState {
    bold: bool,
    underline: bool,
}

impl RenderState {
    /// Creates a new RenderState with no styles active.
    pub(crate) fn new() -> Self {
        RenderState {
            bold: false,
            underline: false,
        }
    }

    /// Transitions to the target style state, emitting ESC/P codes as needed.
    ///
    /// Only emits codes when the state actually changes.
    pub(crate) fn transition_to(&mut self, target: StyleFlags, output: &mut Vec<u8>) {
        let target_bold = target.bold();
        let target_underline = target.underline();

        // Bold transition
        if target_bold != self.bold {
            if target_bold {
                output.extend_from_slice(ESC_BOLD_ON);
            } else {
                output.extend_from_slice(ESC_BOLD_OFF);
            }
            self.bold = target_bold;
        }

        // Underline transition
        if target_underline != self.underline {
            if target_underline {
                output.extend_from_slice(ESC_UNDERLINE_ON);
            } else {
                output.extend_from_slice(ESC_UNDERLINE_OFF);
            }
            self.underline = target_underline;
        }
    }

    /// Resets all styles to off, emitting necessary codes.
    pub(crate) fn reset(&mut self, output: &mut Vec<u8>) {
        if self.bold {
            output.extend_from_slice(ESC_BOLD_OFF);
            self.bold = false;
        }
        if self.underline {
            output.extend_from_slice(ESC_UNDERLINE_OFF);
            self.underline = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_state_new() {
        let state = RenderState::new();
        assert!(!state.bold);
        assert!(!state.underline);
    }

    #[test]
    fn test_transition_to_bold() {
        let mut state = RenderState::new();
        let mut output = Vec::new();

        state.transition_to(StyleFlags::BOLD, &mut output);

        assert!(state.bold);
        assert_eq!(output, ESC_BOLD_ON);
    }

    #[test]
    fn test_transition_to_underline() {
        let mut state = RenderState::new();
        let mut output = Vec::new();

        state.transition_to(StyleFlags::UNDERLINE, &mut output);

        assert!(state.underline);
        assert_eq!(output, ESC_UNDERLINE_ON);
    }

    #[test]
    fn test_transition_to_both() {
        let mut state = RenderState::new();
        let mut output = Vec::new();

        let style = StyleFlags::BOLD.with_underline();
        state.transition_to(style, &mut output);

        assert!(state.bold);
        assert!(state.underline);
        // Should contain both codes
        assert!(output.starts_with(ESC_BOLD_ON));
    }

    #[test]
    fn test_no_redundant_transition() {
        let mut state = RenderState::new();
        let mut output = Vec::new();

        state.transition_to(StyleFlags::BOLD, &mut output);
        let first_len = output.len();

        // Transition to same state - should not emit again
        state.transition_to(StyleFlags::BOLD, &mut output);
        assert_eq!(output.len(), first_len);
    }

    #[test]
    fn test_reset() {
        let mut state = RenderState::new();
        let mut output = Vec::new();

        // Set styles
        state.transition_to(StyleFlags::BOLD.with_underline(), &mut output);
        output.clear();

        // Reset
        state.reset(&mut output);

        assert!(!state.bold);
        assert!(!state.underline);
        assert!(!output.is_empty()); // Should have emitted OFF codes
    }
}

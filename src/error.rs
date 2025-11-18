//! Error types for layout operations.

use std::fmt;

/// Errors that can occur during layout operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    /// Region dimensions exceed page boundaries (160×51)
    RegionOutOfBounds {
        /// X coordinate
        x: u16,
        /// Y coordinate
        y: u16,
        /// Width
        width: u16,
        /// Height
        height: u16,
    },

    /// Invalid region dimensions (zero width/height or overflow)
    InvalidDimensions {
        /// Width
        width: u16,
        /// Height
        height: u16,
    },

    /// Invalid region split (child dimensions exceed parent)
    InvalidSplit {
        /// Parent dimension
        parent_size: u16,
        /// Requested split size
        split_size: u16,
    },
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutError::RegionOutOfBounds {
                x,
                y,
                width,
                height,
            } => {
                write!(
                    f,
                    "Region out of bounds: position ({}, {}), size ({}×{}) exceeds page dimensions (160×51)",
                    x, y, width, height
                )
            }
            LayoutError::InvalidDimensions { width, height } => {
                write!(
                    f,
                    "Invalid region dimensions: {}×{} (must be non-zero and within page bounds)",
                    width, height
                )
            }
            LayoutError::InvalidSplit {
                parent_size,
                split_size,
            } => {
                write!(
                    f,
                    "Invalid region split: split size {} exceeds parent size {}",
                    split_size, parent_size
                )
            }
        }
    }
}

impl std::error::Error for LayoutError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_out_of_bounds_display() {
        let err = LayoutError::RegionOutOfBounds {
            x: 100,
            y: 40,
            width: 80,
            height: 20,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Region out of bounds"));
        assert!(msg.contains("100"));
        assert!(msg.contains("40"));
    }

    #[test]
    fn test_invalid_dimensions_display() {
        let err = LayoutError::InvalidDimensions {
            width: 0,
            height: 10,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid region dimensions"));
        assert!(msg.contains("0×10"));
    }

    #[test]
    fn test_invalid_split_display() {
        let err = LayoutError::InvalidSplit {
            parent_size: 50,
            split_size: 60,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid region split"));
        assert!(msg.contains("60"));
        assert!(msg.contains("50"));
    }
}

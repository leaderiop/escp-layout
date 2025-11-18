//! Region types for defining rectangular areas within pages.

use crate::error::LayoutError;

/// Page dimensions (Epson LQ-2090II condensed mode)
pub const PAGE_WIDTH: u16 = 160;
/// Page height in lines
pub const PAGE_HEIGHT: u16 = 51;

/// Represents a rectangular area within a Page.
///
/// Regions are lightweight value types (8 bytes) that define boundaries
/// for content placement. They can be split vertically or horizontally,
/// and can have padding applied.
///
/// All region operations validate against the 160×51 page bounds.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Region {
    /// Starting column (0-based)
    pub(crate) x: u16,
    /// Starting row (0-based)
    pub(crate) y: u16,
    /// Width in characters
    pub(crate) width: u16,
    /// Height in lines
    pub(crate) height: u16,
}

impl Region {
    /// Creates a new region with the specified position and dimensions.
    ///
    /// Returns an error if the region would extend beyond page boundaries (160×51)
    /// or if width/height are zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::Region;
    ///
    /// // Valid region
    /// let region = Region::new(0, 0, 80, 25).unwrap();
    ///
    /// // Out of bounds
    /// let err = Region::new(100, 0, 80, 25);
    /// assert!(err.is_err());
    /// ```
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Result<Self, LayoutError> {
        // Validate non-zero dimensions
        if width == 0 || height == 0 {
            return Err(LayoutError::InvalidDimensions { width, height });
        }

        // Validate bounds (check for overflow using checked_add)
        let end_x = x.checked_add(width).ok_or(LayoutError::RegionOutOfBounds {
            x,
            y,
            width,
            height,
        })?;

        let end_y = y
            .checked_add(height)
            .ok_or(LayoutError::RegionOutOfBounds {
                x,
                y,
                width,
                height,
            })?;

        if end_x > PAGE_WIDTH || end_y > PAGE_HEIGHT {
            return Err(LayoutError::RegionOutOfBounds {
                x,
                y,
                width,
                height,
            });
        }

        Ok(Region {
            x,
            y,
            width,
            height,
        })
    }

    /// Creates a region covering the entire page (160×51).
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::Region;
    ///
    /// let full_page = Region::full_page();
    /// assert_eq!(full_page.width(), 160);
    /// assert_eq!(full_page.height(), 51);
    /// ```
    pub fn full_page() -> Self {
        Region {
            x: 0,
            y: 0,
            width: PAGE_WIDTH,
            height: PAGE_HEIGHT,
        }
    }

    /// Returns the starting column
    #[inline]
    pub fn x(&self) -> u16 {
        self.x
    }

    /// Returns the starting row
    #[inline]
    pub fn y(&self) -> u16 {
        self.y
    }

    /// Returns the width in characters
    #[inline]
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height in lines
    #[inline]
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Splits the region vertically into top and bottom regions.
    ///
    /// # Arguments
    ///
    /// * `top_height` - Number of lines for the top region
    ///
    /// Returns a tuple of (top_region, bottom_region).
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::Region;
    ///
    /// let region = Region::new(0, 0, 160, 51).unwrap();
    /// let (header, body) = region.split_vertical(10).unwrap();
    ///
    /// assert_eq!(header.height(), 10);
    /// assert_eq!(body.height(), 41);
    /// ```
    pub fn split_vertical(&self, top_height: u16) -> Result<(Region, Region), LayoutError> {
        if top_height > self.height {
            return Err(LayoutError::InvalidSplit {
                parent_size: self.height,
                split_size: top_height,
            });
        }

        let bottom_height = self.height - top_height;

        let top = Region {
            x: self.x,
            y: self.y,
            width: self.width,
            height: top_height,
        };

        let bottom = Region {
            x: self.x,
            y: self.y + top_height,
            width: self.width,
            height: bottom_height,
        };

        Ok((top, bottom))
    }

    /// Splits the region horizontally into left and right regions.
    ///
    /// # Arguments
    ///
    /// * `left_width` - Number of characters for the left region
    ///
    /// Returns a tuple of (left_region, right_region).
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::Region;
    ///
    /// let region = Region::new(0, 0, 160, 51).unwrap();
    /// let (sidebar, main) = region.split_horizontal(40).unwrap();
    ///
    /// assert_eq!(sidebar.width(), 40);
    /// assert_eq!(main.width(), 120);
    /// ```
    pub fn split_horizontal(&self, left_width: u16) -> Result<(Region, Region), LayoutError> {
        if left_width > self.width {
            return Err(LayoutError::InvalidSplit {
                parent_size: self.width,
                split_size: left_width,
            });
        }

        let right_width = self.width - left_width;

        let left = Region {
            x: self.x,
            y: self.y,
            width: left_width,
            height: self.height,
        };

        let right = Region {
            x: self.x + left_width,
            y: self.y,
            width: right_width,
            height: self.height,
        };

        Ok((left, right))
    }

    /// Applies padding to the region, reducing usable space.
    ///
    /// # Arguments
    ///
    /// * `top` - Lines of padding from the top
    /// * `right` - Characters of padding from the right
    /// * `bottom` - Lines of padding from the bottom
    /// * `left` - Characters of padding from the left
    ///
    /// Returns a new region with padding applied. Returns an error if padding
    /// exceeds region dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::Region;
    ///
    /// let region = Region::new(0, 0, 100, 50).unwrap();
    /// let padded = region.with_padding(2, 5, 2, 5).unwrap();
    ///
    /// assert_eq!(padded.width(), 90);  // 100 - 5 - 5
    /// assert_eq!(padded.height(), 46); // 50 - 2 - 2
    /// ```
    pub fn with_padding(
        &self,
        top: u16,
        right: u16,
        bottom: u16,
        left: u16,
    ) -> Result<Region, LayoutError> {
        let horizontal_padding = left.saturating_add(right);
        let vertical_padding = top.saturating_add(bottom);

        if horizontal_padding >= self.width || vertical_padding >= self.height {
            return Err(LayoutError::InvalidDimensions {
                width: self.width.saturating_sub(horizontal_padding),
                height: self.height.saturating_sub(vertical_padding),
            });
        }

        let new_width = self.width - horizontal_padding;
        let new_height = self.height - vertical_padding;

        Ok(Region {
            x: self.x + left,
            y: self.y + top,
            width: new_width,
            height: new_height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_new_valid() {
        let region = Region::new(0, 0, 160, 51);
        assert!(region.is_ok());

        let region = region.unwrap();
        assert_eq!(region.x(), 0);
        assert_eq!(region.y(), 0);
        assert_eq!(region.width(), 160);
        assert_eq!(region.height(), 51);
    }

    #[test]
    fn test_region_new_out_of_bounds() {
        assert!(Region::new(100, 0, 80, 25).is_err());
        assert!(Region::new(0, 40, 160, 20).is_err());
    }

    #[test]
    fn test_region_new_zero_dimensions() {
        assert!(Region::new(0, 0, 0, 10).is_err());
        assert!(Region::new(0, 0, 10, 0).is_err());
    }

    #[test]
    fn test_region_full_page() {
        let region = Region::full_page();
        assert_eq!(region.x(), 0);
        assert_eq!(region.y(), 0);
        assert_eq!(region.width(), 160);
        assert_eq!(region.height(), 51);
    }

    #[test]
    fn test_split_vertical_valid() {
        let region = Region::new(0, 0, 160, 51).unwrap();
        let (top, bottom) = region.split_vertical(10).unwrap();

        assert_eq!(top.y(), 0);
        assert_eq!(top.height(), 10);
        assert_eq!(bottom.y(), 10);
        assert_eq!(bottom.height(), 41);
    }

    #[test]
    fn test_split_vertical_invalid() {
        let region = Region::new(0, 0, 160, 51).unwrap();
        assert!(region.split_vertical(60).is_err());
    }

    #[test]
    fn test_split_horizontal_valid() {
        let region = Region::new(0, 0, 160, 51).unwrap();
        let (left, right) = region.split_horizontal(40).unwrap();

        assert_eq!(left.x(), 0);
        assert_eq!(left.width(), 40);
        assert_eq!(right.x(), 40);
        assert_eq!(right.width(), 120);
    }

    #[test]
    fn test_split_horizontal_invalid() {
        let region = Region::new(0, 0, 160, 51).unwrap();
        assert!(region.split_horizontal(200).is_err());
    }

    #[test]
    fn test_with_padding_valid() {
        let region = Region::new(0, 0, 100, 50).unwrap();
        let padded = region.with_padding(2, 5, 2, 5).unwrap();

        assert_eq!(padded.x(), 5);
        assert_eq!(padded.y(), 2);
        assert_eq!(padded.width(), 90);
        assert_eq!(padded.height(), 46);
    }

    #[test]
    fn test_with_padding_excessive() {
        let region = Region::new(0, 0, 20, 10).unwrap();
        assert!(region.with_padding(5, 0, 10, 0).is_err());
        assert!(region.with_padding(0, 15, 0, 10).is_err());
    }

    #[test]
    fn test_region_boundary_conditions() {
        // Exact boundary
        assert!(Region::new(0, 0, 160, 51).is_ok());
        assert!(Region::new(159, 50, 1, 1).is_ok());

        // Just over boundary
        assert!(Region::new(0, 0, 161, 51).is_err());
        assert!(Region::new(0, 0, 160, 52).is_err());
    }
}

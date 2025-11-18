//! KeyValueList widget for aligned key-value pairs.

use super::Widget;
use crate::{PageBuilder, Region, StyleFlags};

/// Vertically aligned list of key-value pairs.
///
/// Renders one entry per line with a customizable separator.
/// Default separator is ": ".
///
/// # Examples
///
/// ```
/// use escp_layout::{Page, Region};
/// use escp_layout::widgets::{Widget, KeyValueList};
///
/// let kv_list = KeyValueList::new(vec![
///     ("Name".into(), "John Doe".into()),
///     ("ID".into(), "12345".into()),
/// ]);
///
/// let mut page = Page::builder();
/// let region = Region::new(0, 0, 40, 10).unwrap();
/// kv_list.render(&mut page, region);
/// ```
pub struct KeyValueList {
    entries: Vec<(String, String)>,
    separator: String,
}

impl KeyValueList {
    /// Creates a new KeyValueList with default separator ": ".
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::widgets::KeyValueList;
    ///
    /// let kv = KeyValueList::new(vec![
    ///     ("Key1".into(), "Value1".into()),
    ///     ("Key2".into(), "Value2".into()),
    /// ]);
    /// ```
    pub fn new(entries: Vec<(String, String)>) -> Self {
        KeyValueList {
            entries,
            separator: ": ".to_string(),
        }
    }

    /// Sets a custom separator between keys and values.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::widgets::KeyValueList;
    ///
    /// let kv = KeyValueList::new(vec![
    ///     ("Key".into(), "Value".into()),
    /// ]).with_separator(" = ");
    /// ```
    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }
}

impl Widget for KeyValueList {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        // Handle zero-size regions
        if region.width() == 0 || region.height() == 0 {
            return;
        }

        for (line_idx, (key, value)) in self.entries.iter().enumerate() {
            if line_idx as u16 >= region.height() {
                break; // Vertical truncation
            }

            // Build the line: "key{separator}value"
            let line = format!("{}{}{}", key, self.separator, value);
            let max_chars = region.width().min(line.len() as u16);

            for (char_idx, ch) in line.chars().take(max_chars as usize).enumerate() {
                page.write_at(
                    region.x() + char_idx as u16,
                    region.y() + line_idx as u16,
                    ch,
                    StyleFlags::NONE,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Page;

    #[test]
    fn test_key_value_list_new() {
        let kv = KeyValueList::new(vec![("Key1".into(), "Value1".into())]);
        assert_eq!(kv.entries.len(), 1);
        assert_eq!(kv.separator, ": ");
    }

    #[test]
    fn test_key_value_list_with_separator() {
        let kv = KeyValueList::new(vec![]).with_separator(" = ");
        assert_eq!(kv.separator, " = ");
    }

    #[test]
    fn test_key_value_list_render() {
        let kv = KeyValueList::new(vec![
            ("Name".into(), "Alice".into()),
            ("ID".into(), "123".into()),
        ]);
        let mut page = Page::builder();
        let region = Region::new(0, 0, 20, 5).unwrap();

        kv.render(&mut page, region);
        let page = page.build();

        // First entry: "Name: Alice"
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'N');
        assert_eq!(page.get_cell(4, 0).unwrap().character(), ':');
        assert_eq!(page.get_cell(6, 0).unwrap().character(), 'A');

        // Second entry: "ID: 123"
        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'I');
        assert_eq!(page.get_cell(2, 1).unwrap().character(), ':');
        assert_eq!(page.get_cell(4, 1).unwrap().character(), '1');
    }

    #[test]
    fn test_key_value_list_custom_separator() {
        let kv = KeyValueList::new(vec![("Key".into(), "Val".into())]).with_separator(" = ");
        let mut page = Page::builder();
        let region = Region::new(0, 0, 20, 5).unwrap();

        kv.render(&mut page, region);
        let page = page.build();

        // "Key = Val"
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'K');
        assert_eq!(page.get_cell(3, 0).unwrap().character(), ' ');
        assert_eq!(page.get_cell(4, 0).unwrap().character(), '=');
        assert_eq!(page.get_cell(6, 0).unwrap().character(), 'V');
    }

    #[test]
    fn test_key_value_list_horizontal_truncation() {
        let kv = KeyValueList::new(vec![("VeryLongKey".into(), "VeryLongValue".into())]);
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 1).unwrap();

        kv.render(&mut page, region);
        let page = page.build();

        // Should truncate at width 10
        // Line would be "VeryLongKey: VeryLongValue" but truncated to "VeryLongKe"
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'V');
        assert_eq!(page.get_cell(9, 0).unwrap().character(), 'e'); // 10th char is 'e'
        assert_eq!(page.get_cell(10, 0).unwrap().character(), ' '); // Empty (out of region)
    }

    #[test]
    fn test_key_value_list_vertical_truncation() {
        let kv = KeyValueList::new(vec![
            ("K1".into(), "V1".into()),
            ("K2".into(), "V2".into()),
            ("K3".into(), "V3".into()),
            ("K4".into(), "V4".into()),
        ]);
        let mut page = Page::builder();
        let region = Region::new(0, 0, 20, 2).unwrap();

        kv.render(&mut page, region);
        let page = page.build();

        // Only first 2 entries should render
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'K');
        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'K');
        assert_eq!(page.get_cell(0, 2).unwrap().character(), ' '); // Empty
    }

    #[test]
    fn test_key_value_list_zero_width() {
        let kv = KeyValueList::new(vec![("Key".into(), "Value".into())]);
        let mut page = Page::builder();

        // Zero-width regions are invalid - test that widget handles it gracefully
        // The widget render method checks region.width() == 0 and returns early
        if let Ok(region) = Region::new(0, 0, 1, 10) {
            kv.render(&mut page, region);
        }
    }
}

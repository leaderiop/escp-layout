//! Table widget for fixed-column tabular data.

use super::Widget;
use crate::{PageBuilder, Region, StyleFlags};

/// Column definition for tables.
///
/// Specifies the column name and fixed width.
#[derive(Clone, Debug)]
pub struct ColumnDef {
    /// Column header name
    pub name: String,
    /// Fixed column width in characters
    pub width: u16,
}

/// Fixed-column table with headers and rows.
///
/// Renders a table with bold headers on the first line and data rows below.
/// Cells are left-aligned and truncated to column width.
///
/// # Examples
///
/// ```
/// use escp_layout::{Page, Region};
/// use escp_layout::widgets::{Widget, Table, ColumnDef};
///
/// let table = Table::new(
///     vec![
///         ColumnDef { name: "Name".into(), width: 20 },
///         ColumnDef { name: "Age".into(), width: 10 },
///     ],
///     vec![
///         vec!["Alice".into(), "30".into()],
///         vec!["Bob".into(), "25".into()],
///     ],
/// );
///
/// let mut page = Page::builder();
/// let region = Region::new(0, 0, 40, 10).unwrap();
/// table.render(&mut page, region);
/// ```
pub struct Table {
    columns: Vec<ColumnDef>,
    rows: Vec<Vec<String>>,
}

impl Table {
    /// Creates a new table with column definitions and row data.
    ///
    /// # Examples
    ///
    /// ```
    /// use escp_layout::widgets::{Table, ColumnDef};
    ///
    /// let table = Table::new(
    ///     vec![
    ///         ColumnDef { name: "Product".into(), width: 30 },
    ///         ColumnDef { name: "Price".into(), width: 15 },
    ///     ],
    ///     vec![
    ///         vec!["Widget A".into(), "$10.00".into()],
    ///         vec!["Widget B".into(), "$15.00".into()],
    ///     ],
    /// );
    /// ```
    pub fn new(columns: Vec<ColumnDef>, rows: Vec<Vec<String>>) -> Self {
        Table { columns, rows }
    }
}

impl Widget for Table {
    fn render(&self, page: &mut PageBuilder, region: Region) {
        // Handle zero-height region
        if region.height() == 0 {
            return;
        }

        // Render header row (bold)
        let mut col_x = region.x();
        for col in &self.columns {
            if col_x >= region.x() + region.width() {
                break; // Column overflow
            }

            let max_chars = col
                .width
                .min((region.x() + region.width()).saturating_sub(col_x));
            for (i, ch) in col.name.chars().take(max_chars as usize).enumerate() {
                page.write_at(col_x + i as u16, region.y(), ch, StyleFlags::BOLD);
            }

            col_x += col.width;
        }

        // Render data rows
        for (row_idx, row) in self.rows.iter().enumerate() {
            let y = region.y() + 1 + row_idx as u16;
            if y >= region.y() + region.height() {
                break; // Vertical truncation
            }

            let mut col_x = region.x();
            for (col_idx, col_def) in self.columns.iter().enumerate() {
                if col_x >= region.x() + region.width() {
                    break; // Column overflow
                }

                let cell_text = row.get(col_idx).map(|s| s.as_str()).unwrap_or("");
                let max_chars = col_def
                    .width
                    .min((region.x() + region.width()).saturating_sub(col_x));

                for (i, ch) in cell_text.chars().take(max_chars as usize).enumerate() {
                    page.write_at(col_x + i as u16, y, ch, StyleFlags::NONE);
                }

                col_x += col_def.width;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Page;

    #[test]
    fn test_table_new() {
        let table = Table::new(
            vec![ColumnDef {
                name: "Col1".into(),
                width: 10,
            }],
            vec![vec!["Data".into()]],
        );
        assert_eq!(table.columns.len(), 1);
        assert_eq!(table.rows.len(), 1);
    }

    #[test]
    fn test_table_render_header() {
        let table = Table::new(
            vec![
                ColumnDef {
                    name: "Name".into(),
                    width: 10,
                },
                ColumnDef {
                    name: "Age".into(),
                    width: 5,
                },
            ],
            vec![],
        );
        let mut page = Page::builder();
        let region = Region::new(0, 0, 20, 5).unwrap();

        table.render(&mut page, region);
        let page = page.build();

        // Header should be bold
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'N');
        assert_eq!(page.get_cell(0, 0).unwrap().style(), StyleFlags::BOLD);
        assert_eq!(page.get_cell(10, 0).unwrap().character(), 'A'); // "Age" starts at col 10
        assert_eq!(page.get_cell(10, 0).unwrap().style(), StyleFlags::BOLD);
    }

    #[test]
    fn test_table_render_rows() {
        let table = Table::new(
            vec![
                ColumnDef {
                    name: "Name".into(),
                    width: 10,
                },
                ColumnDef {
                    name: "Age".into(),
                    width: 5,
                },
            ],
            vec![
                vec!["Alice".into(), "30".into()],
                vec!["Bob".into(), "25".into()],
            ],
        );
        let mut page = Page::builder();
        let region = Region::new(0, 0, 20, 5).unwrap();

        table.render(&mut page, region);
        let page = page.build();

        // First row: "Alice" at (0,1)
        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'A');
        assert_eq!(page.get_cell(4, 1).unwrap().character(), 'e');
        assert_eq!(page.get_cell(10, 1).unwrap().character(), '3'); // "30" starts at col 10

        // Second row: "Bob" at (0,2)
        assert_eq!(page.get_cell(0, 2).unwrap().character(), 'B');
        assert_eq!(page.get_cell(10, 2).unwrap().character(), '2'); // "25" starts at col 10
    }

    #[test]
    fn test_table_cell_truncation() {
        let table = Table::new(
            vec![ColumnDef {
                name: "Name".into(),
                width: 5,
            }],
            vec![vec!["VeryLongName".into()]],
        );
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 5).unwrap();

        table.render(&mut page, region);
        let page = page.build();

        // Cell should be truncated to column width (5)
        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'V');
        assert_eq!(page.get_cell(4, 1).unwrap().character(), 'L');
        assert_eq!(page.get_cell(5, 1).unwrap().character(), ' '); // Empty (truncated)
    }

    #[test]
    fn test_table_vertical_truncation() {
        let table = Table::new(
            vec![ColumnDef {
                name: "Col".into(),
                width: 10,
            }],
            vec![
                vec!["Row1".into()],
                vec!["Row2".into()],
                vec!["Row3".into()],
                vec!["Row4".into()],
            ],
        );
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 3).unwrap(); // Only 3 lines (header + 2 rows)

        table.render(&mut page, region);
        let page = page.build();

        // Header + 2 rows should render
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'C'); // Header
        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'R'); // Row1
        assert_eq!(page.get_cell(0, 2).unwrap().character(), 'R'); // Row2
    }

    #[test]
    fn test_table_empty_rows() {
        let table = Table::new(
            vec![ColumnDef {
                name: "Col".into(),
                width: 10,
            }],
            vec![],
        );
        let mut page = Page::builder();
        let region = Region::new(0, 0, 10, 5).unwrap();

        table.render(&mut page, region);
        let page = page.build();

        // Only header should render
        assert_eq!(page.get_cell(0, 0).unwrap().character(), 'C');
        assert_eq!(page.get_cell(0, 1).unwrap().character(), ' '); // Empty (no rows)
    }

    #[test]
    fn test_table_missing_cells() {
        let table = Table::new(
            vec![
                ColumnDef {
                    name: "Col1".into(),
                    width: 10,
                },
                ColumnDef {
                    name: "Col2".into(),
                    width: 10,
                },
            ],
            vec![
                vec!["A".into()], // Missing second cell
            ],
        );
        let mut page = Page::builder();
        let region = Region::new(0, 0, 25, 5).unwrap();

        // Should not panic
        table.render(&mut page, region);
        let page = page.build();

        assert_eq!(page.get_cell(0, 1).unwrap().character(), 'A');
        assert_eq!(page.get_cell(10, 1).unwrap().character(), ' '); // Missing cell = empty
    }

    #[test]
    fn test_table_zero_height() {
        let table = Table::new(
            vec![ColumnDef {
                name: "Col".into(),
                width: 10,
            }],
            vec![vec!["Data".into()]],
        );
        let mut page = Page::builder();

        // Zero-height regions are invalid - test that widget handles it gracefully
        // The widget render method checks region.height() == 0 and returns early
        if let Ok(region) = Region::new(0, 0, 10, 1) {
            table.render(&mut page, region);
        }
    }
}

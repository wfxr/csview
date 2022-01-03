use crate::{
    cli::Border,
    table::{RowSep, TableFormat, TableFormatBuilder},
};

impl From<Border> for TableFormat {
    fn from(style: Border) -> Self {
        match style {
            Border::None => TableFormatBuilder::new().clear_seps().build(),
            Border::Ascii => TableFormatBuilder::new()
                .col_sep('|')
                .row_seps(
                    RowSep::new('-', '+', '+', '+'),
                    RowSep::new('-', '+', '+', '+'),
                    None,
                    RowSep::new('-', '+', '+', '+'),
                )
                .build(),
            Border::Sharp => TableFormatBuilder::new()
                .col_sep('│')
                .row_seps(
                    RowSep::new('─', '┌', '┬', '┐'),
                    RowSep::new('─', '├', '┼', '┤'),
                    None,
                    RowSep::new('─', '└', '┴', '┘'),
                )
                .build(),
            Border::Rounded => TableFormatBuilder::new()
                .col_sep('│')
                .row_seps(
                    RowSep::new('─', '╭', '┬', '╮'),
                    RowSep::new('─', '├', '┼', '┤'),
                    None,
                    RowSep::new('─', '╰', '┴', '╯'),
                )
                .build(),
            Border::Reinforced => TableFormatBuilder::new()
                .col_sep('│')
                .row_seps(
                    RowSep::new('─', '┏', '┬', '┓'),
                    RowSep::new('─', '├', '┼', '┤'),
                    None,
                    RowSep::new('─', '┗', '┴', '┛'),
                )
                .build(),
            Border::Markdown => TableFormatBuilder::new()
                .col_sep('|')
                .row_seps(None, RowSep::new('-', '|', '|', '|'), None, None)
                .build(),
            Border::Grid => TableFormatBuilder::new()
                .col_sep('│')
                .row_seps(
                    RowSep::new('─', '┌', '┬', '┐'),
                    RowSep::new('─', '├', '┼', '┤'),
                    RowSep::new('─', '├', '┼', '┤'),
                    RowSep::new('─', '└', '┴', '┘'),
                )
                .build(),
        }
    }
}

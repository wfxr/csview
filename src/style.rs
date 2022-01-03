use crate::{
    cli::Style,
    table::{RowSep, TableFormat, TableFormatBuilder},
};

impl From<Style> for TableFormat {
    fn from(style: Style) -> Self {
        match style {
            Style::None => TableFormatBuilder::new().clear_seps().build(),
            Style::Ascii => TableFormatBuilder::new()
                .col_sep('|')
                .row_seps(
                    RowSep::new('-', '+', '+', '+'),
                    RowSep::new('-', '+', '+', '+'),
                    None,
                    RowSep::new('-', '+', '+', '+'),
                )
                .build(),
            Style::Sharp => TableFormatBuilder::new()
                .col_sep('│')
                .row_seps(
                    RowSep::new('─', '┌', '┬', '┐'),
                    RowSep::new('─', '├', '┼', '┤'),
                    None,
                    RowSep::new('─', '└', '┴', '┘'),
                )
                .build(),
            Style::Rounded => TableFormatBuilder::new()
                .col_sep('│')
                .row_seps(
                    RowSep::new('─', '╭', '┬', '╮'),
                    RowSep::new('─', '├', '┼', '┤'),
                    None,
                    RowSep::new('─', '╰', '┴', '╯'),
                )
                .build(),
            Style::Reinforced => TableFormatBuilder::new()
                .col_sep('│')
                .row_seps(
                    RowSep::new('─', '┏', '┬', '┓'),
                    RowSep::new('─', '├', '┼', '┤'),
                    None,
                    RowSep::new('─', '┗', '┴', '┛'),
                )
                .build(),
            Style::Markdown => TableFormatBuilder::new()
                .col_sep('|')
                .row_seps(None, RowSep::new('-', '|', '|', '|'), None, None)
                .build(),
            Style::Grid => TableFormatBuilder::new()
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

use crate::{
    cli::TableStyle,
    table::{RowSep, Style, StyleBuilder},
};

pub fn table_style(style: TableStyle, padding: usize, indent: usize) -> Style {
    let builder = match style {
        TableStyle::None => StyleBuilder::new().clear_seps(),
        TableStyle::Ascii => StyleBuilder::new().col_sep('|').row_seps(
            RowSep::new('-', '+', '+', '+'),
            RowSep::new('-', '+', '+', '+'),
            None,
            RowSep::new('-', '+', '+', '+'),
        ),
        TableStyle::Ascii2 =>
            StyleBuilder::new()
                .col_seps(' ', '|', ' ')
                .row_seps(None, RowSep::new('-', ' ', '+', ' '), None, None),
        TableStyle::Sharp => StyleBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┌', '┬', '┐'),
            RowSep::new('─', '├', '┼', '┤'),
            None,
            RowSep::new('─', '└', '┴', '┘'),
        ),
        TableStyle::Rounded => StyleBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '╭', '┬', '╮'),
            RowSep::new('─', '├', '┼', '┤'),
            None,
            RowSep::new('─', '╰', '┴', '╯'),
        ),
        TableStyle::Reinforced => StyleBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┏', '┬', '┓'),
            RowSep::new('─', '├', '┼', '┤'),
            None,
            RowSep::new('─', '┗', '┴', '┛'),
        ),
        TableStyle::Markdown =>
            StyleBuilder::new()
                .col_sep('|')
                .row_seps(None, RowSep::new('-', '|', '|', '|'), None, None),
        TableStyle::Grid => StyleBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┌', '┬', '┐'),
            RowSep::new('─', '├', '┼', '┤'),
            RowSep::new('─', '├', '┼', '┤'),
            RowSep::new('─', '└', '┴', '┘'),
        ),
    };
    builder.padding(padding).indent(indent).build()
}

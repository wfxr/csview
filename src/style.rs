use crate::{
    cli::Style,
    table::{RowSep, TableStyle, TableStyleBuilder},
};

pub fn table_format(style: Style, padding: usize, indent: usize) -> TableStyle {
    let builder = match style {
        Style::None => TableStyleBuilder::new().clear_seps(),
        Style::Ascii => TableStyleBuilder::new().col_sep('|').row_seps(
            RowSep::new('-', '+', '+', '+'),
            RowSep::new('-', '+', '+', '+'),
            None,
            RowSep::new('-', '+', '+', '+'),
        ),
        Style::Sharp => TableStyleBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┌', '┬', '┐'),
            RowSep::new('─', '├', '┼', '┤'),
            None,
            RowSep::new('─', '└', '┴', '┘'),
        ),
        Style::Rounded => TableStyleBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '╭', '┬', '╮'),
            RowSep::new('─', '├', '┼', '┤'),
            None,
            RowSep::new('─', '╰', '┴', '╯'),
        ),
        Style::Reinforced => TableStyleBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┏', '┬', '┓'),
            RowSep::new('─', '├', '┼', '┤'),
            None,
            RowSep::new('─', '┗', '┴', '┛'),
        ),
        Style::Markdown =>
            TableStyleBuilder::new()
                .col_sep('|')
                .row_seps(None, RowSep::new('-', '|', '|', '|'), None, None),
        Style::Grid => TableStyleBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┌', '┬', '┐'),
            RowSep::new('─', '├', '┼', '┤'),
            RowSep::new('─', '├', '┼', '┤'),
            RowSep::new('─', '└', '┴', '┘'),
        ),
    };
    builder.padding(padding).indent(indent).build()
}

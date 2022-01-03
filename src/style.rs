use crate::{
    cli::Style,
    table::{RowSep, TableFormat, TableFormatBuilder},
};

pub fn table_format(style: Style, padding: usize, indent: usize) -> TableFormat {
    let builder = match style {
        Style::None => TableFormatBuilder::new().clear_seps(),
        Style::Ascii => TableFormatBuilder::new().col_sep('|').row_seps(
            RowSep::new('-', '+', '+', '+'),
            RowSep::new('-', '+', '+', '+'),
            None,
            RowSep::new('-', '+', '+', '+'),
        ),
        Style::Sharp => TableFormatBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┌', '┬', '┐'),
            RowSep::new('─', '├', '┼', '┤'),
            None,
            RowSep::new('─', '└', '┴', '┘'),
        ),
        Style::Rounded => TableFormatBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '╭', '┬', '╮'),
            RowSep::new('─', '├', '┼', '┤'),
            None,
            RowSep::new('─', '╰', '┴', '╯'),
        ),
        Style::Reinforced => TableFormatBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┏', '┬', '┓'),
            RowSep::new('─', '├', '┼', '┤'),
            None,
            RowSep::new('─', '┗', '┴', '┛'),
        ),
        Style::Markdown =>
            TableFormatBuilder::new()
                .col_sep('|')
                .row_seps(None, RowSep::new('-', '|', '|', '|'), None, None),
        Style::Grid => TableFormatBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┌', '┬', '┐'),
            RowSep::new('─', '├', '┼', '┤'),
            RowSep::new('─', '├', '┼', '┤'),
            RowSep::new('─', '└', '┴', '┘'),
        ),
    };
    builder.padding(padding).indent(indent).build()
}

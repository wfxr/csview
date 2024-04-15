use cli::Alignment;

use crate::{
    cli::{self, TableStyle},
    table::{RowSep, Style, StyleBuilder},
};

pub fn table_style(
    style: TableStyle,
    padding: usize,
    indent: usize,
    header_align: Alignment,
    body_align: Alignment,
) -> Style {
    let builder = match style {
        TableStyle::None => StyleBuilder::new().clear_seps(),
        TableStyle::Ascii => StyleBuilder::new().col_sep('|').row_seps(
            RowSep::new('-', '+', '+', '+'),
            RowSep::new('-', '+', '+', '+'),
            None,
            RowSep::new('-', '+', '+', '+'),
        ),
        TableStyle::Ascii2 => {
            StyleBuilder::new()
                .col_seps(' ', '|', ' ')
                .row_seps(None, RowSep::new('-', ' ', '+', ' '), None, None)
        }
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
        TableStyle::Markdown => {
            StyleBuilder::new()
                .col_sep('|')
                .row_seps(None, RowSep::new('-', '|', '|', '|'), None, None)
        }
        TableStyle::Grid => StyleBuilder::new().col_sep('│').row_seps(
            RowSep::new('─', '┌', '┬', '┐'),
            RowSep::new('─', '├', '┼', '┤'),
            RowSep::new('─', '├', '┼', '┤'),
            RowSep::new('─', '└', '┴', '┘'),
        ),
    };
    builder
        .padding(padding)
        .indent(indent)
        .header_align(header_align.into())
        .body_align(body_align.into())
        .build()
}

impl From<Alignment> for unicode_truncate::Alignment {
    fn from(a: Alignment) -> Self {
        match a {
            Alignment::Left => unicode_truncate::Alignment::Left,
            Alignment::Center => unicode_truncate::Alignment::Center,
            Alignment::Right => unicode_truncate::Alignment::Right,
        }
    }
}

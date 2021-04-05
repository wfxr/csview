use crate::cli::Border;
use prettytable::format::{self, FormatBuilder, LinePosition, LineSeparator};

impl From<Border> for format::TableFormat {
    fn from(style: Border) -> Self {
        match style {
            Border::None => *format::consts::FORMAT_CLEAN,
            Border::Ascii => FormatBuilder::new()
                .column_separator('|')
                .borders('|')
                .separator(LinePosition::Title, LineSeparator::new('-', '+', '+', '+'))
                .separator(LinePosition::Bottom, LineSeparator::new('-', '+', '+', '+'))
                .separator(LinePosition::Top, LineSeparator::new('-', '+', '+', '+'))
                .hide_bottom_sep_when_empty(true)
                .padding(1, 1)
                .build(),
            Border::Sharp => FormatBuilder::new()
                .column_separator('│')
                .borders('│')
                .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '┌', '┐'))
                .separators(&[LinePosition::Title], LineSeparator::new('─', '┼', '├', '┤'))
                .separators(&[LinePosition::Bottom], LineSeparator::new('─', '┴', '└', '┘'))
                .hide_bottom_sep_when_empty(true)
                .padding(1, 1)
                .build(),
            Border::Rounded => FormatBuilder::new()
                .column_separator('│')
                .borders('│')
                .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '╭', '╮'))
                .separators(&[LinePosition::Title], LineSeparator::new('─', '┼', '├', '┤'))
                .separators(&[LinePosition::Bottom], LineSeparator::new('─', '┴', '╰', '╯'))
                .hide_bottom_sep_when_empty(true)
                .padding(1, 1)
                .build(),
            Border::Reinforced => FormatBuilder::new()
                .column_separator('│')
                .borders('│')
                .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '┏', '┓'))
                .separators(&[LinePosition::Title], LineSeparator::new('─', '┼', '├', '┤'))
                .separators(&[LinePosition::Bottom], LineSeparator::new('─', '┴', '┗', '┛'))
                .hide_bottom_sep_when_empty(true)
                .padding(1, 1)
                .build(),
            Border::Markdown => FormatBuilder::new()
                .column_separator('|')
                .borders('|')
                .separators(&[LinePosition::Title], LineSeparator::new('-', '|', '|', '|'))
                .hide_bottom_sep_when_empty(true)
                .padding(1, 1)
                .build(),
            Border::Grid => FormatBuilder::new()
                .column_separator('│')
                .borders('│')
                .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '┌', '┐'))
                .separators(&[LinePosition::Intern], LineSeparator::new('─', '┼', '├', '┤'))
                .separators(&[LinePosition::Bottom], LineSeparator::new('─', '┴', '└', '┘'))
                .hide_bottom_sep_when_empty(true)
                .padding(1, 1)
                .build(),
        }
    }
}

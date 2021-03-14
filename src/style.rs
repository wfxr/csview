use crate::cli::Border;
use prettytable::format::{self, FormatBuilder, LinePosition, LineSeparator};

impl From<Border> for format::TableFormat {
    fn from(style: Border) -> Self {
        match style {
            Border::None => *format::consts::FORMAT_CLEAN,
            Border::Ascii => *format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
            Border::Sharp => FormatBuilder::new()
                .column_separator('│')
                .borders('│')
                .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '┌', '┐'))
                .separators(&[LinePosition::Title], LineSeparator::new('─', '┼', '├', '┤'))
                .separators(&[LinePosition::Bottom], LineSeparator::new('─', '┴', '└', '┘'))
                .padding(1, 1)
                .build(),
            Border::Rounded => FormatBuilder::new()
                .column_separator('│')
                .borders('│')
                .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '╭', '╮'))
                .separators(&[LinePosition::Title], LineSeparator::new('─', '┼', '├', '┤'))
                .separators(&[LinePosition::Bottom], LineSeparator::new('─', '┴', '╰', '╯'))
                .padding(1, 1)
                .build(),
            Border::Reinforced => FormatBuilder::new()
                .column_separator('│')
                .borders('│')
                .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '┏', '┓'))
                .separators(&[LinePosition::Title], LineSeparator::new('─', '┼', '├', '┤'))
                .separators(&[LinePosition::Bottom], LineSeparator::new('─', '┴', '┗', '┛'))
                .padding(1, 1)
                .build(),
            Border::Markdown => FormatBuilder::new()
                .column_separator('|')
                .borders('|')
                .separators(&[LinePosition::Title], LineSeparator::new('-', '|', '|', '|'))
                .padding(1, 1)
                .build(),
            Border::Grid => *format::consts::FORMAT_BOX_CHARS,
        }
    }
}

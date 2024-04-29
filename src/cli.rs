use std::path::PathBuf;

use clap::{Parser, ValueEnum, ValueHint};

#[derive(Parser)]
#[command(about, version)]
#[command(disable_help_subcommand = true)]
#[command(next_line_help = true)]
pub struct App {
    /// File to view.
    #[arg(name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// Specify that the input has no header row.
    #[arg(short = 'H', long = "no-headers")]
    pub no_headers: bool,

    /// Prepend a column of line numbers to the table.
    #[arg(short, long, alias = "seq")]
    pub number: bool,

    /// Use '\t' as delimiter for tsv.
    #[arg(short, long, conflicts_with = "delimiter")]
    pub tsv: bool,

    /// Specify the field delimiter.
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// Specify the border style.
    #[arg(short, long, value_enum, default_value_t = TableStyle::Sharp, ignore_case = true)]
    pub style: TableStyle,

    /// Specify padding for table cell.
    #[arg(short, long, default_value_t = 1)]
    pub padding: usize,

    /// Specify global indent for table.
    #[arg(short, long, default_value_t = 0)]
    pub indent: usize,

    /// Limit column widths sniffing to the specified number of rows. Specify "0" to cancel limit.
    #[arg(long, default_value_t = 1000, name = "LIMIT")]
    pub sniff: usize,

    /// Specify the alignment of the table header.
    #[arg(long, value_enum, default_value_t = Alignment::Center, ignore_case = true)]
    pub header_align: Alignment,

    /// Specify the alignment of the table body.
    #[arg(long, value_enum, default_value_t = Alignment::Left, ignore_case = true)]
    pub body_align: Alignment,

    #[cfg(all(feature = "pager", unix))]
    /// Disable pager.
    #[arg(long, short = 'P')]
    pub disable_pager: bool,
}

#[derive(Copy, Clone, ValueEnum)]
pub enum TableStyle {
    None,
    Ascii,
    Ascii2,
    Sharp,
    Rounded,
    Reinforced,
    Markdown,
    Grid,
}

#[derive(Copy, Clone, ValueEnum)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

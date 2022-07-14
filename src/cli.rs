use std::path::PathBuf;

use clap::{AppSettings, Parser, ValueHint};
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

#[derive(Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(disable_help_subcommand = true)]
#[clap(next_line_help = true)]
pub struct App {
    /// File to view.
    #[clap(name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// Specify that the input has no header row.
    #[clap(short = 'H', long = "no-headers")]
    pub no_headers: bool,

    /// Prepend a column of line numbers to the table.
    #[clap(short, long, alias = "seq")]
    pub number: bool,

    /// Use '\t' as delimiter for tsv.
    #[clap(short, long, conflicts_with = "delimiter")]
    pub tsv: bool,

    /// Specify the field delimiter.
    #[clap(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// Specify the border style.
    #[clap(short, long, default_value_t = TableStyle::Sharp, possible_values = TableStyle::VARIANTS, ignore_case = true)]
    pub style: TableStyle,

    /// Specify padding for table cell.
    #[clap(short, long, default_value_t = 1)]
    pub padding: usize,

    /// Specify global indent for table.
    #[clap(short, long, default_value_t = 0)]
    pub indent: usize,

    /// Limit column widths sniffing to the specified number of rows. Specify "0" to cancel limit.
    #[clap(long, default_value_t = 1000, name = "LIMIT")]
    pub sniff: usize,
}

#[derive(Display, EnumString, EnumVariantNames)]
#[strum(ascii_case_insensitive)]
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

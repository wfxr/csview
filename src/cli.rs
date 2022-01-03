use std::path::PathBuf;

use clap::{AppSettings, Parser, ValueHint};
use clap_complete::Shell;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

#[derive(Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
pub struct App {
    /// File to read.
    #[clap(name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,

    /// Specify that the input has no header row.
    #[clap(short = 'H', long = "no-headers")]
    pub no_headers: bool,

    /// Use '\t' as delimiter for tsv.
    #[clap(short, long, conflicts_with = "delimiter")]
    pub tsv: bool,

    /// Specify the field delimiter.
    #[clap(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// Specify the border style.
    #[clap(short, long, default_value = Style::VARIANTS[1], possible_values = Style::VARIANTS, ignore_case = true)]
    pub style: Style,

    /// Specify padding for table cell.
    #[clap(short, long, default_value_t = 1)]
    pub padding: usize,

    /// Specify global indent for table.
    #[clap(short, long, default_value_t = 0)]
    pub indent: usize,

    /// Limit column widths sniffing to the specified number of rows. Specify "0" to cancel limit.
    #[clap(long, default_value_t = 1000, name = "LIMIT")]
    pub sniff: usize,

    /// Subcommand
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Parser)]
pub enum Subcommand {
    /// Generate shell completion file.
    Completion {
        /// Target shell name.
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Display, EnumString, EnumVariantNames, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[strum(ascii_case_insensitive)]
pub enum Style {
    None,
    Ascii,
    Sharp,
    Rounded,
    Reinforced,
    Markdown,
    Grid,
}

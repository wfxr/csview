use std::path::PathBuf;

use clap::{AppSettings, Parser};
use clap_generate::Shell;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

#[derive(Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
pub struct App {
    /// File to read
    #[clap(name = "FILE")]
    pub file: Option<PathBuf>,

    /// Specify that the input has no header row.
    #[clap(short = 'H', long = "no-headers")]
    pub no_headers: bool,

    /// Use '\t' as delimiter for tsv
    #[clap(short = 't', long = "tsv", conflicts_with = "delimiter")]
    pub tsv: bool,

    /// Specify the field delimiter
    #[clap(short = 'd', long = "delimiter", default_value = ",")]
    pub delimiter: char,

    /// Specify the border style
    #[clap(long = "style", default_value = Border::VARIANTS[1], possible_values = Border::VARIANTS, ignore_case = true)]
    pub border: Border,

    /// Subcommand
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Parser)]
pub enum Subcommand {
    /// Generate shell completion file
    Completion {
        /// Target shell name
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Display, EnumString, EnumVariantNames, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[strum(ascii_case_insensitive)]
pub enum Border {
    None,
    Ascii,
    Sharp,
    Rounded,
    Reinforced,
    Markdown,
    Grid,
}

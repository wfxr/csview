use std::path::PathBuf;

use structopt::clap::{self, arg_enum, AppSettings};
pub use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(global_settings(&[AppSettings::ColoredHelp]))]
pub struct Opt {
    /// File to read
    #[structopt(name = "FILE")]
    pub file: Option<PathBuf>,

    /// Specify that the input has no header row.
    #[structopt(short = "H", long = "no-headers")]
    pub no_headers: bool,

    /// Use '\t' as delimiter for tsv, overrides '-d' option
    #[structopt(short = "t", long = "tsv")]
    pub tsv: bool,

    /// Specify the field delimiter
    #[structopt(short = "d", long = "delimiter", default_value = ",")]
    pub delimiter: char,

    /// Specify the border style
    #[structopt(long = "style", default_value = "Ascii", possible_values = &Border::variants(), case_insensitive = true)]
    pub border: Border,

    /// Subcommand
    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(StructOpt)]
pub enum Subcommand {
    /// Generate shell completion file
    Completion(CompletionOpt),
}

#[derive(StructOpt)]
pub struct CompletionOpt {
    /// Target shell name
    #[structopt(possible_values = &clap::Shell::variants())]
    pub shell: clap::Shell,
}

arg_enum! {
    pub enum Border {
        None,
        Ascii,
        Sharp,
        Rounded,
        Reinforced,
        Markdown,
    }
}

use std::path::PathBuf;

use structopt::clap::{self, arg_enum, AppSettings};
pub use structopt::StructOpt;

use prettytable::format::{self, FormatBuilder, LinePosition, LineSeparator};

#[derive(StructOpt)]
#[structopt(global_settings(&[AppSettings::ColoredHelp]))]
pub struct Opt {
    /// File to read
    #[structopt(name = "FIEL")]
    pub file: Option<PathBuf>,

    /// Set if csv has no title
    #[structopt(short = "H", long = "no-headers")]
    pub no_headers: bool,

    /// Use '\t' as delimiter for tsv, overrides '-d' option
    #[structopt(short = "t", long = "tsv")]
    pub tsv: bool,

    /// Field delimiter
    #[structopt(short = "d", long = "delimiter", default_value = ",")]
    pub delimiter: char,

    /// Border style
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

    /// Check for updates
    Update,
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
    }
}

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
        }
    }
}

/// This should be called before calling any cli method or printing any output.
pub fn reset_signal_pipe_handler() {
    #[cfg(target_family = "unix")]
    {
        use nix::sys::signal;
        unsafe {
            signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl).unwrap();
        }
    }
}

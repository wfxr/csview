use std::path::PathBuf;

use structopt::clap::{self, AppSettings};
pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
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

    /// Subcommand
    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    /// Generate shell completion file
    Completion(CompletionOpt),

    /// Check for updates
    Update,
}

#[derive(StructOpt, Debug)]
pub struct CompletionOpt {
    /// Target shell name
    pub shell: clap::Shell,
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

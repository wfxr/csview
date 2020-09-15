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
    #[structopt(long = "no-title")]
    pub no_title: bool,

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
    // #[structopt(name = "completion")]
    Completion(CompletionOpt),

    /// Check for updates
    // #[structopt(name = "update")]
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

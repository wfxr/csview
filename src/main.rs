mod cli;
mod core;
use cli::{CompletionOpt, Opt, StructOpt, Subcommand};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::reset_signal_pipe_handler();
    let opt: Opt = Opt::from_args();
    match opt.subcommand {
        Some(Subcommand::Completion(CompletionOpt { shell })) => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
        }
        Some(Subcommand::Update) => todo!("self update"),
        None => {
            let reader: Box<dyn BufRead> = match opt.file {
                Some(path) => Box::new(BufReader::new(File::open(path)?)),
                None => Box::new(BufReader::new(io::stdin())),
            };
            let delimiter = if opt.tsv { '\t' } else { opt.delimiter };
            core::print_csv(reader, !opt.no_headers, delimiter);
        }
    }
    Ok(())
}

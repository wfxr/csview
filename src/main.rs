mod cli;
mod core;
mod style;
use cli::{CompletionOpt, Opt, StructOpt, Subcommand};
use csv::ErrorKind;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn main() {
    if let Err(e) = try_main() {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                process::exit(0);
            }
        }

        if let Some(csverr) = e.root_cause().downcast_ref::<csv::Error>() {
            match csverr.kind() {
                ErrorKind::Utf8 { .. } => {
                    eprintln!("[error] input is not utf8 encoded");
                    process::exit(exitcode::DATAERR)
                }
                ErrorKind::UnequalLengths { pos, expected_len, len } => {
                    let pos_info = pos
                        .as_ref()
                        .map(|p| format!(" at (byte: {}, line: {}, record: {})", p.byte(), p.line(), p.record()))
                        .unwrap_or_else(|| "".to_string());
                    eprintln!(
                        "[error] unequal lengths{}: expected length is {}, but got {}",
                        pos_info, expected_len, len
                    );
                    process::exit(exitcode::DATAERR)
                }
                ErrorKind::Io(e) => {
                    eprintln!("[error] io error: {}", e);
                    process::exit(exitcode::IOERR)
                }
                e => {
                    eprintln!("[error] failed to process input: {:?}", e);
                    process::exit(exitcode::SOFTWARE)
                }
            }
        }

        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
        std::process::exit(1)
    }
}

fn try_main() -> anyhow::Result<()> {
    let opt: Opt = Opt::from_args();
    match opt.subcommand {
        Some(Subcommand::Completion(CompletionOpt { shell })) => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
        }
        None => {
            let reader: Box<dyn BufRead> = match opt.file {
                Some(path) => Box::new(BufReader::new(File::open(path)?)),
                None => Box::new(BufReader::new(io::stdin())),
            };
            let delimiter = if opt.tsv { '\t' } else { opt.delimiter };
            core::print(reader, !opt.no_headers, delimiter, opt.border.into())?;
        }
    }
    Ok(())
}

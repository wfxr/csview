#![feature(iter_intersperse)]

mod cli;
mod core;
mod style;
mod table;

use clap::{IntoApp, Parser};
use cli::{App, Subcommand};
use csv::ErrorKind;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

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
    let app: App = App::parse();
    match app.subcommand {
        Some(Subcommand::Completion { shell }) => {
            let app = &mut App::into_app();
            clap_complete::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
        None => {
            let reader: Box<dyn BufRead> = match app.file {
                Some(path) => Box::new(BufReader::new(File::open(path)?)),
                None => Box::new(BufReader::new(io::stdin())),
            };
            let delimiter = if app.tsv { '\t' } else { app.delimiter };
            core::print(reader, !app.no_headers, delimiter, app.border.into())?;
        }
    }
    Ok(())
}

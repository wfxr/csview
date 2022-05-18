mod cli;
mod table;
mod util;

use clap::Parser;
use cli::App;
use csv::{ErrorKind, ReaderBuilder};
use std::{
    fs::File,
    io::{self, BufWriter, Read},
    process,
};
use table::Table;
use util::table_style;

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
    let App {
        file,
        no_headers,
        number,
        tsv,
        delimiter,
        style,
        padding,
        indent,
        sniff,
    } = App::parse();

    let stdout = io::stdout();
    let wtr = &mut BufWriter::new(stdout.lock());
    let rdr = ReaderBuilder::new()
        .delimiter(if tsv { b'\t' } else { delimiter as u8 })
        .has_headers(!no_headers)
        .from_reader(match file {
            Some(path) => Box::new(File::open(path)?) as Box<dyn Read>,
            None => Box::new(io::stdin()),
        });

    let sniff = if sniff == 0 { usize::MAX } else { sniff };
    let table = Table::new(rdr, sniff, number)?;
    table.writeln(wtr, &table_style(style, padding, indent))?;
    Ok(())
}

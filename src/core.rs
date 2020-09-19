use csv::{self, ErrorKind, ReaderBuilder, StringRecord};
use prettytable::{format, Cell, Row, Table};
use std::io::BufRead;
use std::process;

pub fn print_csv(reader: Box<dyn BufRead>, has_headers: bool, delimiter: char, style: format::TableFormat) {
    let csv_reader = &mut ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(has_headers)
        .from_reader(reader);
    let mut table = Table::init(csv_reader.records().map(record_to_row).collect());

    if has_headers {
        table.set_titles(record_to_row(csv_reader.headers().map(|r| r.to_owned())));
    }
    table.set_format(style);
    table.printstd();
}

fn record_to_row(record: csv::Result<StringRecord>) -> Row {
    match record {
        Ok(record) => Row::new(record.into_iter().map(|cell| Cell::new(&cell)).collect()),
        Err(error) => match error.kind() {
            ErrorKind::Utf8 { .. } => {
                eprintln!("[error] input is not utf8 encoded");
                process::exit(exitcode::DATAERR)
            }
            ErrorKind::UnequalLengths { pos, expected_len, len } => {
                let pos_info = if let Some(p) = pos {
                    format!(" at (byte: {}, line: {}, record: {})", p.byte(), p.line(), p.record())
                } else {
                    "".to_string()
                };
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
        },
    }
}

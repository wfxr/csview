use csv::{self, ErrorKind, ReaderBuilder, StringRecord};
use prettytable::{format, Cell, Row, Table};
use std::io::BufRead;

pub fn print_csv(reader: Box<dyn BufRead>, has_headers: bool, delimiter: char, style: format::TableFormat) {
    let csv_reader = &mut ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(has_headers)
        .from_reader(reader);
    let mut table = Table::init(csv_reader.records().map(|record| record_to_row(record)).collect());

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
                eprintln!("[error] input is non-utf8 encoding");
                std::process::exit(1)
            }
            ErrorKind::UnequalLengths { pos, expected_len, len } => {
                let pos_info = if let Some(pos) = pos {
                    format!(
                        " at (byte: {}, line: {}, record: {})",
                        pos.byte(),
                        pos.line(),
                        pos.record()
                    )
                } else {
                    "".to_string()
                };
                eprintln!(
                    "[error] unequal lengths{}: expected length is {}, but got {}",
                    pos_info, expected_len, len
                );
                std::process::exit(2)
            }
            e => {
                eprintln!("[error] failed to reading rows: {:?}", e);
                std::process::exit(3)
            }
        },
    }
}

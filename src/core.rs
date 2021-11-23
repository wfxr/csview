use csv::{self, ReaderBuilder, StringRecord};
use prettytable::{format, Cell, Row, Table};
use std::io::BufRead;

pub fn print(
    reader: impl BufRead,
    has_headers: bool,
    delimiter: char,
    style: format::TableFormat,
) -> anyhow::Result<()> {
    let csv_reader = &mut ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(has_headers)
        .from_reader(reader);
    let rows: Vec<_> = csv_reader
        .records()
        .map(|r| r.map(|r| record_to_row(&r)))
        .collect::<csv::Result<_>>()?;
    let mut table = Table::init(rows);

    if has_headers {
        let header = csv_reader.headers()?;
        table.set_titles(record_to_row(header));
    }
    table.set_format(style);
    table.print_tty(false)?;
    Ok(())
}

fn record_to_row(record: &StringRecord) -> Row {
    Row::new(record.iter().map(Cell::new).collect())
}

use prettytable::{csv::ReaderBuilder, format, Cell, Row, Table};
use std::io::BufRead;
use csv::StringRecord;

pub fn print_csv(reader: Box<dyn BufRead>, has_headers: bool, delimiter: char, style: format::TableFormat) {
    let csv_reader = &mut ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(has_headers)
        .from_reader(reader);
    let mut table = Table::init(
        csv_reader
            .records()
            .map(|row| Row::new(row.unwrap_or(StringRecord::from(vec!["utf8 error"])).into_iter().map(|cell| Cell::new(&cell)).collect()))
            .collect(),
    );
    if has_headers {
        table.set_titles(
            csv_reader
                .headers()
                .unwrap()
                .into_iter()
                .map(|cell| Cell::new(&cell))
                .collect(),
        );
    }
    table.set_format(style);
    table.printstd();
}

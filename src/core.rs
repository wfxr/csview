use prettytable::{csv::ReaderBuilder, format, Cell, Row, Table};
use std::io::BufRead;

pub fn print_csv(reader: Box<dyn BufRead>, has_headers: bool) {
    let reader = &mut ReaderBuilder::new().has_headers(has_headers).from_reader(reader);
    let mut table = Table::init(
        reader
            .records()
            .map(|row| Row::new(row.unwrap().into_iter().map(|cell| Cell::new(&cell)).collect()))
            .collect(),
    );
    if has_headers {
        table.set_titles(
            reader
                .headers()
                .unwrap()
                .into_iter()
                .map(|cell| Cell::new(&cell))
                .collect(),
        );
    }
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
}

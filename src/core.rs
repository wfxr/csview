use crate::table::{CsvTableWriter, TableFormat};
use csv::{self, ReaderBuilder};
use std::io::Read;

pub fn print(
    reader: impl Read + 'static,
    has_headers: bool,
    delimiter: char,
    style: TableFormat,
) -> anyhow::Result<()> {
    let rdr = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(has_headers)
        .from_reader(reader);
    let table = CsvTableWriter::new(rdr, 100)?;
    table.writeln(&mut std::io::stdout(), &style)?;
    Ok(())
}

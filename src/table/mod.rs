mod row;
mod style;

use csv::{Reader, StringRecord};
use row::Row;
use std::io::{self, Result, Write};
use unicode_width::UnicodeWidthStr;

pub use style::{RowSep, Style, StyleBuilder};

pub struct Table {
    header:  Option<StringRecord>,
    widths:  Vec<usize>,
    records: Box<dyn Iterator<Item = csv::Result<StringRecord>>>,
}

impl Table {
    pub(crate) fn new<R: 'static + io::Read>(mut rdr: Reader<R>, sniff_rows: usize) -> Result<Self> {
        let header = rdr.has_headers().then(|| rdr.headers()).transpose()?.cloned();
        let (widths, buf) = sniff_widths(&mut rdr, header.as_ref(), sniff_rows)?;
        let records = Box::new(buf.into_iter().map(Ok).chain(rdr.into_records()));
        Ok(Self { header, widths, records })
    }

    pub(crate) fn writeln<W: Write>(self, wtr: &mut W, fmt: &Style) -> Result<()> {
        let widths = &self.widths;
        fmt.rowseps
            .top
            .map(|sep| fmt.write_row_sep(wtr, widths, &sep))
            .transpose()?;

        let mut iter = self.records.peekable();

        if let Some(header) = self.header {
            let row: Row = header.into_iter().collect();
            row.writeln(wtr, fmt, widths)?;
            if iter.peek().is_some() {
                fmt.rowseps
                    .snd
                    .map(|sep| fmt.write_row_sep(wtr, widths, &sep))
                    .transpose()?;
            }
        }

        while let Some(record) = iter.next().transpose()? {
            let row: Row = record.into_iter().collect();
            row.writeln(wtr, fmt, widths)?;
            if let Some(mid) = &fmt.rowseps.mid {
                if iter.peek().is_some() {
                    fmt.write_row_sep(wtr, widths, mid)?;
                }
            }
        }

        fmt.rowseps
            .bot
            .map(|sep| fmt.write_row_sep(wtr, widths, &sep))
            .transpose()?;

        wtr.flush()
    }
}

fn sniff_widths<R: io::Read>(
    rdr: &mut Reader<R>,
    header: Option<&StringRecord>,
    mut sniff_rows: usize,
) -> Result<(Vec<usize>, Vec<StringRecord>)> {
    let sniff = |record: &StringRecord, widths: &mut Vec<usize>| {
        widths.resize(record.len(), 0);
        record
            .iter()
            .map(UnicodeWidthStr::width_cjk)
            .enumerate()
            .for_each(|(i, width)| widths[i] = widths[i].max(width))
    };

    let mut widths = Vec::new();
    let mut buf = Vec::new();

    let mut record = header.cloned().unwrap_or_default();
    sniff(&record, &mut widths);
    sniff_rows -= 1;

    while sniff_rows > 0 && rdr.read_record(&mut record)? {
        sniff(&record, &mut widths);
        sniff_rows -= 1;
        buf.push(record.clone());
    }
    Ok((widths, buf))
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use csv::ReaderBuilder;

    #[test]
    fn test_write() -> Result<()> {
        let text = "a,b,c\n1,2,3\n4,5,6";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3)?;

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &Style::default())?;

        assert_eq!(
            "
+---+---+---+
| a | b | c |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
"
            .trim_start(),
            std::str::from_utf8(&mut buf)?
        );
        Ok(())
    }

    #[test]
    fn test_write_without_padding() -> Result<()> {
        let text = "a,b,c\n1,2,3\n4,5,6";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3)?;
        let fmt = StyleBuilder::default().padding(0).build();

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &fmt)?;

        assert_eq!(
            "
+-+-+-+
|a|b|c|
+-+-+-+
|1|2|3|
+-+-+-+
|4|5|6|
+-+-+-+
"
            .trim_start(),
            std::str::from_utf8(&mut buf)?
        );
        Ok(())
    }

    #[test]
    fn test_write_with_indent() -> Result<()> {
        let text = "a,b,c\n1,2,3\n4,5,6";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3)?;
        let fmt = StyleBuilder::default().indent(4).build();

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &fmt)?;

        assert_eq!(
            "
    +---+---+---+
    | a | b | c |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
"
            .trim_start_matches(|c: char| c == '\n'),
            std::str::from_utf8(&mut buf)?
        );
        Ok(())
    }

    #[test]
    fn test_only_header() -> Result<()> {
        let text = "a,ab,abc";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3)?;
        let fmt = Style::default();

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &fmt)?;

        assert_eq!(
            "
+---+----+-----+
| a | ab | abc |
+---+----+-----+
"
            .trim_start_matches(|c: char| c == '\n'),
            std::str::from_utf8(&mut buf)?
        );
        Ok(())
    }

    #[test]
    fn test_without_header() -> Result<()> {
        let text = "1,123,35\n383,2, 17";
        let rdr = ReaderBuilder::new().has_headers(false).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3)?;
        let fmt = StyleBuilder::new()
            .col_sep('│')
            .row_seps(
                RowSep::new('─', '╭', '┬', '╮'),
                RowSep::new('─', '├', '┼', '┤'),
                None,
                RowSep::new('─', '╰', '┴', '╯'),
            )
            .build();

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &fmt)?;

        assert_eq!(
            "
╭─────┬─────┬─────╮
│ 1   │ 123 │ 35  │
│ 383 │ 2   │  17 │
╰─────┴─────┴─────╯
"
            .trim_start_matches(|c: char| c == '\n'),
            std::str::from_utf8(&mut buf)?
        );
        Ok(())
    }
}

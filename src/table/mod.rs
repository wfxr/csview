mod row;
mod style;

use csv::{Reader, StringRecord};
use row::Row;
use std::{
    io::{self, Result, Write},
    iter,
};
use unicode_width::UnicodeWidthStr;

pub use style::{RowSep, Style, StyleBuilder};

pub struct Table {
    header:   Option<StringRecord>,
    widths:   Vec<usize>,
    records:  Box<dyn Iterator<Item = csv::Result<StringRecord>>>,
    with_seq: bool,
}

impl Table {
    pub(crate) fn new<R: 'static + io::Read>(mut rdr: Reader<R>, sniff_rows: usize, with_seq: bool) -> Result<Self> {
        let header = rdr.has_headers().then(|| rdr.headers()).transpose()?.cloned();
        let (widths, buf) = sniff_widths(&mut rdr, header.as_ref(), sniff_rows, with_seq)?;
        let records = Box::new(buf.into_iter().map(Ok).chain(rdr.into_records()));
        Ok(Self { header, widths, records, with_seq })
    }

    pub(crate) fn writeln<W: Write>(self, wtr: &mut W, fmt: &Style) -> Result<()> {
        let widths = &self.widths;
        fmt.rowseps
            .top
            .map(|sep| fmt.write_row_sep(wtr, widths, &sep))
            .transpose()?;

        let mut iter = self.records.peekable();

        if let Some(header) = self.header {
            let row: Row = match self.with_seq {
                true => iter::once("#").chain(header.into_iter()).collect(),
                false => header.into_iter().collect(),
            };
            row.writeln(wtr, fmt, widths)?;
            if iter.peek().is_some() {
                fmt.rowseps
                    .snd
                    .map(|sep| fmt.write_row_sep(wtr, widths, &sep))
                    .transpose()?;
            }
        }

        let mut seq = 1;
        while let Some(record) = iter.next().transpose()? {
            match self.with_seq {
                true => {
                    let seq = seq.to_string();
                    let row: Row = iter::once(seq.as_ref()).chain(record.into_iter()).collect();
                    row.writeln(wtr, fmt, widths)?;
                }
                false => {
                    let row: Row = record.into_iter().collect();
                    row.writeln(wtr, fmt, widths)?;
                }
            };
            if let Some(mid) = &fmt.rowseps.mid {
                if iter.peek().is_some() {
                    fmt.write_row_sep(wtr, widths, mid)?;
                }
            }
            seq += 1;
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
    sniff_rows: usize,
    with_seq: bool,
) -> Result<(Vec<usize>, Vec<StringRecord>)> {
    let mut widths = Vec::new();
    let mut buf = Vec::new();

    let mut record = header.cloned().unwrap_or_default();
    update_widths(&record, &mut widths, with_seq.then_some("#"));

    let mut seq = 1;
    while seq <= sniff_rows && rdr.read_record(&mut record)? {
        update_widths(&record, &mut widths, with_seq.then(|| seq.to_string()));
        seq += 1;
        buf.push(record.clone());
    }
    Ok((widths, buf))
}

fn update_widths<S: AsRef<str>>(record: &StringRecord, widths: &mut Vec<usize>, seq: Option<S>) {
    let fields = seq.iter().map(|s| s.as_ref()).chain(record);
    let total_len = record.len() + seq.is_some() as usize;
    widths.resize(total_len, 0);
    fields
        .map(UnicodeWidthStr::width_cjk)
        .enumerate()
        .for_each(|(i, width)| widths[i] = widths[i].max(width))
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use csv::ReaderBuilder;

    macro_rules! gen_table {
        ($($line:expr)*) => {
            concat!(
                $($line, "\n",)*
            )
        };
    }

    #[test]
    fn test_write() -> Result<()> {
        let text = "a,b,c\n1,2,3\n4,5,6";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3, false)?;

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &Style::default())?;

        assert_eq!(
            gen_table!(
                "+---+---+---+"
                "| a | b | c |"
                "+---+---+---+"
                "| 1 | 2 | 3 |"
                "+---+---+---+"
                "| 4 | 5 | 6 |"
                "+---+---+---+"
            ),
            std::str::from_utf8(&buf)?
        );
        Ok(())
    }

    #[test]
    fn test_write_without_padding() -> Result<()> {
        let text = "a,b,c\n1,2,3\n4,5,6";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3, false)?;
        let fmt = StyleBuilder::default().padding(0).build();

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &fmt)?;

        assert_eq!(
            gen_table!(
                "+-+-+-+"
                "|a|b|c|"
                "+-+-+-+"
                "|1|2|3|"
                "+-+-+-+"
                "|4|5|6|"
                "+-+-+-+"
            ),
            std::str::from_utf8(&buf)?
        );
        Ok(())
    }

    #[test]
    fn test_write_with_indent() -> Result<()> {
        let text = "a,b,c\n1,2,3\n4,5,6";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3, false)?;
        let fmt = StyleBuilder::default().indent(4).build();

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &fmt)?;

        assert_eq!(
            gen_table!(
                "    +---+---+---+"
                "    | a | b | c |"
                "    +---+---+---+"
                "    | 1 | 2 | 3 |"
                "    +---+---+---+"
                "    | 4 | 5 | 6 |"
                "    +---+---+---+"
            ),
            std::str::from_utf8(&buf)?
        );
        Ok(())
    }

    #[test]
    fn test_only_header() -> Result<()> {
        let text = "a,ab,abc";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3, false)?;
        let fmt = Style::default();

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &fmt)?;

        assert_eq!(
            gen_table!(
                "+---+----+-----+"
                "| a | ab | abc |"
                "+---+----+-----+"
            ),
            std::str::from_utf8(&buf)?
        );
        Ok(())
    }

    #[test]
    fn test_without_header() -> Result<()> {
        let text = "1,123,35\n383,2, 17";
        let rdr = ReaderBuilder::new().has_headers(false).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3, false)?;
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
            gen_table!(
                "╭─────┬─────┬─────╮"
                "│ 1   │ 123 │ 35  │"
                "│ 383 │ 2   │  17 │"
                "╰─────┴─────┴─────╯"
            ),
            std::str::from_utf8(&buf)?
        );
        Ok(())
    }

    #[test]
    fn test_with_seq() -> Result<()> {
        let text = "a,b,c\n1,2,3\n4,5,6";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());
        let wtr = Table::new(rdr, 3, true)?;

        let mut buf = Vec::new();
        wtr.writeln(&mut buf, &Style::default())?;

        assert_eq!(
            gen_table!(
                "+---+---+---+---+"
                "| # | a | b | c |"
                "+---+---+---+---+"
                "| 1 | 1 | 2 | 3 |"
                "+---+---+---+---+"
                "| 2 | 4 | 5 | 6 |"
                "+---+---+---+---+"
            ),
            std::str::from_utf8(&buf)?
        );
        Ok(())
    }
}

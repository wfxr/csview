use super::{row::Row, style::Style};
use csv::{Reader, StringRecord};
use std::io::{self, Result, Write};
use unicode_width::UnicodeWidthStr;

pub struct TablePrinter {
    style: Style,
    header: Option<StringRecord>,
    widths: Vec<usize>,
    records: Box<dyn Iterator<Item = csv::Result<StringRecord>>>,
    with_seq: bool,
}

impl TablePrinter {
    pub(crate) fn new<R: 'static + io::Read>(
        mut rdr: Reader<R>,
        sniff_rows: usize,
        with_seq: bool,
        style: Style,
    ) -> Result<Self> {
        let header = rdr.has_headers().then(|| rdr.headers()).transpose()?.cloned();
        let header = header.map(|record| prepare_record(record, &style));
        let (widths, buf) = sniff_widths(&mut rdr, header.as_ref(), sniff_rows, with_seq, &style)?;
        let records = Box::new(
            buf.into_iter().map(Ok).chain(
                rdr.into_records()
                    .map(move |record| record.map(|record| prepare_record(record, &style))),
            ),
        );
        Ok(Self { style, header, widths, records, with_seq })
    }

    pub(crate) fn writeln<W: Write>(self, wtr: &mut W) -> Result<()> {
        let fmt = &self.style;
        let widths = &self.widths;
        fmt.rowseps
            .top
            .map(|sep| fmt.write_row_sep(wtr, widths, &sep))
            .transpose()?;

        let mut iter = self.records.peekable();

        if let Some(header) = self.header {
            let row: Row = self.with_seq.then_some("#").into_iter().chain(header.iter()).collect();
            row.writeln(wtr, fmt, widths, fmt.header_align)?;
            if iter.peek().is_some() {
                fmt.rowseps
                    .snd
                    .map(|sep| fmt.write_row_sep(wtr, widths, &sep))
                    .transpose()?;
            }
        }

        let mut seq = 1;
        while let Some(record) = iter.next().transpose()? {
            let seq_str = self.with_seq.then(|| seq.to_string());
            let row: Row = seq_str.iter().map(|s| s.as_str()).chain(record.into_iter()).collect();
            row.writeln(wtr, fmt, widths, fmt.body_align)?;
            if let Some(mid) = &fmt.rowseps.mid
                && iter.peek().is_some()
            {
                fmt.write_row_sep(wtr, widths, mid)?;
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

// Markdown needs one physical line per record; terminal styles retain cell lines.
fn prepare_record(record: StringRecord, style: &Style) -> StringRecord {
    if style.html_line_breaks && record.iter().any(|cell| cell.contains('\n')) {
        record
            .iter()
            .map(|cell| cell.replace("\r\n", "\n").replace('\n', "<br>"))
            .collect()
    } else {
        record
    }
}

fn sniff_widths<R: io::Read>(
    rdr: &mut Reader<R>,
    header: Option<&StringRecord>,
    sniff_rows: usize,
    with_seq: bool,
    style: &Style,
) -> Result<(Vec<usize>, Vec<StringRecord>)> {
    let mut widths = Vec::new();
    let mut buf = Vec::new();

    fn update_widths(record: &StringRecord, widths: &mut Vec<usize>) {
        widths.resize(record.len(), 0);
        record
            .into_iter()
            .map(|cell| {
                cell.split('\n')
                    .map(|line| line.strip_suffix('\r').unwrap_or(line).width_cjk())
                    .max()
                    .unwrap_or(0)
            })
            .enumerate()
            .for_each(|(i, width)| widths[i] = widths[i].max(width))
    }

    let mut record = header.cloned().unwrap_or_default();
    update_widths(&record, &mut widths);

    let mut seq = 1;
    while seq <= sniff_rows && rdr.read_record(&mut record)? {
        record = prepare_record(record, style);
        update_widths(&record, &mut widths);
        seq += 1;
        buf.push(record.clone());
    }

    if with_seq {
        widths.insert(0, seq.to_string().width());
    }
    Ok((widths, buf))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::table::{RowSep, StyleBuilder};
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
    fn multiline_cells_and_headers() -> Result<()> {
        let text = "\"a\nb\",c\n\"中\r\n文\n\",x\nz,\"1\n2\"\n";
        let rdr = ReaderBuilder::new().from_reader(text.as_bytes());
        let printer = TablePrinter::new(rdr, 1, true, Style::default())?;
        let mut buf = Vec::new();
        printer.writeln(&mut buf)?;
        assert_eq!(
            gen_table!(
                "+---+----+---+"
                "| # | a  | c |"
                "|   | b  |   |"
                "+---+----+---+"
                "| 1 | 中 | x |"
                "|   | 文 |   |"
                "|   |    |   |"
                "+---+----+---+"
                "| 2 | z  | 1 |"
                "|   |    | 2 |"
                "+---+----+---+"
            ),
            std::str::from_utf8(&buf)?
        );
        Ok(())
    }

    #[test]
    fn markdown_keeps_one_line_per_record() -> Result<()> {
        use crate::cli::{Alignment, TableStyle};
        let style = crate::util::table_style(TableStyle::Markdown, 1, 0, Alignment::Left, Alignment::Left);
        let text = "\"a\nb\",c\n\"1\r\n2\",x\nz,\"3\n4\"\n";
        let rdr = ReaderBuilder::new().from_reader(text.as_bytes());
        let printer = TablePrinter::new(rdr, 2, false, style)?;
        let mut buf = Vec::new();
        printer.writeln(&mut buf)?;
        assert_eq!(
            gen_table!(
                "| a<br>b | c      |"
                "|--------|--------|"
                "| 1<br>2 | x      |"
                "| z      | 3<br>4 |"
            ),
            std::str::from_utf8(&buf)?
        );
        Ok(())
    }

    #[test]
    fn markdown_normalizes_records_after_sniffing() -> Result<()> {
        let style = StyleBuilder::new().html_line_breaks(true).build();
        let text = "long header\nfirst\n\"a\nb\"\n";
        let rdr = ReaderBuilder::new().has_headers(false).from_reader(text.as_bytes());
        let printer = TablePrinter::new(rdr, 1, false, style)?;
        let mut buf = Vec::new();
        printer.writeln(&mut buf)?;
        assert!(std::str::from_utf8(&buf)?.contains("| a<br>b      |\n"));
        Ok(())
    }

    #[test]
    fn test_write() -> Result<()> {
        let text = "a,b,c\n1,2,3\n4,5,6";
        let rdr = ReaderBuilder::new().has_headers(true).from_reader(text.as_bytes());

        let mut buf = Vec::new();
        let wtr = TablePrinter::new(rdr, 3, false, Style::default())?;
        wtr.writeln(&mut buf)?;

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
        let fmt = StyleBuilder::default().padding(0).build();

        let mut buf = Vec::new();
        let wtr = TablePrinter::new(rdr, 3, false, fmt)?;
        wtr.writeln(&mut buf)?;

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
        let fmt = StyleBuilder::default().indent(4).build();

        let mut buf = Vec::new();
        let wtr = TablePrinter::new(rdr, 3, false, fmt)?;
        wtr.writeln(&mut buf)?;

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
        let fmt = Style::default();

        let mut buf = Vec::new();
        let wtr = TablePrinter::new(rdr, 3, false, fmt)?;
        wtr.writeln(&mut buf)?;

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
        let wtr = TablePrinter::new(rdr, 3, false, fmt)?;
        wtr.writeln(&mut buf)?;

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

        let mut buf = Vec::new();
        let wtr = TablePrinter::new(rdr, 3, true, Style::default())?;
        wtr.writeln(&mut buf)?;

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

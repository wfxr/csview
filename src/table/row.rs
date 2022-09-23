use std::io::{Result, Write};

use itertools::Itertools;
use unicode_truncate::{Alignment, UnicodeTruncateStr};

use crate::table::Style;

/// Represent a table row made of cells
#[derive(Clone, Debug)]
pub struct Row<'a> {
    cells: Vec<&'a str>,
}

impl<'a> FromIterator<&'a str> for Row<'a> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        Self { cells: iter.into_iter().collect() }
    }
}

impl<'a> Row<'a> {
    pub fn write<T: Write>(&self, wtr: &mut T, fmt: &Style, widths: &[usize], align: Alignment) -> Result<()> {
        let sep = fmt.colseps.mid.map(|c| c.to_string()).unwrap_or_default();
        write!(wtr, "{:indent$}", "", indent = fmt.indent)?;
        fmt.colseps.lhs.map(|sep| fmt.write_col_sep(wtr, sep)).transpose()?;
        Itertools::intersperse(
            self.cells
                .iter()
                .zip(widths)
                .map(|(cell, &width)| cell.unicode_pad(width, align, true))
                .map(|s| format!("{:pad$}{}{:pad$}", "", s, "", pad = fmt.padding)),
            sep,
        )
        .try_for_each(|s| write!(wtr, "{}", s))?;
        fmt.colseps.rhs.map(|sep| fmt.write_col_sep(wtr, sep)).transpose()?;
        Ok(())
    }

    pub fn writeln<T: Write>(&self, wtr: &mut T, fmt: &Style, widths: &[usize], align: Alignment) -> Result<()> {
        self.write(wtr, fmt, widths, align).and_then(|_| writeln!(wtr))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use std::str;

    #[test]
    fn write_ascii_row() -> Result<()> {
        let row = Row::from_iter(["a", "b"]);
        let buf = &mut Vec::new();
        let fmt = Style::default();
        let widths = [3, 4];

        row.writeln(buf, &fmt, &widths, Alignment::Left)?;
        assert_eq!("| a   | b    |\n", str::from_utf8(buf)?);
        Ok(())
    }

    #[test]
    fn write_cjk_row() -> Result<()> {
        let row = Row::from_iter(["李磊(Jack)", "四川省成都市", "💍"]);
        let buf = &mut Vec::new();
        let fmt = Style::default();
        let widths = [10, 8, 2];

        row.writeln(buf, &fmt, &widths, Alignment::Left)?;
        assert_eq!("| 李磊(Jack) | 四川省成 | 💍 |\n", str::from_utf8(buf)?);
        Ok(())
    }

    #[test]
    fn write_align_center() -> Result<()> {
        let row = Row::from_iter(["a", "b"]);
        let buf = &mut Vec::new();
        let fmt = Style::default();
        let widths = [3, 4];

        row.writeln(buf, &fmt, &widths, Alignment::Center)?;
        assert_eq!("|  a  |  b   |\n", str::from_utf8(buf)?);
        Ok(())
    }

    #[test]
    fn write_align_right() -> Result<()> {
        let row = Row::from_iter(["a", "b"]);
        let buf = &mut Vec::new();
        let fmt = Style::default();
        let widths = [3, 4];

        row.writeln(buf, &fmt, &widths, Alignment::Right)?;
        assert_eq!("|   a |    b |\n", str::from_utf8(buf)?);
        Ok(())
    }
}

use anyhow::Result;
use std::io::Write;

use crate::table::{format::ColPos, Cell, TableFormat};

/// Represent a table row made of cells
#[derive(Clone, Debug)]
pub struct Row<'a> {
    cells: Vec<Cell<'a>>,
}

impl<'a> FromIterator<&'a str> for Row<'a> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        Self { cells: iter.into_iter().map(Cell::new).collect() }
    }
}

impl<'a> Row<'a> {
    pub fn new(cells: Vec<Cell<'a>>) -> Row<'a> {
        Row { cells }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn writeln<T: Write>(&self, wtr: &mut T, fmt: &TableFormat, widths: &[usize]) -> Result<()> {
        let sep = fmt.get_col_sep(ColPos::Mid).map(|c| c.to_string()).unwrap_or_default();

        write!(wtr, "{:indent$}", "", indent = fmt.indent)?;
        fmt.write_col_sep(wtr, ColPos::Lhs)?;
        self.cells
            .iter()
            .zip(widths)
            .map(|(cell, &width)| cell.truncate(width))
            .map(|s| format!("{:pad$}{s}{:pad$}", "", "", pad = fmt.padding))
            .intersperse(sep)
            .try_for_each(|s| write!(wtr, "{}", s))?;
        fmt.write_col_sep(wtr, ColPos::Rhs)?;

        writeln!(wtr)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn write_ascii_row() -> Result<()> {
        let row = Row::from_iter(["a", "b"]);
        let buf = &mut Vec::new();
        let fmt = TableFormat::default();
        let widths = [3, 4];

        row.writeln(buf, &fmt, &widths)?;
        assert_eq!("| a   | b    |\n", std::str::from_utf8(buf)?);
        Ok(())
    }

    #[test]
    fn write_cjk_row() -> Result<()> {
        let row = Row::from_iter(["李磊", "四川省成都市"]);
        let buf = &mut Vec::new();
        let fmt = TableFormat::default();
        let widths = [5, 8];

        row.writeln(buf, &fmt, &widths)?;
        assert_eq!("| 李磊  | 四川省成 |\n", std::str::from_utf8(buf)?);
        Ok(())
    }
}

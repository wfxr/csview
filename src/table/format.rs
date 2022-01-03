use anyhow::Result;
use std::io::Write;

/// Position of row separator
#[derive(Clone, Debug, PartialEq, Copy, Hash, Eq)]
pub enum RowPos {
    /// Top separator row (top border)
    /// ```
    /// >┌───┬───┐
    ///  │ a │ b │
    /// ```
    Top,
    /// Second separator row (between the header row and the first data row)
    /// ```
    ///  ┌───┬───┐
    ///  │ a │ b │
    /// >├───┼───┤
    /// ```
    Snd,
    /// Middle separator row (between data rows)
    /// ```
    /// >├───┼───┤
    ///  │ 2 │ 2 │
    /// >├───┼───┤
    /// ```
    Mid,
    /// Bottom separator row (bottom border)
    /// ```
    ///  │ 3 │ 3 │
    /// >└───┴───┘
    /// ```
    Bot,
}

/// Position of column separator
#[derive(Clone, Debug, PartialEq, Copy, Hash, Eq)]
pub enum ColPos {
    /// Left separator column (left border)
    /// ```
    ///  │ 1 │ 2 │
    ///  ^
    /// ```
    Lhs,
    /// Middle column separators
    /// ```
    ///  │ 1 │ 2 │
    ///      ^
    /// ```
    Mid,
    /// Right separator column (right border)
    /// ```
    ///  │ 1 │ 2 │
    ///          ^
    /// ```
    Rhs,
}

/// The characters used for printing a row separator
#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct RowSep {
    /// Normal row separator
    /// ```
    /// ┌───┬───┐
    ///   ^
    /// ```
    sep:   char,
    /// Left junction separator
    /// ```
    /// ┌───┬───┐
    /// ^
    /// ```
    ljunc: char,
    /// Crossing junction separator
    /// ```
    /// ┌───┬───┐
    ///     ^
    /// ```
    cjunc: char,
    /// Right junction separator
    /// ```
    /// ┌───┬───┐
    ///         ^
    /// ```
    rjunc: char,
}

impl RowSep {
    pub fn new(sep: char, ljunc: char, cjunc: char, rjunc: char) -> RowSep {
        RowSep { sep, ljunc, cjunc, rjunc }
    }
}

impl Default for RowSep {
    fn default() -> Self {
        RowSep::new('-', '+', '+', '+')
    }
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct RowSeps {
    /// Optional top line separator
    pub top: Option<RowSep>,
    /// Optional title line separator
    pub snd: Option<RowSep>,
    /// Optional internal row separator
    pub mid: Option<RowSep>,
    /// Optional bottom line separator
    pub bot: Option<RowSep>,
}

impl Default for RowSeps {
    fn default() -> Self {
        Self {
            top: Some(RowSep::default()),
            snd: Some(RowSep::default()),
            mid: Some(RowSep::default()),
            bot: Some(RowSep::default()),
        }
    }
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct ColSeps {
    /// Optional left border character
    pub lhs: Option<char>,
    /// Optional inner column separator character
    pub mid: Option<char>,
    /// Optional right border character
    pub rhs: Option<char>,
}

impl Default for ColSeps {
    fn default() -> Self {
        Self { lhs: Some('|'), mid: Some('|'), rhs: Some('|') }
    }
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub struct TableFormat {
    pub colseps: ColSeps,
    pub rowseps: RowSeps,

    /// Left and right padding
    pub padding: usize,
    /// Global indentation
    pub indent:  usize,
}

impl Default for TableFormat {
    fn default() -> Self {
        Self {
            indent:  0,
            padding: 1,
            colseps: ColSeps::default(),
            rowseps: RowSeps::default(),
        }
    }
}

impl TableFormat {
    fn get_row_sep(&self, pos: RowPos) -> &Option<RowSep> {
        match pos {
            RowPos::Mid => &self.rowseps.mid,
            RowPos::Top => &self.rowseps.top,
            RowPos::Bot => &self.rowseps.bot,
            RowPos::Snd => match &self.rowseps.snd {
                s @ &Some(_) => s,
                &None => &self.rowseps.mid,
            },
        }
    }

    pub fn get_col_sep(&self, pos: ColPos) -> Option<char> {
        match pos {
            ColPos::Lhs => self.colseps.lhs,
            ColPos::Mid => self.colseps.mid,
            ColPos::Rhs => self.colseps.rhs,
        }
    }

    pub(crate) fn write_row_sep<W: Write>(&self, wtr: &mut W, widths: &[usize], pos: RowPos) -> Result<usize> {
        match *self.get_row_sep(pos) {
            Some(ref row) => {
                write!(wtr, "{:indent$}", "", indent = self.indent)?;
                if self.colseps.lhs.is_some() {
                    write!(wtr, "{}", row.ljunc)?;
                }
                let mut iter = widths.iter().peekable();
                while let Some(width) = iter.next() {
                    for _ in 0..width + self.padding * 2 {
                        write!(wtr, "{}", row.sep)?;
                    }
                    if self.colseps.mid.is_some() && iter.peek().is_some() {
                        write!(wtr, "{}", row.cjunc)?;
                    }
                }
                if self.colseps.rhs.is_some() {
                    write!(wtr, "{}", row.rjunc)?;
                }
                writeln!(wtr)?;
                Ok(1)
            }
            None => Ok(0),
        }
    }

    pub(crate) fn write_col_sep<W: Write>(&self, wtr: &mut W, pos: ColPos) -> Result<()> {
        match self.get_col_sep(pos) {
            Some(s) => Ok(write!(wtr, "{}", s)?),
            None => Ok(()),
        }
    }
}

#[derive(Default)]
pub struct TableFormatBuilder {
    format: Box<TableFormat>,
}

impl TableFormatBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn padding(mut self, padding: usize) -> Self {
        self.format.padding = padding;
        self
    }

    pub fn col_sep(self, sep: impl Into<Option<char>>) -> Self {
        let sep = sep.into();
        self.col_seps(sep, sep, sep)
    }

    pub fn col_seps<L, M, R>(mut self, lhs: L, mid: M, rhs: R) -> Self
    where
        L: Into<Option<char>>,
        M: Into<Option<char>>,
        R: Into<Option<char>>,
    {
        self.format.colseps = ColSeps { lhs: lhs.into(), mid: mid.into(), rhs: rhs.into() };
        self
    }

    pub fn row_seps<S1, S2, S3, S4>(mut self, top: S1, snd: S2, mid: S3, bot: S4) -> Self
    where
        S1: Into<Option<RowSep>>,
        S2: Into<Option<RowSep>>,
        S3: Into<Option<RowSep>>,
        S4: Into<Option<RowSep>>,
    {
        self.format.rowseps = RowSeps {
            top: top.into(),
            snd: snd.into(),
            mid: mid.into(),
            bot: bot.into(),
        };
        self
    }

    pub fn clear_seps(self) -> Self {
        self.col_seps(None, None, None).row_seps(None, None, None, None)
    }

    pub fn indent(mut self, indent: usize) -> Self {
        self.format.indent = indent;
        self
    }

    pub fn build(&self) -> TableFormat {
        *self.format
    }
}

impl From<TableFormatBuilder> for TableFormat {
    fn from(val: TableFormatBuilder) -> Self {
        *val.format
    }
}

impl From<TableFormat> for TableFormatBuilder {
    fn from(fmt: TableFormat) -> Self {
        TableFormatBuilder { format: Box::new(fmt) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_column_separator() -> Result<()> {
        let fmt = TableFormatBuilder::new().col_seps('|', '|', '|').padding(1).build();
        let mut out = Vec::new();
        fmt.write_col_sep(&mut out, ColPos::Lhs)?;

        let out = String::from_utf8(out)?;
        assert_eq!("|", out);
        Ok(())
    }

    #[test]
    fn test_print_line_separator() -> Result<()> {
        let fmt = TableFormatBuilder::new().indent(4).build();
        let mut out = Vec::new();
        fmt.write_row_sep(&mut out, &[2, 4, 6], RowPos::Top)?;

        let out = String::from_utf8(out)?;
        assert_eq!("    +----+------+--------+\n", out);
        Ok(())
    }
}

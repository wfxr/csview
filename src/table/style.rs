use std::io::{Result, Write};

#[derive(Debug, Clone, Copy)]
pub struct RowSeps {
    /// Top separator row (top border)
    /// ```
    /// >┌───┬───┐
    ///  │ a │ b │
    /// ```
    pub top: Option<RowSep>,
    /// Second separator row (between the header row and the first data row)
    /// ```
    ///  ┌───┬───┐
    ///  │ a │ b │
    /// >├───┼───┤
    /// ```
    pub snd: Option<RowSep>,
    /// Middle separator row (between data rows)
    /// ```
    /// >├───┼───┤
    ///  │ 2 │ 2 │
    /// >├───┼───┤
    /// ```
    pub mid: Option<RowSep>,
    /// Bottom separator row (bottom border)
    /// ```
    ///  │ 3 │ 3 │
    /// >└───┴───┘
    /// ```
    pub bot: Option<RowSep>,
}

/// The characters used for printing a row separator
#[derive(Debug, Clone, Copy)]
pub struct RowSep {
    /// Inner row separator
    /// ```
    /// ┌───┬───┐
    ///   ^
    /// ```
    inner: char,
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

#[derive(Debug, Clone, Copy)]
pub struct ColSeps {
    /// Left separator column (left border)
    /// ```
    ///  │ 1 │ 2 │
    ///  ^
    /// ```
    pub lhs: Option<char>,
    /// Middle column separators
    /// ```
    ///  │ 1 │ 2 │
    ///      ^
    /// ```
    pub mid: Option<char>,
    /// Right separator column (right border)
    /// ```
    ///  │ 1 │ 2 │
    ///          ^
    /// ```
    pub rhs: Option<char>,
}

impl RowSep {
    pub fn new(sep: char, ljunc: char, cjunc: char, rjunc: char) -> RowSep {
        RowSep { inner: sep, ljunc, cjunc, rjunc }
    }
}

impl Default for RowSep {
    fn default() -> Self {
        RowSep::new('-', '+', '+', '+')
    }
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

impl Default for ColSeps {
    fn default() -> Self {
        Self { lhs: Some('|'), mid: Some('|'), rhs: Some('|') }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// Column style
    pub colseps: ColSeps,

    /// Row style
    pub rowseps: RowSeps,

    /// Left and right padding
    pub padding: usize,

    /// Global indentation
    pub indent: usize,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            indent:  0,
            padding: 1,
            colseps: ColSeps::default(),
            rowseps: RowSeps::default(),
        }
    }
}

impl Style {
    pub fn write_row_sep<W: Write>(&self, wtr: &mut W, widths: &[usize], sep: &RowSep) -> Result<()> {
        write!(wtr, "{:indent$}", "", indent = self.indent)?;
        if self.colseps.lhs.is_some() {
            write!(wtr, "{}", sep.ljunc)?;
        }
        let mut iter = widths.iter().peekable();
        while let Some(width) = iter.next() {
            for _ in 0..width + self.padding * 2 {
                write!(wtr, "{}", sep.inner)?;
            }
            if self.colseps.mid.is_some() && iter.peek().is_some() {
                write!(wtr, "{}", sep.cjunc)?;
            }
        }
        if self.colseps.rhs.is_some() {
            write!(wtr, "{}", sep.rjunc)?;
        }
        writeln!(wtr)
    }

    #[inline]
    pub fn write_col_sep<W: Write>(&self, wtr: &mut W, sep: char) -> Result<()> {
        write!(wtr, "{}", sep)
    }
}

#[derive(Default, Debug, Clone)]
pub struct StyleBuilder {
    format: Box<Style>,
}

impl StyleBuilder {
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

    pub fn build(&self) -> Style {
        *self.format
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use std::str;

    #[test]
    fn test_write_column_separator() -> Result<()> {
        let fmt = StyleBuilder::new().col_seps('|', '|', '|').padding(1).build();
        let buf = &mut Vec::new();

        fmt.colseps.lhs.map(|sep| fmt.write_col_sep(buf, sep)).transpose()?;

        assert_eq!("|", str::from_utf8(buf)?);
        Ok(())
    }

    #[test]
    fn test_write_row_separator() -> Result<()> {
        let fmt = StyleBuilder::new().indent(4).build();
        let buf = &mut Vec::new();
        let widths = &[2, 4, 6];

        fmt.rowseps
            .top
            .map(|sep| fmt.write_row_sep(buf, widths, &sep))
            .transpose()?;

        assert_eq!("    +----+------+--------+\n", str::from_utf8(buf)?);
        Ok(())
    }
}

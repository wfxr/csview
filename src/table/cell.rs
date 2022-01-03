use std::borrow::Cow;
use unicode_truncate::{Alignment, UnicodeTruncateStr};
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Default, Debug)]
pub struct Cell<'a> {
    text: &'a str,
}

impl<'a> Cell<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn display_len(&self) -> usize {
        UnicodeWidthStr::width_cjk(self.text)
    }

    pub fn truncate(&self, width: usize) -> Cow<str> {
        self.text.unicode_pad(width, Alignment::Left, true)
    }
}

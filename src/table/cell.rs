use std::borrow::Cow;
use unicode_truncate::{Alignment, UnicodeTruncateStr};

#[derive(Clone, Default, Debug)]
pub struct Cell<'a> {
    text: &'a str,
}

impl<'a> Cell<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn truncate(&self, width: usize) -> Cow<str> {
        self.text.unicode_pad(width, Alignment::Left, true)
    }
}

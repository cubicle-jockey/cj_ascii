use crate::ascii_consts::*;
use crate::ascii_translators::*;
use std::collections::vec_deque::Iter;

/// Enum representing the group an ASCII char belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsciiGroup {
    NonPrintableCtrl(u8),
    PrintableCtrl(u8),
    Printable(u8),
    Extended(u8),
}
impl AsciiGroup {
    #[inline]
    pub fn new(c: &u8) -> Self {
        if ASCII_CTRL_PRINTABLES.binary_search(c).is_ok() {
            Self::PrintableCtrl(*c)
        } else if ASCII_PRINTABLE_RANGE.contains(c) {
            Self::Printable(*c)
        } else if ASCII_EXTENDED_RANGE.contains(c) {
            Self::Extended(*c)
        } else {
            Self::NonPrintableCtrl(*c)
        }
    }
    #[inline]
    pub const fn as_byte(&self) -> u8 {
        match self {
            AsciiGroup::NonPrintableCtrl(v) => *v,
            AsciiGroup::PrintableCtrl(v) => *v,
            AsciiGroup::Printable(v) => *v,
            AsciiGroup::Extended(v) => *v,
        }
    }
    #[inline]
    pub const fn as_char(&self) -> char {
        ascii_ord_to_char(self.as_byte())
    }
}

pub struct AsciiGroupIter<'a> {
    inner: Iter<'a, u8>,
}

impl AsciiGroupIter<'_> {
    pub fn new<'a>(iter: Iter<'a, u8>) -> AsciiGroupIter<'a> {
        AsciiGroupIter { inner: iter }
    }
}

impl Iterator for AsciiGroupIter<'_> {
    type Item = AsciiGroup;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|c| AsciiGroup::new(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ascii_group() {
        assert_eq!(AsciiGroup::new(&0x00), AsciiGroup::NonPrintableCtrl(0x00));
        assert_eq!(AsciiGroup::new(&0x1F), AsciiGroup::NonPrintableCtrl(0x1F));
        assert_eq!(AsciiGroup::new(&0x20), AsciiGroup::Printable(0x20));
        assert_eq!(AsciiGroup::new(&0x7E), AsciiGroup::Printable(0x7E));
        assert_eq!(AsciiGroup::new(&0x7F), AsciiGroup::NonPrintableCtrl(0x7F));
        assert_eq!(AsciiGroup::new(&0x80), AsciiGroup::Extended(0x80));
        assert_eq!(AsciiGroup::new(&0xFF), AsciiGroup::Extended(0xFF));
    }

    #[test]
    fn test_ascii_group_as_u8() {
        assert_eq!(AsciiGroup::NonPrintableCtrl(0x00).as_byte(), 0x00);
        assert_eq!(AsciiGroup::NonPrintableCtrl(0x1F).as_byte(), 0x1F);
        assert_eq!(AsciiGroup::Printable(0x20).as_byte(), 0x20);
        assert_eq!(AsciiGroup::Printable(0x7E).as_byte(), 0x7E);
        assert_eq!(AsciiGroup::NonPrintableCtrl(0x7F).as_byte(), 0x7F);
        assert_eq!(AsciiGroup::Extended(0x80).as_byte(), 0x80);
        assert_eq!(AsciiGroup::Extended(0xFF).as_byte(), 0xFF);
    }

    #[test]
    fn test_ascii_group_as_char() {
        assert_eq!(AsciiGroup::NonPrintableCtrl(0x00).as_char(), '\u{0}');
        assert_eq!(AsciiGroup::NonPrintableCtrl(0x1F).as_char(), '\u{1F}');
        assert_eq!(AsciiGroup::Printable(0x20).as_char(), ' ');
        assert_eq!(AsciiGroup::Printable(0x7E).as_char(), '~');
        assert_eq!(AsciiGroup::NonPrintableCtrl(0x7F).as_char(), '\u{7F}');
        assert_eq!(AsciiGroup::Extended(0x80).as_char(), '\u{80}');
        assert_eq!(AsciiGroup::Extended(0xFF).as_char(), '\u{FF}');
    }
}

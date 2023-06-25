use crate::ascii_translators::*;

pub trait AsciiOrdToChar {
    /// Returns the character represented by the ASCII ordinal value.
    fn to_ascii_char(&self) -> char;
}

impl AsciiOrdToChar for u8 {
    #[inline]
    fn to_ascii_char(&self) -> char {
        ascii_ord_to_char(*self)
    }
}

impl AsciiOrdToChar for &u8 {
    #[inline]
    fn to_ascii_char(&self) -> char {
        ascii_ord_ref_to_char(self)
    }
}

pub trait CharToAsciiOrd {
    /// Returns the ASCII ordinal value of the character, or `None` if the character is not ASCII.
    fn ascii_ord(&self) -> Option<u8>;
    /// Returns the ASCII ordinal value of the character, or `default` if the character is not ASCII.
    #[inline]
    fn ascii_ord_or(&self, default: u8) -> u8 {
        self.ascii_ord().unwrap_or(default)
    }
    /// # Safety
    /// This function is unsafe because it does not check that the char is a valid ASCII character and will panic if conversion fails.
    #[inline]
    fn ascii_ord_unchecked(&self) -> u8 {
        self.ascii_ord().unwrap()
    }
}

impl CharToAsciiOrd for char {
    #[inline]
    fn ascii_ord(&self) -> Option<u8> {
        char_to_ascii_ord(*self)
    }
}

impl CharToAsciiOrd for &char {
    #[inline]
    fn ascii_ord(&self) -> Option<u8> {
        char_ref_to_ascii_ord(self)
    }
}

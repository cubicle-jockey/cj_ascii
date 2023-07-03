use crate::ascii_group::AsciiGroupIter;
use crate::ascii_traits::{AsciiOrdToChar, CharToAsciiOrd};
use std::collections::vec_deque::{Iter, IterMut};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Map;
use std::ops::{Add, AddAssign, Index, IndexMut};

/// A String like struct that contains ASCII and Extended ASCII characters.
/// <br>
/// Because it accepts Extended ASCII, all u8 values are accepted.
/// # samples
/// ```
/// use cj_ascii::prelude::*;
///
/// let mut astring = AsciiString::try_from("This is a test").unwrap();
/// let mut astring2 = AsciiString::with_capacity(14);
/// astring2 += "This";
/// astring2 += " is";
/// astring2 += " a";
/// astring2 += " test";
/// assert_eq!(astring, astring2);
/// ```
/// Mix of char, u8, &str and AsciiString concatenation.
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring += 'A';
/// astring += 66;
/// astring += "C";
/// astring += "DEF";
/// let astring2 = AsciiString::from(&astring);
/// astring += astring2;
/// assert_eq!(&astring.to_string(), "ABCDEFABCDEF");
/// ```
/// Indexing
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring += 'A';
/// astring += 66;
/// astring += "C";
/// astring += "DEF";
/// assert_eq!(astring[0], 'A'.ascii_ord_unchecked());
/// assert_eq!(astring[1].to_ascii_char(), 'B');
/// assert_eq!(astring[2], 67);
/// assert_eq!(astring[3], 68);
/// assert_eq!(astring[4], 69);
/// assert_eq!(astring[5], 70);
/// ```
/// Indexing Mut
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring += "ABCDEF";
/// astring[0] = 71;
/// astring[1] = 'H'.ascii_ord_unchecked();
/// astring[2] = 73;
/// astring[3] = 74;
/// astring[4] = 75;
/// astring[5] = 76;
/// assert_eq!(astring[0], 'G'.ascii_ord_unchecked());
/// assert_eq!(astring.to_string(), "GHIJKL");
/// ```
/// Iteration
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring += "ABCDEF";
/// let mut iter = astring.iter();
/// assert_eq!(iter.next(), Some(&65));
/// assert_eq!(iter.next(), Some(&66));
/// assert_eq!(iter.next(), Some(&67));
/// assert_eq!(iter.next(), Some(&68));
/// assert_eq!(iter.next(), Some(&69));
/// assert_eq!(iter.next(), Some(&70));
/// assert_eq!(iter.next(), None);
/// // or
/// let mut iter = astring.iter();
/// assert_eq!(iter.next(), Some(&'A'.ascii_ord_unchecked()));
/// assert_eq!(iter.next(), Some(&'B'.ascii_ord_unchecked()));
/// assert_eq!(iter.next(), Some(&'C'.ascii_ord_unchecked()));
/// assert_eq!(iter.next(), Some(&'D'.ascii_ord_unchecked()));
/// assert_eq!(iter.next(), Some(&'E'.ascii_ord_unchecked()));
/// assert_eq!(iter.next(), Some(&'F'.ascii_ord_unchecked()));
/// assert_eq!(iter.next(), None);
/// ```
/// Iteration Mut
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring += "ABCDEF";
/// let mut iter = astring.iter_mut();
/// for c in iter {
///    *c = *c + 1;
/// }
/// assert_eq!(astring.to_string(), "BCDEFG");
/// ```
/// Iter Ascii
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring += "ABCDEF";
/// let mut iter = astring.iter_ascii();
/// assert_eq!(iter.next(), Some('A'));
/// assert_eq!(iter.next(), Some('B'));
/// assert_eq!(iter.next(), Some('C'));
/// assert_eq!(iter.next(), Some('D'));
/// assert_eq!(iter.next(), Some('E'));
/// assert_eq!(iter.next(), Some('F'));
/// assert_eq!(iter.next(), None);
/// ```
/// Push
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring.push('A');
/// astring.push(66);
/// astring.push('C');
/// astring.push('D');
/// assert_eq!(astring.to_string(), "ABCD");
/// astring.push_front('Z');
/// astring.push_front(89);
/// astring.push_front('X');
/// assert_eq!(astring.to_string(), "XYZABCD");
/// ```
/// Try Push
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring.try_push('A').unwrap();
/// astring.try_push(66).unwrap();
/// astring.try_push('C').unwrap();
/// astring.try_push('D').unwrap();
/// assert!(astring.try_push('€').is_err());
/// assert_eq!(astring.to_string(), "ABCD");
///
/// let mut astring = AsciiString::new();
/// astring.try_push_front('A').unwrap();
/// astring.try_push_front(66).unwrap();
/// astring.try_push_front('C').unwrap();
/// astring.try_push_front('D').unwrap();
/// assert!(astring.try_push_front('€').is_err());
/// assert_eq!(astring.to_string(), "DCBA");
/// ```
/// Push str
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring.push_str("ABC");
/// astring.push_str("DEF");
/// assert_eq!(astring.to_string(), "ABCDEF");
/// ```
/// Push str Lossy
/// ```
/// # use cj_ascii::prelude::*;
/// let mut astring = AsciiString::new();
/// astring.push_str_lossy("ABCD");
/// astring.push_str_lossy("€");
/// assert_eq!(astring.to_string(), "ABCD ");
/// ```
/// Invalid Ascii
/// ```
/// # use cj_ascii::prelude::*;
/// let string = "ABC€";
/// let result = AsciiString::try_from(string);
/// assert!(result.is_err());
/// ```

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AsciiString {
    bytes: VecDeque<u8>,
}

impl AsciiString {
    /// Creates a new empty `AsciiString` with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: VecDeque::with_capacity(capacity),
        }
    }
    /// Creates a new empty `AsciiString`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns the length of the `AsciiString` in bytes.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    /// Returns true if the `AsciiString` contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
    /// Pushes a char or byte onto the end of the `AsciiString`.
    /// # Panics
    /// * If a char is supplied and is not ASCII/Extended ASCII.
    /// * using byte (u8) will never panic.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::new();
    /// astring.push('A');
    /// astring.push(66);
    /// astring.push('C');
    /// astring.push('D');
    /// //astring.push('€'); // Will panic!
    /// ```
    pub fn push<T: CharToAsciiOrd>(&mut self, value: T) {
        self.bytes.push_back(value.ascii_ord_unchecked());
    }
    /// Pushes a char or byte onto the end of the `AsciiString`.
    /// # Errors
    /// * If a char is supplied and is not ASCII/Extended ASCII.
    /// * using byte (u8) will never error.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::new();
    /// astring.try_push('A').unwrap();
    /// astring.try_push(66).unwrap();
    /// astring.try_push('C').unwrap();
    /// astring.try_push('D').unwrap();
    /// assert!(astring.try_push('€').is_err());
    /// ```
    pub fn try_push<T: CharToAsciiOrd + Display>(&mut self, value: T) -> Result<(), String> {
        self.bytes.push_back(value.try_ascii_ord()?);
        Ok(())
    }
    /// Pops a byte from the end of the `AsciiString`.
    pub fn pop(&mut self) -> Option<u8> {
        self.bytes.pop_back()
    }
    /// Pushes a char or byte onto the front of the `AsciiString`.
    /// # Panics
    /// * If a char is supplied and is not ASCII/Extended ASCII.
    /// * using byte (u8) will never panic.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::new();
    /// astring.push_front('A');
    /// astring.push_front(66);
    /// astring.push_front('C');
    /// astring.push_front('D');
    /// //astring.push_front('€'); // Will panic!
    /// assert_eq!(astring.to_string(), "DCBA");
    /// ```
    pub fn push_front<T: CharToAsciiOrd>(&mut self, value: T) {
        self.bytes.push_front(value.ascii_ord_unchecked());
    }
    /// Pushes a char or byte onto the front of the `AsciiString`.
    /// # Errors
    /// * If a char is supplied and is not ASCII/Extended ASCII.
    /// * using byte (u8) will never error.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::new();
    /// astring.try_push_front('A').unwrap();
    /// astring.try_push_front(66).unwrap();
    /// astring.try_push_front('C').unwrap();
    /// astring.try_push_front('D').unwrap();
    /// assert!(astring.try_push_front('€').is_err());
    /// assert_eq!(astring.to_string(), "DCBA");
    /// ```
    pub fn try_push_front<T: CharToAsciiOrd + Display>(&mut self, value: T) -> Result<(), String> {
        self.bytes.push_front(value.try_ascii_ord()?);
        Ok(())
    }
    /// Pops a byte from the front of the `AsciiString`.
    pub fn pop_front(&mut self) -> Option<u8> {
        self.bytes.pop_front()
    }
    /// Pushes a char onto the end of the `AsciiString`.
    /// # Panics
    /// * if the char is not ASCII/Extended ASCII.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::new();
    /// astring.push_char('A');
    /// astring.push_char('B');
    /// astring.push_char('C');
    /// astring.push_char('D');
    /// //astring.push_char('€'); // Will panic!
    /// ```
    pub fn push_char(&mut self, character: char) {
        *self += character;
    }
    /// Pushes a char onto the end of the `AsciiString`.
    /// # Errors
    /// * if the char is not ASCII/Extended ASCII.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::new();
    /// astring.try_push_char('A').unwrap();
    /// astring.try_push_char('B').unwrap();
    /// astring.try_push_char('C').unwrap();
    /// astring.try_push_char('D').unwrap();
    /// assert!(astring.try_push_char('€').is_err());
    /// ```
    pub fn try_push_char(&mut self, character: char) -> Result<(), String> {
        self.bytes.push_back(character.try_ascii_ord()?);
        Ok(())
    }
    /// Pushes a char onto the front of the `AsciiString`.
    /// # Panics
    /// * if the char is not ASCII/Extended ASCII.
    pub fn push_front_char(&mut self, character: char) {
        self.bytes.push_front(character.ascii_ord_unchecked());
    }
    /// Pushes a char onto the front of the `AsciiString`.
    /// # Errors
    /// * if the char is not ASCII/Extended ASCII.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::new();
    /// astring.try_push_front_char('A').unwrap();
    /// astring.try_push_front_char('B').unwrap();
    /// astring.try_push_front_char('C').unwrap();
    /// astring.try_push_front_char('D').unwrap();
    /// assert!(astring.try_push_front_char('€').is_err());
    /// assert_eq!(astring.to_string(), "DCBA");
    /// ```
    pub fn try_push_front_char(&mut self, character: char) -> Result<(), String> {
        self.bytes.push_front(character.try_ascii_ord()?);
        Ok(())
    }
    /// Pops a char from the end of the `AsciiString`.
    pub fn pop_char(&mut self) -> Option<char> {
        self.bytes.pop_back().map(|byte| byte.to_ascii_char())
    }
    /// Pops a char from the front of the `AsciiString`.
    pub fn pop_front_char(&mut self) -> Option<char> {
        self.bytes.pop_front().map(|byte| byte.to_ascii_char())
    }
    /// Pushes a string onto the end of the `AsciiString`.
    /// # Panics
    /// * if the string contains any non ASCII/Extended ASCII characters.
    pub fn push_str(&mut self, string: &str) {
        *self += string;
    }
    /// Pushes a string onto the end of the `AsciiString`.
    /// # Errors
    /// * if the string contains any non ASCII/Extended ASCII characters.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::new();
    /// astring.try_push_str("ABCD").unwrap();
    /// assert!(astring.try_push_str("€").is_err());
    /// ```
    pub fn try_push_str(&mut self, string: &str) -> Result<(), String> {
        let astr = AsciiString::try_from(string)?;
        self.bytes.extend(astr.bytes);

        Ok(())
    }
    /// Pushes a string onto the end of the `AsciiString`, replacing non ASCII/Extended ASCII characters with a space.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::new();
    /// astring.push_str_lossy("ABCD");
    /// astring.push_str_lossy("€");
    /// assert_eq!(astring.to_string(), "ABCD ");
    /// ```
    pub fn push_str_lossy(&mut self, string: &str) {
        let str_len = string.len();
        let my_remaining_capacity = self.remaining_capacity();
        if str_len > my_remaining_capacity {
            self.bytes.reserve(str_len - my_remaining_capacity);
        }
        for character in string.chars() {
            self.push(character.ascii_ord_or(32));
        }
    }
    /// Pushes an `AsciiString` onto the end of the `AsciiString`.
    pub fn push_ascii_string(&mut self, string: &AsciiString) {
        *self += string;
    }
    /// Clears the `AsciiString`, removing all bytes.
    pub fn clear(&mut self) {
        self.bytes.clear();
    }
    /// Truncates the `AsciiString` to the given length.
    pub fn truncate(&mut self, len: usize) {
        self.bytes.truncate(len);
    }
    /// Returns the capacity of the `AsciiString` in bytes.
    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }
    /// Returns the number of bytes remaining to full capacity.
    pub fn remaining_capacity(&self) -> usize {
        self.bytes.capacity() - self.bytes.len()
    }
    /// Reserves capacity for at least `additional` more bytes to be inserted in the given `AsciiString`.
    pub fn reserve(&mut self, additional: usize) {
        self.bytes.reserve(additional);
    }
    /// Reserves the minimum capacity for exactly `additional` more bytes to be inserted in the given `AsciiString`.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.bytes.reserve_exact(additional);
    }
    /// Shrinks the capacity of the `AsciiString` to match its length.
    pub fn shrink_to_fit(&mut self) {
        self.bytes.shrink_to_fit();
    }
    /// Shrinks the capacity of the `AsciiString` to the given value.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.bytes.shrink_to(min_capacity);
    }
    /// Returns a byte slice of the `AsciiString`.
    pub fn as_bytes(&mut self) -> &[u8] {
        self.bytes.make_contiguous();
        self.bytes.as_slices().0
    }
    /// Returns a mutable byte slice of the `AsciiString`.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.bytes.make_contiguous();
        self.bytes.as_mut_slices().0
    }
    /// Sorts the `AsciiString` in place.
    pub fn sort(&mut self) {
        self.bytes.make_contiguous().sort_unstable();
    }
    /// Returns true if the `AsciiString` contains the given char or byte.
    /// # Panics
    /// * If a char is supplied and is not ASCII/Extended ASCII.
    /// * using byte (u8) will never panic.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::try_from("Hello World!").unwrap();
    /// assert!(astring.contains('H'));
    /// assert!(astring.contains(72));
    /// // assert!(!astring.contains('€')); // Will panic!
    pub fn contains<T: CharToAsciiOrd>(&self, value: T) -> bool {
        self.bytes.contains(&value.ascii_ord_unchecked())
    }
    /// Returns a u8 iterator over the `AsciiString`.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let astring = AsciiString::try_from("Hello World!").unwrap();
    /// let mut iter = astring.iter();
    /// assert_eq!(iter.next(), Some(&72));
    /// assert_eq!(iter.next(), Some(&101));
    /// assert_eq!(iter.next(), Some(&108));
    /// assert_eq!(iter.next(), Some(&108));
    /// assert_eq!(iter.next(), Some(&111));
    /// assert_eq!(iter.next(), Some(&32));
    /// assert_eq!(iter.next(), Some(&87));
    /// assert_eq!(iter.next(), Some(&111));
    /// assert_eq!(iter.next(), Some(&114));
    /// assert_eq!(iter.next(), Some(&108));
    /// assert_eq!(iter.next(), Some(&100));
    /// assert_eq!(iter.next(), Some(&33));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<u8> {
        self.bytes.iter()
    }

    /// Returns a mutable u8 iterator over the `AsciiString`.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::try_from("Hello World!").unwrap();
    /// let mut iter = astring.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 72));
    /// assert_eq!(iter.next(), Some(&mut 101));
    /// assert_eq!(iter.next(), Some(&mut 108));
    /// assert_eq!(iter.next(), Some(&mut 108));
    /// assert_eq!(iter.next(), Some(&mut 111));
    /// assert_eq!(iter.next(), Some(&mut 32));
    /// assert_eq!(iter.next(), Some(&mut 87));
    /// assert_eq!(iter.next(), Some(&mut 111));
    /// assert_eq!(iter.next(), Some(&mut 114));
    /// assert_eq!(iter.next(), Some(&mut 108));
    /// assert_eq!(iter.next(), Some(&mut 100));
    /// assert_eq!(iter.next(), Some(&mut 33));
    /// assert_eq!(iter.next(), None);
    /// ```
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let mut astring = AsciiString::try_from("Hello World!").unwrap();
    /// for byte in astring.iter_mut() {
    ///    if *byte == 32 {
    ///       *byte = 95;
    ///   }
    /// }
    /// assert_eq!(astring, AsciiString::try_from("Hello_World!").unwrap());
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<u8> {
        self.bytes.iter_mut()
    }

    /// Returns a char iterator over the `AsciiString`.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let astring = AsciiString::try_from("Hello World!").unwrap();
    /// let mut iter = astring.iter_ascii();
    /// assert_eq!(iter.next(), Some('H'));
    /// assert_eq!(iter.next(), Some('e'));
    /// assert_eq!(iter.next(), Some('l'));
    /// assert_eq!(iter.next(), Some('l'));
    /// assert_eq!(iter.next(), Some('o'));
    /// assert_eq!(iter.next(), Some(' '));
    /// assert_eq!(iter.next(), Some('W'));
    /// assert_eq!(iter.next(), Some('o'));
    /// assert_eq!(iter.next(), Some('r'));
    /// assert_eq!(iter.next(), Some('l'));
    /// assert_eq!(iter.next(), Some('d'));
    /// assert_eq!(iter.next(), Some('!'));
    /// assert_eq!(iter.next(), None);
    /// ```  
    #[inline]
    pub fn iter_ascii(&self) -> Map<Iter<'_, u8>, fn(&u8) -> char> {
        self.bytes.iter().map(|byte| byte.to_ascii_char())
    }
    /// Returns an AsciiGroup iterator over the `AsciiString`.
    /// # Examples
    /// ```
    /// # use cj_ascii::prelude::*;
    /// let astring = AsciiString::try_from("Hello World!").unwrap();
    /// for x in astring.iter_ascii_group() {
    ///     match x {
    ///        AsciiGroup::PrintableCtrl(_) => println!("PrintableCtrl: {}", x.as_char()),
    ///        AsciiGroup::Printable(_) => println!("PrintableAscii: {}", x.as_char()),
    ///        AsciiGroup::NonPrintableCtrl(_) => println!("NonPrintableCtrl: {}", x.as_byte()),
    ///        AsciiGroup::Extended(_) => println!("Extended: {}", x.as_byte()),
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn iter_ascii_group(&self) -> AsciiGroupIter {
        let iter = self.iter();
        AsciiGroupIter::new(iter)
    }
}

impl Index<usize> for AsciiString {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bytes[index]
    }
}

impl IndexMut<usize> for AsciiString {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bytes[index]
    }
}

impl Add<AsciiString> for AsciiString {
    type Output = Self;

    /// Concatenates two `AsciiString`s.
    fn add(mut self, rhs: AsciiString) -> Self::Output {
        self.bytes.extend(rhs.bytes);
        self
    }
}

impl Add<&AsciiString> for AsciiString {
    type Output = Self;

    /// Concatenates two `AsciiString`s.
    fn add(mut self, rhs: &AsciiString) -> Self::Output {
        self.bytes.extend(rhs.bytes.clone());
        self
    }
}

impl Add<String> for AsciiString {
    type Output = Self;

    /// Concatenates an `AsciiString` and a `String`.
    /// # Panics
    /// Panics if the `String` contains non-ASCII/Extended ASCII characters.
    fn add(mut self, rhs: String) -> Self::Output {
        let rhs: Self = rhs.try_into().unwrap();
        self.bytes.extend(rhs.bytes);
        self
    }
}

impl Add<&String> for AsciiString {
    type Output = Self;

    /// Concatenates an `AsciiString` and a `&String`.
    /// # Panics
    /// Panics if the `String` contains non-ASCII/Extended ASCII characters.
    fn add(mut self, rhs: &String) -> Self::Output {
        let rhs: Self = rhs.try_into().unwrap();
        self.bytes.extend(rhs.bytes);
        self
    }
}

impl Add<&str> for AsciiString {
    type Output = Self;

    /// Concatenates an `AsciiString` and a `&str`.
    /// # Panics
    /// Panics if the `str` contains non-ASCII/Extended ASCII characters.
    fn add(mut self, rhs: &str) -> Self::Output {
        let rhs: Self = rhs.try_into().unwrap();
        self.bytes.extend(rhs.bytes);
        self
    }
}

impl Add<char> for AsciiString {
    type Output = Self;
    /// Concatenates an `AsciiString` and a `char`.
    /// # Panics
    /// Panics if the `char` is not ASCII/Extended ASCII.
    fn add(mut self, rhs: char) -> Self::Output {
        self.bytes.push_back(rhs.ascii_ord_unchecked());
        self
    }
}

impl Add<&char> for AsciiString {
    type Output = Self;
    /// Concatenates an `AsciiString` and a `&char`.
    /// # Panics
    /// Panics if the `char` is not ASCII/Extended ASCII.
    fn add(mut self, rhs: &char) -> Self::Output {
        self.bytes.push_back(rhs.ascii_ord_unchecked());
        self
    }
}

impl AddAssign<AsciiString> for AsciiString {
    fn add_assign(&mut self, rhs: AsciiString) {
        self.bytes.extend(rhs.bytes);
    }
}

impl AddAssign<&AsciiString> for AsciiString {
    fn add_assign(&mut self, rhs: &AsciiString) {
        self.bytes.extend(rhs.bytes.clone());
    }
}

impl AddAssign<&str> for AsciiString {
    /// Concatenates an `AsciiString` and a `&str`.
    /// # Panics
    /// Panics if the `str` contains non-ASCII/Extended ASCII characters.
    fn add_assign(&mut self, rhs: &str) {
        let rhs = AsciiString::try_from(rhs).unwrap();
        self.bytes.extend(rhs.bytes);
    }
}

impl AddAssign<String> for AsciiString {
    /// Concatenates an `AsciiString` and a `String`.
    /// # Panics
    /// Panics if the `String` contains non-ASCII/Extended ASCII characters.
    fn add_assign(&mut self, rhs: String) {
        let rhs = AsciiString::try_from(rhs).unwrap();
        self.bytes.extend(rhs.bytes);
    }
}

impl AddAssign<char> for AsciiString {
    /// Concatenates an `AsciiString` and a `char`.
    /// # Panics
    /// Panics if the `char` is not ASCII/Extended ASCII.
    fn add_assign(&mut self, rhs: char) {
        self.bytes.push_back(rhs.ascii_ord_unchecked());
    }
}

impl AddAssign<&char> for AsciiString {
    /// Concatenates an `AsciiString` and a `&char`.
    /// # Panics
    /// Panics if the `char` is not ASCII/Extended ASCII.
    fn add_assign(&mut self, rhs: &char) {
        self.bytes.push_back(rhs.ascii_ord_unchecked());
    }
}

impl AddAssign<u8> for AsciiString {
    fn add_assign(&mut self, rhs: u8) {
        self.bytes.push_back(rhs);
    }
}

impl From<AsciiString> for String {
    fn from(value: AsciiString) -> Self {
        let mut result = String::with_capacity(value.bytes.len());
        for byte in value.bytes {
            result.push(byte.to_ascii_char());
        }
        result
    }
}

impl From<&AsciiString> for String {
    fn from(value: &AsciiString) -> Self {
        let mut result = String::with_capacity(value.bytes.len());
        for byte in &value.bytes {
            result.push(byte.to_ascii_char());
        }
        result
    }
}

impl From<&AsciiString> for AsciiString {
    fn from(value: &AsciiString) -> Self {
        let mut result = Self::with_capacity(value.bytes.len());
        for byte in &value.bytes {
            result.bytes.push_back(*byte);
        }
        result
    }
}

// impl From<String> for AsciiString {
//     fn from(value: String) -> Self {
//         let mut result = Self::with_capacity(value.len());
//         for character in value.chars() {
//             result.bytes.push_back(character.ascii_ord_unchecked());
//         }
//         result
//     }
// }

impl TryFrom<String> for AsciiString {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut result = Self::with_capacity(value.len());
        for (inx, character) in value.chars().enumerate() {
            if let Some(character) = character.ascii_ord() {
                result.bytes.push_back(character);
            } else {
                return Err(format!(
                    r#"Non-ASCII character "{character}" found at index {inx}"#
                ));
            }
        }
        Ok(result)
    }
}

// impl From<&str> for AsciiString {
//     fn from(value: &str) -> Self {
//         let mut result = Self::with_capacity(value.len());
//         for character in value.chars() {
//             // result.bytes.push_back(character.ascii_ord_unchecked());
//             result
//                 .bytes
//                 .push_back(character.ascii_ord().expect("Non-ASCII character found"));
//         }
//         result
//     }
// }

impl TryFrom<&str> for AsciiString {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut result = Self::with_capacity(value.len());
        for (inx, character) in value.chars().enumerate() {
            if let Some(character) = character.ascii_ord() {
                result.bytes.push_back(character);
            } else {
                return Err(format!(
                    r#"Non-ASCII character "{character}" found at index {inx}"#
                ));
            }
        }
        Ok(result)
    }
}

// impl From<&String> for AsciiString {
//     fn from(value: &String) -> Self {
//         let mut result = Self::with_capacity(value.len());
//         for character in value.chars() {
//             result.bytes.push_back(character.ascii_ord_unchecked());
//         }
//         result
//     }
// }

impl TryFrom<&String> for AsciiString {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut result = Self::with_capacity(value.len());
        for (inx, character) in value.chars().enumerate() {
            if let Some(character) = character.ascii_ord() {
                result.bytes.push_back(character);
            } else {
                return Err(format!(
                    r#"Non-ASCII character "{character}" found at index {inx}"#
                ));
            }
        }
        Ok(result)
    }
}

impl From<&AsciiString> for Vec<u8> {
    fn from(value: &AsciiString) -> Self {
        value.bytes.clone().into()
    }
}

impl From<AsciiString> for Vec<u8> {
    fn from(value: AsciiString) -> Self {
        value.bytes.into()
    }
}

impl From<&AsciiString> for VecDeque<u8> {
    fn from(value: &AsciiString) -> Self {
        value.bytes.clone()
    }
}

impl From<AsciiString> for VecDeque<u8> {
    fn from(value: AsciiString) -> Self {
        value.bytes
    }
}

impl From<&AsciiString> for Vec<char> {
    fn from(value: &AsciiString) -> Self {
        let mut result = Vec::with_capacity(value.bytes.len());
        for byte in &value.bytes {
            result.push(byte.to_ascii_char());
        }
        result
    }
}

impl From<AsciiString> for Vec<char> {
    fn from(value: AsciiString) -> Self {
        let mut result = Vec::with_capacity(value.bytes.len());
        for byte in value.bytes {
            result.push(byte.to_ascii_char());
        }
        result
    }
}

impl From<&AsciiString> for VecDeque<char> {
    fn from(value: &AsciiString) -> Self {
        let mut result = VecDeque::with_capacity(value.bytes.len());
        for byte in &value.bytes {
            result.push_back(byte.to_ascii_char());
        }
        result
    }
}

impl From<AsciiString> for VecDeque<char> {
    fn from(value: AsciiString) -> Self {
        let mut result = VecDeque::with_capacity(value.bytes.len());
        for byte in value.bytes {
            result.push_back(byte.to_ascii_char());
        }
        result
    }
}

impl From<VecDeque<u8>> for AsciiString {
    fn from(value: VecDeque<u8>) -> Self {
        Self { bytes: value }
    }
}

impl From<Vec<u8>> for AsciiString {
    fn from(value: Vec<u8>) -> Self {
        Self {
            bytes: value.into(),
        }
    }
}

impl From<&[u8]> for AsciiString {
    fn from(value: &[u8]) -> Self {
        let mut result = Self::with_capacity(value.len());
        result.bytes.extend(value.iter().copied());
        result
    }
}

// impl From<char> for AsciiString {
//     fn from(value: char) -> Self {
//         let mut result = Self::with_capacity(1);
//         result.bytes.push_back(value.ascii_ord_unchecked());
//         result
//     }
// }

impl TryFrom<char> for AsciiString {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let mut result = Self::with_capacity(1);

        if let Some(character) = value.ascii_ord() {
            result.bytes.push_back(character);
        } else {
            return Err(format!(r#"Non-ASCII character "{value}" found"#));
        }

        Ok(result)
    }
}

impl TryFrom<&char> for AsciiString {
    type Error = String;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        let mut result = Self::with_capacity(1);

        if let Some(character) = value.ascii_ord() {
            result.bytes.push_back(character);
        } else {
            return Err(format!(r#"Non-ASCII character "{value}" found"#));
        }

        Ok(result)
    }
}

impl Display for AsciiString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{}", byte.to_ascii_char())?;
        }
        Ok(())
    }
}

impl Debug for AsciiString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{}", byte.to_ascii_char())?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl Serialize for AsciiString {
    // fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    //     let mut seq = serializer.serialize_seq(Some(self.bytes.len()))?;
    //     for byte in &self.bytes {
    //         seq.serialize_element(byte)?;
    //     }
    //     seq.end()
    // }
    // this is fine for json and other formats where utf-8 is the "norm", but binary formats should use self.as_bytes directly instead
    // since it's already serialized as a sequence of bytes.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&String::from(self))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for AsciiString {
    // fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    //     let bytes = VecDeque::deserialize(deserializer)?;
    //     Ok(Self { bytes })
    // }
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        //Ok(Self::try_from(string)?)
        Self::try_from(string)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ascii_group::AsciiGroup;

    #[test]
    fn test_add_ascii_string() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string.push(65);
        string.push(66);
        string.push(67);
        let mut string2 = AsciiString::with_capacity(3);
        string2.push(68);
        string2.push(69);
        string2.push(70);
        let result = string + string2;
        assert_eq!(result.bytes.len(), 6);
        assert_eq!(result.bytes[0], 65);
        assert_eq!(result.bytes[1], 66);
        assert_eq!(result.bytes[2], 67);
        assert_eq!(result.bytes[3], 68);
        assert_eq!(result.bytes[4], 69);
        assert_eq!(result.bytes[5], 70);
    }

    #[test]
    fn test_add_chars() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string.push('A');
        string.push('B');
        string.push('C');
        let result = string + 'D';
        assert_eq!(result.bytes.len(), 4);
        assert_eq!(result.bytes[0], 65);
        assert_eq!(result.bytes[1], 66);
        assert_eq!(result.bytes[2], 67);
        assert_eq!(result.bytes[3], 68);
    }

    #[test]
    fn test_from_ascii_string() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string.push(65);
        string.push(66);
        string.push(67);
        let result = String::from(&string);
        assert_eq!(result.len(), 3);
        assert_eq!(result.chars().nth(0).unwrap(), 'A');
        assert_eq!(result.chars().nth(1).unwrap(), 'B');
        assert_eq!(result.chars().nth(2).unwrap(), 'C');
    }

    #[test]
    fn test_from_string() {
        use crate::ascii_string::AsciiString;
        let string = String::from("ABC");
        let result = AsciiString::try_from(&string).unwrap();
        assert_eq!(result.bytes.len(), 3);
        assert_eq!(result.bytes[0], 65);
        assert_eq!(result.bytes[1], 66);
        assert_eq!(result.bytes[2], 67);
    }

    #[test]
    fn test_from_str() {
        use crate::ascii_string::AsciiString;
        let string = "ABC";
        let result = AsciiString::try_from(string).unwrap();
        assert_eq!(result.bytes.len(), 3);
        assert_eq!(result.bytes[0], 65);
        assert_eq!(result.bytes[1], 66);
        assert_eq!(result.bytes[2], 67);
    }

    #[test]
    fn test_try_from_str() {
        use crate::ascii_string::AsciiString;
        let string = "ABC";
        let result = AsciiString::try_from(string);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.bytes.len(), 3);
        assert_eq!(result.bytes[0], 65);
        assert_eq!(result.bytes[1], 66);
        assert_eq!(result.bytes[2], 67);
    }

    #[test]
    fn test_try_from_str_err() {
        use crate::ascii_string::AsciiString;
        let string = "ABC€";
        let result = AsciiString::try_from(string);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_assign_str() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += "ABC"; //.into();
        string += "DEF"; //.into();
        assert_eq!(&string.to_string(), "ABCDEF");
    }

    #[test]
    fn test_assign_str() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string = string + "ABC";
        assert_eq!(&string.to_string(), "ABC");
    }

    #[test]
    fn test_add_assign_char() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += 'A';
        string += 'B';
        string += 'C';
        assert_eq!(&string.to_string(), "ABC");
    }

    #[test]
    fn test_add_assign_byte() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += 65;
        string += 66;
        string += 67;
        assert_eq!(&string.to_string(), "ABC");
    }

    #[test]
    fn test_add_assign_various() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::new();
        string += 'A';
        string += 66;
        string += "C";
        string += "DEF";
        let string2 = AsciiString::from(&string);
        string += string2;
        assert_eq!(&string.to_string(), "ABCDEFABCDEF");
    }

    #[test]
    fn test_sort() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += "CBA";
        string.sort();
        assert_eq!(&string.to_string(), "ABC");
    }

    #[test]
    fn test_index() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += "ABC";
        assert_eq!(string[0], 65);
        assert_eq!(string[1], 66);
        assert_eq!(string[2], 67);
    }

    #[test]
    fn test_index_mut() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += "ABC";
        string[0] = 'D'.ascii_ord_unchecked();
        string[1] = 'E'.ascii_ord_unchecked();
        string[2] = 'F'.ascii_ord_unchecked();
        assert_eq!(string[0], 68);
        assert_eq!(string[1], 69);
        assert_eq!(string[2], 70);
    }

    #[test]
    fn test_equals() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += "ABC";
        let mut string2 = AsciiString::with_capacity(3);
        string2 += "ABC";
        assert_eq!(string, string2);
    }

    #[test]
    fn test_equals_string() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += "ABC";
        let string2: String = "ABC".to_string();
        let r = string2 == string.to_string();
        assert!(r);
    }

    #[test]
    fn test_contains() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += "ABC";
        assert!(string.contains('A'));
        assert!(string.contains('B'));
        assert!(string.contains('C'));
        assert!(!string.contains('D'));
    }

    #[test]
    fn test_iter_mut() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string += "ABC";
        for c in string.iter_mut() {
            *c = 'D'.ascii_ord_unchecked();
        }
        assert_eq!(&string.to_string(), "DDD");
    }

    #[test]
    fn test_ascii_group_iter() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(256);
        for i in 0..=255 {
            string += i;
        }

        let mut c = 0u8;
        for x in string.iter_ascii_group() {
            assert_eq!(x.as_byte(), c);
            assert_eq!(x.as_char(), c.to_ascii_char());

            if c < 255 {
                c += 1;
            }
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string = "ABC".into();
        let serialized = serde_json::to_string(&string).unwrap();
        assert_eq!(serialized, "\"ABC\"");
        let deserialized: AsciiString = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, string);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_full_u8_range() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(256);
        for i in 0..=255 {
            string += i;
        }
        let serialized = serde_json::to_string(&string).unwrap();
        let deserialized: AsciiString = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, string);
    }
}

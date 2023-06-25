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
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    /// Pushes a byte onto the end of the `AsciiString`.
    pub fn push(&mut self, byte: u8) {
        self.bytes.push_back(byte);
    }
    /// Pops a byte from the end of the `AsciiString`.
    pub fn pop(&mut self) -> Option<u8> {
        self.bytes.pop_back()
    }
    /// Pushes a byte onto the front of the `AsciiString`.
    pub fn push_front(&mut self, byte: u8) {
        self.bytes.push_front(byte);
    }
    /// Pops a byte from the front of the `AsciiString`.
    pub fn pop_front(&mut self) -> Option<u8> {
        self.bytes.pop_front()
    }
    /// Pushes a char onto the end of the `AsciiString`.
    /// * If the char is not ASCII/Extended ASCII, it will be ignored.
    pub fn push_char(&mut self, character: char) {
        *self += character //.into();
    }
    /// Pushes a char onto the front of the `AsciiString`.
    /// * If the char is not ASCII/Extended ASCII, it will be ignored.
    pub fn push_front_char(&mut self, character: char) {
        if let Some(byte) = character.ascii_ord() {
            self.bytes.push_front(byte);
        }
    }
    /// Pops a char from the end of the `AsciiString`.
    pub fn pop_char(&mut self) -> Option<char> {
        if let Some(byte) = self.bytes.pop_back() {
            Some(byte.to_ascii_char())
        } else {
            None
        }
    }
    /// Pops a char from the front of the `AsciiString`.
    pub fn pop_front_char(&mut self) -> Option<char> {
        if let Some(byte) = self.bytes.pop_front() {
            Some(byte.to_ascii_char())
        } else {
            None
        }
    }
    /// Pushes a string onto the end of the `AsciiString`.
    pub fn push_str(&mut self, string: &str) {
        *self += string //.into();
    }
    /// Pushes an `AsciiString` onto the end of the `AsciiString`.
    pub fn push_ascii_string(&mut self, string: &AsciiString) {
        *self += string //.into();
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

    /// Returns a u8 iterator over the `AsciiString`.
    #[inline]
    pub fn iter(&self) -> Iter<u8> {
        self.bytes.iter()
    }

    /// Returns a mutable u8 iterator over the `AsciiString`.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<u8> {
        self.bytes.iter_mut()
    }

    /// Returns a char iterator over the `AsciiString`.    
    #[inline]
    pub fn iter_ascii(&self) -> Map<Iter<'_, u8>, fn(&u8) -> char> {
        self.bytes.iter().map(|byte| byte.to_ascii_char())
    }
}

impl Default for AsciiString {
    fn default() -> Self {
        Self {
            bytes: VecDeque::new(),
        }
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
    fn add(mut self, rhs: String) -> Self::Output {
        let rhs: Self = rhs.into();
        self.bytes.extend(rhs.bytes);
        self
    }
}

impl Add<&String> for AsciiString {
    type Output = Self;

    /// Concatenates an `AsciiString` and a `&String`.
    fn add(mut self, rhs: &String) -> Self::Output {
        let rhs: Self = rhs.into();
        self.bytes.extend(rhs.bytes);
        self
    }
}

impl Add<&str> for AsciiString {
    type Output = Self;

    /// Concatenates an `AsciiString` and a `&str`.
    fn add(mut self, rhs: &str) -> Self::Output {
        let rhs: Self = rhs.into();
        self.bytes.extend(rhs.bytes);
        self
    }
}

impl Add<char> for AsciiString {
    type Output = Self;
    /// Concatenates an `AsciiString` and a `char`.
    fn add(mut self, rhs: char) -> Self::Output {
        self.bytes.push_back(rhs.ascii_ord_unchecked());
        self
    }
}

impl Add<&char> for AsciiString {
    type Output = Self;
    /// Concatenates an `AsciiString` and a `&char`.
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
    fn add_assign(&mut self, rhs: &str) {
        let rhs = AsciiString::from(rhs); // = rhs.into();
        self.bytes.extend(rhs.bytes);
    }
}

impl AddAssign<String> for AsciiString {
    fn add_assign(&mut self, rhs: String) {
        let rhs = AsciiString::from(rhs); // = rhs.into();
        self.bytes.extend(rhs.bytes);
    }
}

impl AddAssign<char> for AsciiString {
    fn add_assign(&mut self, rhs: char) {
        self.bytes.push_back(rhs.ascii_ord_unchecked());
    }
}

impl AddAssign<&char> for AsciiString {
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

impl From<String> for AsciiString {
    fn from(value: String) -> Self {
        let mut result = Self::with_capacity(value.len());
        for character in value.chars() {
            result.bytes.push_back(character.ascii_ord_unchecked());
        }
        result
    }
}

impl From<&str> for AsciiString {
    fn from(value: &str) -> Self {
        let mut result = Self::with_capacity(value.len());
        for character in value.chars() {
            result.bytes.push_back(character.ascii_ord_unchecked());
        }
        result
    }
}

impl From<&String> for AsciiString {
    fn from(value: &String) -> Self {
        let mut result = Self::with_capacity(value.len());
        for character in value.chars() {
            result.bytes.push_back(character.ascii_ord_unchecked());
        }
        result
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

impl From<char> for AsciiString {
    fn from(value: char) -> Self {
        let mut result = Self::with_capacity(1);
        result.bytes.push_back(value.ascii_ord_unchecked());
        result
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::ascii_traits::*;
    #[test]
    fn test_add_ascii_string() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string.bytes.push_back(65);
        string.bytes.push_back(66);
        string.bytes.push_back(67);
        let mut string2 = AsciiString::with_capacity(3);
        string2.bytes.push_back(68);
        string2.bytes.push_back(69);
        string2.bytes.push_back(70);
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
    fn test_from_ascii_string() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string.bytes.push_back(65);
        string.bytes.push_back(66);
        string.bytes.push_back(67);
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
        let result = AsciiString::from(&string);
        assert_eq!(result.bytes.len(), 3);
        assert_eq!(result.bytes[0], 65);
        assert_eq!(result.bytes[1], 66);
        assert_eq!(result.bytes[2], 67);
    }

    #[test]
    fn test_from_str() {
        use crate::ascii_string::AsciiString;
        let string = "ABC";
        let result = AsciiString::from(string);
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
    #[should_panic]
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
    fn test_sort() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string = "CBA".into();
        string.sort();
        assert_eq!(&string.to_string(), "ABC");
    }

    #[test]
    fn test_index() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string = "ABC".into();
        assert_eq!(string[0], 65);
        assert_eq!(string[1], 66);
        assert_eq!(string[2], 67);
    }

    #[test]
    fn test_index_mut() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string = "ABC".into();
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
        string = "ABC".into();
        let mut string2 = AsciiString::with_capacity(3);
        string2 = "ABC".into();
        assert_eq!(string, string2);
    }

    #[test]
    fn test_equals_string() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string = "ABC".into();
        let string2: String = "ABC".to_string();
        let r = string2 == string.to_string();
        assert!(r);
    }

    #[test]
    fn test_iter_mut() {
        use crate::ascii_string::AsciiString;
        let mut string = AsciiString::with_capacity(3);
        string = "ABC".into();
        for c in string.iter_mut() {
            *c = 'D'.ascii_ord_unchecked();
        }
        assert_eq!(&string.to_string(), "DDD");
    }
}

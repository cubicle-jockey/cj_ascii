# cj_ascii

[![Rust](https://github.com/cubicle-jockey/cj_ascii/actions/workflows/rust.yml/badge.svg)](https://github.com/cubicle-jockey/cj_ascii/actions/workflows/rust.yml)
[![Dependency Review](https://github.com/cubicle-jockey/cj_ascii/actions/workflows/dependency-review.yml/badge.svg)](https://github.com/cubicle-jockey/cj_ascii/actions/workflows/dependency-review.yml)
[![Crate](https://img.shields.io/crates/v/cj_ascii.svg)](https://crates.io/crates/cj_ascii)
[![API](https://docs.rs/cj_ascii/badge.svg)](https://docs.rs/cj_ascii)

A library for working with ASCII strings in Rust

AsciiString is a String like struct for working with ASCII and Extended ASCII characters.
<br>
Because it accepts Extended ASCII, all u8 values are accepted.

Key features:
```text
* TryFrom traits have been implemented for string types.
* push/pop push_front/pop_front have been implemented for both char and u8.
* u8 (ascii ordinals) can be accessed by index and updated directly. 
* Concatenation of char, u8, &str and AsciiString is supported (will panic if char/string is not valid ASCII).
* Iterators for both u8 and char.
* raw bytes via as_bytes() and as_bytes_mut(), where the bytes are guaranteed to be valid ASCII (one byte = one char).
```
# samples
basic example
```rust
fn main() {
    use cj_ascii::prelude::*;

    let mut astring = AsciiString::try_from("This is a test").unwrap();
    let mut astring2 = AsciiString::with_capacity(14);
    astring2 += "This";
    astring2 += " is";
    astring2 += " a";
    astring2 += " test";
    assert_eq!(astring, astring2);
}
```
Mix of char, u8, &str and AsciiString concatenation.
```rust
fn main() {
    use cj_ascii::prelude::*;
    
    let mut astring = AsciiString::new();
    astring += 'A';
    astring += 66;
    astring += "C";
    astring += "DEF";
    let astring2 = AsciiString::from(&astring);
    astring += astring2;
    assert_eq!(&astring.to_string(), "ABCDEFABCDEF");
}
```
Indexing
```rust
fn main() {
    use cj_ascii::prelude::*;

    let mut astring = AsciiString::new();
    astring += 'A';
    astring += 66;
    astring += "C";
    astring += "DEF";
    assert_eq!(astring[0], 'A'.ascii_ord_unchecked());
    assert_eq!(astring[1].to_ascii_char(), 'B');
    assert_eq!(astring[2], 67);
    assert_eq!(astring[3], 68);
    assert_eq!(astring[4], 69);
    assert_eq!(astring[5], 70);
}
```
Indexing Mut
```rust
fn main() {
    use cj_ascii::prelude::*;
    let mut astring = AsciiString::new();
    astring += "ABCDEF";
    astring[0] = 71;
    astring[1] = 'H'.ascii_ord_unchecked();
    astring[2] = 73;
    astring[3] = 74;
    astring[4] = 75;
    astring[5] = 76;
    assert_eq!(astring[0], 'G'.ascii_ord_unchecked());
    assert_eq!(astring.to_string(), "GHIJKL");
}
```
Iter
* user iter() to iterate over the raw bytes.
```rust
fn main() {
    use cj_ascii::prelude::*;
    let mut astring = AsciiString::new();
    astring += "ABCDEF";
    let mut iter = astring.iter();
    assert_eq!(iter.next(), Some(&65));
    assert_eq!(iter.next(), Some(&66));
    assert_eq!(iter.next(), Some(&67));
    assert_eq!(iter.next(), Some(&68));
    assert_eq!(iter.next(), Some(&69));
    assert_eq!(iter.next(), Some(&70));
    assert_eq!(iter.next(), None);
// or
    let mut iter = astring.iter();
    assert_eq!(iter.next(), Some(&'A'.ascii_ord_unchecked()));
    assert_eq!(iter.next(), Some(&'B'.ascii_ord_unchecked()));
    assert_eq!(iter.next(), Some(&'C'.ascii_ord_unchecked()));
    assert_eq!(iter.next(), Some(&'D'.ascii_ord_unchecked()));
    assert_eq!(iter.next(), Some(&'E'.ascii_ord_unchecked()));
    assert_eq!(iter.next(), Some(&'F'.ascii_ord_unchecked()));
    assert_eq!(iter.next(), None);
}
```
Iter Mut
* user iter_mut() to iterate over and modify the raw bytes.
```rust
fn main() {
    use cj_ascii::prelude::*;
    let mut astring = AsciiString::new();
    astring += "ABCDEF";
    let mut iter = astring.iter_mut();
    for c in iter {
        *c = *c + 1;
    }
    assert_eq!(astring.to_string(), "BCDEFG");
}
```
Iter Ascii
* use iter_ascii() to iterate as chars.
```rust
fn main() {
    use cj_ascii::prelude::*;
    let mut astring = AsciiString::new();
    astring += "ABCDEF";
    let mut iter = astring.iter_ascii();
    assert_eq!(iter.next(), Some('A'));
    assert_eq!(iter.next(), Some('B'));
    assert_eq!(iter.next(), Some('C'));
    assert_eq!(iter.next(), Some('D'));
    assert_eq!(iter.next(), Some('E'));
    assert_eq!(iter.next(), Some('F'));
    assert_eq!(iter.next(), None);
}
```
Iter AsciiGroup
* AsciiGroup is an enum that represents the different categories of ASCII characters.
  * PrintableCtrl - ASCII characters that are printable control characters.
  * NonPrintableCtrl - ASCII characters that are non-printable control characters.
  * Printable - ASCII characters that are printable (space through tilda).
  * Extended - ASCII characters in the extended ascii range (ordinal 128 through 255).
```rust
fn main() {
    use cj_ascii::prelude::*;
    let astring = AsciiString::try_from("Hello World!").unwrap();
    for x in astring.iter_ascii_group() {
        match x {
            AsciiGroup::PrintableCtrl(_) => println!("PrintableCtrl: {}", x.as_char()),
            AsciiGroup::Printable(_) => println!("PrintableAscii: {}", x.as_char()),
            AsciiGroup::NonPrintableCtrl(_) => println!("NonPrintableCtrl: {}", x.as_byte()),
            AsciiGroup::Extended(_) => println!("Extended: {}", x.as_byte()),
        }
    }
}
```
Push (char or u8)
* u8 values are pushed as is and will never fail.
* char values will panic if they are not ASCII.
```rust
fn main() {
    use cj_ascii::prelude::*;
    let mut astring = AsciiString::new();
    astring.push('A');
    astring.push(66);
    astring.push('C');
    astring.push('D');
    assert_eq!(astring.to_string(), "ABCD");
    astring.push_front('Z');
    astring.push_front(89);
    astring.push_front('X');
    assert_eq!(astring.to_string(), "XYZABCD");
}
```
Try Push (char or u8)
```rust
fn main() {
    use cj_ascii::prelude::*;
    let mut astring = AsciiString::new();
    astring.try_push('A').unwrap();
    astring.try_push(66).unwrap();
    astring.try_push('C').unwrap();
    astring.try_push('D').unwrap();
    assert!(astring.try_push('€').is_err());
    assert_eq!(astring.to_string(), "ABCD");

    let mut astring = AsciiString::new();
    astring.try_push_front('A').unwrap();
    astring.try_push_front(66).unwrap();
    astring.try_push_front('C').unwrap();
    astring.try_push_front('D').unwrap();
    assert!(astring.try_push_front('€').is_err());
    assert_eq!(astring.to_string(), "DCBA");
}
```
Push str
* use push_str if you know the string only contains ascii.
* push_str will panic if the string contains non-ascii characters.
```rust
fn main() {
    use cj_ascii::prelude::*;
    let mut astring = AsciiString::new();
    astring.push_str("ABC");
    astring.push_str("DEF");
    assert_eq!(astring.to_string(), "ABCDEF");
}
```
Push str Lossy
* push_str_lossy will replace non-ascii characters with a space.
```rust
fn main() {
    use cj_ascii::prelude::*;
    let mut astring = AsciiString::new();
    astring.push_str_lossy("ABCD");
    astring.push_str_lossy("€");
    assert_eq!(astring.to_string(), "ABCD ");
}
```
Invalid Ascii
* try_from will return an error if the string contains non-ascii characters.
```rust
fn main() {
    use cj_ascii::prelude::*;
    let string = "ABC€";
    let result = AsciiString::try_from(string);
    assert!(result.is_err());
}
```
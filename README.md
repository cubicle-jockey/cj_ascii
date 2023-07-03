# cj_ascii

A String like struct that contains ASCII and Extended ASCII characters.
<br>
Because it accepts Extended ASCII, all u8 values are accepted.
# samples
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
Iteration
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
Iteration Mut
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
Push
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
Try Push
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
```rust
fn main() {
    use cj_ascii::prelude::*;
    let string = "ABC€";
    let result = AsciiString::try_from(string);
    assert!(result.is_err());
}
```
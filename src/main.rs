mod ascii_consts;
mod ascii_translators;

use ascii_translators::*;
use std::io::Read;

mod ascii_string;
mod ascii_traits;
use crate::ascii_string::*;
use ascii_traits::*;

pub fn main() {
    for i in 0..=255 {
        println!("{}: {}", i, ascii_ord_to_char(i));
    }

    println!("Just the chars");
    for i in 0..=255 {
        println!("{}", ascii_ord_to_char(i));
    }

    for i in 0u8..=255 {
        println!("{}: {}", i, i.to_ascii_char());
    }

    let mut string = AsciiString::from("This is a test");

    println!("{}", string);

    string.clear();
    for b in 0u8..=255 {
        string.push(b);
    }
    println!("p1");
    println!("{}", string);
    println!("p2");
    println!("{string:?}");

    let mut string2 = AsciiString::new();
    for b in 0u8..=255 {
        let ch: char = b.to_ascii_char();
        string2.push_char(ch);
    }

    println!("{}", string2 == string);

    for c in string2.iter() {
        println!("{}", c);
    }

    for c in string2.iter() {
        println!("{}", c.to_ascii_char());
    }

    for c in string2.iter_ascii() {
        println!("{}", c);
    }

    perf_test();
}

fn perf_test() {
    let file_name = "C:/Temp/EnglishWords/words_ansi.txt";
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();

    let start = std::time::Instant::now();

    let ascii_string: AsciiString = string.as_str().into();

    let end = start.elapsed().as_millis();

    println!("{} ms", end);

    println!("String = {}", string.len());
    println!("AsciiString = {}", ascii_string.len());

    // let string2: String = ascii_string.clone().into();
    let string2 = String::from(&ascii_string);

    println!("{}", string2 == string);

    let start = std::time::Instant::now();
    let mut z_count = 0;
    for c in string2.chars() {
        if c == 'z' {
            z_count += 1;
        }
    }

    let end = start.elapsed().as_millis();
    println!("String {} z's found in {} ms", z_count, end);

    let start = std::time::Instant::now();
    let mut z_count = 0;
    for c in ascii_string.iter_ascii() {
        if c == 'z' {
            z_count += 1;
        }
    }

    let end = start.elapsed().as_millis();
    println!("AnsiString {} z's found in {} ms", z_count, end);

    let start = std::time::Instant::now();
    let mut z_count = 0;
    for b in ascii_string.iter() {
        if b == &b'z' {
            z_count += 1;
        }
    }

    let end = start.elapsed().as_millis();
    println!("AnsiByte {} z's found in {} ms", z_count, end);

    let start = std::time::Instant::now();
    let mut z_count = 0;
    for b in ascii_string.iter() {
        let c = b.to_ascii_char();
        let b = c.ascii_ord_unchecked();
        if b == b'z' {
            z_count += 1;
        }
    }

    let end = start.elapsed().as_millis();
    println!("AnsiByte to/from {} z's found in {} ms", z_count, end);
}

use crate::ascii_string::AsciiString;
use std::io::{BufRead, BufReader, Read};

pub struct AsciiStreamReader<R> {
    inner: BufReader<R>,
}

impl<R: Read> AsciiStreamReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner: BufReader::new(inner),
        }
    }

    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        Self {
            inner: BufReader::with_capacity(capacity, inner),
        }
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    pub fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }

    pub fn read_line(&mut self, buf: &mut AsciiString) -> std::io::Result<usize> {
        let result = self.read_until(b'\n', buf);
        if let Ok(result) = result {
            if result > 0 {
                if buf[result - 1] == b'\n' {
                    buf.pop();
                    if result > 1 && buf[result - 2] == b'\r' {
                        buf.pop();
                    }
                }
                return Ok(buf.len());
            }
        }
        result
    }

    #[inline]
    pub fn read_until(&mut self, byte: u8, buf: &mut AsciiString) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = Vec::new();
        let result = self.inner.read_until(byte, &mut vec);
        if result.is_ok() {
            *buf = AsciiString::from(vec);
        }
        result
    }

    pub fn read_to_end(&mut self, buf: &mut AsciiString) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = Vec::new();
        let result = self.inner.read_to_end(&mut vec);
        if result.is_ok() {
            *buf = AsciiString::from(vec);
        }
        result
    }

    pub fn read_bytes(&mut self, buf: &mut AsciiString, len: usize) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = Vec::new();
        vec.resize(len, 0);
        let result = self.inner.read(&mut vec);
        if result.is_ok() {
            *buf = AsciiString::from(vec);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    #[test]
    fn test_ascii_stream() {
        use super::*;
        let vec = b"This is test1\nThis is test2\r\nThis is test3";
        let mut stream = AsciiStreamReader::new(vec.as_slice());
        let mut buf = AsciiString::new();
        stream.read_line(&mut buf).unwrap();
        assert_eq!(buf.to_string(), "This is test1");
        stream.read_line(&mut buf).unwrap();
        assert_eq!(buf.to_string(), "This is test2");
        stream.read_line(&mut buf).unwrap();
        assert_eq!(buf.to_string(), "This is test3");
        stream.read_line(&mut buf).unwrap();
        assert_eq!(buf.to_string(), "");
    }
}

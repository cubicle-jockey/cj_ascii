use crate::ascii_string::AsciiString;
use std::io::{BufRead, BufReader, Read};

/// The result of a call to `AsciiStreamReader::read_line()`.
#[derive(Debug)]
pub enum ReadLineResult {
    /// The number of bytes read.
    Success(usize),
    /// The end of the stream has been reached.
    EOF,
    /// An error occurred.
    Error(std::io::Error),
}

impl ReadLineResult {
    /// Returns true if the result is `Success`.
    #[inline(always)]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }
    /// Returns true if the result is `EOF`.
    #[inline(always)]
    pub fn is_eof(&self) -> bool {
        matches!(self, Self::EOF)
    }
    /// Returns true if the result is `Error`.
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }
    /// Returns the number of bytes read.
    /// # Panics
    /// Panics if the result is `EOF` or `Error`.
    #[inline(always)]
    pub fn unwrap(self) -> usize {
        match self {
            Self::Success(v) => v,
            Self::EOF => panic!("Called unwrap on EOF"),
            Self::Error(e) => panic!("Called unwrap on Error: {}", e),
        }
    }
}

/// A buffered reader which reads data as ascii characters.
pub struct AsciiStreamReader<R> {
    inner: BufReader<R>,
}

impl<R: Read> AsciiStreamReader<R> {
    /// Creates a new AsciiStreamReader with a default 8KB buffer capacity.
    pub fn new(inner: R) -> Self {
        Self {
            inner: BufReader::new(inner),
        }
    }
    /// Creates a new AsciiStreamReader with the specified buffer capacity.
    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        Self {
            inner: BufReader::with_capacity(capacity, inner),
        }
    }
    /// Returns the number of bytes the internal buffer can hold.
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }
    /// Reads the next line of ascii characters into the specified AsciiString.
    #[inline]
    pub fn read_line(&mut self, buf: &mut AsciiString) -> ReadLineResult {
        let result = self.read_until(b'\n', buf);
        match result {
            Ok(result) => {
                if result > 0 {
                    if buf[result - 1] == b'\n' {
                        buf.pop();
                        if result > 1 && buf[result - 2] == b'\r' {
                            buf.pop();
                        }
                    }
                    ReadLineResult::Success(buf.len())
                } else {
                    ReadLineResult::EOF
                }
            }
            Err(err) => ReadLineResult::Error(err),
        }
        // if let Ok(result) = result {
        //     if result > 0 {
        //         if buf[result - 1] == b'\n' {
        //             buf.pop();
        //             if result > 1 && buf[result - 2] == b'\r' {
        //                 buf.pop();
        //             }
        //         }
        //         return Ok(Success(buf.len()));
        //     } else {
        //         return Ok(ReadLineResult::EOF);
        //     }
        // }
        // result
    }
    /// Reads until the specified byte is encountered, or EOF is reached, into the specified AsciiString.
    /// * the specified byte is included in the AsciiString.
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
    /// Reads until EOF is reached, into the specified AsciiString.
    pub fn read_to_end(&mut self, buf: &mut AsciiString) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = Vec::new();
        let result = self.inner.read_to_end(&mut vec);
        if result.is_ok() {
            *buf = AsciiString::from(vec);
        }
        result
    }
    /// Reads the specified number of bytes into the specified AsciiString.
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
        stream.read_line(&mut buf);
        assert_eq!(buf.to_string(), "This is test1");
        stream.read_line(&mut buf);
        assert_eq!(buf.to_string(), "This is test2");
        stream.read_line(&mut buf);
        assert_eq!(buf.to_string(), "This is test3");
        let r = stream.read_line(&mut buf);
        assert_eq!(buf.to_string(), "");
        assert!(r.is_eof());
    }
}

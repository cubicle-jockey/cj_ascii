#![cfg(feature = "async")]

use crate::ascii_consts::*;
use crate::ascii_stream::ReadLineResult;
use crate::ascii_string::AsciiString;
use futures::io::{AsyncBufRead, AsyncRead, AsyncWrite, BufReader};
use futures::{AsyncBufReadExt, AsyncReadExt};

/// An asynchronous buffered reader which reads data as ascii characters.
/// # Examples
/// ```
/// async fn read_example() {
///     use futures::io::Cursor;
///     use cj_ascii::ascii_stream_async::AsciiStreamReaderAsync;
///     use cj_ascii::ascii_string::AsciiString;
///
///     let mut stream = AsciiStreamReaderAsync::new(Cursor::new(b"abc\r\ndef\r\nghi"));
///     let mut buf = AsciiString::new();
///     while stream.read_line(&mut buf).await.is_success() {
///         println!("{}", buf);
///     }
/// }
/// ```
/// tokio example
/// ``` no_run
/// async fn read_example_tokio() {
///     use cj_ascii::ascii_stream_async::AsciiStreamReaderAsync;
///     use cj_ascii::ascii_string::AsciiString;
///     use tokio_util::compat::*;
///
///     let file_name = "C:/Temp/EnglishWords/words_ansi.txt";
///     let file = tokio::fs::File::open(file_name).await.unwrap();
///     let mut stream = AsciiStreamReaderAsync::new(file.compat());
///
///     let mut line = AsciiString::new();
///     while stream.read_line(&mut line).await.is_success() {
///         println!("{}", line);
///     }
/// }
/// ```

#[derive(Debug)]
pub struct AsciiStreamReaderAsync<R> {
    inner: BufReader<R>,
}

impl<R: AsyncRead + Unpin> AsciiStreamReaderAsync<R> {
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

    // pub fn capacity(&self) -> usize {
    //     self.inner.capacity()
    // }

    pub async fn read_line(&mut self, buf: &mut AsciiString) -> ReadLineResult {
        let result = self.read_until(LF, buf).await;
        match result {
            Ok(result) => {
                if result > 0 {
                    if buf[result - 1] == LF {
                        buf.pop();
                        if result > 1 && buf[result - 2] == CR {
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
    }

    pub async fn read_until(&mut self, byte: u8, buf: &mut AsciiString) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = Vec::new();
        let result = self.inner.read_until(byte, &mut vec).await;
        if result.is_ok() {
            *buf = AsciiString::from(vec);
        }
        result
    }

    pub async fn read_to_end(&mut self, buf: &mut AsciiString) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = Vec::new();
        let result = self.inner.read_to_end(&mut vec).await;
        if result.is_ok() {
            *buf = AsciiString::from(vec);
        }
        result
    }

    pub async fn read_bytes(
        &mut self,
        buf: &mut AsciiString,
        len: usize,
    ) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = Vec::with_capacity(len);
        vec.resize(len, 0);
        let result = self.inner.read(&mut vec).await;
        if let Ok(result) = result {
            vec.truncate(result);
            *buf = AsciiString::from(vec);
            Ok(result)
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_read_line() {
        use super::*;
        use futures::io::Cursor;

        let mut stream = AsciiStreamReaderAsync::new(Cursor::new(b"abc\r\ndef\r\nghi"));
        let mut buf = AsciiString::new();
        let result = stream.read_line(&mut buf).await;
        assert_eq!(result.unwrap(), 3);
        assert_eq!(buf.to_string(), "abc");
        let result = stream.read_line(&mut buf).await;
        assert_eq!(result.unwrap(), 3);
        assert_eq!(buf.to_string(), "def");
        let result = stream.read_line(&mut buf).await;
        assert_eq!(result.unwrap(), 3);
        assert_eq!(buf.to_string(), "ghi");
        let result = stream.read_line(&mut buf).await;
        assert!(result.is_eof());
    }

    #[tokio::test]
    async fn test_read_lines() {
        use super::*;
        use futures::io::Cursor;

        let mut stream = AsciiStreamReaderAsync::new(Cursor::new(b"abc\r\ndef\r\nghi"));
        let mut buf = AsciiString::new();
        while stream.read_line(&mut buf).await.is_success() {
            println!("{}", buf);
        }
    }
}

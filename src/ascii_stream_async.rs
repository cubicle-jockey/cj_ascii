#![cfg(feature = "async")]

use crate::ascii_consts::*;
use crate::ascii_stream::ReadLineResult;
use crate::ascii_string::AsciiString;
use futures::io::{AsyncRead, AsyncWrite, BufReader, BufWriter};
use futures::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

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
    /// Creates a new `AsciiStreamReaderAsync` with a default buffer capacity.
    pub fn new(inner: R) -> Self {
        Self {
            inner: BufReader::new(inner),
        }
    }
    /// Creates a new `AsciiStreamReaderAsync` with the specified buffer capacity.
    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        Self {
            inner: BufReader::with_capacity(capacity, inner),
        }
    }
    /// Reads a line from the stream into the specified buffer, removing the line ending.
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
    /// Reads data from the stream until the specified byte is encountered.
    /// * The specified byte is included in the returned data.
    pub async fn read_until(&mut self, byte: u8, buf: &mut AsciiString) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = Vec::new();
        let result = self.inner.read_until(byte, &mut vec).await;
        if result.is_ok() {
            *buf = AsciiString::from(vec);
        }
        result
    }

    /// Reads all data from the stream until EOF is encountered.
    pub async fn read_to_end(&mut self, buf: &mut AsciiString) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = Vec::new();
        let result = self.inner.read_to_end(&mut vec).await;
        if result.is_ok() {
            *buf = AsciiString::from(vec);
        }
        result
    }
    /// Reads the specified number of bytes from the stream.
    pub async fn read_bytes(
        &mut self,
        buf: &mut AsciiString,
        len: usize,
    ) -> std::io::Result<usize> {
        buf.clear();
        let mut vec = vec![0u8; len]; // Vec::with_capacity(len);
                                      //vec.resize(len, 0);
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

/// An asynchronous buffered writer which writes data as ascii characters.
/// # Examples
/// ```
/// async fn write_example() {
///     use cj_ascii::ascii_stream_async::AsciiStreamWriterAsync;
///     use cj_ascii::ascii_string::AsciiString;
///     use futures::io::Cursor;
///
///     let mut stream = AsciiStreamWriterAsync::new(Cursor::new(Vec::new()));
///     let mut buf = AsciiString::new();
///     buf += "abc";
///     stream.write_line(&buf).await.unwrap();
///
///     buf.clear();
///     buf += "def";
///     stream.write_line_crlf(&buf).await.unwrap();
///
///     buf.clear();
///     buf += "ghi";
///     stream.write(&buf).await.unwrap();
///
///     let result = stream.flush().await;
///     assert!(result.is_ok());
///
///     let result = stream.into_inner();
///     assert_eq!(result.into_inner(), b"abc\ndef\r\nghi");
///}
/// ```
#[derive(Debug)]
pub struct AsciiStreamWriterAsync<W> {
    inner: BufWriter<W>,
}

impl<W: AsyncWrite + Unpin> AsciiStreamWriterAsync<W> {
    /// Creates a new `AsciiStreamWriterAsync` with a default buffer capacity.
    pub fn new(inner: W) -> Self {
        Self {
            inner: BufWriter::new(inner),
        }
    }

    /// Creates a new `AsciiStreamWriterAsync` with the specified buffer capacity.
    pub fn with_capacity(capacity: usize, inner: W) -> Self {
        Self {
            inner: BufWriter::with_capacity(capacity, inner),
        }
    }

    /// Writes the specified buffer to the stream.
    pub async fn write(&mut self, buf: &AsciiString) -> std::io::Result<()> {
        //self.inner.write_all(buf.as_bytes())
        let (a, b) = buf.bytes.as_slices();
        let mut result = Ok(());
        if !a.is_empty() {
            result = self.inner.write_all(a).await;
        }
        if result.is_ok() && !b.is_empty() {
            result = self.inner.write_all(b).await;
        }
        result
    }

    /// Writes the specified buffer to the stream, followed by a line feed.
    pub async fn write_line(&mut self, buf: &AsciiString) -> std::io::Result<()> {
        self.write(buf).await?;
        self.inner.write_all(&[LF]).await
    }

    /// Writes the specified buffer to the stream, followed by a carriage return and a line feed.
    pub async fn write_line_crlf(&mut self, buf: &AsciiString) -> std::io::Result<()> {
        self.write(buf).await?;
        self.inner.write_all(&[CR, LF]).await
    }

    /// Flushes the stream.
    pub async fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush().await
    }

    /// Consumes self, returning the underlying writer.
    pub fn into_inner(self) -> W {
        self.inner.into_inner()
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

    #[tokio::test]
    async fn test_write_line() {
        use super::*;
        use futures::io::Cursor;

        let mut stream = AsciiStreamWriterAsync::new(Cursor::new(Vec::new()));
        let mut buf = AsciiString::new();
        buf += "abc";
        stream.write_line(&buf).await.unwrap();

        buf.clear();
        buf += "def";
        stream.write_line_crlf(&buf).await.unwrap();

        buf.clear();
        buf += "ghi";
        stream.write(&buf).await.unwrap();

        let result = stream.flush().await;
        assert!(result.is_ok());
        let result = stream.into_inner();
        assert_eq!(
            result.into_inner(),
            [97, 98, 99, 10, 100, 101, 102, 13, 10, 103, 104, 105]
        );
    }
}

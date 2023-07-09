pub use crate::ascii_common::ReadLineResult;
use crate::ascii_consts::*;
use crate::ascii_string::AsciiString;
use std::io::{BufRead, BufReader, BufWriter, IntoInnerError, Read, Write};

/// A buffered reader which reads data as ascii characters.
/// # Sample Usage
/// ```
/// use cj_ascii::ascii_stream::*;
/// use cj_ascii::ascii_string::*;
/// use std::io::Cursor;
///
/// let mut reader = AsciiStreamReader::new(
///     // the binary data below contains "This is test 1\nThis is test 2\r\nThis is test 3"
///     Cursor::new(
///         [84, 104, 105, 115, 32, 105, 115, 32, 116, 101, 115, 116, 32, 49, 10, 84,
///          104, 105, 115, 32, 105, 115, 32, 116, 101, 115, 116, 32, 50, 13, 10, 84,
///          104, 105, 115, 32, 105, 115, 32, 116, 101, 115, 116, 32, 51]
///     )
/// );
///
/// let mut astring = AsciiString::new();
///
/// // line 1 is terminated by \n. the \n is discarded.
/// let result = reader.read_line(&mut astring);
/// assert!(result.is_success());
/// assert_eq!(astring.to_string(), "This is test 1");
///
/// // line 2 is terminated by \r\n. both are discarded.
/// let result = reader.read_line(&mut astring);
/// assert!(result.is_success());
/// assert_eq!(astring.to_string(), "This is test 2");
///
/// // line 3 is the remainder of the data.
/// let result = reader.read_line(&mut astring);
/// assert!(result.is_success());
/// assert_eq!(astring.to_string(), "This is test 3");
///
/// // the end of the stream has been reached, so the result is EOF.
/// let result = reader.read_line(&mut astring);
/// assert!(result.is_eof());
/// ```
/// shorter example with the same data:
/// ```
/// # use cj_ascii::ascii_stream::*;
/// # use cj_ascii::ascii_string::*;
/// # use std::io::Cursor;
///
/// let mut reader = AsciiStreamReader::new(
///     Cursor::new(
///         [84, 104, 105, 115, 32, 105, 115, 32, 116, 101, 115, 116, 32, 49, 10, 84,
///          104, 105, 115, 32, 105, 115, 32, 116, 101, 115, 116, 32, 50, 13, 10, 84,
///          104, 105, 115, 32, 105, 115, 32, 116, 101, 115, 116, 32, 51]
///     )
/// );
///
/// let mut astring = AsciiString::new();
/// while reader.read_line(&mut astring).is_success() {
///    println!("{}", astring);
/// }
/// ```
#[derive(Debug)]
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
    /// * the line terminator is discarded.
    /// * the returned Success(value) is the number of bytes pushed to AsciiString, not the number of bytes read
    ///   * meaning that if the line terminator is \r\n, the returned value is 2 less than the number of bytes read.
    ///   * returned value of 0 does not mean EOF. It means that the line is empty, but EOF has not been reached.
    #[inline]
    pub fn read_line(&mut self, buf: &mut AsciiString) -> ReadLineResult {
        let result = self.read_until(LF, buf);
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
        let mut vec = vec![0; len];
        let result = self.inner.read(&mut vec);
        if let Ok(result) = result {
            vec.truncate(result);
            *buf = AsciiString::from(vec);
            Ok(result)
        } else {
            result
        }
    }
}

/// A buffered writer that writes ascii characters to an underlying stream.
/// # Sample Usage
/// ```
/// # use cj_ascii::ascii_stream::*;
/// # use cj_ascii::ascii_string::*;
/// let mut writer = AsciiStreamWriter::new(Vec::new());
///
/// let mut astring = AsciiString::new();
/// astring += "The beginning.";
/// writer.write_line(&astring).unwrap();
///
/// astring.clear();
/// astring += "The middle.";
/// writer.write_line(&astring).unwrap();
///
/// astring.clear();
/// astring += "The end.";
/// writer.write(&astring).unwrap();
///
/// let result = writer.flush();
/// assert!(result.is_ok());
/// let vec = writer.into_inner().unwrap();
/// assert_eq!(vec,
///            [84, 104, 101, 32, 98, 101, 103, 105, 110, 110, 105, 110, 103,
///             46, 10, 84, 104, 101, 32, 109, 105, 100, 100, 108, 101, 46,
///             10, 84, 104, 101, 32, 101, 110, 100, 46]
/// );
///
/// let result = AsciiString::from(vec);
/// assert_eq!(result.to_string(),"The beginning.\nThe middle.\nThe end.");
/// ```

#[derive(Debug)]
pub struct AsciiStreamWriter<W: Write> {
    inner: BufWriter<W>,
}

impl<W: Write> AsciiStreamWriter<W> {
    /// Creates a new AsciiStreamWriter with a default 8KB buffer capacity.
    pub fn new(inner: W) -> Self {
        Self {
            inner: BufWriter::new(inner),
        }
    }
    /// Creates a new AsciiStreamWriter with the specified buffer capacity.
    pub fn with_capacity(capacity: usize, inner: W) -> Self {
        Self {
            inner: BufWriter::with_capacity(capacity, inner),
        }
    }
    /// Returns the number of bytes the internal buffer can hold.
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }
    /// Writes the entire AsciiString to the stream.    
    #[inline]
    pub fn write(&mut self, buf: &AsciiString) -> std::io::Result<()> {
        //self.inner.write_all(buf.as_bytes())
        let (a, b) = buf.bytes.as_slices();
        let mut result = Ok(());
        if !a.is_empty() {
            result = self.inner.write_all(a);
        }
        if result.is_ok() && !b.is_empty() {
            result = self.inner.write_all(b);
        }
        result
    }
    /// Writes the entire AsciiString to the stream, followed by a newline.   
    #[inline]
    pub fn write_line(&mut self, buf: &AsciiString) -> std::io::Result<()> {
        self.write(buf)?;
        self.inner.write_all(&[LF])
    }
    /// Writes the entire AsciiString to the stream, followed by a carriage return and a newline.    
    #[inline]
    pub fn write_line_crlf(&mut self, buf: &AsciiString) -> std::io::Result<()> {
        self.write(buf)?;
        self.inner.write_all(&[CR, LF])
    }
    /// Flushes the internal buffer, writing all buffered bytes to the underlying stream.
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
    /// Unwraps this AsciiStreamWriter, returning the underlying writer.
    /// The buffer is written out before returning the writer
    pub fn into_inner(self) -> Result<W, IntoInnerError<BufWriter<W>>> {
        self.inner.into_inner()
    }
}

#[cfg(test)]
mod test {
    use std::io::{Cursor, Write};

    #[test]
    fn test_ascii_stream_reader() {
        use super::*;

        let mut stream = AsciiStreamReader::new(Cursor::new(
            b"This is test1\nThis is test2\r\nThis is test3",
        ));
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

    #[test]
    fn test_ascii_stream_reader_bytes() {
        use super::*;

        let mut stream = AsciiStreamReader::new(Cursor::new(
            b"This is test1\nThis is test2\r\nThis is test3",
        ));
        let mut buf = AsciiString::new();
        stream.read_bytes(&mut buf, 14);
        assert_eq!(buf.to_string(), "This is test1\n");
        stream.read_bytes(&mut buf, 15);
        assert_eq!(buf.to_string(), "This is test2\r\n");
        stream.read_bytes(&mut buf, 13);
        assert_eq!(buf.to_string(), "This is test3");
        let _r = stream.read_bytes(&mut buf, 14);
        assert_eq!(buf.to_string(), "");
    }

    #[test]
    fn test_ascii_stream_writer() {
        use super::*;

        let mut stream = AsciiStreamWriter::new(Vec::new());
        let mut buf = AsciiString::new();
        buf.push_str("This is test1");
        stream.write_line(&buf).unwrap();

        buf.clear();
        buf += "test2";
        stream.write_line_crlf(&buf).unwrap();

        buf.clear();
        buf += "test3";
        stream.write(&buf).unwrap();

        assert_eq!(
            stream.into_inner().unwrap(),
            b"This is test1\ntest2\r\ntest3"
        );
    }
}

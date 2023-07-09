/// The result of a call to `AsciiStreamReader::read_line()`.
#[derive(Debug)]
pub enum ReadLineResult {
    /// The number of bytes pushed to the result buf.
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

#[cfg(test)]
mod test {
    #[test]
    fn test_is_success() {
        use super::ReadLineResult;
        assert!(ReadLineResult::Success(0).is_success());
        assert!(!ReadLineResult::EOF.is_success());
        assert!(
            !ReadLineResult::Error(std::io::Error::new(std::io::ErrorKind::Other, "test"))
                .is_success()
        );
    }
    #[test]
    fn test_is_eof() {
        use super::ReadLineResult;
        assert!(!ReadLineResult::Success(0).is_eof());
        assert!(ReadLineResult::EOF.is_eof());
        assert!(
            !ReadLineResult::Error(std::io::Error::new(std::io::ErrorKind::Other, "test")).is_eof()
        );
    }
    #[test]
    fn test_is_error() {
        use super::ReadLineResult;
        assert!(!ReadLineResult::Success(0).is_error());
        assert!(!ReadLineResult::EOF.is_error());
        assert!(
            ReadLineResult::Error(std::io::Error::new(std::io::ErrorKind::Other, "test"))
                .is_error()
        );
    }
}

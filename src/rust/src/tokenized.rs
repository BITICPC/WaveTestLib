use std::io::{Read, BufReader, ErrorKind};


/// Provide the ability to read textual contents token by token. Tokens are
/// recognized and splitted by whitespace characters (space, `\t`, `\r` and `\n`).
pub trait TokenizedRead {
    /// Read one token from the reader. If no more tokens are available, returns
    /// [`None`]. Panics if the raw bytes of the next token cannot be interpreted
    /// as a valid UTF-8 string.
    fn read_token(&mut self) -> Option<String>;

    /// Read until either EOF or end-of-line is hit. If no more tokens are available,
    /// returns [`None`]. The end-of-line character (`\n` on unix and `\r\n` on
    /// Windows) and any leading whitespace characters will not be present in the 
    /// returned string. Panics if the raw bytes of the next line cannot be 
    /// interpreted as a valid UTF-8 string.
    fn read_line(&mut self) -> Option<String>;
}

/// The [`TokenizedReader`] tokenize the content of the underlying reader.
pub struct TokenizedReader<T: Read> {
    inner: BufReader<T>
}

impl<T: Read> TokenizedReader<T> {
    /// Whitespace characters.
    const WHITESPACES: &'static [u8] = &[b' ', b'\t', b'\r', b'\n'];

    /// Create a new [`TokenizedReader`] instance, wrapping around the given
    /// [`Read`] instance. The size of the internal buffer will be set to a
    /// default value.
    pub fn new(source: T) -> TokenizedReader<T> {
        TokenizedReader {
            inner: BufReader::new(source)
        }
    }

    /// Read a single byte from the underlying [`Read`] instance.
    fn read_byte(&mut self) -> Option<u8> {
        let mut buffer = [0u8];
        match self.inner.read_exact(&mut buffer) {
            Ok(..) => Some(buffer[0]),
            Err(err) => match err.kind() {
                ErrorKind::UnexpectedEof => None,
                _ => panic!("Unexpected error while reading: {}", err)
            }
        }
    }

    /// Read raw bytes from the underlying [`Read`] instance until one of the given
    /// delimiters or EOF are hit. Returns `Some(())` if at least one bytes are
    /// retrived into `buffer`; otherwise returns `None`.
    fn read_until(&mut self, delimiters: &[u8], buffer: &mut Vec<u8>) -> Option<()> {
        // Skip any leading whitespace characters.
        let mut ch: u8;
        loop {
            ch = match self.read_byte() {
                Some(b) => b,
                None => return None
            };
            if !delimiters.contains(&ch) {
                break;
            }
        }

        while !delimiters.contains(&ch) {
            buffer.push(ch);
            ch = match self.read_byte() {
                Some(b) => b,
                None => break
            };
        }

        Some(())
    }

    /// Get the inner [`BufReader`] instance
    pub fn inner_reader(&self) -> &BufReader<T> {
        &self.inner
    }

    /// Get the mutable inner [`BufRead`] instance that can be used to read
    /// raw bytes directly.
    pub fn inner_reader_mut(&mut self) -> &mut BufReader<T> {
        &mut self.inner
    }
}

impl<T: Read> TokenizedRead for TokenizedReader<T> {
    fn read_token(&mut self) -> Option<String> {
        let mut buffer = Vec::new();
        self.read_until(TokenizedReader::<T>::WHITESPACES, &mut buffer)
            .map(|_| String::from_utf8(buffer).unwrap())
    }

    fn read_line(&mut self) -> Option<String> {
        let mut buffer = Vec::new();
        self.read_until(&[b'\r', b'\n'], &mut buffer)
            .map(|_| String::from_utf8(buffer).unwrap())
    }
}

pub mod preclude {
    pub use super::{TokenizedRead, TokenizedReader};
}


#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::preclude::*;

    struct MemoryRead<'a> {
        buffer: &'a [u8],
        ptr: usize
    }

    impl<'a> MemoryRead<'a> {
        fn new(buffer: &'a [u8]) -> MemoryRead<'a> {
            MemoryRead {
                buffer,
                ptr: 0
            }
        }
    }

    impl<'a> Read for MemoryRead<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let mut buf_ptr = 0usize;
            while buf_ptr < buf.len() && self.ptr < self.buffer.len() {
                buf[buf_ptr] = self.buffer[self.ptr];
                buf_ptr += 1;
                self.ptr += 1;
            }

            Ok(buf_ptr)
        }
    }

    fn create_test_reader() -> TokenizedReader<MemoryRead<'static>> {
        TokenizedReader::new(
            MemoryRead::new(
                "The quick brown fox jumps over the lazy dog\nThe quick dog jumps over the lazy brown fox\n"
                .as_bytes()
            )
        )
    }

    #[test]
    fn test_read_token() {
        let mut reader = create_test_reader();

        assert_eq!(reader.read_token(), Some(String::from("The")));
        assert_eq!(reader.read_token(), Some(String::from("quick")));
        assert_eq!(reader.read_token(), Some(String::from("brown")));
        assert_eq!(reader.read_token(), Some(String::from("fox")));
        assert_eq!(reader.read_token(), Some(String::from("jumps")));
        assert_eq!(reader.read_token(), Some(String::from("over")));
        assert_eq!(reader.read_token(), Some(String::from("the")));
        assert_eq!(reader.read_token(), Some(String::from("lazy")));
        assert_eq!(reader.read_token(), Some(String::from("dog")));
        assert_eq!(reader.read_token(), Some(String::from("The")));
        assert_eq!(reader.read_token(), Some(String::from("quick")));
        assert_eq!(reader.read_token(), Some(String::from("dog")));
        assert_eq!(reader.read_token(), Some(String::from("jumps")));
        assert_eq!(reader.read_token(), Some(String::from("over")));
        assert_eq!(reader.read_token(), Some(String::from("the")));
        assert_eq!(reader.read_token(), Some(String::from("lazy")));
        assert_eq!(reader.read_token(), Some(String::from("brown")));
        assert_eq!(reader.read_token(), Some(String::from("fox")));
        assert_eq!(reader.read_token(), None);
    }

    #[test]
    fn test_read_line() {
        let mut reader = create_test_reader();

        assert_eq!(reader.read_line(), 
            Some(String::from("The quick brown fox jumps over the lazy dog")));
        assert_eq!(reader.read_token(), Some(String::from("The")));
        assert_eq!(reader.read_line(), 
            Some(String::from("quick dog jumps over the lazy brown fox")));
        assert_eq!(reader.read_line(), None);
    }
}

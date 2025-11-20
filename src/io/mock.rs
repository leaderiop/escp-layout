//! Mock I/O implementations for testing

#[cfg(test)]
pub use testing::*;

#[cfg(test)]
mod testing {
    use std::io::{self, Read, Write};

    /// Mock writer for testing
    pub struct MockWriter {
        buffer: Vec<u8>,
        write_count: usize,
    }

    impl Default for MockWriter {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MockWriter {
        pub fn new() -> Self {
            Self {
                buffer: Vec::new(),
                write_count: 0,
            }
        }

        /// Get all bytes written so far
        pub fn written(&self) -> &[u8] {
            &self.buffer
        }

        /// Get number of write calls
        pub fn write_count(&self) -> usize {
            self.write_count
        }
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.write_count += 1;
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// Mock reader for testing
    pub struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        pub fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.position >= self.data.len() {
                return Ok(0); // EOF
            }

            let remaining = &self.data[self.position..];
            let to_read = std::cmp::min(buf.len(), remaining.len());
            buf[..to_read].copy_from_slice(&remaining[..to_read]);
            self.position += to_read;
            Ok(to_read)
        }
    }

    #[test]
    fn test_mock_writer() {
        let mut writer = MockWriter::new();

        writer.write_all(b"Hello").unwrap();
        writer.write_all(b", World!").unwrap();

        assert_eq!(writer.written(), b"Hello, World!");
        assert_eq!(writer.write_count(), 2);
    }

    #[test]
    fn test_mock_reader() {
        let mut reader = MockReader::new(vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
        let mut buf = [0u8; 5];

        let n = reader.read(&mut buf).unwrap();

        assert_eq!(n, 5);
        assert_eq!(&buf, b"Hello");

        // Reading again should return EOF
        let n = reader.read(&mut buf).unwrap();
        assert_eq!(n, 0);
    }
}

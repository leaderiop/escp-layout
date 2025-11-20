//! Retry logic for partial writes

use std::io::{self, Write};

/// Write all data with automatic retry for partial writes
///
/// This function ensures that all bytes are written even if the underlying
/// Write implementation returns partial writes. It automatically retries
/// until all data is written or an error occurs.
pub fn write_all_with_retry<W: Write>(writer: &mut W, mut data: &[u8]) -> io::Result<()> {
    while !data.is_empty() {
        match writer.write(data) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ));
            }
            Ok(n) => {
                data = &data[n..];
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                // Retry on interrupt
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    // Ensure bytes are flushed to device
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PartialWriter {
        buffer: Vec<u8>,
        chunk_size: usize,
    }

    impl PartialWriter {
        fn new(chunk_size: usize) -> Self {
            Self {
                buffer: Vec::new(),
                chunk_size,
            }
        }
    }

    impl Write for PartialWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let to_write = buf.len().min(self.chunk_size);
            self.buffer.extend_from_slice(&buf[..to_write]);
            Ok(to_write)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_write_all_with_retry_full_write() {
        let mut writer = Vec::new();
        let data = b"Hello, World!";

        write_all_with_retry(&mut writer, data).unwrap();

        assert_eq!(writer, data);
    }

    #[test]
    fn test_write_all_with_retry_partial_writes() {
        let mut writer = PartialWriter::new(3);
        let data = b"0123456789";

        write_all_with_retry(&mut writer, data).unwrap();

        assert_eq!(writer.buffer, data);
    }

    #[test]
    fn test_write_all_with_retry_single_byte() {
        let mut writer = PartialWriter::new(1);
        let data = b"ABC";

        write_all_with_retry(&mut writer, data).unwrap();

        assert_eq!(writer.buffer, data);
    }
}

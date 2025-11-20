//! Timeout handling for read operations

use std::io::{self, Read};
use std::time::{Duration, Instant};

/// Read a single byte with timeout
///
/// This function attempts to read one byte from the reader, retrying if the
/// operation would block, until the timeout expires.
pub fn read_byte_with_timeout<R: Read>(reader: &mut R, timeout: Duration) -> io::Result<u8> {
    let mut buf = [0u8; 1];
    let start = Instant::now();

    loop {
        match reader.read(&mut buf) {
            Ok(0) => {
                // EOF - printer disconnected
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "printer disconnected",
                ));
            }
            Ok(_) => {
                return Ok(buf[0]);
            }
            Err(e)
                if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut =>
            {
                if start.elapsed() >= timeout {
                    return Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        format!("timeout after {:?}", timeout),
                    ));
                }
                // Retry with small sleep to avoid busy waiting
                std::thread::sleep(Duration::from_millis(10));
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_byte_with_timeout_success() {
        let data = vec![0x42];
        let mut reader = Cursor::new(data);

        let result = read_byte_with_timeout(&mut reader, Duration::from_secs(1));

        assert_eq!(result.unwrap(), 0x42);
    }

    #[test]
    fn test_read_byte_with_timeout_eof() {
        let data = vec![];
        let mut reader = Cursor::new(data);

        let result = read_byte_with_timeout(&mut reader, Duration::from_secs(1));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }
}

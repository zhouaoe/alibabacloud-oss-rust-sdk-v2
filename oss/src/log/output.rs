use std::io::{self, Write};
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

/// Represents the possible output destinations for logging.
#[derive(Default)]
pub enum LogOutput {
    /// Output to standard output. Default option.
    #[default]
    Stdout,
    /// Output to a file with the specified path.
    File(String),
    /// Output to a custom writer implementing the `Write` trait and `Send`
    /// marker trait.
    Custom(Arc<RwLock<dyn Write + Send + Sync>>),
}

/// A struct that provides a string buffer for writing log messages.
#[derive(Default)]
pub struct StringWriter {
    buffer: RwLock<String>,
}

impl StringWriter {
    /// Creates a new `StringWriter` instance.
    pub fn new() -> Self {
        StringWriter::default()
    }

    /// Retrieves the logged string from the buffer.
    pub fn get_logged_string(&self) -> String {
        self.buffer
            .read()
            .expect("Already locked by current thread")
            .clone()
    }
}

impl Write for StringWriter {
    /// Writes the given byte slice to the string buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The byte slice to write.
    ///
    /// # Returns
    ///
    /// The number of bytes written.
    ///
    /// # Errors
    ///
    /// An error is returned if the byte slice contains invalid UTF-8 data.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut buffer = self
            .buffer
            .write()
            .expect("Already locked by current thread");
        let s = std::str::from_utf8(buf)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"))?;
        buffer.deref_mut().push_str(s);
        Ok(buf.len())
    }

    /// Flushes the string buffer. Here we do nothing.
    ///
    /// # Errors
    ///
    /// Always ok.
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut writer = StringWriter::new();
        writer.write_all(b"Hello, world!").unwrap();
        assert_eq!(writer.get_logged_string(), "Hello, world!");
    }

    #[test]
    fn test_flush() {
        let mut writer = StringWriter::new();
        writer.write_all(b"Hello, world!").unwrap();
        assert!(writer.flush().is_ok());
        assert_eq!(writer.get_logged_string(), "Hello, world!");
    }
}

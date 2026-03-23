use std::io::{self, Write};

/// Type alias for the progress function.
#[allow(unused)]
pub(crate) type ProgressFunc = fn(increment: i64, transferred: i64, total: i64);

/// Struct that tracks the progress of a write operation.
#[allow(unused)]
pub(crate) struct ProgressTracker {
    progress: ProgressFunc,
    written: i64,
    last_written: i64,
    total: i64,
}

#[allow(unused)]
impl ProgressTracker {
    /// Creates a new `ProgressTracker` instance.
    ///
    /// # Arguments
    ///
    /// * `progress` - The progress function to be called during the write
    ///   operation.
    /// * `total` - The total number of bytes to be written.
    pub(crate) fn new(progress: ProgressFunc, total: i64) -> Self {
        ProgressTracker {
            progress,
            written: 0,
            last_written: 0,
            total,
        }
    }

    /// Resets the progress tracker by setting the written bytes to zero.
    pub(crate) fn reset(&mut self) {
        self.last_written = self.written;
        self.written = 0;
    }
}

impl Write for ProgressTracker {
    /// Writes the contents of `buf` to the underlying writer and updates the
    /// progress.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer containing the data to be written.
    ///
    /// # Returns
    ///
    /// The number of bytes written.
    ///
    /// # Errors
    ///
    /// This function will return an `io::Error` if the underlying writer
    /// encounters an error.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = buf.len();
        self.written += n as i64;
        if self.written > self.last_written {
            (self.progress)(n as i64, self.written, self.total);
        }
        Ok(n)
    }

    /// Flushes the underlying writer.
    ///
    /// # Errors
    ///
    /// This function will return an `io::Error` if the underlying writer
    /// encounters an error.
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_progress_func(_: i64, _: i64, _: i64) {}

    #[test]
    fn test_write_updates_written() {
        let mut progress = ProgressTracker::new(dummy_progress_func, 100);
        let buf = [0u8; 10];
        let _ = progress.write(&buf).unwrap();
        assert_eq!(progress.written, 10);
    }

    #[test]
    fn test_write_calls_progress_func() {
        let mut progress = ProgressTracker::new(dummy_progress_func, 100);
        let buf = [0u8; 10];
        let _ = progress.write(&buf).unwrap();
        assert_eq!(progress.last_written, 0);
        assert_eq!(progress.written, 10);
    }

    #[test]
    fn test_write_does_not_call_progress_func_if_written_not_greater_than_last_written() {
        let mut progress = ProgressTracker::new(dummy_progress_func, 100);
        let buf = [0u8; 5];
        let _ = progress.write(&buf).unwrap();
        assert_eq!(progress.last_written, 0);
        assert_eq!(progress.written, 5);
    }

    #[test]
    fn test_reset_resets_written() {
        let mut progress = ProgressTracker::new(dummy_progress_func, 100);
        let buf = [0u8; 10];
        let _ = progress.write(&buf).unwrap();
        progress.reset();
        assert_eq!(progress.last_written, 10);
        assert_eq!(progress.written, 0);
    }

    #[test]
    fn test_flush_does_not_return_error() {
        let mut progress = ProgressTracker::new(dummy_progress_func, 100);
        assert!(progress.flush().is_ok());
    }
}

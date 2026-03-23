use std::io::{self, Read};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

/// Reads the contents of a reader into a string.
///
/// This function takes an `Arc<Mutex<dyn Read + Send + Sync>>` as input, which
/// allows for concurrent access to the reader. It locks the reader, reads its
/// contents into a `String`, and returns the result as an `io::Result<String>`.
///
/// # Arguments
///
/// * `arc_reader` - An `Arc<Mutex<dyn Read + Send + Sync>>` representing the
///   reader to be read from.
///
/// # Returns
///
/// An `io::Result<String>` containing the contents of the reader if the read
/// operation is successful, or an `io::Error` if an error occurs.
#[allow(unused)]
pub(crate) fn read_to_string(arc_reader: Arc<Mutex<dyn Read + Send + Sync>>) -> io::Result<String> {
    let mut buffer = String::new();
    let mut reader = arc_reader.lock().expect("Already locked by current thread");
    reader.deref_mut().read_to_string(&mut buffer)?;
    Ok(buffer)
}

// TODO add a seek_to_string or something to handle the case where we need to
// read the data multiple times using a `dyn Read + Seek` reader

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_read_to_string() {
        // Test with an empty reader
        let reader = Arc::new(Mutex::new(Cursor::new(Vec::new())));
        let result = read_to_string(reader.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");

        // Test with a reader containing a single line
        let reader = Arc::new(Mutex::new(Cursor::new("Hello, World!".as_bytes().to_vec())));
        let result = read_to_string(reader.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");

        // Test with a reader containing multiple lines
        let reader = Arc::new(Mutex::new(Cursor::new(
            "Line 1\nLine 2\nLine 3".as_bytes().to_vec(),
        )));
        let result = read_to_string(reader.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Line 1\nLine 2\nLine 3");

        // Test with a reader containing large data
        let reader = Arc::new(Mutex::new(Cursor::new(
            "A".repeat(1_000_000).into_bytes().to_vec(),
        )));
        let result = read_to_string(reader.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1_000_000);
    }
}

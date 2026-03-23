use std::io;

use crate::retry::traits::ErrorRetryable;

pub struct ConnectionErrorRetryable;

// TODO should be rust version of error code
static RETRYABLE_ERROR_STRINGS: &[&str] = &[
    "connection reset",
    "connection refused",
    "use of closed network connection",
    "unexpected EOF reading trailer",
    "transport connection broken",
    "server closed idle connection",
    "bad record MAC",
    "stream error:",
    "tls: use of closed connection",
    "connection was forcibly closed",
    "broken pipe",
    "crc is inconsistent", // oss crc check error pattern
];

impl ErrorRetryable for ConnectionErrorRetryable {
    fn is_error_retryable(&self, err: &(dyn std::error::Error + 'static)) -> bool {
        if let Some(io_err) = err.downcast_ref::<io::Error>() {
            if matches!(
                io_err.kind(),
                io::ErrorKind::ConnectionReset
                    | io::ErrorKind::ConnectionRefused
                    | io::ErrorKind::UnexpectedEof
            ) {
                return true;
            }
        }
        RETRYABLE_ERROR_STRINGS
            .iter()
            .any(|&s| err.to_string().contains(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_error_retryable_connection_reset() {
        let retryable = ConnectionErrorRetryable;
        let err = io::Error::new(io::ErrorKind::ConnectionReset, "connection reset");
        assert!(retryable.is_error_retryable(&err));
    }

    #[test]
    fn test_is_error_retryable_connection_refused() {
        let retryable = ConnectionErrorRetryable;
        let err = io::Error::new(io::ErrorKind::ConnectionRefused, "connection refused");
        assert!(retryable.is_error_retryable(&err));
    }

    #[test]
    fn test_is_error_retryable_unexpected_eof() {
        let retryable = ConnectionErrorRetryable;
        let err = io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "unexpected EOF reading trailer",
        );
        assert!(retryable.is_error_retryable(&err));
    }

    #[test]
    fn test_is_error_retryable_retryable_error_strings() {
        let retryable = ConnectionErrorRetryable;
        let err = io::Error::new(io::ErrorKind::Other, "connection was forcibly closed");
        assert!(retryable.is_error_retryable(&err));
    }

    #[test]
    fn test_is_error_retryable_non_retryable_error() {
        let retryable = ConnectionErrorRetryable;
        let err = io::Error::new(io::ErrorKind::Other, "other error");
        assert!(!retryable.is_error_retryable(&err));
    }
}

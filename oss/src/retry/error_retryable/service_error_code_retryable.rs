use crate::retry::traits::ErrorRetryable;

pub struct ServiceErrorCodeRetryable;

// TODO fix RequestTimeTooSkewed by client
static RETRY_SERVICE_ERROR_CODES: &[&str] = &[
    "RequestTimeTooSkewed", // AWS S3
    "BadRequest"
];

impl ErrorRetryable for ServiceErrorCodeRetryable {
    fn is_error_retryable(&self, err: &(dyn std::error::Error + 'static)) -> bool {
        let err_code = err.to_string();  
        let res = RETRY_SERVICE_ERROR_CODES.iter().any(|code| err_code.contains(code));
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_error_retryable_with_retryable_error_code() {
        let retryable_error =
            std::io::Error::new(std::io::ErrorKind::Other, "RequestTimeTooSkewed");
        let retryable = ServiceErrorCodeRetryable;
        assert!(retryable.is_error_retryable(&retryable_error));
    }

    #[test]
    fn test_is_error_retryable_with_non_retryable_error_code() {
        let non_retryable_error = std::io::Error::new(std::io::ErrorKind::Other, "NotFound");
        let retryable = ServiceErrorCodeRetryable;
        assert!(!retryable.is_error_retryable(&non_retryable_error));
    }

    #[test]
    fn test_is_error_retryable_with_unknown_error_code() {
        let unknown_error = std::io::Error::new(std::io::ErrorKind::Other, "UnknownErrorCode");
        let retryable = ServiceErrorCodeRetryable;
        assert!(!retryable.is_error_retryable(&unknown_error));
    }
}

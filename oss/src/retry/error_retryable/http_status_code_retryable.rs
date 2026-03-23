use crate::retry::traits::ErrorRetryable;

pub struct HTTPStatusCodeRetryable;

/// retryable HTTP status codes
static RETRY_ERROR_CODES: &[http::StatusCode] = &[
    http::StatusCode::UNAUTHORIZED,
    http::StatusCode::REQUEST_TIMEOUT,
    http::StatusCode::TOO_MANY_REQUESTS,
];

impl ErrorRetryable for HTTPStatusCodeRetryable {
    fn is_error_retryable(&self, err: &(dyn std::error::Error + 'static)) -> bool {
        if let Some(http_error) = err.downcast_ref::<reqwest::Error>() {
            if let Some(status) = http_error.status() {
                RETRY_ERROR_CODES.contains(&status)
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "cannot create a reqwest Error using new()"]
    fn test_is_error_retryable_with_retryable_status_code() {
        // let retryable = HTTPStatusCodeRetryable;
        // let err = reqwest::Error::new(reqwest::StatusCode::UNAUTHORIZED,
        // None); assert!(retryable.is_error_retryable(&err));
    }

    #[test]
    #[ignore = "cannot create a reqwest Error using new()"]
    fn test_is_error_retryable_with_non_retryable_status_code() {
        // let retryable = HTTPStatusCodeRetryable;
        // let err = reqwest::Error::new(reqwest::StatusCode::NOT_FOUND, None);
        // assert!(!retryable.is_error_retryable(&err));
    }

    #[test]
    fn test_is_error_retryable_with_non_http_error() {
        let retryable = HTTPStatusCodeRetryable;
        let err = std::io::Error::new(std::io::ErrorKind::Other, "Some error");
        assert!(!retryable.is_error_retryable(&err));
    }
}

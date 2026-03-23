use std::time::Duration;

use super::traits::Retryer;

/// A retryer implementation that does not perform any retries.
#[derive(Default)]
pub struct NopRetryer;

impl NopRetryer {
    pub fn new() -> Self {
        NopRetryer
    }
}

impl Retryer for NopRetryer {
    /// Determines if the given error is retryable.
    ///
    /// In the case of `NopRetryer`, it always returns `false`.
    fn is_error_retryable(&self, _err: &(dyn std::error::Error + 'static)) -> bool {
        false
    }

    /// Returns the maximum number of attempts allowed.
    ///
    /// In the case of `NopRetryer`, it always returns `1`.
    fn max_attempts(&self) -> u32 {
        1
    }

    /// Returns the delay before the next retry attempt.
    ///
    /// In the case of `NopRetryer`, it always returns an `Err` containing a
    /// `NopRetryError`.
    fn retry_delay(
        &self,
        _attempt: u32,
        _err: &(dyn std::error::Error + 'static),
    ) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>> {
        Err("Not retrying any attempt errors".into())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;
    use std::io::ErrorKind::Other;

    use super::*;

    #[test]
    fn test_nop_retryer() {
        let nop_retryer = NopRetryer;

        // use arbitrary kind of error
        assert!(!nop_retryer.is_error_retryable(&Error::new(Other, "arbitrary error",)));
        assert_eq!(nop_retryer.max_attempts(), 1);
        assert!(nop_retryer
            .retry_delay(1, &Error::new(Other, "arbitrary error"))
            .is_err());
    }
}

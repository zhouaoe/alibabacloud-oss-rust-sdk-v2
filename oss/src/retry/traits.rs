use std::time::Duration;

/// Trait for determining if an error is retryable.
pub trait ErrorRetryable {
    /// Checks if the given error is retryable.
    ///
    /// # Arguments
    ///
    /// * `err` - The error to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if the error is retryable, `false` otherwise.
    fn is_error_retryable(&self, err: &(dyn std::error::Error + 'static)) -> bool;
}

/// Trait for providing backoff delay between retries.
pub trait BackoffDelayer {
    /// Calculates the backoff delay for the given retry attempt.
    ///
    /// # Arguments
    ///
    /// * `attempt` - The current retry attempt number.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the backoff delay as a `Duration` if
    /// successful, or an error as a `Box<dyn std::error::Error >` if there was
    /// an error calculating the delay.
    fn backoff_delay(
        &self,
        attempt: u32,
    ) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for implementing retry logic.
pub trait Retryer {
    /// Checks if the given error is retryable.
    ///
    /// # Arguments
    ///
    /// * `err` - The error to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if the error is retryable, `false` otherwise.
    fn is_error_retryable(&self, err: &(dyn std::error::Error + 'static)) -> bool;

    /// Gets the maximum number of retry attempts.
    ///
    /// # Returns
    ///
    /// Returns the maximum number of retry attempts as a `u32`.
    fn max_attempts(&self) -> u32;

    /// Calculates the delay between retries for the given retry attempt and
    /// error.
    ///
    /// # Arguments
    ///
    /// * `attempt` - The current retry attempt number.
    /// * `op_err` - The error that occurred during the operation.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the delay as a `Duration` if successful,
    /// or an error as a `Box<dyn std::error::Error >` if there was an error
    /// calculating the delay.
    fn retry_delay(
        &self,
        attempt: u32,
        op_err: &(dyn std::error::Error + 'static),
    ) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>>;
}

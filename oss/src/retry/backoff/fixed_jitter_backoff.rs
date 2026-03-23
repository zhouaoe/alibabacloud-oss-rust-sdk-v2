use std::time::Duration;

use crate::retry::traits::BackoffDelayer;

/// A backoff strategy that uses a fixed delay for each attempt.
pub struct FixedDelayBackoff {
    fixed_backoff: Duration,
}

impl FixedDelayBackoff {
    /// Creates a new `FixedDelayBackoff` with the specified fixed backoff
    /// duration.
    pub fn new(fixed_backoff: Duration) -> Self {
        Self { fixed_backoff }
    }
}

impl BackoffDelayer for FixedDelayBackoff {
    /// Calculates the backoff delay for the given attempt.
    ///
    /// # Arguments
    ///
    /// * `attempt` - The attempt number.
    ///
    /// # Returns
    ///
    /// The backoff delay as a `Result` containing a `Duration` or an error.
    fn backoff_delay(
        &self,
        _attempt: u32,
    ) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.fixed_backoff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retry::backoff::RETRIES_ATTEMPTED_CEILING;

    #[test]
    fn test_fixed_delay_backoff() {
        let fixed_delay = Duration::from_secs(20);
        assert!(fixed_delay > Duration::ZERO);

        let backoff = FixedDelayBackoff::new(fixed_delay);
        for i in 0..RETRIES_ATTEMPTED_CEILING * 2 {
            let delay = backoff.backoff_delay(i).unwrap();
            assert_eq!(delay, fixed_delay);
        }
    }
}

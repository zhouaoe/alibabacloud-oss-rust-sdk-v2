use std::cmp;
use std::ops::{Mul, Shl};
use std::time::Duration;

use rand::Rng;

use crate::retry::traits::BackoffDelayer;

/// A backoff strategy that implements the Full Jitter algorithm.
pub struct FullJitterBackoff {
    base_delay: Duration,
    max_backoff: Duration,
    attempt_ceiling: u32,
}

impl FullJitterBackoff {
    /// Creates a new instance of `FullJitterBackoff` with the specified base
    /// delay and maximum backoff.
    ///
    /// # Arguments
    ///
    /// * `base_delay` - The base delay duration.
    /// * `max_backoff` - The maximum backoff duration.
    ///
    /// # Returns
    ///
    /// A new instance of `FullJitterBackoff`.
    pub fn new(base_delay: Duration, max_backoff: Duration) -> Self {
        Self {
            base_delay,
            max_backoff,
            attempt_ceiling: (u64::MAX - base_delay.as_secs()).ilog2(),
        }
    }
}

impl BackoffDelayer for FullJitterBackoff {
    /// Calculates the backoff delay duration for the specified attempt.
    ///
    /// # Arguments
    ///
    /// * `attempt` - The attempt number.
    ///
    /// # Returns
    ///
    /// The backoff delay duration for the specified attempt.
    ///
    /// # Errors
    ///
    /// Returns an error if the backoff delay calculation fails.
    fn backoff_delay(
        &self,
        attempt: u32,
    ) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>> {
        let attempt = cmp::min(attempt, self.attempt_ceiling);
        let delay_duration = cmp::min(
            Duration::from_secs_f64(self.base_delay.as_secs_f64().mul(1u128.shl(attempt) as f64)),
            self.max_backoff,
        );

        Ok(Duration::from_secs_f64(
            rand::thread_rng().gen::<f64>() * delay_duration.as_secs_f64(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retry::backoff::RETRIES_ATTEMPTED_CEILING;

    #[test]
    fn test_full_jitter_backoff() {
        let base_delay = Duration::from_secs(1);
        let max_delay = Duration::from_secs(20);
        assert!(base_delay <= max_delay);

        let backoff = FullJitterBackoff::new(base_delay, max_delay);
        for i in 0..RETRIES_ATTEMPTED_CEILING * 2 {
            let delay = backoff.backoff_delay(i).unwrap();
            assert!(delay > Duration::ZERO);
            assert!(delay <= max_delay + Duration::from_secs(1));
        }
    }
}

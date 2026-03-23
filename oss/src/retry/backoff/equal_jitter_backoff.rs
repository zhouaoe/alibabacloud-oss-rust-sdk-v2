use std::cmp;
use std::ops::{Mul, Shl};
use std::time::Duration;

use rand::Rng;

use crate::retry::traits::BackoffDelayer;

/// Represents an equal jitter backoff strategy.
///
/// The equal jitter backoff strategy introduces randomness to the backoff delay
/// in order to prevent synchronization between concurrent retries. It
/// calculates the backoff delay using the base delay and the maximum backoff
/// value provided.
///
/// # Examples
///
/// ```
/// # use std::time::Duration;
/// #
/// # use alibabacloud_oss_sdk_rust_v2::retry::backoff::EqualJitterBackoff;
/// #
/// let base_delay = Duration::from_secs(1);
/// let max_backoff = Duration::from_secs(10);
///
/// let backoff = EqualJitterBackoff::new(base_delay, max_backoff);
/// ```
pub struct EqualJitterBackoff {
    /// The base delay used for the equal jitter backoff strategy.
    /// This is the initial delay before applying jitter.
    /// The actual delay will be a random value between `0` and `2 *
    /// base_delay`.
    base_delay: Duration,
    /// The maximum duration for backoff.
    max_backoff: Duration,
    /// The maximum number of attempts to make before giving up.
    attempt_ceiling: u32,
}

impl EqualJitterBackoff {
    /// Creates a new instance of `EqualJitterBackoff`.
    ///
    /// # Arguments
    ///
    /// * `base_delay` - The base delay for backoff.
    /// * `max_backoff` - The maximum duration for backoff.
    ///
    /// # Returns
    ///
    /// A new instance of `EqualJitterBackoff`.
    pub fn new(base_delay: Duration, max_backoff: Duration) -> Self {
        Self {
            base_delay,
            max_backoff,
            attempt_ceiling: (u64::MAX - base_delay.as_secs()).ilog2(),
        }
    }
}

impl BackoffDelayer for EqualJitterBackoff {
    /// Calculates the backoff delay for the given attempt.
    ///
    /// The backoff delay is calculated using the base delay and the maximum
    /// backoff value. It introduces randomness to prevent synchronization
    /// between concurrent retries.
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
        attempt: u32,
    ) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>> {
        let attempt = cmp::min(attempt, self.attempt_ceiling);
        let delay_duration = cmp::min(
            Duration::from_secs_f64(self.base_delay.as_secs_f64().mul(1u128.shl(attempt) as f64)),
            self.max_backoff,
        );

        let half: f64 = delay_duration.as_secs_f64() / 2.0;
        Ok(Duration::from_secs_f64(
            half + rand::thread_rng().gen::<f64>() * (half + 1.0),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retry::backoff::RETRIES_ATTEMPTED_CEILING;

    #[test]
    fn test_equal_jitter_backoff() {
        let base_delay = Duration::from_secs(1);
        let max_delay = Duration::from_secs(20);
        assert!(base_delay <= max_delay);

        let backoff = EqualJitterBackoff::new(base_delay, max_delay);
        for i in 0..RETRIES_ATTEMPTED_CEILING * 2 {
            let delay = backoff.backoff_delay(i).unwrap();
            assert!(delay > Duration::ZERO);
            assert!(delay <= max_delay + Duration::from_secs(1));
        }
    }
}

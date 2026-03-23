mod equal_jitter_backoff;
mod fixed_jitter_backoff;
mod full_jitter_backoff;

pub use self::equal_jitter_backoff::EqualJitterBackoff;
pub use self::fixed_jitter_backoff::FixedDelayBackoff;
pub use self::full_jitter_backoff::FullJitterBackoff;

#[cfg(test)]
const RETRIES_ATTEMPTED_CEILING: u32 = 64;

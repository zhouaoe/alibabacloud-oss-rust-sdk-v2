use std::sync::Arc;

use tokio::sync::Semaphore;

#[allow(unused)]
pub(crate) const BW_TOKEN_BUCKET_SLOT_RX: usize = 0;
#[allow(unused)]
pub(crate) const BW_TOKEN_BUCKET_SLOT_TX: usize = 1;
pub(crate) const BW_TOKEN_BUCKET_SLOTS: usize = 2;

pub type BwTokenBuckets = [Option<BwTokenBucket>; BW_TOKEN_BUCKET_SLOTS];

/// Represents a token bucket for rate limiting bandwidth.
#[derive(Clone)]
pub struct BwTokenBucket {
    #[allow(unused)]
    max_bandwidth: u64,
    limiter: Arc<Semaphore>,
}

impl BwTokenBucket {
    /// Creates a new `BwTokenBucket` with the specified bandwidth.
    ///
    /// # Arguments
    ///
    /// * `bandwidth` - The maximum bandwidth in bytes per second.
    ///
    /// # Returns
    ///
    /// A new `BwTokenBucket` instance.
    pub fn new(bandwidth: u64) -> Self {
        BwTokenBucket {
            max_bandwidth: bandwidth,
            limiter: Arc::new(Semaphore::new(new_empty_token_bucket(bandwidth))),
        }
    }

    /// Limits the bandwidth by acquiring the specified number of tokens.
    ///
    /// This method is asynchronous and will suspend the current task until the
    /// required number of tokens are acquired.
    ///
    /// # Arguments
    ///
    /// * `bandwidth` - The limited bandwidth in bytes per second.
    pub async fn limit_bandwidth(&self, bandwidth: u32) {
        let permit = self.limiter.acquire_many(bandwidth).await.unwrap();
        drop(permit);
    }
}

fn new_empty_token_bucket(bandwidth: u64) -> usize {
    const DEFAULT_MAX_BURST_SIZE: u64 = 4 * 1024 * 1024; // 4M
    let max_burst_size = (bandwidth * DEFAULT_MAX_BURST_SIZE) / (256 * 1024 * 1024);
    if max_burst_size < DEFAULT_MAX_BURST_SIZE {
        DEFAULT_MAX_BURST_SIZE as usize
    } else {
        max_burst_size as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_limit_bandwidth() {
        let bucket = BwTokenBucket::new(1024 * 1024); // 1 MB/s bandwidth

        // Simulating bandwidth-limited data transfer
        tokio::spawn(async move {
            bucket.limit_bandwidth(512 * 1024).await; // Limit to 512 KB
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_new_empty_token_bucket() {
        let bandwidth = 1024 * 1024; // 1 MB/s bandwidth
        let expected_max_burst_size = 4 * 1024 * 1024; // 4M

        let result = new_empty_token_bucket(bandwidth);

        assert_eq!(result, expected_max_burst_size as usize);
    }
}

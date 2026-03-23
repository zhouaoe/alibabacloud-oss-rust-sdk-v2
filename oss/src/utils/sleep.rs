#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
use std::time::Duration;

// #[coverage(off)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub(crate) async fn sleep_with_context(dur: Duration) {
    tokio::time::sleep(dur).await;
}

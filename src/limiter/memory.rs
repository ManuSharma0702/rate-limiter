use std::collections::HashMap;

use async_trait::async_trait;
use tokio::time::Instant;

use crate::limiter::types::RateLimiter;

#[derive(Debug)]
pub struct Bucket{
    tokens: u64,
    capacity: u64,
    last_refill: Instant
}

pub struct InMemoryTokenBucket {
    pub buckets: HashMap<String, Bucket>,
}

#[async_trait]
impl RateLimiter for InMemoryTokenBucket {
    async fn allow(&mut self, key: String) -> bool {
        let bucket = self.buckets.entry(key).or_insert(Bucket {
            tokens: 10,
            capacity: 10,
            last_refill: Instant::now(),
        });
        let now = Instant::now();
        let time_elapsed = now.duration_since(bucket.last_refill);

        bucket.tokens = (bucket.tokens + time_elapsed.as_secs()).min(bucket.capacity);
        bucket.last_refill = now;

        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            true
        } else {
            false
        }
    }
}

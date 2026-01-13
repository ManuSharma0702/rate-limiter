use std::collections::HashMap;

use async_trait::async_trait;

use crate::limiter::{rate_limiter::RateLimiter, types::TokenBucket};

//Map of IP to token count
pub struct InMemoryTokenBucket {
    pub buckets: HashMap<String, TokenBucket>,
}

#[async_trait]
impl RateLimiter for InMemoryTokenBucket {
    async fn allow(&self, key: String) -> bool {
        false
    }
}

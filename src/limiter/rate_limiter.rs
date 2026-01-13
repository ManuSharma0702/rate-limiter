use async_trait::async_trait;

#[async_trait]
pub trait RateLimiter {
    async fn allow(&self, key: String) -> bool;
}

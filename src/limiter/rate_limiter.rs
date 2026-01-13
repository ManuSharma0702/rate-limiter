use async_trait::async_trait;

#[async_trait]
pub trait RateLimiter: Send + Sync + 'static {
    async fn allow();
}

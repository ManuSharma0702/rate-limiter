use tokio::{sync::oneshot::Sender, time::Instant};
use async_trait::async_trait;

#[derive(Debug)]
pub struct Bucket{
    pub tokens: u64,
    pub capacity: u64,
    pub last_refill: Instant
}

#[async_trait]
pub trait RateLimiter {
    async fn allow(&mut self, key: String) -> bool;
}

#[derive(Debug)]
pub enum LimiterMsg {
    Hit {
        key: String,
        resp: Sender<Request>
    }
}

#[derive(Debug)]
pub enum Request {
    Allow,
    Reject
}


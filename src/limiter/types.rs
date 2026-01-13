use tokio::sync::oneshot::Sender;
use async_trait::async_trait;

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


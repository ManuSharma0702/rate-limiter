use async_trait::async_trait;
use crate::limiter::types::RateLimiter;
use redis::aio::MultiplexedConnection;

pub struct RedisTokenBucket {
    pub conn: MultiplexedConnection,
    pub script: redis::Script
}

#[async_trait]
impl RateLimiter for RedisTokenBucket {
    async fn allow(&mut self, key: String) -> bool {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let result: i32 = self.script
            .key(format!("rate:{}", key))
            .arg(10)      // capacity
            .arg(1)       // refill tokens/sec
            .arg(now_ms)
            .invoke_async(&mut self.conn)
            .await
            .unwrap_or(0);

        result == 1
    }
}


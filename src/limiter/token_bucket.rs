use std::collections::HashMap;

use axum::{
    extract::{Request as AxumRequest, State}, http::StatusCode, middleware::Next, response::{Response, IntoResponse}
};
use tokio::sync::{mpsc::{self, Receiver}, oneshot};

use crate::limiter::{memory::InMemoryTokenBucket, rate_limiter::RateLimiter, types::{LimiterMsg, Request}};

pub async fn rate_limit_middleware(
    State(tx_limiter): State<mpsc::Sender<LimiterMsg>>,
    req: AxumRequest,
    next: Next,
) -> Response {
    let key = "a".to_string();

    let (tx_middleware, rx_middleware) = oneshot::channel();

    let cmd = LimiterMsg::Hit {
        key,
        resp: tx_middleware,
    };

    if tx_limiter.send(cmd).await.is_err() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    // Wait for decision
    match rx_middleware.await {
        Ok(Request::Allow) => {
            dbg!("Allowed");
            next.run(req).await
        }
        Ok(Request::Reject) => {
            dbg!("REJECTED");
            StatusCode::FORBIDDEN.into_response()
        }
        Err(_) => {
            dbg!("SERVER DOWN");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}


pub async fn token_bucket_task(mut rx_limiter: Receiver<LimiterMsg>) {
    
    let token_bucket = InMemoryTokenBucket {
        buckets: HashMap::new()
    };

    while let Some(val) = rx_limiter.recv().await {
        match val {
            LimiterMsg::Hit { key, resp } => {
                let result = token_bucket.allow(key.to_string()).await;
                if result {
                    let _ = resp.send(Request::Allow);
                } else {
                    let _ = resp.send(Request::Reject);
                }
            }
        }
    }
}

use std::{collections::HashMap, net::SocketAddr};

use axum::{
    extract::{ConnectInfo, Request as AxumRequest, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Response}
};
use tokio::sync::{mpsc::{self, Receiver}, oneshot};

use crate::limiter::{memory::InMemoryTokenBucket, types::{LimiterMsg, Request, RateLimiter}};

pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(tx_limiter): State<mpsc::Sender<LimiterMsg>>,
    req: AxumRequest,
    next: Next,
) -> Response {

    let api_key = req
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("anon");

    let key = format!("{}:{}", addr.ip(), api_key);

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
            next.run(req).await
        }
        Ok(Request::Reject) => {
            StatusCode::TOO_MANY_REQUESTS.into_response()
        }
        Err(_) => {
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}


pub async fn token_bucket_task(mut rx_limiter: Receiver<LimiterMsg>, mut limiter: Box<dyn RateLimiter + Send>) {

    while let Some(val) = rx_limiter.recv().await {
        match val {
            LimiterMsg::Hit { key, resp } => {
                let result = limiter.allow(key.to_string()).await;
                if result {
                    let _ = resp.send(Request::Allow);
                } else {
                    let _ = resp.send(Request::Reject);
                }
            }
        }
    }
}




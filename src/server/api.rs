use std::net::SocketAddr;

use axum::{Router, middleware::from_fn_with_state, routing::get};
use tokio::sync::mpsc;

use crate::limiter::token_bucket::{rate_limit_middleware, token_bucket_task};

pub async fn run() {

    let (tx_limiter, rx_limiter) = mpsc::channel(1024);

    tokio::spawn(token_bucket_task(rx_limiter));

    let app = Router::new().
        route("/limited", get(handle_limited))
        .route_layer(from_fn_with_state(tx_limiter.clone(), rate_limit_middleware))
        .route("/unlimited", get(handle_unlimited));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn handle_limited() -> &'static str {
    "Limited, Dont over use me"
}
async fn handle_unlimited() -> &'static str {
    "Unlimited, Let's go"
}

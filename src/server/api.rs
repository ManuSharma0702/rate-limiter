use axum::{Router, middleware, routing::get};

use crate::limiter::token_bucket::rate_limit_middleware;

pub async fn run() {
    let app = Router::new().
        route("/limited", get(handle_limited))
        .route("/unlimited", get(handle_unlimited))
        .layer(middleware::from_fn(rate_limit_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handle_limited() -> &'static str {
    "Limited, Dont over use me"
}
async fn handle_unlimited() -> &'static str {
    "Unlimited, Let's go"
}

use std::{collections::HashMap, net::SocketAddr};

use axum::{Router, middleware::from_fn_with_state, routing::get};
use tokio::sync::mpsc;

use crate::limiter::{memory::InMemoryTokenBucket, redis_store::RedisTokenBucket, token_bucket::{rate_limit_middleware, token_bucket_task}, types::RateLimiter};


pub const TOKEN_BUCKET_LUA: &str = r#"
local key = KEYS[1]
local capacity = tonumber(ARGV[1])
local refill_rate = tonumber(ARGV[2])
local now = tonumber(ARGV[3])

local data = redis.call("HMGET", key, "tokens", "last_refill")
local tokens = tonumber(data[1])
local last_refill = tonumber(data[2])

if tokens == nil then
    tokens = capacity
    last_refill = now
end

local elapsed = math.max(0, now - last_refill) / 1000
local refill = elapsed * refill_rate
tokens = math.min(capacity, tokens + refill)

local allowed = 0
if tokens >= 1 then
    tokens = tokens - 1
    allowed = 1
end

redis.call("HMSET", key, "tokens", tokens, "last_refill", now)
redis.call("EXPIRE", key, 120)

return allowed
"#;

pub async fn run() {

    let (tx_limiter, rx_limiter) = mpsc::channel(1024);

    let use_redis = std::env::var("USE_REDIS")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let conn = redis_client.get_multiplexed_async_connection().await.unwrap();
    let script = redis::Script::new(TOKEN_BUCKET_LUA);

    let limiter: Box<dyn RateLimiter + Send> = if use_redis {
        Box::new(RedisTokenBucket{
            conn,
            script
        })
        } else{
        Box::new(InMemoryTokenBucket
            {
                buckets: HashMap::new()
            })
        };


    tokio::spawn(token_bucket_task(rx_limiter, limiter));

    let app = Router::new().
        route("/limited", get(handle_limited))
        .route_layer(from_fn_with_state(tx_limiter.clone(), rate_limit_middleware))
        .route("/unlimited", get(handle_unlimited));

    let port = std::env::var("PORT").unwrap_or("8080".into());
    let addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    println!("Server listening on port: {:?}", port);
    println!("Redis connected {:?}", &redis_client);
    println!("Use redis: {:?}", use_redis);
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn handle_limited() -> &'static str {
    "Limited, Dont over use me"
}
async fn handle_unlimited() -> &'static str {
    "Unlimited, Let's go"
}

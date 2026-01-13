use rate_limiter::server::api::run;

#[tokio::main]
async fn main() {
    run().await;
}



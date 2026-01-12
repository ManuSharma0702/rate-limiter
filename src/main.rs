mod server;

#[tokio::main]
async fn main() {
    server::api::run().await;
}



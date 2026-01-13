use axum::{
    middleware::Next,
    response::Response,
    extract::Request as AxumRequest,
};

pub async fn rate_limit_middleware(req: AxumRequest, next: Next) -> Response {
    next.run(req).await
}

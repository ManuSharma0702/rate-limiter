use tokio::sync::oneshot::Sender;

pub struct TokenBucket{
    count: i32
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


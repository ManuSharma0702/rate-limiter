use std::collections::HashMap;

use crate::limiter::rate_limiter::RateLimiter;

pub struct InMemoryTokenBucket {
    buckets: HashMap<String, String>
}

impl RateLimiter for InMemoryTokenBucket {
    fn allow<'async_trait>() ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send+'async_trait> > {
        
        !unimplemented!()
    }
}

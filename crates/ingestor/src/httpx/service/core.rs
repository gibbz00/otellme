use std::marker::PhantomData;

use axum::Router;

use crate::*;

pub struct HttpService<M: SignalMessage> {
    marker: PhantomData<M>,
}

impl<M: SignalMessage> HttpService<M> {
    pub fn add_to_router(router: Router) -> Router
    where
        M: HttpSignalPath,
    {
        router.route(M::DEFAULT_HTTP_PATH, axum::routing::get(|| async { "temp" }))
    }
}

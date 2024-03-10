use std::marker::PhantomData;

use axum::{
    extract::Request,
    response::{IntoResponse, Response},
    Router,
};

use crate::*;

pub struct HttpService<M: SignalMessage> {
    marker: PhantomData<M>,
}

impl<M: SignalMessage> HttpService<M> {
    async fn handler(_request: Request) -> Response {
        "temp".into_response()
    }

    pub fn add_to_router(router: Router) -> Router
    where
        M: HttpSignalPath,
    {
        router.route(M::DEFAULT_HTTP_PATH, axum::routing::post(Self::handler))
    }
}

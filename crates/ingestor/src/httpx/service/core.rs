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
    async fn handler(request: Request) -> Response {
        let _content_type = match ContentType::try_from(request.headers()) {
            Ok(content_type) => content_type,
            Err(err) => return err.into_response(),
        };

        // TODO: branch on content-type header
        // return error if not matched
        "temp".into_response()
    }

    pub fn add_to_router(router: Router) -> Router
    where
        M: HttpSignalPath,
    {
        router.route(M::DEFAULT_HTTP_PATH, axum::routing::post(Self::handler))
    }
}

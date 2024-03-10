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

#[cfg(test)]
mod tests {
    use super::*;

    // Signal should not matter for theese tests.
    // Hence the use of only one variant.
    #[cfg(feature = "logs")]
    mod logs {

        use axum::body::Body;
        use http::{header::CONTENT_TYPE, HeaderValue};

        use super::*;

        #[tokio::test]
        async fn handler_requires_content_type() {
            let content_type_not_set_request = Request::new(Body::empty());

            let content_type_not_set_response_status = HttpService::<LogsMessage>::handler(content_type_not_set_request).await.status();

            assert!(content_type_not_set_response_status.is_client_error());

            let mut invalid_content_type_set_request = Request::new(Body::empty());
            invalid_content_type_set_request
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("gibberish"));

            let invalid_content_type_set_response_status =
                HttpService::<LogsMessage>::handler(invalid_content_type_set_request).await.status();

            assert!(invalid_content_type_set_response_status.is_client_error());

            assert_ne!(content_type_not_set_response_status, invalid_content_type_set_response_status);
        }

        #[tokio::test]
        async fn handler_accepts_json_content_type() {
            assert_content_type_acceptance(ContentType::Json).await;
        }

        #[tokio::test]
        async fn handler_accepts_protobuf_content_type() {
            assert_content_type_acceptance(ContentType::Protobuf).await;
        }

        async fn assert_content_type_acceptance(content_type: ContentType) {
            let mut request = Request::new(Body::empty());
            content_type.add_to_headers(request.headers_mut());

            let response = HttpService::<LogsMessage>::handler(request).await;

            assert!(response.status().is_success());
        }
    }
}

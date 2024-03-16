use std::marker::PhantomData;

use axum::{
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
    Router,
};
use bytes::Bytes;
use http::StatusCode;

use crate::*;

pub struct HttpService<M: SignalMessage> {
    marker: PhantomData<M>,
}

impl<M: SignalMessage> HttpService<M> {
    async fn handler(request: Request) -> Response {
        let content_type = match ContentType::try_from(request.headers()) {
            Ok(content_type) => content_type,
            Err(err) => return err.into_response(),
        };

        // TODO: consider  configuring the request body limit to anything higher than 2MB
        // https://docs.rs/axum/latest/axum/extract/index.html#request-body-limits
        let payload = match Bytes::from_request(request, &()).await {
            Ok(bytes) => bytes,
            Err(err) => return err.into_response(),
        };

        let signal_request = match payload.deserialize_by_content_type::<M::Request>(content_type) {
            Ok(signal_request) => signal_request,
            // FIXME: correct error according to spec
            Err(_err) => return StatusCode::BAD_REQUEST.into_response(),
        };

        ingest_service::<M>(signal_request)
            .await
            .map(|response| {
                response
                    .serialize_by_content_type(content_type)
                    .expect("success response should be serializable")
            })
            // TEMP: add correct error handling
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            .into_response()
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
        async fn handler_accepts_json_format() {
            assert_format_acceptance(ContentType::Json).await;
        }

        #[tokio::test]
        async fn handler_accepts_protobuf_format() {
            assert_format_acceptance(ContentType::Protobuf).await;
        }

        async fn assert_format_acceptance(content_type: ContentType) {
            let mut request = Request::new(Body::from(
                <LogsMessage as SignalMessage>::Request::empty()
                    .serialize_by_content_type(content_type)
                    .unwrap(),
            ));
            content_type.add_to_headers(request.headers_mut());

            let response = HttpService::<LogsMessage>::handler(request).await;

            assert!(response.status().is_success());
        }
    }
}

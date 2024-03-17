use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestorServiceError {
    #[error("test1")]
    Error1(String),
    #[error("test2")]
    Error2(String),
}

impl From<&IngestorServiceError> for tonic::Code {
    fn from(error: &IngestorServiceError) -> Self {
        todo!()
    }
}

impl From<IngestorServiceError> for tonic_types::ErrorDetails {
    fn from(error: IngestorServiceError) -> Self {
        todo!()
    }
}

#[cfg(feature = "http")]
mod httpx {
    use axum::response::IntoResponse;
    use bytes::Bytes;
    use tonic_types::{CodeExt, Status as GrpcStatus};

    use crate::*;

    pub struct HttpIngestorServiceError {
        error: IngestorServiceError,
        content_type: ContentType,
    }

    impl HttpIngestorServiceError {
        pub fn new(error: IngestorServiceError, content_type: ContentType) -> Self {
            Self { error, content_type }
        }
    }

    // The response body for all HTTP 4xx and HTTP 5xx responses
    // MUST be a Protobuf-encoded Status message that describes the problem.
    // UNTESTED:
    impl IntoResponse for HttpIngestorServiceError {
        fn into_response(self) -> axum::response::Response {
            let code = tonic::Code::from(&self.error);

            let grpc_status = tonic_types::Status {
                code: code.into(),
                message: self.error.to_string(),
                details: tonic_types::ErrorDetails::from(self.error).into(),
            };

            let http_status = code.http_status();
        }
    }
}

use axum::response::{IntoResponse, Response};
use http::{header::ToStrError, StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContentTypeError {
    #[error("content-type not present")]
    HeaderNotFound,
    #[error("invalid content-type header")]
    InvalidHeaderString(#[from] ToStrError),
    #[error("unsupported media type")]
    UnsupportedMediaType,
}

impl From<&ContentTypeError> for StatusCode {
    fn from(encoding_error: &ContentTypeError) -> Self {
        match encoding_error {
            ContentTypeError::HeaderNotFound | ContentTypeError::InvalidHeaderString(_) => StatusCode::BAD_REQUEST,
            ContentTypeError::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        }
    }
}

// NOTE: Can't add gRPC Status in body per spec because
// the encoding to respond with is unknown.
impl IntoResponse for ContentTypeError {
    fn into_response(self) -> Response {
        StatusCode::from(&self).into_response()
    }
}

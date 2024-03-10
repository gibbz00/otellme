use http::{header::CONTENT_TYPE, HeaderMap};
use strum::{AsRefStr, EnumString};

use crate::*;

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
pub enum ContentType {
    #[strum(serialize = "application/json")]
    Json,
    #[strum(serialize = "application/x-protobuf")]
    Protobuf,
}

impl TryFrom<&HeaderMap> for ContentType {
    type Error = ContentTypeError;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        headers
            .get(CONTENT_TYPE)
            .ok_or(ContentTypeError::HeaderNotFound)?
            .to_str()?
            .parse()
            .map_err(|_| ContentTypeError::UnsupportedMediaType)
    }
}

#[cfg(test)]
mod tests {
    use http::{HeaderName, HeaderValue};

    use super::*;

    #[test]
    fn extracts_json_content_type_from_headers() {
        let headers = HeaderMap::from_iter([(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        )]);

        assert_eq!(ContentType::Json, (&headers).try_into().unwrap())
    }

    #[test]
    fn extracts_protobuf_content_type_from_headers() {
        let headers = HeaderMap::from_iter([(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/x-protobuf"),
        )]);

        assert_eq!(ContentType::Protobuf, (&headers).try_into().unwrap())
    }
}

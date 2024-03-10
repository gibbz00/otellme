use http::{header::CONTENT_TYPE, HeaderMap, HeaderValue};
use strum::{EnumString, IntoStaticStr};

use crate::*;

#[derive(Debug, PartialEq, Clone, Copy, IntoStaticStr, EnumString)]
pub enum ContentType {
    #[strum(serialize = "application/json")]
    Json,
    #[strum(serialize = "application/x-protobuf")]
    Protobuf,
}

impl ContentType {
    pub fn add_to_headers(self, headers: &mut HeaderMap) {
        headers.insert(CONTENT_TYPE, HeaderValue::from_static(self.into()));
    }
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
    use http::HeaderName;

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

    #[test]
    fn updates_json_content_type_headers() {
        assert_insertion(ContentType::Json)
    }

    #[test]
    fn updates_protobuf_content_type_headers() {
        assert_insertion(ContentType::Protobuf)
    }

    fn assert_insertion(content_type: ContentType) {
        let mut headers = HeaderMap::new();
        content_type.add_to_headers(&mut headers);
        assert_eq!(content_type, (&headers).try_into().unwrap())
    }
}

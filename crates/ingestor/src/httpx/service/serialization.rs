pub(crate) use deserialize::DeserializePayload;
mod deserialize {
    use bytes::Bytes;
    use serde::de::DeserializeOwned;

    use crate::*;

    pub trait DeserializePayload {
        fn deserialize_by_content_type<T: prost::Message + Default + DeserializeOwned>(
            self,
            content_type: ContentType,
        ) -> anyhow::Result<T>;
    }

    impl DeserializePayload for Bytes {
        fn deserialize_by_content_type<T: prost::Message + Default + DeserializeOwned>(
            self,
            content_type: ContentType,
        ) -> anyhow::Result<T> {
            Ok(match content_type {
                ContentType::Json => serde_json::from_slice(&self)?,
                ContentType::Protobuf => T::decode(self)?,
            })
        }
    }
}

pub(crate) use serialize::SerializePayload;
mod serialize {
    use bytes::Bytes;
    use serde::Serialize;

    use crate::*;

    pub trait SerializePayload: Sized + prost::Message + Serialize {
        fn serialize_by_content_type(&self, content_type: ContentType) -> anyhow::Result<Bytes> {
            Ok(match content_type {
                ContentType::Json => serde_json::to_vec(self)?,
                ContentType::Protobuf => self.encode_to_vec(),
            }
            .into())
        }
    }

    impl<T: Sized + prost::Message + Serialize> SerializePayload for T {}
}

#[cfg(test)]
mod tests {
    use prost::Message;
    use serde::{Deserialize, Serialize};

    use crate::*;

    #[derive(PartialEq, Message, Serialize, Deserialize)]
    struct Mock {
        #[prost(int32, tag = "1")]
        value: i32,
    }

    #[test]
    fn json_serialization_is_bijective() {
        assert_serialization_bjiectivity(ContentType::Json)
    }

    #[test]
    fn protobuf_serialization_is_bijective() {
        assert_serialization_bjiectivity(ContentType::Protobuf)
    }

    fn assert_serialization_bjiectivity(content_type: ContentType) {
        let mock = Mock::default();
        let serialized = mock.serialize_by_content_type(content_type).unwrap();
        let deserialized = serialized.deserialize_by_content_type(content_type).unwrap();

        assert_eq!(mock, deserialized)
    }
}

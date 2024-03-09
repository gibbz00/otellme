//! Otellme Ingestor crate.

mod ingestor;
pub use ingestor::OtlpIngestor;

#[cfg(feature = "grpc")]
mod grpc;
pub(crate) use grpc::GrpcSignalServer;

mod message_generics;
pub(crate) use message_generics::*;

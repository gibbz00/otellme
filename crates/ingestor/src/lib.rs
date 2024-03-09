//! Otellme Ingestor library crate.

mod ingestor;
pub use ingestor::OtlpIngestor;
pub(crate) use ingestor::*;

mod server;
pub(crate) use server::OtlpServer;

mod message_generics;
pub(crate) use message_generics::*;

#[cfg(feature = "grpc")]
mod grpc;
pub(crate) use grpc::*;

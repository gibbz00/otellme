//! Otellme Ingestor library crate.

mod message_generics;
pub(crate) use message_generics::*;

mod ingestor;
pub use ingestor::OtlpIngestor;
pub(crate) use ingestor::*;

mod server;
pub(crate) use server::OtlpServer;

#[cfg(feature = "grpc")]
mod grpc;
#[cfg(feature = "grpc")]
pub(crate) use grpc::*;

#[cfg(feature = "http")]
mod httpx;
#[cfg(feature = "http")]
pub(crate) use httpx::*;

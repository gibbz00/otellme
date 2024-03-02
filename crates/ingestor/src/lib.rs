//! Otellme Ingestor crate.

mod services;
pub(crate) use services::*;

mod server;
pub use server::OtlpIngestor;

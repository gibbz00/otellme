use std::net::SocketAddr;

use tonic::transport::{Error as TonicError, Server as TonicServer};

use crate::*;

/// Server implementation of the [OTLP Specification 1.0](https://opentelemetry.io/docs/specs/otlp/).
/// Recieves OTLP gRPC calls and places the contained singals into the configured storage.
pub struct OtlpIngestor;

impl OtlpIngestor {
    /// Beging building a new ingestor up for configuration before finally calling [`Self::serve`]
    pub fn builder() -> Self {
        Self
    }

    /// Consume the ingestor and beging serving.
    pub async fn serve(self, addr: impl Into<SocketAddr>) -> Result<(), TonicError> {
        let mut server = TonicServer::builder();

        #[cfg(feature = "logs")]
        let server = server.add_service(GrpcSignalServer::<LogsMessage>::new());
        #[cfg(feature = "metrics")]
        let server = server.add_service(GrpcSignalServer::<MetricsMessage>::new());
        #[cfg(feature = "traces")]
        let server = server.add_service(GrpcSignalServer::<TracesMessage>::new());

        server.serve(addr.into()).await
    }
}

use std::net::SocketAddr;

use futures_util::{
    future::{self, try_join_all},
    FutureExt,
};

use crate::*;

/// Server implementation of the [OTLP Specification 1.0](https://opentelemetry.io/docs/specs/otlp/).
/// Recieves OTLP calls and places the contained singals into the configured storage.
#[derive(Default)]
pub struct OtlpIngestor {
    #[cfg(feature = "grpc")]
    optional_grpc_server: Option<GrpcServer>,
}

impl OtlpIngestor {
    /// Configure the ingestor for OTLP/gRPC serving.
    #[cfg(feature = "grpc")]
    pub fn configure_grpc(mut self, addr: impl Into<SocketAddr>) -> Self {
        self.optional_grpc_server = Some(GrpcServer::configure(addr));
        self
    }

    // IMPROVEMENT: use typestate pattern to hide this method unless
    // the respective config server methods have been called.
    //
    /// Consume the ingestor and beging serving.
    /// This won't do much unless the `OtlpIngestor`
    /// has been configured with a OTLP/gRPC or an OTLP/HTTP server.
    pub async fn serve(self) -> anyhow::Result<()> {
        #[cfg(feature = "grpc")]
        let grpc_join = self
            .optional_grpc_server
            .map(|grpc_server| {
                async move {
                    tokio::spawn(grpc_server.serve())
                        .await
                        .map(|serve_result| serve_result.map_err(anyhow::Error::from))
                        .map_err(anyhow::Error::from)?
                }
                .left_future()
            })
            .unwrap_or(future::ok(()).right_future());

        try_join_all([
            #[cfg(feature = "grpc")]
            grpc_join,
        ])
        .await?;

        Ok(())
    }
}

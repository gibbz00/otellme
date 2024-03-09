pub(crate) use state::{IngestorWithServer, IngestorWithoutServer, OtlpIngestorState};
mod state {
    pub trait OtlpIngestorState: private::SealedOtlpIngestorState {}
    impl<S: private::SealedOtlpIngestorState> OtlpIngestorState for S {}

    pub struct IngestorWithServer;

    pub struct IngestorWithoutServer;

    mod private {
        use crate::*;

        pub trait SealedOtlpIngestorState {}

        impl SealedOtlpIngestorState for IngestorWithServer {}

        impl SealedOtlpIngestorState for IngestorWithoutServer {}
    }
}

pub use core::OtlpIngestor;
mod core {
    use std::marker::PhantomData;

    use futures_util::{future::try_join_all, FutureExt};

    use crate::*;

    /// Server implementation of the [OTLP Specification 1.0](https://opentelemetry.io/docs/specs/otlp/).
    /// Recieves OTLP calls and places the contained singals into the configured storage.
    pub struct OtlpIngestor<S: OtlpIngestorState> {
        #[cfg(feature = "grpc")]
        optional_grpc_server: Option<GrpcServer>,
        state_marker: PhantomData<S>,
    }

    impl OtlpIngestor<IngestorWithoutServer> {
        /// Begin constructing a new `OtlpIngestor`.
        ///
        /// Ingestor must be configured with an OTLP/gRPC or an OTLP/HTTP server in order to finally call [`OtlpIngestor::serve`].
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {
                optional_grpc_server: None,
                state_marker: PhantomData,
            }
        }
    }

    impl<S: OtlpIngestorState> OtlpIngestor<S> {
        /// Configure the ingestor for OTLP/gRPC serving.
        #[cfg(feature = "grpc")]
        pub fn configure_grpc(self, addr: impl Into<std::net::SocketAddr>) -> OtlpIngestor<IngestorWithServer> {
            OtlpIngestor {
                optional_grpc_server: Some(GrpcServer::configure(addr)),
                state_marker: PhantomData,
            }
        }
    }

    impl OtlpIngestor<IngestorWithServer> {
        /// Consume the ingestor and beging serving.
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
                .unwrap_or(futures_util::future::ok(()).right_future());

            try_join_all([
                #[cfg(feature = "grpc")]
                grpc_join,
            ])
            .await?;

            Ok(())
        }
    }
}

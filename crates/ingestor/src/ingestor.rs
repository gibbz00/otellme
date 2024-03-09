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
    use std::{marker::PhantomData, net::SocketAddr};

    use futures_util::{
        future::{try_join_all, BoxFuture, Either, Ready},
        FutureExt,
    };

    use crate::*;

    /// Server implementation of the [OTLP Specification 1.0](https://opentelemetry.io/docs/specs/otlp/).
    /// Recieves OTLP calls and places the contained singals into the configured storage.
    pub struct OtlpIngestor<S: OtlpIngestorState> {
        #[cfg(feature = "grpc")]
        optional_grpc_server: Option<GrpcServer>,
        #[cfg(feature = "http")]
        optional_http_server: Option<HttpServer>,
        state_marker: PhantomData<S>,
    }

    impl Default for OtlpIngestor<IngestorWithoutServer> {
        fn default() -> Self {
            Self {
                optional_grpc_server: None,
                optional_http_server: None,
                state_marker: PhantomData,
            }
        }
    }

    impl OtlpIngestor<IngestorWithoutServer> {
        /// Begin constructing a new `OtlpIngestor`.
        ///
        /// Ingestor must be configured with an OTLP/gRPC or an OTLP/HTTP server in order to finally call [`OtlpIngestor::serve`].
        /// Calling [`Self::new`] is functionally equivalent to calling [`Self::default`]
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<S: OtlpIngestorState> OtlpIngestor<S> {
        /// Configure the ingestor for OTLP/gRPC serving.
        #[cfg(feature = "grpc")]
        pub fn configure_grpc(self, socket_address: impl Into<SocketAddr>) -> OtlpIngestor<IngestorWithServer> {
            OtlpIngestor {
                optional_grpc_server: Some(GrpcServer::configure(socket_address)),
                #[cfg(feature = "http")]
                optional_http_server: self.optional_http_server,
                state_marker: PhantomData,
            }
        }

        /// Configure the ingestor for OTLP/HTTP serving.
        #[cfg(feature = "http")]
        pub fn configure_http(self, socket_address: impl Into<SocketAddr>) -> OtlpIngestor<IngestorWithServer> {
            OtlpIngestor {
                #[cfg(feature = "grpc")]
                optional_grpc_server: self.optional_grpc_server,
                optional_http_server: Some(HttpServer::configure(socket_address)),
                state_marker: PhantomData,
            }
        }
    }

    impl OtlpIngestor<IngestorWithServer> {
        /// Consume the ingestor and beging serving.
        pub async fn serve(self) -> anyhow::Result<()> {
            try_join_all([
                #[cfg(feature = "grpc")]
                spawn_optional_server::<GrpcServer>(self.optional_grpc_server),
                #[cfg(feature = "http")]
                spawn_optional_server::<HttpServer>(self.optional_http_server),
            ])
            .await?;

            return Ok(());

            fn spawn_optional_server<S: OtlpServer + Send + 'static>(
                optional_server: Option<S>,
            ) -> Either<BoxFuture<'static, anyhow::Result<()>>, Ready<anyhow::Result<()>>> {
                optional_server
                    .map(|server| async move { tokio::spawn(server.serve()).await? }.boxed().left_future())
                    .unwrap_or(futures_util::future::ok(()).right_future())
            }
        }
    }
}

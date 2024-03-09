mod core;
pub(crate) use core::GrpcSignalServer;

mod namings;

mod service;

pub(crate) use server::GrpcServer;
mod server {
    use std::net::SocketAddr;

    use tonic::transport::server::Router;

    use crate::*;

    pub struct GrpcServer {
        server: Router,
        socket_address: SocketAddr,
    }

    impl OtlpServer for GrpcServer {
        type ServeError = tonic::transport::Error;

        fn configure(socket_address: impl Into<SocketAddr>) -> Self {
            let mut server = tonic::transport::Server::builder();

            #[cfg(feature = "logs")]
            let server = server.add_service(GrpcSignalServer::<LogsMessage>::new());
            #[cfg(feature = "metrics")]
            let server = server.add_service(GrpcSignalServer::<MetricsMessage>::new());
            #[cfg(feature = "traces")]
            let server = server.add_service(GrpcSignalServer::<TracesMessage>::new());

            Self {
                server,
                socket_address: socket_address.into(),
            }
        }

        async fn serve(self) -> Result<(), Self::ServeError> {
            self.server.serve(self.socket_address).await
        }
    }
}

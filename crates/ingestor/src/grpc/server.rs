use std::net::SocketAddr;

use tonic::transport::server::Router;

use crate::*;

pub struct GrpcServer {
    server: Router,
    socket_address: SocketAddr,
}

impl OtlpServer for GrpcServer {
    fn configure(socket_address: impl Into<SocketAddr>) -> Self {
        let mut server = tonic::transport::Server::builder();

        #[cfg(feature = "logs")]
        let server = server.add_service(GrpcSignalService::<LogsMessage>::new());
        #[cfg(feature = "metrics")]
        let server = server.add_service(GrpcSignalService::<MetricsMessage>::new());
        #[cfg(feature = "traces")]
        let server = server.add_service(GrpcSignalService::<TracesMessage>::new());

        Self {
            server,
            socket_address: socket_address.into(),
        }
    }

    async fn serve(self) -> anyhow::Result<()> {
        self.server.serve(self.socket_address).await.map_err(Into::into)
    }
}

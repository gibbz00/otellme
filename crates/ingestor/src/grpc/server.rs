use std::net::SocketAddr;

use tonic::transport::server::Router;

use crate::*;

pub struct GrpcServer {
    router: Router,
    socket_address: SocketAddr,
}

impl OtlpServer for GrpcServer {
    fn configure(socket_address: impl Into<SocketAddr>) -> Self {
        let mut router = tonic::transport::Server::builder();

        #[cfg(feature = "logs")]
        let router = router.add_service(GrpcSignalService::<LogsMessage>::new());
        #[cfg(feature = "metrics")]
        let router = router.add_service(GrpcSignalService::<MetricsMessage>::new());
        #[cfg(feature = "traces")]
        let router = router.add_service(GrpcSignalService::<TracesMessage>::new());

        Self {
            router,
            socket_address: socket_address.into(),
        }
    }

    async fn serve(self) -> anyhow::Result<()> {
        self.router.serve(self.socket_address).await.map_err(Into::into)
    }
}

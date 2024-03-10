use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;

use crate::*;

pub struct HttpServer {
    socket_address: SocketAddr,
    router: Router,
}

impl OtlpServer for HttpServer {
    fn configure(socket_address: impl Into<std::net::SocketAddr>) -> Self {
        let router = Router::new();

        #[cfg(feature = "logs")]
        let router = HttpService::<LogsMessage>::add_to_router(router);
        #[cfg(feature = "metrics")]
        let router = HttpService::<MetricsMessage>::add_to_router(router);
        #[cfg(feature = "traces")]
        let router = HttpService::<TracesMessage>::add_to_router(router);

        Self {
            socket_address: socket_address.into(),
            router,
        }
    }

    async fn serve(self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.socket_address).await?;
        axum::serve(listener, self.router).await.map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddrV6;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower_service::Service;

    use super::*;

    fn mock_address() -> SocketAddrV6 {
        "[::1]:9000".parse().unwrap()
    }

    async fn assert_serves_signal_service<M: SignalMessage + HttpSignalPath>() {
        let mut router = HttpServer::configure(mock_address()).router;

        let response = router
            .call(Request::builder().uri(M::DEFAULT_HTTP_PATH).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[cfg(feature = "logs")]
    #[tokio::test]
    async fn serves_logs_service() {
        assert_serves_signal_service::<LogsMessage>().await
    }

    #[cfg(feature = "metrics")]
    #[tokio::test]
    async fn serves_metrics_service() {
        assert_serves_signal_service::<MetricsMessage>().await
    }

    #[cfg(feature = "traces")]
    #[tokio::test]
    async fn serves_traces_service() {
        assert_serves_signal_service::<TracesMessage>().await
    }
}

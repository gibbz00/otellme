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

    use axum::body::Body;
    use http::{Request, StatusCode};
    use tower_service::Service;

    use super::*;

    fn mock_address() -> SocketAddrV6 {
        "[::1]:9000".parse().unwrap()
    }

    async fn assert_serves_post_signal_endpoint<M: SignalMessage + HttpSignalPath>() {
        let mut router = HttpServer::configure(mock_address()).router;

        // From spec: Telemetry data is sent via HTTP POST request.
        let mut request = Request::post(M::DEFAULT_HTTP_PATH).body(Body::empty()).unwrap();
        ContentType::Json.add_to_headers(request.headers_mut());

        let response = router.call(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[cfg(feature = "logs")]
    #[tokio::test]
    async fn serves_post_logs_endpoint() {
        assert_serves_post_signal_endpoint::<LogsMessage>().await
    }

    #[cfg(feature = "metrics")]
    #[tokio::test]
    async fn serves_post_metrics_endpoint() {
        assert_serves_post_signal_endpoint::<MetricsMessage>().await
    }

    #[cfg(feature = "traces")]
    #[tokio::test]
    async fn serves_post_traces_endpoint() {
        assert_serves_post_signal_endpoint::<TracesMessage>().await
    }
}

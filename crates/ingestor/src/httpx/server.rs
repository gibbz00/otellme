use std::net::SocketAddr;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

use crate::*;

pub struct HttpServer {
    socket_address: SocketAddr,
    router: Router,
}

impl OtlpServer for HttpServer {
    fn configure(socket_address: impl Into<std::net::SocketAddr>) -> Self {
        let router = Router::new().route("/temp", get(|| async { "temp" }));

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

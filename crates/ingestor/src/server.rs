use std::{future::Future, net::SocketAddr};

pub trait OtlpServer {
    fn configure(socket_address: impl Into<SocketAddr>) -> Self;

    fn serve(self) -> impl Future<Output = anyhow::Result<()>> + Send + 'static;
}

use std::net::SocketAddr;

pub trait OtlpServer {
    type ServeError: Into<anyhow::Error>;

    fn configure(socket_address: impl Into<SocketAddr>) -> Self;

    async fn serve(self) -> Result<(), Self::ServeError>;
}

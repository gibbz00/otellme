//! Binary for starting the ingestor.

// TEMP: for experimentation, should probably be moved to a `ingestor-cli` or the like

use std::net::SocketAddrV6;

use otellme_ingestor::*;
use otellme_utils::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    InternalInstrumentor::init()?;

    OtlpIngestor::new()
        .configure_grpc("[::1]:4317".parse::<SocketAddrV6>()?)
        .serve()
        .await?;

    Ok(())
}

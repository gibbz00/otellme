//! Binary for starting the ingestor.

// TEMP: for experimentation, should probably be moved to a `ingestor-cli` or the like

use std::net::SocketAddrV6;

use otellme_ingestor::*;
use otellme_utils::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    InternalInstrumentor::init()?;

    // TODO: support port overrides by config, env, var cli flag, etc.
    let addr = "[::1]:4317".parse::<SocketAddrV6>()?;
    OtlpIngestor::builder().serve_grpc(addr).await?;

    Ok(())
}

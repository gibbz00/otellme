//! Binary for starting the ingestor.

use std::net::SocketAddrV6;

use otellme_ingestor::*;
use otellme_utils::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    InternalInstrumentor::init()?;

    // TODO: support port overrides
    let addr = "[::1]:4317".parse::<SocketAddrV6>()?;
    OtlpIngestor::builder().serve(addr).await?;

    Ok(())
}

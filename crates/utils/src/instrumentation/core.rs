use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::*;

/// Unifies instrumentation within Otellme binaries.
pub struct InternalInstrumentor;

impl InternalInstrumentor {
    /// Supposed to be called first thing inside the main function.
    pub fn init() -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(IntrumentationLayers::stdout())
            .with(Self::setup_targets_filter())
            .try_init()?;

        std::panic::set_hook(Box::new(tracing_panic::panic_hook));

        Ok(())
    }

    fn setup_targets_filter() -> tracing_subscriber::filter::Targets {
        let targets_filter = tracing_subscriber::filter::Targets::new();

        // Example:
        // #[cfg(feature = "aws-s3")]
        // let targets_filter = targets_filter.with_target("aws_smithy_runtime", Level::WARN);

        targets_filter.with_default(Level::INFO)
    }
}

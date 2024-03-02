use tracing::Subscriber;
use tracing_subscriber::{registry::LookupSpan, Layer};

pub struct IntrumentationLayers;

impl IntrumentationLayers {
    pub fn stdout<S>() -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        tracing_subscriber::fmt::Layer::default()
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .pretty()
    }
}

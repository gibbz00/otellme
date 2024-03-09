use crate::*;

pub trait HttpSignalPath {
    const DEFAULT_HTTP_PATH: &'static str;
}

#[cfg(feature = "logs")]
impl HttpSignalPath for LogsMessage {
    const DEFAULT_HTTP_PATH: &'static str = "/v1/logs";
}

#[cfg(feature = "metrics")]
impl HttpSignalPath for MetricsMessage {
    const DEFAULT_HTTP_PATH: &'static str = "/v1/metrics";
}

#[cfg(feature = "traces")]
impl HttpSignalPath for TracesMessage {
    const DEFAULT_HTTP_PATH: &'static str = "/v1/traces";
}

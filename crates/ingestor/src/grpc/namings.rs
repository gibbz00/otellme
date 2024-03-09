use tonic::server::NamedService;

use crate::*;

#[cfg(feature = "logs")]
impl NamedService for GrpcSignalServer<LogsMessage> {
    const NAME: &'static str = "opentelemetry.proto.collector.logs.v1.LogsService";
}

#[cfg(feature = "traces")]
impl NamedService for GrpcSignalServer<TracesMessage> {
    const NAME: &'static str = "opentelemetry.proto.collector.trace.v1.TraceService";
}

#[cfg(feature = "metrics")]
impl NamedService for GrpcSignalServer<MetricsMessage> {
    const NAME: &'static str = "opentelemetry.proto.collector.metrics.v1.MetricsService";
}

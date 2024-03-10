use tonic::server::NamedService;

use crate::*;

#[cfg(feature = "logs")]
impl NamedService for GrpcSignalService<LogsMessage> {
    const NAME: &'static str = "opentelemetry.proto.collector.logs.v1.LogsService";
}

#[cfg(feature = "traces")]
impl NamedService for GrpcSignalService<TracesMessage> {
    const NAME: &'static str = "opentelemetry.proto.collector.trace.v1.TraceService";
}

#[cfg(feature = "metrics")]
impl NamedService for GrpcSignalService<MetricsMessage> {
    const NAME: &'static str = "opentelemetry.proto.collector.metrics.v1.MetricsService";
}

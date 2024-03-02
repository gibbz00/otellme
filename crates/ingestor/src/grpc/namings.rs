use tonic::server::NamedService;

use crate::*;

impl NamedService for GrpcSignalServer<LogsMessage> {
    const NAME: &'static str = "opentelemetry.proto.collector.logs.v1.LogsService";
}

impl NamedService for GrpcSignalServer<TracesMessage> {
    const NAME: &'static str = "opentelemetry.proto.collector.trace.v1.TraceService";
}

impl NamedService for GrpcSignalServer<MetricsMessage> {
    const NAME: &'static str = "opentelemetry.proto.collector.metrics.v1.MetricsService";
}

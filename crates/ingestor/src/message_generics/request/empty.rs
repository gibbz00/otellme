use crate::*;

pub trait EmptyRequest: SealedRequest {
    fn empty() -> Self;
}

#[cfg(feature = "logs")]
mod logs {
    use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;

    impl super::EmptyRequest for ExportLogsServiceRequest {
        fn empty() -> Self {
            Self { resource_logs: vec![] }
        }
    }
}

#[cfg(feature = "metrics")]
mod metrics {
    use opentelemetry_proto::tonic::collector::metrics::v1::ExportMetricsServiceRequest;

    impl super::EmptyRequest for ExportMetricsServiceRequest {
        fn empty() -> Self {
            Self { resource_metrics: vec![] }
        }
    }
}

#[cfg(feature = "traces")]
mod traces {
    use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;

    impl super::EmptyRequest for ExportTraceServiceRequest {
        fn empty() -> Self {
            Self { resource_spans: vec![] }
        }
    }
}

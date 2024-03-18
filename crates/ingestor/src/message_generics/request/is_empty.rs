use crate::*;

pub trait IsEmptyRequest: SealedRequest {
    fn is_empty(&self) -> bool;
}

#[cfg(feature = "logs")]
mod logs {
    use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;

    impl super::IsEmptyRequest for ExportLogsServiceRequest {
        fn is_empty(&self) -> bool {
            self.resource_logs.is_empty()
        }
    }
}

#[cfg(feature = "metrics")]
mod metrics {
    use opentelemetry_proto::tonic::collector::metrics::v1::ExportMetricsServiceRequest;

    impl super::IsEmptyRequest for ExportMetricsServiceRequest {
        fn is_empty(&self) -> bool {
            self.resource_metrics.is_empty()
        }
    }
}

#[cfg(feature = "traces")]
mod traces {
    use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;

    impl super::IsEmptyRequest for ExportTraceServiceRequest {
        fn is_empty(&self) -> bool {
            self.resource_spans.is_empty()
        }
    }
}

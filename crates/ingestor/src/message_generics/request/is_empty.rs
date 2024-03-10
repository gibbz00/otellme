use crate::*;

pub trait IsEmtpyRequest: SealedRequest {
    fn is_empty(&self) -> bool;
}

#[cfg(feature = "logs")]
mod logs {
    use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;

    impl super::IsEmtpyRequest for ExportLogsServiceRequest {
        fn is_empty(&self) -> bool {
            self.resource_logs.is_empty()
        }
    }
}

#[cfg(feature = "metrics")]
mod metrics {
    use opentelemetry_proto::tonic::collector::metrics::v1::ExportMetricsServiceRequest;

    impl super::IsEmtpyRequest for ExportMetricsServiceRequest {
        fn is_empty(&self) -> bool {
            self.resource_metrics.is_empty()
        }
    }
}

#[cfg(feature = "traces")]
mod traces {
    use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;

    impl super::IsEmtpyRequest for ExportTraceServiceRequest {
        fn is_empty(&self) -> bool {
            self.resource_spans.is_empty()
        }
    }
}

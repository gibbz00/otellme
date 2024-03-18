pub(crate) use core::SignalMessage;
mod core {
    use crate::*;

    pub trait SignalMessage: Send + Sync + 'static {
        #[cfg(not(feature = "http"))]
        type Request: SealedRequest + IsEmptyRequest + prost::Message + Default + Send + Sync + 'static;
        #[cfg(feature = "http")]
        type Request: SealedRequest + IsEmptyRequest + prost::Message + Default + serde::de::DeserializeOwned + Send + Sync + 'static;

        #[cfg(not(feature = "http"))]
        type Response: SealedResponse
            + SuccessResponse
            + PartialSuccessResponse
            + WarningResponse
            + prost::Message
            + Default
            + Send
            + Sync
            + 'static;
        #[cfg(feature = "http")]
        type Response: SealedResponse
            + SuccessResponse
            + PartialSuccessResponse
            + WarningResponse
            + prost::Message
            + Default
            + Send
            + Sync
            + 'static
            + serde::Serialize;
    }
}

#[cfg(feature = "logs")]
pub(crate) use logs::LogsMessage;
#[cfg(feature = "logs")]
mod logs {
    use opentelemetry_proto::tonic::collector::logs::v1::{ExportLogsServiceRequest, ExportLogsServiceResponse};

    use crate::*;

    pub struct LogsMessage;

    impl SignalMessage for LogsMessage {
        type Request = ExportLogsServiceRequest;
        type Response = ExportLogsServiceResponse;
    }
}

#[cfg(feature = "metrics")]
pub(crate) use metrics::MetricsMessage;
#[cfg(feature = "metrics")]
mod metrics {
    use opentelemetry_proto::tonic::collector::metrics::v1::{ExportMetricsServiceRequest, ExportMetricsServiceResponse};

    use crate::*;

    pub struct MetricsMessage;

    impl SignalMessage for MetricsMessage {
        type Request = ExportMetricsServiceRequest;
        type Response = ExportMetricsServiceResponse;
    }
}

#[cfg(feature = "traces")]
pub(crate) use traces::TracesMessage;
#[cfg(feature = "traces")]
mod traces {
    use opentelemetry_proto::tonic::collector::trace::v1::{ExportTraceServiceRequest, ExportTraceServiceResponse};

    use crate::*;

    pub struct TracesMessage;

    impl SignalMessage for TracesMessage {
        type Request = ExportTraceServiceRequest;
        type Response = ExportTraceServiceResponse;
    }
}

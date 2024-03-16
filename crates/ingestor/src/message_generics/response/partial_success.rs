use std::num::NonZeroU32;

use crate::*;

// TEMP: ?
#[allow(dead_code)]
pub trait PartialSuccessResponse: SealedResponse {
    /// `rejected_count` signifies the number of spans, data points or log records rejected.
    /// `reason` SHOULD be populated with a human-readable error message in English.
    ///
    /// See [`WarningResponse::warning`] for sending success responses when `rejected_count` is zero.
    //
    // NOTE: spec says i64 but we limit ourselves to u32 as
    // a means of preventing non-sensical negative values.
    fn partial_success(rejected_count: NonZeroU32, reason: Option<String>) -> Self;
}

#[cfg(feature = "logs")]
mod logs {
    use std::num::NonZeroU32;

    use opentelemetry_proto::tonic::collector::logs::v1::{ExportLogsPartialSuccess, ExportLogsServiceResponse};

    use crate::*;

    impl PartialSuccessResponse for <LogsMessage as SignalMessage>::Response {
        fn partial_success(rejected_count: NonZeroU32, reason: Option<String>) -> Self {
            ExportLogsServiceResponse {
                partial_success: Some(ExportLogsPartialSuccess {
                    rejected_log_records: rejected_count.get().into(),
                    error_message: reason.unwrap_or_default(),
                }),
            }
        }
    }
}

#[cfg(feature = "metrics")]
mod metrics {
    use std::num::NonZeroU32;

    use opentelemetry_proto::tonic::collector::metrics::v1::{ExportMetricsPartialSuccess, ExportMetricsServiceResponse};

    use crate::*;

    impl PartialSuccessResponse for <MetricsMessage as SignalMessage>::Response {
        fn partial_success(rejected_count: NonZeroU32, reason: Option<String>) -> Self {
            ExportMetricsServiceResponse {
                partial_success: Some(ExportMetricsPartialSuccess {
                    rejected_data_points: rejected_count.get().into(),
                    error_message: reason.unwrap_or_default(),
                }),
            }
        }
    }
}

#[cfg(feature = "traces")]
mod traces {
    use std::num::NonZeroU32;

    use opentelemetry_proto::tonic::collector::trace::v1::{ExportTracePartialSuccess, ExportTraceServiceResponse};

    use crate::*;

    impl PartialSuccessResponse for <TracesMessage as SignalMessage>::Response {
        fn partial_success(rejected_count: NonZeroU32, reason: Option<String>) -> Self {
            ExportTraceServiceResponse {
                partial_success: Some(ExportTracePartialSuccess {
                    rejected_spans: rejected_count.get().into(),
                    error_message: reason.unwrap_or_default(),
                }),
            }
        }
    }
}

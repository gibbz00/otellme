use otellme_utils::*;

use crate::*;

// TEMP: ?
#[allow(dead_code)]
pub trait WarningResponse: SealedResponse {
    /// When server fully accepts a request but whishes to convey warnings/suggestions the clients.
    //
    // For implementors:
    // The rejected count field MUST have a value of 0
    // Error message field MUST be non-empty.
    //
    // <https://opentelemetry.io/docs/specs/otlp/#partial-success>
    fn warning(reason: NonEmptyString) -> Self;
}

#[cfg(test)]
mod mock {
    use otellme_utils::*;

    use crate::*;

    pub fn mock_warning<M: SignalMessage>() -> M::Response {
        M::Response::warning(NonEmptyString::new("warning!").unwrap())
    }
}

#[cfg(feature = "logs")]
mod logs {
    use opentelemetry_proto::tonic::collector::logs::v1::{ExportLogsPartialSuccess, ExportLogsServiceResponse};
    use otellme_utils::*;

    use crate::*;

    impl WarningResponse for <LogsMessage as SignalMessage>::Response {
        fn warning(reason: NonEmptyString) -> Self {
            ExportLogsServiceResponse {
                partial_success: Some(ExportLogsPartialSuccess {
                    rejected_log_records: 0,
                    error_message: reason.into(),
                }),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn rejected_count_is_zero() {
            assert_eq!(
                0,
                super::super::mock::mock_warning::<LogsMessage>()
                    .partial_success
                    .unwrap()
                    .rejected_log_records
            )
        }
    }
}

#[cfg(feature = "metrics")]
mod metrics {
    use opentelemetry_proto::tonic::collector::metrics::v1::{ExportMetricsPartialSuccess, ExportMetricsServiceResponse};
    use otellme_utils::*;

    use crate::*;

    impl WarningResponse for <MetricsMessage as SignalMessage>::Response {
        fn warning(reason: NonEmptyString) -> Self {
            ExportMetricsServiceResponse {
                partial_success: Some(ExportMetricsPartialSuccess {
                    rejected_data_points: 0,
                    error_message: reason.into(),
                }),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn rejected_count_is_zero() {
            assert_eq!(
                0,
                super::super::mock::mock_warning::<MetricsMessage>()
                    .partial_success
                    .unwrap()
                    .rejected_data_points
            )
        }
    }
}

#[cfg(feature = "traces")]
mod traces {
    use opentelemetry_proto::tonic::collector::trace::v1::{ExportTracePartialSuccess, ExportTraceServiceResponse};
    use otellme_utils::*;

    use crate::*;

    impl WarningResponse for <TracesMessage as SignalMessage>::Response {
        fn warning(reason: NonEmptyString) -> Self {
            ExportTraceServiceResponse {
                partial_success: Some(ExportTracePartialSuccess {
                    rejected_spans: 0,
                    error_message: reason.into(),
                }),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn rejected_count_is_zero() {
            assert_eq!(
                0,
                super::super::mock::mock_warning::<TracesMessage>()
                    .partial_success
                    .unwrap()
                    .rejected_spans
            )
        }
    }
}

use crate::*;

pub trait SuccessResponse: SealedResponse {
    /// The server MUST leave the partial_success field unset in case of a successful response.
    fn sucessful() -> Self;
}

#[cfg(feature = "logs")]
mod logs {
    use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceResponse;

    use crate::*;

    impl SuccessResponse for ExportLogsServiceResponse {
        fn sucessful() -> Self {
            Self { partial_success: None }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn successful_logs_response_has_unset_partial() {
            assert!(ExportLogsServiceResponse::sucessful().partial_success.is_none())
        }
    }
}

#[cfg(feature = "metrics")]
mod metrics {
    use opentelemetry_proto::tonic::collector::metrics::v1::ExportMetricsServiceResponse;

    use crate::*;

    impl SuccessResponse for ExportMetricsServiceResponse {
        fn sucessful() -> Self {
            Self { partial_success: None }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn successful_metric_response_has_unset_partial() {
            assert!(ExportMetricsServiceResponse::sucessful().partial_success.is_none())
        }
    }
}

#[cfg(feature = "traces")]
mod traces {
    use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceResponse;

    use crate::*;

    impl SuccessResponse for ExportTraceServiceResponse {
        fn sucessful() -> Self {
            Self { partial_success: None }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn successful_trace_response_has_unset_partial() {
            assert!(ExportTraceServiceResponse::sucessful().partial_success.is_none())
        }
    }
}

pub(crate) use request::SealedRequest;
mod request {
    pub trait SealedRequest {}

    #[cfg(feature = "logs")]
    impl SealedRequest for opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest {}
    #[cfg(feature = "traces")]
    impl SealedRequest for opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest {}
    #[cfg(feature = "metrics")]
    impl SealedRequest for opentelemetry_proto::tonic::collector::metrics::v1::ExportMetricsServiceRequest {}
}

pub(crate) use response::SealedResponse;
mod response {
    pub trait SealedResponse {}

    #[cfg(feature = "logs")]
    impl SealedResponse for opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceResponse {}
    #[cfg(feature = "traces")]
    impl SealedResponse for opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceResponse {}
    #[cfg(feature = "metrics")]
    impl SealedResponse for opentelemetry_proto::tonic::collector::metrics::v1::ExportMetricsServiceResponse {}
}

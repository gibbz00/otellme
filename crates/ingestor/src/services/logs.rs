use opentelemetry_proto::tonic::collector::logs::v1::{
    logs_service_server::{LogsService, LogsServiceServer},
    ExportLogsServiceRequest, ExportLogsServiceResponse,
};

use tonic::{Request as TonicRequest, Response as TonicResponse, Status as TonicStatus};

#[derive(Debug)]
pub struct LogsServiceImplementation;

impl LogsServiceImplementation {
    pub fn service() -> LogsServiceServer<Self> {
        LogsServiceServer::new(Self)
    }
}

#[async_trait::async_trait]
impl LogsService for LogsServiceImplementation {
    #[tracing::instrument]
    async fn export(
        &self,
        _request: TonicRequest<ExportLogsServiceRequest>,
    ) -> Result<TonicResponse<ExportLogsServiceResponse>, TonicStatus> {
        tracing::info!("recieved export request");

        Ok(TonicResponse::new(ExportLogsServiceResponse { partial_success: None }))
    }
}

use std::marker::PhantomData;

use tonic::{Request, Response, Status};

use crate::*;

#[derive(Debug)]
#[impl_tools::autoimpl(Clone)]
pub struct GrpcSignalService<M: SignalMessage>(PhantomData<M>);

impl<M: SignalMessage> GrpcSignalService<M> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    // TODO: Configure keep-alive: https://grpc.io/docs/guides/keepalive/
    // For performance reasons, it is recommended to keep this RPC
    // alive for the entire life of the application.
    //
    // possibly with tonic::Servrer.http2_keepalive_interval()
    //
    // https://github.com/open-telemetry/opentelemetry-proto/tree/main/opentelemetry/proto/collector
    pub async fn ingest(&self, request: Request<M::Request>) -> Result<Response<M::Response>, Status> {
        tracing::info!("recieved request!");

        // TODO: combine with HTTP service
        if request.get_ref().is_empty() {
            return Ok(Response::new(M::Response::sucessful()));
        }

        Ok(Response::new(M::Response::sucessful()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // IMPROVEMENT: somewhat rendant but the other signals can have the respective tests too.

    #[cfg(feature = "logs")]
    mod logs {

        use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;

        use super::*;

        // If the server receives an empty request
        // (a request that does not carry any telemetry data)
        // the server SHOULD respond with success.
        //
        // https://opentelemetry.io/docs/specs/otlp/#full-success
        #[tokio::test]
        async fn empty_request_returns_successful_response() {
            let empty_request = Request::new(ExportLogsServiceRequest { resource_logs: vec![] });
            let actual_response = GrpcSignalService::<LogsMessage>::new().ingest(empty_request).await.unwrap();
            assert_eq!(&<LogsMessage as SignalMessage>::Response::sucessful(), actual_response.get_ref());
        }
    }
}

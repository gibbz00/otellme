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
        tracing::info!("received request!");

        ingest_service::<M>(request.into_inner())
            .await
            .map(Response::new)
            .map_err(tonic_types::Status::from)
            .map_err(|status| status.into())
    }
}

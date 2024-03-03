use std::marker::PhantomData;

use tonic::{Request, Response, Status};

use crate::*;

#[derive(Debug)]
#[impl_tools::autoimpl(Clone)]
pub struct GrpcSignalServer<M: SignalMessage>(PhantomData<M>);

impl<M: SignalMessage> GrpcSignalServer<M> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    // TODO: Configure keep-alive: https://grpc.io/docs/guides/keepalive/
    // For performance reasons, it is recommended to keep this RPC
    // alive for the entire life of the application.
    //
    // https://github.com/open-telemetry/opentelemetry-proto/tree/main/opentelemetry/proto/collector
    pub async fn ingest(&self, request: Request<M::Request>) -> Result<Response<M::Response>, Status> {
        tracing::info!("recieved request!");

        // If the server receives an empty request
        // (a request that does not carry any telemetry data)
        // the server SHOULD respond with success.
        //
        // https://opentelemetry.io/docs/specs/otlp/#full-success
        if request.get_ref().is_empty() {
            return Ok(Response::new(M::Response::sucessful()));
        }

        Ok(Response::new(M::Response::sucessful()))
    }
}

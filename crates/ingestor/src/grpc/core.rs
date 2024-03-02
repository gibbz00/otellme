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

    // TODO: how?
    /// For performance reasons, it is recommended to keep this RPC
    /// alive for the entire life of the application.
    pub async fn ingest(&self, _request: Request<M::Request>) -> Result<Response<M::Response>, Status> {
        tracing::info!("recieved request!");

        Ok(Response::new(M::Response::sucessful()))
    }
}

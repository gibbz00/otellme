use std::convert::Infallible;

use once_cell::sync::Lazy;
use tonic::{
    body::BoxBody,
    codec::ProstCodec,
    codegen::*,
    server::{Grpc, NamedService, UnaryService},
    Code, Request, Response,
};

use crate::*;

// All server components MUST support the following transport compression options:
//
// https://opentelemetry.io/docs/specs/otlp/#protocol-details
static OTEL_GRPC_ENABLED_COMPRESSION_ENCODING: Lazy<EnabledCompressionEncodings> = Lazy::new(|| {
    let mut compression_encodings = EnabledCompressionEncodings::default();
    compression_encodings.enable(CompressionEncoding::Gzip);
    compression_encodings
});

impl<M: SignalMessage> UnaryService<M::Request> for GrpcSignalService<M> {
    type Response = M::Response;
    type Future = BoxFuture<Response<Self::Response>, tonic::Status>;

    fn call(&mut self, request: Request<M::Request>) -> Self::Future {
        Box::pin(async move { GrpcSignalService::<M>::new().ingest(request).await })
    }
}

impl<M: SignalMessage, B> tower_service::Service<http::Request<B>> for GrpcSignalService<M>
where
    B: Body + Send + 'static,
    B::Error: Into<StdError> + Send + 'static,
    Self: NamedService,
{
    type Response = http::Response<BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        if req.uri().path() == format!("/{}/Export", Self::NAME) {
            Box::pin(async move {
                Ok(Grpc::new(ProstCodec::default())
                    .apply_compression_config(*OTEL_GRPC_ENABLED_COMPRESSION_ENCODING, *OTEL_GRPC_ENABLED_COMPRESSION_ENCODING)
                    .unary(GrpcSignalService::<M>::new(), req)
                    .await)
            })
        } else {
            Box::pin(async move {
                Ok(http::Response::builder()
                    .status(http::StatusCode::OK)
                    .header("grpc-status", Code::Unimplemented as i32)
                    .header(http::header::CONTENT_TYPE, "application/grpc")
                    .body(empty_body())
                    .expect("invalid http response configuration"))
            })
        }
    }
}

//! Common service logic shared by both gRPC and HTTP services.

use crate::*;

pub async fn ingest_service<M: SignalMessage>(request: M::Request) -> Result<M::Response, IngestorServiceError> {
    if request.is_empty() {
        return Ok(M::Response::successful());
    }

    // TODO: send to storage:

    Ok(M::Response::successful())
}

#[cfg(test)]
mod tests {
    use super::*;

    // IMPROVEMENT: somewhat rendant but the other signals can have the respective tests too.

    #[cfg(feature = "logs")]
    mod logs {
        use super::*;

        // If the server receives an empty request
        // (a request that does not carry any telemetry data)
        // the server SHOULD respond with success.
        //
        // https://opentelemetry.io/docs/specs/otlp/#full-success
        #[tokio::test]
        async fn empty_request_returns_successful_response() {
            let empty_request = <LogsMessage as SignalMessage>::Request::empty();
            let actual_response = ingest_service::<LogsMessage>(empty_request).await.unwrap();
            assert_eq!(<LogsMessage as SignalMessage>::Response::successful(), actual_response);
        }
    }
}

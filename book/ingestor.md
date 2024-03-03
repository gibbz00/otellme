# Ingestor

The goal of the ingestor is to act as server component in the OpenTelemetry Protocol (OLTP) [Specification](https://opentelemetry.io/docs/specs/otlp/). The payload is based on Protocol Buffer Schemas defined [here](https://github.com/open-telemetry/opentelemetry-proto/tree/main/opentelemetry/proto).

This chapter aims to present which features the ingestor implements to reach the goal of full OLTP server compliance, but also how they can be configured.

### Transport

OLTP supports a wide-variety of transportation methods summarized in the following table:

| System    | HTTP     | Content-Type             | Supported |
|---        |---       |---                       |:-:        |
| OLTP/gRPC | HTTP/2   | `application/x-protobuf` |🚧         |
| OLTP/HTTP | HTTP/1.1 | `application/x-protobuf` |🚧         |
| OLTP/HTTP | HTTP/2   | `application/x-protobuf` |🚧         |
| OLTP/HTTP | HTTP/1.1 | `application/json`       |🚧         |
| OLTP/HTTP | HTTP/2   | `application/json`       |🚧         |

#### System

##### OLTP/gRPC

Default port is `4317`.

##### OLTP/HTTP

Default port is `4318`.

#### Additional Transport features

##### Compression 🚧

##### TLS 🚧

##### Authentication 🚧

##### Throttling 🚧

### OLTP Services

For the ingestor to be fully compliant, 3 services must be implemented:

- [LogsService](https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/collector/logs/v1/logs_service.proto) 🚧
- [TraceService](https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/collector/trace/v1/trace_service.proto) 🚧
- [MetricsService](https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/collector/metrics/v1/metrics_service.proto) 🚧

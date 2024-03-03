# Ingestor

The goal of the ingestor is to act as server component in the OpenTelemetry Protocol (OLTP) [Specification](https://opentelemetry.io/docs/specs/otlp/). The payload is based on Protocol Buffer Schemas defined [here](https://github.com/open-telemetry/opentelemetry-proto/tree/main/opentelemetry/proto).

This chapter aims to present which features the ingestor implements to reach the goal of full OLTP server compliance, but also how they can be configured.

### Transport

OLTP supports a wide-variety of transportation methods summarized in the following table:

| System    | HTTP     | Content-Type             | Supported |
|---        |---       |---                       |:-:        |
| OLTP/gRPC | HTTP/2   | `application/x-protobuf` |✅         |
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

| Feature \ System | OLTP/gRPC | OLTP/HTTP | Notes         |
|---               |:-:        |:-:        | ---           |
| Compression      | ✅        | 🚧        | Gzip or none. |
| TLS              | 🚧        | 🚧        |               |
| Authentication   | 🚧        | 🚧        |               |
| Throttling       | 🚧        | 🚧        |               |

### Supported signals

| Signal \ System | OLTP/gRPC                                                                                                                            | OLTP/HTTP |
|---              |:-:                                                                                                                                   |:-:        |
| Logs            | [✅](https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/collector/logs/v1/logs_service.proto)       | 🚧        |
| Metrics         | [✅](https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/collector/trace/v1/trace_service.proto)     | 🚧        |
| Traces          | [✅](https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/collector/metrics/v1/metrics_service.proto) | 🚧        |

# Otellme - One-stop solution for OpenTelemetry collection.

[![ci_status](https://img.shields.io/github/actions/workflow/status/gibbz00/otellme/ci.yaml?style=for-the-badge)](https://github.com/gibbz00/otellme/actions/workflows/ci.yaml)
[![codecov](https://img.shields.io/codecov/c/gh/gibbz00/otellme?token=6QOtoiZk7d&style=for-the-badge)](https://codecov.io/gh/gibbz00/otellme)
[![license](https://img.shields.io/github/license/gibbz00/otellme.svg?style=for-the-badge)](https://github.com/gibbz00/otellme/blob/main/LICENSE.md)
[![discord](https://img.shields.io/discord/1211287533393154138?label=discord&color=5865f2&style=for-the-badge)](https://discord.gg/yD5xKbtjDm)

<!-- TEMP: uncomment after 0.1 release
[![crates_io](https://img.shields.io/crates/v/otellme.svg?style=for-the-badge)](https://crates.io/crates/otellme)
[![docs_rs](https://img.shields.io/docsrs/otellme/latest.svg?style=for-the-badge)](https://docs.rs/otellme)
-->

## Goals:

* Support all OpenTelemetry signals (traces, logs and metrics).
* Built-in alert solution.
* Simple setup that just works.
* Offer signal data specific interfaces for processing, analysis and visualization, that is; through a library, a REST API, or a Web GUI.
* Configurable for a wide variety of needs. Notably in terms of availability and scalability.
* Powerful querying and possible integrations with well established data analytics technologies.
* Open development from day one.

### Non-goals

* Support multiple ingestion formats, better handled by the ingestors behind an [OpenTelemetry Collector](https://opentelemetry.io/docs/collector/).

## Project status

Otellme remains to get of the ground by producing a proof of concept, or in other words; very far away from being in a usable state that partially fulfills the project goals. Currently developed by one person during the weekends. Help is nonetheless warmly welcomed :)

## Further reading

* [Architecture](https://excalidraw.com/#json=WO87mhQIxyjwbQhMA2-jN,20b3WxqKmMTPx5UIYbaX1Q)
* [Contributing.](/CODE_OF_CONDUCT.md)
* [Code of conduct.](/CODE_OF_CONDUCT.md)
* [Security policy.](/SECURITY.md)

<!-- TODO: Acknowledge the rest large players (but also the small, and the contributors!):
# Acknowledgements

* [Apache Parquet™](https://parquet.apache.org/) is used for signal storage.
* Storage flexibilities have been made possible by [Apache OpenDAL™](https://opendal.apache.org/).
* Querying made possible with [Apache Arrow DataFusion](https://arrow.apache.org/datafusion/).
* Poem
* Tokio/Tonic
* Opeltelemetry Rust
 -->

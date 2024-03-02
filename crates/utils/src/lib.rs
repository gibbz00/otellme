//! Utility crates for internal Otellme operations.

mod instrumentation;
pub use instrumentation::InternalInstrumentor;
pub(crate) use instrumentation::*;

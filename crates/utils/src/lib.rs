//! Utility crates for internal Otellme operations.

mod instrumentation;
pub use instrumentation::InternalInstrumentor;
pub(crate) use instrumentation::*;

mod non_emtpy_string;
pub use non_emtpy_string::NonEmptyString;

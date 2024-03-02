//! Otellme Ingestor crate.

mod services;
pub(crate) use services::*;

mod server;
pub use server::OtlpIngestor;

/// Temp documentation
pub fn hello_otellme() -> &'static str {
    "hello!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp() {
        assert_eq!("hello!", hello_otellme())
    }
}

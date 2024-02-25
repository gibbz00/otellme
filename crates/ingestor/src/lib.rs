//! Temp crate documentation

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

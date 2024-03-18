/// Mostly used for API expressivess.
#[derive(derive_more::Into)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Returns `Option::None` if the string is empty.
    pub fn new(string: impl Into<String>) -> Option<Self> {
        let string = string.into();
        (!string.is_empty()).then_some(Self(string))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_non_empty_string() {
        assert!(NonEmptyString::new("🤙").is_some())
    }

    #[test]
    fn disallows_empty_string() {
        assert!(NonEmptyString::new("").is_none())
    }
}

use std::fmt::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    fn parse(email: String) -> Result<Self, Error> {
        if !email.contains("@") {
            return Err(Error);
        }
        Ok(Self(email))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_returns_ok_for_valid_email() {
        let email = Email::parse("test@example.com".to_string());
        assert!(email.is_ok());

        let email = email.unwrap();
        assert_eq!(email.as_ref(), "test@example.com");
    }

    #[test]
    fn parse_returns_err_if_missing_at_symbol() {
        let email = Email::parse("testexample.com".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn parse_allows_readonly_access_via_as_ref_str() {
        let email = Email::parse("a@b.com".to_string()).unwrap();
        let as_str: &str = email.as_ref();
        assert_eq!(as_str, "a@b.com");
    }
}

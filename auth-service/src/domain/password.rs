use std::fmt::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    fn parse(password: String) -> Result<Self, Error> {
        if password.len() < 8 {
            return Err(Error);
        }
        Ok(Self(password))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_returns_ok_for_valid_password() {
        let password = Password::parse("ajsaòdfksdaf".to_string());
        assert!(password.is_ok());

        let password = password.unwrap();
        assert_eq!(password.as_ref(), "ajsaòdfksdaf");
    }

    #[test]
    fn parse_returns_err_if_lower_than_8() {
        let password = Password::parse("1234567".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn parse_allows_readonly_access_via_as_ref_str() {
        let password = Password::parse("asdfasdf".to_string()).unwrap();
        let as_str: &str = password.as_ref();
        assert_eq!(as_str, "asdfasdf");
    }
}

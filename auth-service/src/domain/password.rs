use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, Version, password_hash::SaltString};
use color_eyre::eyre::{eyre, Context, Result};
use rand::rngs::OsRng;
use secrecy::{ExposeSecret, SecretString};

use crate::domain::UserStoreError;

#[derive(Debug, Clone)] // Updated!
pub struct HashedPassword(SecretString); // Updated!

impl PartialEq for HashedPassword {
    // New!
    fn eq(&self, other: &Self) -> bool {
        // We can use the expose_secret method to expose the SecretString
        // in a controlled manner when needed!
        self.0.expose_secret() == other.0.expose_secret() // Updated!
    }
}

impl HashedPassword {
    #[tracing::instrument(name = "HashedPassword Parse", skip_all)]
    pub fn parse(s: SecretString) -> Result<HashedPassword> {
        // Updated!
        if validate_password(&s) {
            let result = compute_password_hash(&s)
                .await
                .map_err(|e| UserStoreError::UnexpectedError)?;

            Ok(Self(result))
        } else {
            Err(eyre!("Failed to parse string to a HashedPassword type"))
        }
    }

    #[tracing::instrument(name = "HashedPassword Parse password hash", skip_all)]
    pub fn parse_password_hash(hash: SecretString, // Updated!
    ) -> Result<HashedPassword> {
        if let Ok(hashed_string) = PasswordHash::new(hash.expose_secret().as_ref()) {
            Ok(Self(SecretString::new(
                hashed_string.to_string().into_boxed_str(),
            )))
        } else {
            Err(eyre!("Failed to parse string to a HashedPassword type"))
        }
    }

    #[tracing::instrument(name = "HashedPassword Verify raw password", skip_all)]
    pub async fn verify_raw_password(
        &self,
        password_candidate: &SecretString, // Updated!
    ) -> Result<()> {
        let current_span: tracing::Span = tracing::Span::current();

        let password_hash = self.as_ref().expose_secret().to_owned(); // updated
        let password_candidate = password_candidate.expose_secret().to_owned();
        let result = tokio::task::spawn_blocking(move || {
            current_span.in_scope(|| {
                let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&password_hash)?;

                Argon2::default()
                    .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                    .wrap_err("failed to verify password hash")
            })
        })
        .await?;

        result
    }
}

fn validate_password(s: &SecretString) -> bool {
    // Updated!
    s.expose_secret().len() >= 8
}

impl AsRef<SecretString> for HashedPassword {
    // Updated!
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
pub async fn compute_password_hash(password: &SecretString, // Updated!
) -> Result<SecretString> {
    let current_span: tracing::Span = tracing::Span::current();

    let password = password.expose_secret().to_owned(); // Updated!
    let result = tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let salt: SaltString = SaltString::generate(&mut OsRng);
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

            Ok(SecretString::new(password_hash.into_boxed_str())) // Updated
        })
    })
    .await?;

    result
}

#[cfg(test)]
mod tests {
    use super::Password;

    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;
    use quickcheck::Gen;
    use secrecy::SecretString; // New!

    #[test]
    fn empty_string_is_rejected() {
        let password = SecretString::new("".to_string().into_boxed_str()); // Updated!
        assert!(Password::parse(password).is_err());
    }
    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = SecretString::new("1234567".to_owned().into_boxed_str()); // Updated!
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub SecretString); // Updated!

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = g.size() as u64;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let password: String = FakePassword(8..30).fake_with_rng(&mut rng);
            Self(SecretString::new(password.into_boxed_str())) // Updated!
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(valid_password.0).is_ok()
    }
}

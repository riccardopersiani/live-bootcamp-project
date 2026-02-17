use std::collections::HashMap;

use crate::domain::{Email, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email.clone()) {
            std::collections::hash_map::Entry::Occupied(_) => {
                Err(UserStoreError::UserAlreadyExists)
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            }
        }
    }

    // A public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    // A public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        let user: &User = self.users.get(email).ok_or(UserStoreError::UserNotFound)?;

        user.password
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::HashedPassword;

    use super::*;

    #[tokio::test]
    pub async fn test_add_user() {
        let mut hashmap = HashmapUserStore::default();
        let user = User {
            email: Email::parse("a@b.com".to_string()).unwrap(),
            password: HashedPassword::parse("12345678".to_string()).await.unwrap(),
            requires_2fa: false,
        };
        hashmap.add_user(user).await.expect("Failed to add user");
    }

    #[tokio::test]
    pub async fn test_get_user() {
        let mut hashmap = HashmapUserStore::default();
        let email = Email::parse("test@mail.com".to_string()).unwrap();
        let password = HashedPassword::parse("12345678".to_string()).await.unwrap();
        let requires_2fa = false;
        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: requires_2fa.clone(),
        };

        hashmap.add_user(user).await.expect("Failed to add user");

        let maybe_user = hashmap.users.get(&email);

        match maybe_user {
            Some(user) => {
                assert_eq!(user.email, email);
                assert_eq!(user.password, password);
                assert_eq!(user.requires_2fa, requires_2fa);
            }
            None => panic!("user is not added"),
        }
    }

    #[tokio::test]
    pub async fn test_validate_user() {
        let mut hashmap = HashmapUserStore::default();
        let email = Email::parse("test@mail.com".to_string()).unwrap();
        let raw_password = "12345678".to_owned();
        let user = User {
            email: email.clone(),
            password: HashedPassword::parse("12345678".to_string()).await.unwrap(),
            requires_2fa: false,
        };
        hashmap.add_user(user).await.expect("Failed to add user");
        hashmap.validate_user(&email, &raw_password).await.ok();
    }
}

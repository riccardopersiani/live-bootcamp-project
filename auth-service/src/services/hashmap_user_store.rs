use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

impl HashmapUserStore {
    pub async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
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
    pub async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
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
    pub async fn validate_user(
        &self,
        email: Email,
        password: Password,
    ) -> Result<(), UserStoreError> {
        let user = match self.get_user(&email).await {
            Ok(user) => user,
            // UserStoreError::UserNotFound frp, get_user
            Err(e) => return Err(e),
        };
        if user.password != password {
            Err(UserStoreError::InvalidCredentials)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_add_user() {
        let mut hashmap = HashmapUserStore::default();
        let user = User {
            email: Email::parse("a@b.com".to_string()).unwrap(),
            password: Password::parse("12345678".to_string()).unwrap(),
            requires_2fa: false,
        };
        hashmap.add_user(user).await.expect("Failed to add user");
    }

    #[tokio::test]
    pub async fn test_get_user() {
        let mut hashmap = HashmapUserStore::default();
        let email = Email::parse("test@mail.com".to_string()).unwrap();
        let user = User {
            email: email.clone(),
            password: Password::parse("12345678".to_string()).unwrap(),
            requires_2fa: false,
        };
        hashmap.add_user(user).await.expect("Failed to add user");

        hashmap.get_user(&email).await.ok();
    }

    #[tokio::test]
    pub async fn test_validate_user() {
        let mut hashmap = HashmapUserStore::default();
        let email = Email::parse("test@mail.com".to_string()).unwrap();
        let password = Password::parse("12345678".to_string()).unwrap();
        let user = User {
            email: email.clone(),
            password: Password::parse("12345678".to_string()).unwrap(),
            requires_2fa: false,
        };
        hashmap.add_user(user).await.expect("Failed to add user");
        hashmap.validate_user(email, password).await.ok();
    }
}

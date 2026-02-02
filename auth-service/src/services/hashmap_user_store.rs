use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
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
    pub fn get_user(&self, email: String) -> Result<User, UserStoreError> {
        self.users
            .get(&email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    // A public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: String, password: String) -> Result<(), UserStoreError> {
        let user = match self.get_user(email) {
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
            email: String::from("test@mail.com"),
            password: String::from("1234"),
            requires_2fa: false,
        };
        hashmap.add_user(user).expect("Failed to add user");
    }

    #[tokio::test]
    pub async fn test_get_user() {
        let mut hashmap = HashmapUserStore::default();
        let email = String::from("test@mail.com");
        let user = User {
            email: email.clone(),
            password: String::from("1234"),
            requires_2fa: false,
        };
        hashmap.add_user(user).expect("Failed to add user");

        hashmap.get_user(email.clone()).ok();
    }

    #[tokio::test]
    pub async fn test_validate_user() {
        let mut hashmap = HashmapUserStore::default();
        let email = String::from("test@mail.com");
        let password = String::from("1234");
        let user = User {
            email: email.clone(),
            password: String::from("1234"),
            requires_2fa: false,
        };
        hashmap.add_user(user).expect("Failed to add user");
        hashmap.validate_user(email, password).ok();
    }
}

use crate::domain::AuthAPIError;

use super::User;

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn store_token(&mut self, value: String) -> Result<&mut Self, AuthAPIError>;
    async fn check_if_exists(&self, value: String) -> bool;
}

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: String) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: String, password: String) -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

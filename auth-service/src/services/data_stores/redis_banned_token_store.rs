use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn store_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        // 1. Create a new key using the get_key helper function.
        let token_key = get_key(token.as_str());
        // The value should simply be a `true` (boolean value).
        let value = true;
        // The expiration time should be set to TOKEN_TTL_SECONDS.
        let seconds = TOKEN_TTL_SECONDS;

        // 2. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).
        let _: () = self
            .conn
            .write()
            .await
            // NOTE: The TTL is expected to be a u64 so you will have to cast TOKEN_TTL_SECONDS to a u64.
            .set_ex(&token_key, value, seconds.try_into().unwrap())
            // Return BannedTokenStoreError::UnexpectedError if casting fails or the call to set_ex fails.
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn check_if_exists(&self, token: String) -> Result<bool, BannedTokenStoreError> {
        // Check if the token exists by calling the exists method on the Redis connection
        let token_key = get_key(&token);

        let is_banned: bool = self
            .conn
            .write()
            .await
            .exists(&token_key)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(is_banned)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}

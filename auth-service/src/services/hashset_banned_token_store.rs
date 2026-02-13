use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};
#[derive(Debug, Default)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn store_token(&mut self, value: String) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(value);
        Ok(())
    }

    async fn check_if_exists(&self, value: String) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(&value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_store_token() {
        let mut hashset = HashsetBannedTokenStore::default();
        let token: String = String::from("test");
        let result = hashset.store_token(token.clone()).await;
        assert!(result.is_ok());
        assert!(hashset.tokens.contains(&token));
    }

    #[tokio::test]
    pub async fn test_check_if_exists_false() {
        let mut hashset: HashsetBannedTokenStore = HashsetBannedTokenStore::default();
        let token = String::from("test");
        hashset.tokens.insert(token.clone());

        let result = hashset.check_if_exists(token).await;
        assert!(result.unwrap())
    }
}

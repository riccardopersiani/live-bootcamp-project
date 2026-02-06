use std::collections::HashSet;

use crate::domain::AuthAPIError;

#[derive(Debug, Default)]
pub struct HashsetBannedTokenStore {
    banned_token_store: HashSet<String>,
}

impl HashsetBannedTokenStore {
    //  storing tokens (as Strings)
    fn store_token(&mut self, value: String) -> Result<&mut Self, AuthAPIError> {
        self.banned_token_store.insert(value);
        Ok(self)
    }
    // checking if a token exists within the banned token store
    fn check_if_exists(&self, value: String) -> bool {
        match self.banned_token_store.get(&value) {
            Some(_) => true,
            None => false,
        }
    }
}
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_store_token() {
        let mut hashset = HashsetBannedTokenStore::default();
        let token: String = String::from("test");
        hashset.store_token(token).expect("Failed to add token");
    }

    #[tokio::test]
    pub async fn test_check_if_exists_false() {
        let mut hashset: HashsetBannedTokenStore = HashsetBannedTokenStore::default();
        let token = String::from("test");
        assert_eq!(hashset.check_if_exists(token), false);
    }

    #[tokio::test]
    pub async fn test_check_if_exists_true() {
        let mut hashset: HashsetBannedTokenStore = HashsetBannedTokenStore::default();
        let token = String::from("test");
        hashset
            .store_token(token.clone())
            .expect("Failed to add token");
        assert_eq!(hashset.check_if_exists(token), true);
    }
}

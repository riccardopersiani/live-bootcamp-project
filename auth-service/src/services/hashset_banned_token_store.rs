use std::collections::HashSet;

use crate::domain::AuthAPIError;

#[derive(Debug, Default)]
pub struct HashsetBannedTokenStore {
    banned_token_store: HashSet<String>,
}

impl HashsetBannedTokenStore {
    //  storing tokens (as Strings)
    pub fn store_token(&mut self, value: String) -> () {
        self.banned_token_store.insert(value);
    }
    // checking if a token exists within the banned token store
    pub fn check_if_exists(&self, value: String) -> bool {
        self.banned_token_store.contains(&value)
    }
}
mod tests {
    use crate::services::HashsetBannedTokenStore;

    #[tokio::test]
    pub async fn test_store_token() {
        let mut hashset = HashsetBannedTokenStore::default();
        let token: String = String::from("test");
        hashset.store_token(token);
    }

    #[tokio::test]
    pub async fn test_check_if_exists_false() {
        let hashset: HashsetBannedTokenStore = HashsetBannedTokenStore::default();
        let token = String::from("test");
        assert_eq!(hashset.check_if_exists(token), false);
    }

    #[tokio::test]
    pub async fn test_check_if_exists_true() {
        let mut hashset: HashsetBannedTokenStore = HashsetBannedTokenStore::default();
        let token = String::from("test");
        hashset.store_token(token.clone());
        assert_eq!(hashset.check_if_exists(token), true);
    }
}

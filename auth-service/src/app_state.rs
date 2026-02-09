use crate::services::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;
pub type BannedTokenStoreType = Arc<RwLock<HashsetBannedTokenStore>>;
pub type TwoFACodeStoreType = Arc<RwLock<HashmapTwoFACodeStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl AppState {
    pub fn new(
        user_store: UserStoreType,
        banned_token_store: BannedTokenStoreType,
        two_fa_code_store: TwoFACodeStoreType,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
            two_fa_code_store,
        }
    }
}

use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);
        Ok(())
    }
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let response = self.codes.get(email);
        match response {
            Some(x) => Ok((x.0.clone(), x.1.clone())),
            None => return Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::hash;

    use crate::{
        domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore},
        services::HashmapTwoFACodeStore,
    };

    #[tokio::test]
    pub async fn test_add_code() {
        let mut hashmap = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@mail.com".to_string()).unwrap();
        let code = TwoFACode::default();
        let login_attempt_id = LoginAttemptId::default();

        hashmap
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .expect("Failed to add code");
        ();

        let maybe_result = hashmap.codes.get(&email);
        match maybe_result {
            Some(result) => {
                assert_eq!(result.0, login_attempt_id);
                assert_eq!(result.1, code);
            }
            None => panic!("code is not added"),
        }
    }
    #[tokio::test]
    pub async fn test_get_code() {
        let mut hashmap = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@mail.com".to_string()).unwrap();
        let code = TwoFACode::default();
        let login_attempt_id = LoginAttemptId::default();

        hashmap
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .expect("Failed to add code");
        ();

        let get_result = hashmap.get_code(&email).await.expect("Failed to get code");

        assert_eq!(get_result.0, login_attempt_id);
        assert_eq!(get_result.1, code);

        let maybe_result = hashmap.codes.get(&email);
        match maybe_result {
            Some(result) => {
                assert_eq!(result.0, get_result.0);
                assert_eq!(result.1, get_result.1);
            }
            None => panic!("code is not get"),
        }
    }
    #[tokio::test]
    pub async fn test_remove_code() {
        let mut hashmap = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@mail.com".to_string()).unwrap();
        let code = TwoFACode::default();
        let login_attempt_id = LoginAttemptId::default();

        hashmap
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .expect("Failed to add code");
        ();

        let result = hashmap.get_code(&email).await.expect("Failed to get code");

        assert_eq!(result.0, login_attempt_id);
        assert_eq!(result.1, code);

        hashmap
            .remove_code(&email)
            .await
            .expect("Failed to remove code");

        let maybe_result = hashmap.codes.get(&email);

        match maybe_result {
            Some(_) => {
                panic!("code is not removed")
            }
            None => {}
        }
    }
}

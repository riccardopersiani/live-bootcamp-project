use secrecy::ExposeSecret;
use sqlx::{PgPool, Row};

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    compute_password_hash, Email, HashedPassword, Password, User,
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let password_hash = compute_password_hash(user.password.as_ref())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        sqlx::query(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(user.email.as_ref().expose_secret())
        .bind(password_hash.expose_secret())
        .bind(user.requires_2fa)
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let row = sqlx::query(
            r#"
            SELECT email, password_hash, requires_2fa
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email.as_ref().expose_secret())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?
        .ok_or(UserStoreError::UserNotFound)?;

        let email: String = row.get("email");
        let password_hash: String = row.get("password_hash");
        let requires_2fa: bool = row.get("requires_2fa");

        Ok(User {
            email: Email::parse(secrecy::SecretString::new(email.into_boxed_str()))
                .map_err(|_| UserStoreError::UnexpectedError)?,
            password: Password::parse(secrecy::SecretString::new(password_hash.into_boxed_str()))
                .map_err(|_| UserStoreError::UnexpectedError)?,
            requires_2fa,
        })
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let row = sqlx::query(
            r#"
            SELECT password_hash
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email.as_ref().expose_secret())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?
        .ok_or(UserStoreError::UserNotFound)?;

        let password_hash: String = row.get("password_hash");
        let hashed_password =
            HashedPassword::parse_password_hash(secrecy::SecretString::new(
                password_hash.into_boxed_str(),
            ))
            .map_err(|_| UserStoreError::UnexpectedError)?;

        hashed_password
            .verify_raw_password(password.as_ref())
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}

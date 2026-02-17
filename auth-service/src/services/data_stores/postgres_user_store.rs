use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, HashedPassword, User,
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
        sqlx::query(
            "INSERT INTO users (email, password, requires_2fa)
            VALUES ($1, $2, $3)",
        )
        .bind(user.email.as_ref())
        .bind(user.password.as_ref())
        .bind(user.requires_2fa)
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        sqlx::query!("SELECT * FROM users WHERE email = $1", email.as_ref())
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?
            .map(|row| {
                Ok(User {
                    email: Email::parse(row.email).map_err(|_| UserStoreError::UnexpectedError)?,
                    password: HashedPassword::parse_password_hash(row.password_hash)
                        .map_err(|_| UserStoreError::UnexpectedError)?,
                    requires_2fa: row.requires_2fa,
                })
            })
            .ok_or(UserStoreError::UserNotFound)?
    }

    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        let user: User = self.get_user(email).await?;
        user.password
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}

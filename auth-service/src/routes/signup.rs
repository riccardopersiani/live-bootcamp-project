use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, HashedPassword, User},
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|e| e)?;

    let password = HashedPassword::parse(request.password)
        .await
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User {
        email: email.clone(),
        password: password,
        requires_2fa: request.requires_2fa,
    };

    let mut store = state.user_store.write().await;

    if store.get_user(&email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    };

    match store.add_user(user).await {
        Ok(v) => v,
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

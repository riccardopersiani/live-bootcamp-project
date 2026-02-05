use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User},
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|e| e)?;

    let password = Password::parse(request.password).map_err(|e| e)?;

    let user = User {
        email: email.clone(),
        password: password.clone(),
        requires_2fa: request.requires_2fa,
    };

    let mut user_store = state.user_store.write().await;

    if !user_store.get_user(email).is_err() {
        return Err(AuthAPIError::UserAlreadyExists);
    };

    match user_store.add_user(user) {
        Ok(v) => v,
        Err(e) => return Err(AuthAPIError::UnexpectedError),
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

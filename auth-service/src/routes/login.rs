use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, UserStore, UserStoreError},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|e| e)?;

    let password = Password::parse(request.password).map_err(|e| e)?;

    let user_store = &state.user_store.read().await;

    let user = match user_store.get_user(email) {
        Ok(user) => user,
        Err(UserStoreError::UserNotFound) => {
            return Err(AuthAPIError::IncorrectCredentials);
        }
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    };

    if user.password.as_ref() != password.as_ref() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

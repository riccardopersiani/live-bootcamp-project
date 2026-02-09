use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, UserStoreError},
    utils::auth::generate_auth_cookie,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use axum_extra::extract::CookieJar;

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    let user_store = &state.user_store.read().await;

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(UserStoreError::UserNotFound) => {
            return (jar, Err(AuthAPIError::IncorrectCredentials));
        }
        Err(UserStoreError::InvalidCredentials) => {
            return (jar, Err(AuthAPIError::InvalidCredentials));
        }
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    if user.password.as_ref() != password.as_ref() {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    }

    let auth_cookie = generate_auth_cookie(&user.email).unwrap();

    let updated_jar = jar.clone().add(auth_cookie);

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (updated_jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    match user.requires_2fa {
        true => handle_2fa(updated_jar).await,
        false => handle_no_2fa(&user.email, updated_jar).await,
    }
}

async fn handle_2fa(
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_string(),
        login_attempt_id: "123456".to_string(),
    }));
    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let response = Json(LoginResponse::RegularAuth);
    (jar, Ok((StatusCode::OK, response)))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

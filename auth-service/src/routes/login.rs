use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode},
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

    let user_store = state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
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
        true => handle_2fa(&email, &state, updated_jar).await,
        false => handle_no_2fa(&user.email, updated_jar).await,
    }
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    if state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    if state
        .email_client
        .read()
        .await
        .send_email(email, "2FA Code", two_fa_code.as_ref())
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_string(), // Add the generated login attempt ID
    }));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

async fn handle_no_2fa(
    #[warn(unused)] _email: &Email,
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

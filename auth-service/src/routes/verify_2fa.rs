use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::domain::{Email, LoginAttemptId, TwoFACode};

pub async fn verify_2fa(Json(request): Json<Verify2FARequest>) -> impl IntoResponse {
    match Email::parse(request.email) {
        Ok(_) => {}
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    }
    match LoginAttemptId::parse(request.login_attempt_id) {
        Ok(_) => {}
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    }
    match TwoFACode::parse(request.code_2fa) {
        Ok(_) => {}
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    }

    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub code_2fa: String,
}

use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie, CookieJar};

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };
    let token = cookie.value().to_owned();

    // Return AuthAPIError::InvalidToken is validation fails.
    let res = validate_token(token.as_str()).await;
    match res {
        Ok(_) => (jar, Ok(StatusCode::OK)),
        Err(_) => (jar, Err(AuthAPIError::InvalidToken)),
    }
}

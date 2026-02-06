use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
    cookie::{self, Cookie},
    CookieJar,
};

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    jar: CookieJar,
    State(state): State<AppState>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };
    let token = cookie.value().to_owned();

    let mut state = state.banned_token_store.write().await;
    // Return AuthAPIError::InvalidToken is validation fails.
    let res = validate_token(token.as_str(), &state).await;
    state.store_token(token.clone());
    match res {
        Ok(_) => {
            let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));
            (jar, Ok(StatusCode::OK))
        }
        Err(_) => (jar, Err(AuthAPIError::InvalidToken)),
    }
}

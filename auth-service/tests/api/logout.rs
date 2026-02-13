use auth_service::{domain::BannedTokenStore, utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email(); // Call helper method to generate email

    let test_cases = [
        // - The email is empty or does not contain '@'
        serde_json::json!({
            "email": "emailtest.com",
            "password": "password123",
        }),
        serde_json::json!({
        // - The password is less than 8 characters
            "email": random_email,
            "password": "1234567",
        }),
        serde_json::json!({
            "email": "",
            "password": "password123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_logout(test_case).await; // call `post_login`
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let random_email = TestApp::get_random_email(); // Call helper method to generate email

    let test_cases = [
        // - The email is empty or does not contain '@'
        serde_json::json!({
            "email": "email@test.com",
            "password": "password123",
        }),
        serde_json::json!({
        // - The password is less than 8 characters
            "email": random_email,
            "password": "password123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_logout(test_case).await; // call `post_login`
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Incorrect credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let logout_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_logout(&logout_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let maybe_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(
        maybe_cookie.is_none() || maybe_cookie.unwrap().value().is_empty(),
        "Expected JWT cookie to be removed/expired"
    );

    let banned_token_store = app.banned_token_store.read().await;
    let exists = banned_token_store
        .check_if_exists(auth_cookie.value().to_string())
        .await
        .expect("Failed to check if token is banned");
    assert!(exists);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let logout_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_logout(&logout_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let maybe_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(
        maybe_cookie.is_none() || maybe_cookie.unwrap().value().is_empty(),
        "Expected JWT cookie to be removed/expired"
    );

    let response = app.post_logout(&logout_body).await;

    assert_eq!(response.status().as_u16(), 400);
}

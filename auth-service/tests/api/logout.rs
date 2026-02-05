use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
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

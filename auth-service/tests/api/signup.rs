use crate::helpers::TestApp;

#[tokio::test]
async fn signup_returns_auth_ui() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let body = serde_json::json!({
        "email": random_email,
                    "password": "password123",
            "requires2FA": true
    });

    let response = app.post_signup(&body).await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email(); // Call helper method to generate email

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

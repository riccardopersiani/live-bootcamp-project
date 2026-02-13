use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = TestApp::get_random_email();

    let test_cases = [
        serde_json::json!({
            "loginAttemptId": "23",
            "2FACode": "23",
        }),
        serde_json::json!({
            "email": random_email,
            "2FACode": "23",
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "23",
        }),
    ];
    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let random_email = TestApp::get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": "",
            "loginAttemptId": "23",
            "2FACode": "23",
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "",
            "2FACode": "",
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "",
            "2FACode": "12",
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "12",
            "2FACode": "",
        }),
    ];
    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

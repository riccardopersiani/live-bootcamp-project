use crate::helpers::TestApp;
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let random_email = TestApp::get_random_email(); // Call helper method to generate email

    let test_cases = [
        serde_json::json!({
            "password": "password123432432",
        }),
        serde_json::json!({
            "email": random_email,
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await; // call `post_login`
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
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
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
        let response = app.post_login(test_case).await; // call `post_login`
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
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    let app = TestApp::new().await;

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
        let response = app.post_login(test_case).await; // call `post_login`
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

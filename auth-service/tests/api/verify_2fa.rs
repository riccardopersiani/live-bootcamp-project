use auth_service::{domain::Email, routes::TwoFactorAuthResponse};

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

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let two_fa_code = code_tuple.1.as_ref();

    let verify_2fa_body = serde_json::json!({
        "email": "a@a.incorrect",
        "loginAttemptId": login_attempt_id.as_str(),
        "2FACode": two_fa_code
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let two_fa_code_old = code_tuple.1.as_ref();

    // second login and verify
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let two_fa_code_new = code_tuple.1.as_ref();

    assert!(two_fa_code_new != two_fa_code_old);

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.as_str(),
        "2FACode": two_fa_code_old
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let two_fa_code_old = code_tuple.1.as_ref();

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.as_str(),
        "2FACode": two_fa_code_old
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let two_fa_code_old = code_tuple.1.as_ref();

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.as_str(),
        "2FACode": two_fa_code_old
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 200);
    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

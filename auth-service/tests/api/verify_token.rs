use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let test_cases = [
        serde_json::json!({
            "token": "",
        }),
        serde_json::json!({
            "token": 0,
        }),
    ];
    for test_case in test_cases.iter() {
        let response = app.post_verify_token(test_case).await; // call `post_login`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

use auth_service::Application;
use serde_json::json;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new(); // Create a Reqwest http client instance

        // Create new `TestApp` instance and return it
        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(&json!({
                "email": "user@example.com",
                "password": "string",
                "requires2FA": true
            }))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn post_login(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(&json!({
                "email": "user@example.com",
                "password": "string"
            }))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(&json!({
                "email": "user@example.com",
                "password": "string",
                "2FACode": "1234"
            }))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn post_logout(&self) -> reqwest::Response {
        let jwt = "egfja";
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .bearer_auth(jwt)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn post_verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(&json!({
                "token": "token1"
            }))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

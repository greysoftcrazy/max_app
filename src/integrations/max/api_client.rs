use reqwest::Client;
use serde::Serialize;
use tracing::{info, error, debug};
use serde_json;

#[derive(Debug, Clone)]
pub struct MaxApiClient {
    auth_token: String,
    api_base_url: String,
    http_client: Client,
}

#[derive(Debug, Serialize)]
pub struct SendMessageRequest {
    pub format: String,
    pub text: String,
}

impl MaxApiClient {
    pub fn new(auth_token: String) -> Self {
        Self {
            auth_token,
            api_base_url: "https://platform-api.max.ru".to_string(),
            http_client: Client::new(),
        }
    }

    pub async fn send_message(&self, chat_id: i64, user_id: i64, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Формируем запрос БЕЗ recipient в теле
        let request = SendMessageRequest {
            format: "html".to_string(),
            text: text.to_string(),
        };

        // Логируем отправляемый JSON
        let request_json = serde_json::to_string_pretty(&request)?;
        debug!("📤 Отправляемый запрос в МАКС:\n{}", request_json);

        // chat_id и user_id передаются в URL как query-параметры!
        let url = format!(
            "{}/messages?chat_id={}&user_id={}",
            self.api_base_url,
            chat_id,
            user_id
        );

        let response = self.http_client
            .post(&url)
            .header("Authorization", &self.auth_token)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            info!("✅ Сообщение отправлено chat_id={}", chat_id);
            Ok(())
        } else {
            let error_text = response.text().await?;
            error!("❌ Ошибка отправки сообщения: {} - {}", status, error_text);
            Err(format!("MAX API error: {} - {}", status, error_text).into())
        }
    }
}
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    body::Bytes,
};
use serde::Deserialize;
use serde_json;
use tracing::info;
use std::sync::Arc;

use crate::{
    error::AppError,
    integrations::max::MaxApiClient,
    state::AppState,
};

// Структура реального вебхука от МАКС
#[derive(Debug, Deserialize)]
pub struct MaxWebhook {
    pub timestamp: u64,
    pub message: Message,
    pub user_locale: String,
    #[serde(rename = "update_type")]
    pub update_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub recipient: Recipient,
    pub timestamp: u64,
    pub body: MessageBody,
    pub sender: Sender,
}

#[derive(Debug, Deserialize)]
pub struct Recipient {
    #[serde(rename = "chat_id")]
    pub chat_id: i64,
    #[serde(rename = "chat_type")]
    pub chat_type: String,
    #[serde(rename = "user_id")]
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct MessageBody {
    pub mid: String,
    pub seq: u64,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Sender {
    #[serde(rename = "user_id")]
    pub user_id: i64,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "is_bot")]
    pub is_bot: bool,
    #[serde(rename = "last_activity_time")]
    pub last_activity_time: u64,
    pub name: String,
}

pub async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    // Парсим вебхук из сырого тела
    let payload: MaxWebhook = match serde_json::from_slice(&body) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Ошибка парсинга вебхука: {}", e);
            return Err(AppError::BadRequest("Invalid webhook format".to_string()));
        }
    };
    
    info!(
        "📨 Вебхук от МАКС | update_type: {}, chat_id: {}, user: {} {}",
        payload.update_type,
        payload.message.recipient.chat_id,
        payload.message.sender.first_name,
        payload.message.sender.last_name
    );

    let text = &payload.message.body.text;
    info!("💬 Текст сообщения: {:?}", text);

    // Обработка команды
    let response_text = if text.starts_with("/start") || text == "привет" {
        handle_start()
    } else if text.starts_with("/help") || text == "помощь" {
        handle_help()
    } else if text.starts_with("/search") || text.starts_with("/поиск") {
        handle_search(text)
    } else if text.starts_with("/work") || text.starts_with("/работа") {
        handle_work(text)
    } else {
        "❌ Неизвестная команда. Введите /help для справки.".to_string()
    };

    // Отправка сообщения через клиент МАКС (используем chat_id и user_id из recipient!)
    let client = MaxApiClient::new(state.max_bot_token.clone());

    match client.send_message(
        payload.message.recipient.chat_id,
        payload.message.recipient.user_id,
        &response_text
    ).await {
        Ok(_) => {
            info!("✅ Ответ отправлен пользователю chat_id={}", payload.message.recipient.chat_id);
            Ok(StatusCode::OK.into_response())
        }
        Err(e) => {
            tracing::error!("Ошибка отправки сообщения: {}", e);
            Err(AppError::Internal(format!("Failed to send message: {}", e)))
        }
    }
}

fn handle_start() -> String {
    r#"👋 Добро пожаловать в Цифровой архив ГПОУ ЮТК им. Павлючкова Г.А.!

📚 Здесь вы можете найти конкурсные работы и статьи обучающихся и преподавателей колледжа.

🔎 Доступные команды:
/search <запрос> — поиск работ
/work <ID> — просмотр работы по ID
/help — справка

💡 Нажмите кнопку «Открыть» ниже для удобного поиска в мини-приложении!"#.to_string()
}

fn handle_help() -> String {
    r#"📖 Справка по командам:

/start — приветствие и основное меню
/search <запрос> — поиск по названию, автору, ключевым словам
/work <ID> — просмотр работы по уникальному идентификатору
/help — эта справка

💡 Совет: для удобного поиска и просмотра работ нажмите кнопку «Открыть» ниже — откроется мини-приложение с полным функционалом."#.to_string()
}

fn handle_search(text: &str) -> String {
    let query = text
        .trim_start_matches("/search")
        .trim_start_matches("/поиск")
        .trim();
    
    if query.is_empty() {
        return "🔍 Укажите критерии поиска.\nПример: /search веб-разработка".to_string();
    }
    
    format!("🔍 Поиск по запросу: \"{}\"\nФункция пока в разработке.", query)
}

fn handle_work(text: &str) -> String {
    let id = text
        .trim_start_matches("/work")
        .trim_start_matches("/работа")
        .trim();
    
    if id.is_empty() {
        return "📄 Укажите ID работы.\nПример: /work 123e4567-e89b-12d3-a456-426614174000".to_string();
    }
    
    format!("📄 Просмотр работы ID: {}\nФункция пока в разработке.", id)
}
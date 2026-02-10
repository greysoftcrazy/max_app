use axum::{extract::{State, Json}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::info;
use std::sync::Arc;
use sqlx::PgPool;

use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct MaxWebhook {
    #[serde(rename = "update_id")]
    pub update_id: u64,
    pub message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: u64,
    pub from: Option<User>,
    pub chat: Chat,
    pub date: u64,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: String,
}

#[derive(Debug, Serialize)]
pub struct BotResponse {
    pub method: String,
    pub chat_id: i64,
    pub text: String,
}

pub async fn handle_webhook(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<MaxWebhook>,
) -> Result<impl IntoResponse, AppError> {
    info!("📨 Получен вебхук от МАКС | update_id: {}", payload.update_id);

    let Some(message) = payload.message else {
        return Ok(StatusCode::OK.into_response());
    };
    
    let Some(text) = message.text else {
        return Ok(StatusCode::OK.into_response());
    };
    
    let chat_id = message.chat.id;
    let is_group = message.chat.chat_type != "private";
    
    // Нормализация команды для групповых чатов
    let normalized_text = if is_group {
        normalize_group_command(&text, "ytk_chat_bot")
    } else {
        text.trim().to_lowercase()
    };

    info!("💬 Текст сообщения: {:?}", normalized_text);

    // Обработка команды
    let response_text = if normalized_text.starts_with("/start") || normalized_text == "привет" {
        handle_start(is_group)
    } else if normalized_text.starts_with("/search") || normalized_text.starts_with("/поиск") {
        handle_search(&normalized_text).await
    } else if normalized_text.starts_with("/work") || normalized_text.starts_with("/работа") {
        handle_work(&normalized_text).await
    } else if normalized_text.starts_with("/help") || normalized_text == "помощь" {
        handle_help(is_group)
    } else {
        if is_group {
            return Ok(StatusCode::OK.into_response());
        }
        "❌ Неизвестная команда. Введите /help для справки.".to_string()
    };

    info!("✅ Отправка ответа пользователю chat_id={}", chat_id);

    // Отправка ответа через отдельный HTTP-запрос к API МАКС
    // (заглушка - реализация будет позже)
    send_bot_message(chat_id, &response_text).await?;

    Ok(StatusCode::OK.into_response())
}

fn normalize_group_command(text: &str, bot_username: &str) -> String {
    let mut normalized = text.trim().to_lowercase();
    
    let bot_mention = format!("@{}", bot_username.to_lowercase());
    if normalized.ends_with(&bot_mention) {
        normalized = &normalized[..normalized.len() - bot_mention.len()];
    }
    
    normalized.to_string()
}

fn handle_start(is_group: bool) -> String {
    if is_group {
        "👋 Я — бот цифрового архива ГПОУ ЮТК им. Павлючкова Г.А.\n\n\
        Для поиска работ используйте команды:\n\
        /search <запрос> — поиск по архиву\n\
        /work <ID> — просмотр работы по ID\n\
        /help — справка".to_string()
    } else {
        r#"👋 <b>Цифровой архив ГПОУ ЮТК им. Павлючкова Г.А.</b>

📚 Здесь вы можете найти конкурсные работы и статьи обучающихся и преподавателей колледжа.

🔎 Доступные команды:
/search <запрос> — поиск работ
/work <ID> — просмотр работы по ID
/help — справка

💡 Нажмите кнопку «Открыть» ниже для удобного поиска в мини-приложении!"#.to_string()
    }
}

fn handle_help(is_group: bool) -> String {
    if is_group {
        "📖 <b>Справка по командам</b>\n\n\
        /search <запрос> — поиск по названию, автору, ключевым словам\n\
        /work <ID> — просмотр работы по уникальному идентификатору\n\
        /help — эта справка".to_string()
    } else {
        r#"📖 <b>Справка по командам</b>

/start — приветствие и основное меню
/search <запрос> — поиск по названию, автору, ключевым словам
/work <ID> — просмотр работы по уникальному идентификатору
/help — эта справка

💡 Совет: для удобного поиска и просмотра работ нажмите кнопку «Открыть» ниже — откроется мини-приложение с полным функционалом."#.to_string()
    }
}

async fn handle_search(_text: &str) -> String {
    "🔍 Эта функция пока в разработке. Скоро поиск работ будет доступен!".to_string()
}

async fn handle_work(_text: &str) -> String {
    "📄 Эта функция пока в разработке. Скоро просмотр работ будет доступен!".to_string()
}

async fn send_bot_message(_chat_id: i64, _text: &str) -> Result<(), AppError> {
    // Заглушка - реализация отправки сообщения через API МАКС
    // Будет реализована позже
    Ok(())
}
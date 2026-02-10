use axum::{http::StatusCode, response::{IntoResponse, Response}};
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Внутренняя ошибка сервера: {0}")]
    Internal(String),
    
    #[error("Неверный запрос: {0}")]
    BadRequest(String),
    
    #[error("Неавторизован")]
    Unauthorized,
    
    #[error("Запрещено")]
    Forbidden,
    
    #[error("Не найдено")]
    NotFound,
    
    #[error("Ошибка валидации: {0}")]
    ValidationError(String),
    
    #[error("Ошибка базы данных: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Internal(msg) => {
                error!("Внутренняя ошибка: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Внутренняя ошибка сервера")
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Требуется авторизация"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Доступ запрещён"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Ресурс не найден"),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, &msg),
            AppError::DatabaseError(e) => {
                error!("Ошибка базы данных: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка базы данных")
            }
        };

        (status, error_message).into_response()
    }
}
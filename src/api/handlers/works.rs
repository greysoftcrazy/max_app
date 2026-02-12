use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::AppError,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub query: Option<String>,
    pub specialty: Option<String>,
    pub work_type: Option<String>,
    pub year: Option<i32>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn get_work_by_id(
    Path(_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Заглушка — возвращает 404 (работа не найдена)
    Err(AppError::NotFound)
}

pub async fn search_works(
    Query(_params): Query<SearchQuery>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    // Заглушка — возвращает пустой массив
    Ok(Json(vec![]))
}

pub async fn list_specialties(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, AppError> {
    // Заглушка — возвращает список специальностей
    // Позже будет получать из БД
    let specialties = vec![
        "Информационные системы и программирование".to_string(),
        "Сетевое и системное администрирование".to_string(),
        "Экономика и бухгалтерский учёт".to_string(),
        "Право и организация социального обеспечения".to_string(),
        "Дошкольное образование".to_string(),
    ];
    
    Ok(Json(specialties))
}
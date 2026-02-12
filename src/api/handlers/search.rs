use axum::{extract::State, Json};
use std::sync::Arc;
use sqlx::PgPool;
use serde::Serialize;

use crate::error::AppError;

#[derive(Serialize)]
pub struct SearchResponse {
    message: String,
}

pub async fn search_works(
    State(_pool): State<Arc<PgPool>>,
) -> Result<Json<SearchResponse>, AppError> {
    Ok(Json(SearchResponse {
        message: "Поиск пока не реализован".to_string(),
    }))
}
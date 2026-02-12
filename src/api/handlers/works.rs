use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    core::services::WorkService,
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

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub works: Vec<serde_json::Value>,
    pub total: usize,
    pub page: u32,
    pub limit: u32,
}

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn get_work_by_id(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let service = WorkService::new(state.pool.clone());
    let work = service.get_by_id(id).await?;
    
    match work {
        Some(w) => Ok(Json(serde_json::json!(w))),
        None => Err(AppError::NotFound),
    }
}

pub async fn search_works(
    Query(params): Query<SearchQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<SearchResponse>, AppError> {
    let service = WorkService::new(state.pool.clone());
    
    let works = service.search(
        params.query.as_deref(),
        params.specialty.as_deref(),
        params.work_type.as_deref(),
        params.year,
        params.page.unwrap_or(1),
        params.limit.unwrap_or(20),
    ).await?;
    
    let total = works.len();  // ← Получаем длину ДО перемещения

    let response = SearchResponse {
        works: works.into_iter()
            .map(|w| serde_json::json!(w))
            .collect(),
        total,
        page: params.page.unwrap_or(1),
        limit: params.limit.unwrap_or(20),
    };
    
    Ok(Json(response))
}

pub async fn list_specialties(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, AppError> {
    let service = WorkService::new(state.pool.clone());
    let specialties = service.list_specialties().await?;
    
    Ok(Json(specialties))
}
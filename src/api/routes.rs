use axum::{Router, routing::{get, post}};
use std::sync::Arc;
use sqlx::PgPool;

use super::handlers;

pub fn create_router(pool: PgPool) -> Router {
    let shared_pool = Arc::new(pool);
    
    Router::new()
        // Вебхук для чат-бота МАКС
        .route("/api/max/webhook", post(handlers::max_webhook::handle_webhook))
        
        // API для поиска и просмотра работ
        .route("/api/works/search", get(handlers::search::search_works))
        .route("/api/works/:id", get(handlers::works::get_work_by_id))
        .route("/api/works/specialties", get(handlers::works::list_specialties))
        
        // Health check
        .route("/health", get(|| async { "OK" }))
        
        .with_state(shared_pool)
}
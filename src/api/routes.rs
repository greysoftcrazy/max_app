use axum::Router;
use std::sync::Arc;

use super::handlers;
use crate::state::AppState;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Вебхук для чат-бота МАКС
        .route("/api/max/webhook", axum::routing::post(handlers::max_webhook::handle_webhook))
        
        // API для поиска и просмотра работ
        .route("/api/works/search", axum::routing::get(handlers::works::search_works))
        .route("/api/works/:id", axum::routing::get(handlers::works::get_work_by_id))
        .route("/api/works/specialties", axum::routing::get(handlers::works::list_specialties))
        
        // Health check
        .route("/health", axum::routing::get(handlers::works::health_check))
        
        .with_state(state)
}
pub mod max_webhook;
pub mod works;
pub mod search;

use axum::http::StatusCode;

pub use max_webhook::handle_webhook;
pub use works::{get_work_by_id, list_specialties};
pub use search::search_works;

pub async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}



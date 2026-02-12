use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;
use std::sync::Arc;

mod config;
mod error;
mod api;
mod core;
mod infrastructure;
mod integrations;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    // Загрузка переменных окружения из .env
    dotenv().ok();

    // Инициализация логирования
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "max_app=trace,tower_http=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    tracing::info!("🚀 Запуск приложения max_app");

    // Загрузка конфигурации
    let config = config::load().expect("Не удалось загрузить конфигурацию");

    // Инициализация базы данных
    let pool = infrastructure::database::connect(&config.database_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    // Создание состояния приложения (оборачиваем в Arc сразу)
    let app_state = Arc::new(AppState {
        pool: pool.clone(),
        max_bot_token: config.max_bot_token.clone(),
    });

    // Создание маршрутов
    let app = api::routes::create_router(app_state);

    // Запуск сервера — НОВЫЙ РЕКОМЕНДОВАННЫЙ СПОСОБ для Axum 0.7+
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("📡 Сервер запущен на http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
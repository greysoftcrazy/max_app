use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod api;
mod core;
mod infrastructure;
mod integrations;

#[tokio::main]
async fn main() {
    // Инициализация логирования
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "max_app=debug,tower_http=debug".into()),
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

    // Выполнение миграций
    #[cfg(feature = "migrate")]
    {
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Миграции не выполнены");
    }

    // Создание маршрутов
    let app = api::routes::create_router(pool);

    // Запуск сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("📡 Сервер запущен на http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
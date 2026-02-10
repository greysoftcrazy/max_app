use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::PgPool;
use std::time::Duration;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let options = database_url.parse::<PgConnectOptions>()?;
    
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await
}
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
  PgPoolOptions::new()
    .max_connections(1)
    .idle_timeout(Duration::from_secs(30))
    .max_lifetime(Some(Duration::from_secs(180)))
    .test_before_acquire(true)
    .acquire_timeout(Duration::from_secs(2))
    .connect(database_url)
    .await
}

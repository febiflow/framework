use std::env;

use sqlx::PgPool;
use crate::connectors::postgres_connector;

pub enum DatabaseDriver {
  Postgres,
}

impl DatabaseDriver {
  pub fn from_env() -> Self {
    match env::var("DB_CONNECTION")
      .unwrap_or_else(|_| "pgsql".to_string())
      .as_str() {
      "pgsql" | "postgres" => DatabaseDriver::Postgres,
      other => panic!("Unsupported DB_CONNECTION: {}", other),
    }
  }
}

pub async fn connect() -> Result<PgPool, sqlx::Error> {
  let database_url = env::var("DB_URL").expect("DB_URL must be set");

  match DatabaseDriver::from_env() {
    DatabaseDriver::Postgres => postgres_connector::connect(&database_url).await,
  }
}

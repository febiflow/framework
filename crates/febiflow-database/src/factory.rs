use sqlx::PgConnection;
use febiflow_lambda::{Error, ErrorKind, Result};

use crate::config::DatabaseConfig;
use crate::postgres;

pub async fn connection() -> Result<PgConnection> {
    let cfg = DatabaseConfig::from_env();

    match cfg.driver.as_str() {
        "pgsql" | "postgres" | "postgresql" => Ok(postgres::connect(&cfg.url).await?),
        other => Err(Error {
          kind: ErrorKind::UnsupportedDatabaseConnection,
          message: format!("Unsupported DB connection: {}", other).into(),
        }),
    }
}

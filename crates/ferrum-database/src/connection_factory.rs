use ferrum_lambda::{Error, ErrorKind, Result};

use crate::config::DatabaseConfig;
use crate::connectors::postgres_connector;
use crate::connection::ConnectionType;

pub async fn create_connection() -> Result<ConnectionType> {
  let config = DatabaseConfig::from_env();

  match config.connection.as_str() {
    "pgsql" | "postgres" | "postgresql" => {
      let conn = postgres_connector::connect(&config.url).await?;
      Ok(ConnectionType::Postgres(conn))
    }
    _ => Err(Error {
      kind: ErrorKind::UnsupportedDatabaseConnection,
      message: format!("Unsupported DB connection: {}", config.connection).into(),
    }),
  }
}

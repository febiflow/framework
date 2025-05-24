use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ErrorKind {
  DeserializationError,
  InvalidRequestBody,
  DatabaseConnectionError,
  QueryExecutionError,
  UnsupportedDatabaseConnection,
  ConfigError,
  EnvironmentError,
  IOError,
  TimeoutError,
  Unknown,
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Self {
      Error {
          kind: ErrorKind::DeserializationError,
          message: format!("Failed to parse request body: {}", err),
      }
  }
}

impl From<sqlx::Error> for Error {
  fn from(err: sqlx::Error) -> Self {
      use sqlx::Error::*;
      let (kind, msg) = match &err {
          Configuration(_) => (ErrorKind::ConfigError, format!("DB config error: {}", err)),
          Io(_) => (ErrorKind::IOError, format!("I/O error: {}", err)),
          PoolTimedOut => (ErrorKind::TimeoutError, "Database pool timeout".into()),
          Database(_) => (ErrorKind::QueryExecutionError, format!("Query error: {}", err)),
          RowNotFound => (ErrorKind::QueryExecutionError, "Row not found".into()),
          _ => (ErrorKind::DatabaseConnectionError, format!("Database connection failed: {}", err)),
      };
      Self { kind, message: msg }
  }
}

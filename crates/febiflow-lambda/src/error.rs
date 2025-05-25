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
  EnvVarNotPresent,
  JwtInvalidToken,
  JwtInvalidSignature,
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

impl From<std::env::VarError> for Error {
  fn from(error: std::env::VarError) -> Self {
    Error {
      kind: ErrorKind::EnvVarNotPresent,
      message: error.to_string()
    }
  }
}

impl From<jsonwebtoken::errors::Error> for Error {
  fn from(error: jsonwebtoken::errors::Error) -> Self {
    let jwt_error_kind = match error.kind() {
      jsonwebtoken::errors::ErrorKind::InvalidToken { .. } => ErrorKind::JwtInvalidToken,
      jsonwebtoken::errors::ErrorKind::InvalidSignature => ErrorKind::JwtInvalidSignature,
      _ => ErrorKind::Unknown,
    };

    Error {
      kind: jwt_error_kind,
      message: error.to_string()
    }
  }
}

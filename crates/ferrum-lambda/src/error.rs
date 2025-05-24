use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ErrorKind {
  DeserializationError,
  InvalidRequestBody,
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

impl Error {
  pub fn deserialization(err: impl ToString) -> Self {
      Self {
          kind: ErrorKind::DeserializationError,
          message: format!("Failed to parse request body: {}", err.to_string()),
      }
  }

  pub fn invalid_body() -> Self {
      Self {
          kind: ErrorKind::InvalidRequestBody,
          message: "The request body is missing or invalid.".into(),
      }
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

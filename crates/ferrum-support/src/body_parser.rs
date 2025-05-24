use ferrum_lambda::{error::ErrorKind, Error, Result};
use lambda_http::Body;
use serde::de::DeserializeOwned;
use validator::Validate;

pub trait BodyParserProvider {
  fn parse<T: DeserializeOwned + Validate>(&self, body: &Body) -> Result<T>;
}

pub struct BodyParser {}

impl BodyParser {
  pub fn new() -> Self {
    BodyParser {}
  }
}

impl BodyParserProvider for BodyParser {
  fn parse<T: DeserializeOwned + Validate>(&self, body: &Body) -> Result<T> {
    match body {
      Body::Text(body_str) => Ok(serde_json::from_str(body_str)?),
      Body::Binary(body_binary) => Ok(serde_json::from_slice(body_binary)?),
      Body::Empty => Err(Error {
        kind: ErrorKind::InvalidRequestBody,
        message: "The request body is missing or invalid.".to_string(),
      }),
    }
  }
}

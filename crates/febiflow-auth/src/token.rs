use std::env;

use febiflow_lambda::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::AccessToken;

pub trait TokenProvider {
  fn encode_token(&self, user_id: Uuid) -> Result<String, Error>;
  fn decode_token(&self, token: &str) -> Result<AccessToken, Error>;
  fn extract_user_id(&self, token: &str) -> Result<Uuid, Error>;
}

pub struct Token {
  encoding_key: EncodingKey,
  decoding_key: DecodingKey,
}

impl Token {
  pub fn new() -> Result<Self, Error> {
    let secret = env::var("APP_KEY")?;
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    Ok(Token { encoding_key, decoding_key })
  }
}

impl TokenProvider for Token {
  fn encode_token(&self, user_id: Uuid) -> Result<String, Error> {
    let claims = AccessToken { user_id };
    Ok(encode(&Header::default(), &claims, &self.encoding_key)?)
  }

  fn decode_token(&self, token: &str) -> Result<AccessToken, Error> {
    Ok(decode(token, &self.decoding_key, &Validation::default())?.claims)
  }

  fn extract_user_id(&self, token: &str) -> Result<Uuid, Error> {
    Ok(self.decode_token(token)?.user_id)
  }
}

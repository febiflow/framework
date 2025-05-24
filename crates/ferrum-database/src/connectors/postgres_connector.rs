use sqlx::{PgConnection, Connection};
use ferrum_lambda::Result;

pub async fn connect(database_url: &str) -> Result<PgConnection> {
  Ok(PgConnection::connect(database_url).await?)
}

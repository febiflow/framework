use sqlx::{PgConnection, Connection};

pub async fn connect(database_url: &str) -> sqlx::Result<PgConnection> {
    PgConnection::connect(database_url).await
}

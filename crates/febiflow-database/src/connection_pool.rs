use sqlx::PgPool;
use tokio::sync::OnceCell;

use super::connection_factory;

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_pool() -> &'static PgPool {
  DB_POOL.get_or_init(|| async {
    connection_factory::connect().await
      .expect("Failed to initialize database connection")
  }).await
}

use sqlx::PgConnection;

pub enum ConnectionType {
    Postgres(PgConnection),
}

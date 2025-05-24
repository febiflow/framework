use std::env;

pub struct DatabaseConfig {
    pub connection: String,
    pub url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let connection = env::var("DB_CONNECTION").unwrap_or_else(|_| "pgsql".to_string());
        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let database = env::var("DB_DATABASE").unwrap_or_else(|_| "postgres".to_string());
        let username = env::var("DB_USERNAME").unwrap_or_else(|_| "postgres".to_string());
        let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());

        let url = match connection.as_str() {
            "pgsql" | "postgres" | "postgresql" => {
                format!("postgres://{}:{}@{}:{}/{}", username, password, host, port, database)
            }
            _ => panic!("Unsupported database driver: {}", connection),
        };

        Self { connection, url }
    }
}

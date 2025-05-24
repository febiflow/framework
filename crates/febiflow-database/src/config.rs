use std::env;

pub struct DatabaseConfig {
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let driver = env::var("DB_CONNECTION").unwrap_or_else(|_| "pgsql".into());
        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".into());
        let port = env::var("DB_PORT").ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(5432);
        let database = env::var("DB_DATABASE").unwrap_or_else(|_| "postgres".into());
        let username = env::var("DB_USERNAME").unwrap_or_else(|_| "postgres".into());
        let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".into());

        let url = match driver.as_str() {
            "pgsql" | "postgres" | "postgresql" => {
                format!("postgres://{}:{}@{}:{}/{}", username, password, host, port, database)
            }
            other => panic!("Unsupported DB driver: {}", other),
        };

        Self { driver, host, port, database, username, password, url }
    }
}

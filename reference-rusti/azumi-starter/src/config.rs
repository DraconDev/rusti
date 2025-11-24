use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
    pub auth_service_url: String,
    pub admin_email: String,
    pub payment_service_url: Option<String>,
    pub payment_service_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .map_err(|_| ConfigError::Parse("SERVER_PORT"))?,
            database_url: env::var("DATABASE_URL")
                .map_err(|_| ConfigError::Missing("DATABASE_URL"))?,
            auth_service_url: env::var("AUTH_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            admin_email: env::var("ADMIN_EMAIL")
                .unwrap_or_else(|_| "admin@example.com".to_string()),
            payment_service_url: env::var("PAYMENT_SERVICE_URL").ok(),
            payment_service_api_key: env::var("PAYMENT_SERVICE_API_KEY").ok(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    Missing(&'static str),
    #[error("Failed to parse environment variable: {0}")]
    Parse(&'static str),
}

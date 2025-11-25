#[derive(Debug)]
pub struct Config {
    // local sqlite for rust testing
    pub db_path: String,
    // port for axum
    pub port: u16,
    // secret key for axum
    // pub secret_key: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            db_path: "./test.db".to_string(),
            port: 3000,
            // secret_key: "secret_key".to_string(),
        }
    }
}

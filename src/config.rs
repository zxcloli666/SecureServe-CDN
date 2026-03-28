use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub admin_token: String,
    pub storage_path: String,
    pub database_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),
            admin_token: env::var("ADMIN_TOKEN").unwrap_or_default(),
            storage_path: env::var("STORAGE_PATH").unwrap_or_else(|_| "./storage".into()),
            database_path: env::var("DATABASE_PATH").unwrap_or_else(|_| "./data/cdn.db".into()),
        }
    }

    pub fn admin_enabled(&self) -> bool {
        !self.admin_token.is_empty()
    }
}

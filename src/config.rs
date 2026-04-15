use std::env;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageMode {
    Local,
    S3,
}

#[derive(Clone)]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub endpoint: Option<String>,
    pub access_key: String,
    pub secret_key: String,
    pub force_path_style: bool,
    pub presign_ttl_secs: u64,
}

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub admin_token: String,
    pub storage_path: String,
    pub database_path: String,
    pub storage_mode: StorageMode,
    pub s3: Option<S3Config>,
}

impl Config {
    pub fn from_env() -> Self {
        let storage_mode = match env::var("STORAGE_MODE").as_deref() {
            Ok("s3") | Ok("S3") => StorageMode::S3,
            _ => StorageMode::Local,
        };

        let s3 = if storage_mode == StorageMode::S3 {
            Some(S3Config {
                bucket: env::var("S3_BUCKET").expect("S3_BUCKET required when STORAGE_MODE=s3"),
                region: env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".into()),
                endpoint: env::var("S3_ENDPOINT").ok().filter(|s| !s.is_empty()),
                access_key: env::var("S3_ACCESS_KEY")
                    .expect("S3_ACCESS_KEY required when STORAGE_MODE=s3"),
                secret_key: env::var("S3_SECRET_KEY")
                    .expect("S3_SECRET_KEY required when STORAGE_MODE=s3"),
                force_path_style: env::var("S3_FORCE_PATH_STYLE")
                    .map(|v| v == "true" || v == "1")
                    .unwrap_or(true),
                presign_ttl_secs: env::var("S3_PRESIGN_TTL")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(3600),
            })
        } else {
            None
        };

        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),
            admin_token: env::var("ADMIN_TOKEN").unwrap_or_default(),
            storage_path: env::var("STORAGE_PATH").unwrap_or_else(|_| "./storage".into()),
            database_path: env::var("DATABASE_PATH").unwrap_or_else(|_| "./data/cdn.db".into()),
            storage_mode,
            s3,
        }
    }

    pub fn admin_enabled(&self) -> bool {
        !self.admin_token.is_empty()
    }
}
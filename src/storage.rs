use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::{BehaviorVersion, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;

use crate::config::{Config, S3Config, StorageMode};
use crate::templates;

pub enum Storage {
    Local {
        root: PathBuf,
    },
    S3 {
        client: S3Client,
        bucket: String,
        presign_ttl: Duration,
    },
}

impl Storage {
    pub async fn from_config(cfg: &Config) -> std::io::Result<Self> {
        match cfg.storage_mode {
            StorageMode::Local => {
                fs::create_dir_all(&cfg.storage_path)?;
                Ok(Storage::Local {
                    root: PathBuf::from(&cfg.storage_path),
                })
            }
            StorageMode::S3 => {
                let s3cfg = cfg
                    .s3
                    .as_ref()
                    .expect("s3 config missing despite STORAGE_MODE=s3");
                let client = build_s3_client(s3cfg);
                Ok(Storage::S3 {
                    client,
                    bucket: s3cfg.bucket.clone(),
                    presign_ttl: Duration::from_secs(s3cfg.presign_ttl_secs),
                })
            }
        }
    }

    pub async fn put_file(
        &self,
        key: &str,
        src_path: &Path,
        content_type: &str,
    ) -> Result<(), String> {
        match self {
            Storage::Local { root } => {
                let dest = root.join(key);
                if let Some(parent) = dest.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)
                            .map_err(|e| format!("failed to create directory: {e}"))?;
                    }
                }
                match fs::rename(src_path, &dest) {
                    Ok(_) => Ok(()),
                    Err(_) => {
                        fs::copy(src_path, &dest)
                            .and_then(|_| fs::remove_file(src_path))
                            .map(|_| ())
                            .map_err(|e| format!("failed to save file: {e}"))
                    }
                }
            }
            Storage::S3 { client, bucket, .. } => {
                let body = ByteStream::from_path(src_path)
                    .await
                    .map_err(|e| format!("failed to read temp file: {e}"))?;
                client
                    .put_object()
                    .bucket(bucket)
                    .key(key)
                    .content_type(content_type)
                    .body(body)
                    .send()
                    .await
                    .map_err(|e| format!("s3 put error: {e}"))?;
                let _ = fs::remove_file(src_path);
                Ok(())
            }
        }
    }

    pub async fn serve(&self, req: &HttpRequest, key: &str) -> HttpResponse {
        match self {
            Storage::Local { root } => {
                let file_path = root.join(key);
                match NamedFile::open(&file_path) {
                    Ok(file) => file.into_response(req),
                    Err(_) => not_found(),
                }
            }
            Storage::S3 {
                client,
                bucket,
                presign_ttl,
            } => {
                match client.head_object().bucket(bucket).key(key).send().await {
                    Ok(_) => {}
                    Err(e) => {
                        let svc = e.into_service_error();
                        if svc.is_not_found() {
                            return not_found();
                        }
                        return HttpResponse::InternalServerError()
                            .body(format!("s3 head error: {svc}"));
                    }
                }

                let presign_cfg = match PresigningConfig::expires_in(*presign_ttl) {
                    Ok(c) => c,
                    Err(e) => {
                        return HttpResponse::InternalServerError()
                            .body(format!("presign config error: {e}"));
                    }
                };

                match client
                    .get_object()
                    .bucket(bucket)
                    .key(key)
                    .presigned(presign_cfg)
                    .await
                {
                    Ok(presigned) => HttpResponse::Found()
                        .insert_header(("location", presigned.uri()))
                        .insert_header(("cache-control", "private, no-store"))
                        .finish(),
                    Err(e) => HttpResponse::InternalServerError()
                        .body(format!("s3 presign error: {e}")),
                }
            }
        }
    }
}

fn build_s3_client(cfg: &S3Config) -> S3Client {
    let creds = Credentials::new(
        &cfg.access_key,
        &cfg.secret_key,
        None,
        None,
        "secureserve-cdn",
    );

    let mut builder = aws_sdk_s3::Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(cfg.region.clone()))
        .credentials_provider(creds)
        .force_path_style(cfg.force_path_style);

    if let Some(endpoint) = &cfg.endpoint {
        builder = builder.endpoint_url(endpoint);
    }

    S3Client::from_conf(builder.build())
}

fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(templates::NOT_FOUND_HTML)
}

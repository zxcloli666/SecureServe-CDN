use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, web, HttpRequest, HttpResponse};
use chrono::Utc;

use crate::auth;
use crate::config::Config;
use crate::db::Db;
use crate::models::{ApiError, PendingUpload, SignUploadRequest, SignUploadResponse};

const UPLOAD_TTL_SECS: i64 = 1800;

#[derive(MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
    pub token: Text<String>,
}

#[post("/api/sign-upload")]
pub async fn sign_upload(
    req: HttpRequest,
    db: web::Data<Arc<Db>>,
    body: web::Json<SignUploadRequest>,
) -> HttpResponse {
    let token = match auth::extract_bearer(&req) {
        Some(t) => t,
        None => return HttpResponse::Forbidden().json(ApiError {
            error: "missing authorization header".into(),
        }),
    };

    match db.validate_upload_token(&token) {
        Ok(true) => {}
        Ok(false) => return HttpResponse::Forbidden().json(ApiError {
            error: "invalid upload token".into(),
        }),
        Err(e) => return HttpResponse::InternalServerError().json(ApiError {
            error: format!("database error: {e}"),
        }),
    }

    if body.path.contains("..")
        || body.path.starts_with('/')
        || body.path.starts_with("api/")
        || body.path == "api"
    {
        return HttpResponse::BadRequest().json(ApiError {
            error: "invalid path".into(),
        });
    }

    let upload_token = auth::generate_token(48);
    let expires = Utc::now().timestamp() + UPLOAD_TTL_SECS;

    let pending = PendingUpload {
        id: auth::generate_token(16),
        upload_token: upload_token.clone(),
        path: body.path.clone(),
        size: body.size,
        content_type: body.content_type.clone(),
        expires,
    };

    if let Err(e) = db.create_pending_upload(&pending) {
        return HttpResponse::InternalServerError().json(ApiError {
            error: format!("database error: {e}"),
        });
    }

    HttpResponse::Ok().json(SignUploadResponse {
        token: upload_token,
        expires_at: expires,
    })
}

#[post("/api/upload")]
pub async fn upload_file(
    config: web::Data<Config>,
    db: web::Data<Arc<Db>>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> HttpResponse {
    let file = form.file;
    let token = form.token.into_inner();

    if file.size < 1 {
        return HttpResponse::NotAcceptable().json(ApiError {
            error: "empty file".into(),
        });
    }

    let pending = match db.find_pending_upload(&token) {
        Ok(Some(p)) => p,
        Ok(None) => return HttpResponse::NotAcceptable().json(ApiError {
            error: "token not found".into(),
        }),
        Err(e) => return HttpResponse::InternalServerError().json(ApiError {
            error: format!("database error: {e}"),
        }),
    };

    if pending.expires < Utc::now().timestamp() {
        let _ = db.delete_pending_upload(&token);
        return HttpResponse::Forbidden().json(ApiError {
            error: "upload session expired".into(),
        });
    }

    let content_type = file
        .content_type
        .as_ref()
        .map(|m| m.to_string())
        .unwrap_or_default();

    if pending.content_type != content_type {
        return HttpResponse::UnsupportedMediaType().json(ApiError {
            error: format!("expected content type '{}', got '{content_type}'", pending.content_type),
        });
    }

    if pending.size != file.size as i64 {
        return HttpResponse::UnsupportedMediaType().json(ApiError {
            error: format!("expected size {}, got {}", pending.size, file.size),
        });
    }

    let _ = db.delete_pending_upload(&token);

    let dest = PathBuf::from(&config.storage_path).join(&pending.path);

    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                return HttpResponse::InternalServerError().json(ApiError {
                    error: format!("failed to create directory: {e}"),
                });
            }
        }
    }

    let save_result = match file.file.persist(&dest) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Cross-device rename fails; fall back to copy + delete
            let temp_path = e.file.path().to_owned();
            fs::copy(&temp_path, &dest).and_then(|_| fs::remove_file(&temp_path))
        }
    };

    match save_result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "message": "uploaded", "path": pending.path })),
        Err(e) => HttpResponse::InternalServerError().json(ApiError {
            error: format!("failed to save file: {e}"),
        }),
    }
}

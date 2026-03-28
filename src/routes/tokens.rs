use std::sync::Arc;

use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use chrono::Utc;

use crate::auth;
use crate::config::Config;
use crate::db::Db;
use crate::models::{ApiError, ApiSuccess, CreateTokenRequest};
use crate::templates;

fn require_admin(req: &HttpRequest, config: &Config) -> Result<(), HttpResponse> {
    if !config.admin_enabled() {
        return Err(HttpResponse::Forbidden().json(ApiError {
            error: "admin is disabled".into(),
        }));
    }

    let token = auth::extract_bearer(req).ok_or_else(|| {
        HttpResponse::Unauthorized().json(ApiError {
            error: "missing authorization header".into(),
        })
    })?;

    if token != config.admin_token {
        return Err(HttpResponse::Forbidden().json(ApiError {
            error: "invalid admin token".into(),
        }));
    }

    Ok(())
}

#[get("/api/tokens")]
pub async fn list_tokens(
    req: HttpRequest,
    config: web::Data<Config>,
    db: web::Data<Arc<Db>>,
) -> HttpResponse {
    if let Err(resp) = require_admin(&req, &config) {
        return resp;
    }

    match db.list_upload_tokens() {
        Ok(tokens) => HttpResponse::Ok().json(tokens),
        Err(e) => HttpResponse::InternalServerError().json(ApiError {
            error: format!("database error: {e}"),
        }),
    }
}

#[post("/api/tokens")]
pub async fn create_token(
    req: HttpRequest,
    config: web::Data<Config>,
    db: web::Data<Arc<Db>>,
    body: web::Json<CreateTokenRequest>,
) -> HttpResponse {
    if let Err(resp) = require_admin(&req, &config) {
        return resp;
    }

    let id = auth::generate_token(16);
    let token = auth::generate_token(48);
    let now = Utc::now().timestamp();

    match db.create_upload_token(&id, &body.name, &token, now) {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "id": id,
            "name": body.name,
            "token": token,
            "created_at": now,
        })),
        Err(e) => HttpResponse::InternalServerError().json(ApiError {
            error: format!("database error: {e}"),
        }),
    }
}

#[delete("/api/tokens/{id}")]
pub async fn delete_token(
    req: HttpRequest,
    config: web::Data<Config>,
    db: web::Data<Arc<Db>>,
    path: web::Path<String>,
) -> HttpResponse {
    if let Err(resp) = require_admin(&req, &config) {
        return resp;
    }

    let id = path.into_inner();
    match db.delete_upload_token(&id) {
        Ok(true) => HttpResponse::Ok().json(ApiSuccess {
            message: "token deleted".into(),
        }),
        Ok(false) => HttpResponse::NotFound().json(ApiError {
            error: "token not found".into(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiError {
            error: format!("database error: {e}"),
        }),
    }
}

#[post("/api/auth/verify")]
pub async fn verify_admin(req: HttpRequest, config: web::Data<Config>) -> HttpResponse {
    if let Err(resp) = require_admin(&req, &config) {
        return resp;
    }
    HttpResponse::Ok().json(ApiSuccess {
        message: "authenticated".into(),
    })
}

#[get("/admin")]
pub async fn admin_page(config: web::Data<Config>) -> HttpResponse {
    if !config.admin_enabled() {
        return HttpResponse::Forbidden()
            .content_type("text/html; charset=utf-8")
            .body(templates::NOT_FOUND_HTML);
    }

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(templates::ADMIN_HTML)
}

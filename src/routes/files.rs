use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::config::Config;
use crate::templates;

pub async fn serve(req: HttpRequest, config: web::Data<Config>) -> HttpResponse {
    let path = req.match_info().query("path");

    if path.is_empty() {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(templates::INDEX_HTML);
    }

    if path.contains("..") {
        return HttpResponse::Forbidden().body("forbidden");
    }

    let file_path = PathBuf::from(&config.storage_path).join(path);

    match NamedFile::open(&file_path) {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::NotFound()
            .content_type("text/html; charset=utf-8")
            .body(templates::NOT_FOUND_HTML),
    }
}

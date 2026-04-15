use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};

use crate::storage::Storage;
use crate::templates;

pub async fn serve(req: HttpRequest, storage: web::Data<Arc<Storage>>) -> HttpResponse {
    let path = req.match_info().query("path").to_string();

    if path.is_empty() {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(templates::INDEX_HTML);
    }

    if path.contains("..") {
        return HttpResponse::Forbidden().body("forbidden");
    }

    storage.serve(&req, &path).await
}

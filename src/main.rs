use std::fs;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use log::info;

mod auth;
mod cleanup;
mod config;
mod db;
mod models;
mod routes;
mod templates;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cfg = config::Config::from_env();

    fs::create_dir_all(&cfg.storage_path).expect("failed to create storage directory");

    info!("opening database at {}", cfg.database_path);
    let database = Arc::new(db::Db::open(&cfg.database_path).expect("failed to open database"));

    cleanup::spawn_cleanup_task(Arc::clone(&database));

    let bind_addr = format!("0.0.0.0:{}", cfg.port);
    info!("starting server on {}", bind_addr);

    if cfg.admin_enabled() {
        info!("admin panel enabled at /admin");
    } else {
        info!("admin panel disabled (ADMIN_TOKEN is empty)");
    }

    let db_data = web::Data::new(database);
    let cfg_data = web::Data::new(cfg.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(db_data.clone())
            .app_data(cfg_data.clone())
            .service(routes::tokens::admin_page)
            .service(routes::tokens::verify_admin)
            .service(routes::tokens::list_tokens)
            .service(routes::tokens::create_token)
            .service(routes::tokens::delete_token)
            .service(routes::upload::sign_upload)
            .service(routes::upload::upload_file)
            .route("/{path:.*}", web::get().to(routes::files::serve))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

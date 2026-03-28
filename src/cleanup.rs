use std::sync::Arc;
use std::time::Duration;

use log::info;
use tokio::time;

use crate::db::Db;

pub fn spawn_cleanup_task(db: Arc<Db>) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let now = chrono::Utc::now().timestamp();
            match db.cleanup_expired(now) {
                Ok(count) if count > 0 => info!("cleaned up {} expired pending uploads", count),
                Err(e) => log::error!("cleanup error: {}", e),
                _ => {}
            }
        }
    });
}

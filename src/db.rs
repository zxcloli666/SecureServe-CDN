use std::fs;
use std::path::Path;
use std::sync::Mutex;

use rusqlite::{params, Connection};

use crate::models::{PendingUpload, UploadToken};

pub struct Db(Mutex<Connection>);

impl Db {
    pub fn open(path: &str) -> Result<Self, rusqlite::Error> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(path)?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA busy_timeout = 10000;
             PRAGMA foreign_keys = ON;",
        )?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS upload_tokens (
                id         TEXT PRIMARY KEY,
                name       TEXT NOT NULL,
                token      TEXT UNIQUE NOT NULL,
                created_at INTEGER NOT NULL
             );

             CREATE TABLE IF NOT EXISTS pending_uploads (
                id           TEXT PRIMARY KEY,
                upload_token TEXT NOT NULL,
                path         TEXT NOT NULL,
                size         INTEGER NOT NULL,
                content_type TEXT NOT NULL,
                expires      INTEGER NOT NULL
             );

             CREATE INDEX IF NOT EXISTS idx_pending_expires
                ON pending_uploads(expires);

             CREATE INDEX IF NOT EXISTS idx_upload_tokens_token
                ON upload_tokens(token);",
        )?;

        Ok(Self(Mutex::new(conn)))
    }

    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.0.lock().expect("database lock poisoned")
    }

    // --- Upload Tokens ---

    pub fn create_upload_token(&self, id: &str, name: &str, token: &str, created_at: i64) -> Result<(), rusqlite::Error> {
        self.conn().execute(
            "INSERT INTO upload_tokens (id, name, token, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![id, name, token, created_at],
        )?;
        Ok(())
    }

    pub fn list_upload_tokens(&self) -> Result<Vec<UploadToken>, rusqlite::Error> {
        let conn = self.conn();
        let mut stmt = conn.prepare("SELECT id, name, token, created_at FROM upload_tokens ORDER BY created_at DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok(UploadToken {
                id: row.get(0)?,
                name: row.get(1)?,
                token: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        rows.collect()
    }

    pub fn delete_upload_token(&self, id: &str) -> Result<bool, rusqlite::Error> {
        let count = self.conn().execute("DELETE FROM upload_tokens WHERE id = ?1", params![id])?;
        Ok(count > 0)
    }

    pub fn validate_upload_token(&self, token: &str) -> Result<bool, rusqlite::Error> {
        let conn = self.conn();
        let mut stmt = conn.prepare("SELECT 1 FROM upload_tokens WHERE token = ?1 LIMIT 1")?;
        let exists = stmt.exists(params![token])?;
        Ok(exists)
    }

    // --- Pending Uploads ---

    pub fn create_pending_upload(&self, upload: &PendingUpload) -> Result<(), rusqlite::Error> {
        self.conn().execute(
            "INSERT INTO pending_uploads (id, upload_token, path, size, content_type, expires)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                upload.id,
                upload.upload_token,
                upload.path,
                upload.size,
                upload.content_type,
                upload.expires,
            ],
        )?;
        Ok(())
    }

    pub fn find_pending_upload(&self, token: &str) -> Result<Option<PendingUpload>, rusqlite::Error> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, upload_token, path, size, content_type, expires
             FROM pending_uploads WHERE upload_token = ?1 LIMIT 1",
        )?;
        let mut rows = stmt.query_map(params![token], |row| {
            Ok(PendingUpload {
                id: row.get(0)?,
                upload_token: row.get(1)?,
                path: row.get(2)?,
                size: row.get(3)?,
                content_type: row.get(4)?,
                expires: row.get(5)?,
            })
        })?;
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    pub fn delete_pending_upload(&self, token: &str) -> Result<(), rusqlite::Error> {
        self.conn().execute("DELETE FROM pending_uploads WHERE upload_token = ?1", params![token])?;
        Ok(())
    }

    pub fn cleanup_expired(&self, now: i64) -> Result<usize, rusqlite::Error> {
        let count = self.conn().execute("DELETE FROM pending_uploads WHERE expires <= ?1", params![now])?;
        Ok(count)
    }
}

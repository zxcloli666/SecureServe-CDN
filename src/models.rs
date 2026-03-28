use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadToken {
    pub id: String,
    pub name: String,
    pub token: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingUpload {
    pub id: String,
    pub upload_token: String,
    pub path: String,
    pub size: i64,
    pub content_type: String,
    pub expires: i64,
}

#[derive(Debug, Deserialize)]
pub struct SignUploadRequest {
    pub path: String,
    pub size: i64,
    pub content_type: String,
}

#[derive(Debug, Serialize)]
pub struct SignUploadResponse {
    pub token: String,
    pub expires_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct ApiSuccess {
    pub message: String,
}

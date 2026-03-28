use actix_web::HttpRequest;
use rand::{distributions::Alphanumeric, Rng};

pub fn extract_bearer(req: &HttpRequest) -> Option<String> {
    let header = req.headers().get("authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ").unwrap_or(header);
    Some(token.to_string())
}

pub fn generate_token(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

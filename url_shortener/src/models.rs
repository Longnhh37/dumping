use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    pub short_code: String,
    pub long_url: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UrlRecord {
    pub long_url: String,
}

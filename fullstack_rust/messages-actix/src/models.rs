use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct IndexResponse {
    pub request_count: usize,
    pub server_id: usize,
    pub message: Vec<String>,
}

#[derive(Deserialize)]
pub struct PostInput {
    pub message: String,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub server_id: usize,
    pub request_count: usize,
    pub message: String,
}

#[derive(Serialize)]
pub struct PostError {
    pub server_id: usize,
    pub request_count: usize,
    pub error: String,
}

#[derive(Serialize)]
pub struct LookUpResponse {
    pub server_id: usize,
    pub request_count: usize,
    pub result: Option<String>,
}

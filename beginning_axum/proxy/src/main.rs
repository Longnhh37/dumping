use axum::{Json, Router, body::Bytes, extract::State, http::StatusCode, routing::post};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Cache = Arc<Mutex<HashMap<String, Bytes>>>;

#[derive(Deserialize)]
struct Data {
    breed: String,
    num_pics: Option<i32>,
}

async fn proxy_handler(State(state): State<Cache>, Json(data): Json<Data>) -> (StatusCode, Bytes) {
    if let Some(body) = state.lock().unwrap().get(&data.breed).cloned() {
        println!("{} cache hit", &data.breed);
        return (StatusCode::OK, body);
    }

    println!("{} cache miss", &data.breed);
    let mut url = format!("https://dog.ceo/api/breed/{}/images/random", &data.breed);
    if let Some(num_pics) = data.num_pics {
        url.push_str(&format!("/{}", num_pics));
    }

    let client = Client::new();
    let res = client.get(url).send().await.unwrap();
    let code = res.status().as_u16();
    let body = res.bytes().await.unwrap();
    let mut cache = state.lock().unwrap();
    cache.insert(data.breed, body.clone());

    (StatusCode::from_u16(code).unwrap(), body)
}

#[tokio::main]
async fn main() {
    let state: Cache = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/", post(proxy_handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

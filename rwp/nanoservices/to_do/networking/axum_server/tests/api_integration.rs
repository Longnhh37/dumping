use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, Response, StatusCode},
};
use serde_json::{Value, json};
use tower::ServiceExt;

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::token::HeaderToken;
use to_do_axum_server::api::basic_actions::basic_actions_router;
use to_do_dal::to_do_items::{
    schema::ToDoItem,
    transactions::{
        create::SaveOne,
        delete::DeleteOne,
        get::{GetAll, GetByName},
        update::UpdateOne,
    },
};

fn auth_token() -> String {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        unsafe { std::env::set_var("JWT_SECRET", "test-secret") };
    });
    HeaderToken {
        unique_id: "test-user".into(),
    }
    .encode()
    .unwrap()
}

#[derive(Clone, Default)]
struct MemoryStore(Arc<Mutex<HashMap<String, ToDoItem>>>);

impl GetAll for MemoryStore {
    async fn get_all(&self) -> Result<Vec<ToDoItem>, NanoServiceError> {
        Ok(self.0.lock().unwrap().values().cloned().collect())
    }
}

impl GetByName for MemoryStore {
    async fn get_by_name(&self, name: &str) -> Result<ToDoItem, NanoServiceError> {
        self.0.lock().unwrap().get(name).cloned().ok_or_else(|| {
            NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound)
        })
    }
}

impl SaveOne for MemoryStore {
    async fn save_one(&self, item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
        self.0
            .lock()
            .unwrap()
            .insert(item.name.clone(), item.clone());
        Ok(item)
    }
}

impl DeleteOne for MemoryStore {
    async fn delete_one(&self, name: &str) -> Result<ToDoItem, NanoServiceError> {
        self.0.lock().unwrap().remove(name).ok_or_else(|| {
            NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound)
        })
    }
}

impl UpdateOne for MemoryStore {
    async fn update_one(&self, item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
        let mut store = self.0.lock().unwrap();
        if !store.contains_key(&item.name) {
            return Err(NanoServiceError::new(
                "Item not found",
                NanoServiceErrorStatus::NotFound,
            ));
        }
        store.insert(item.name.clone(), item.clone());
        Ok(item)
    }
}

async fn send(
    app: &Router,
    method: Method,
    path: &str,
    body: Option<Value>,
    token: Option<&str>,
) -> Response<Body> {
    let mut request = Request::builder().method(method).uri(path);
    if body.is_some() {
        request = request.header("content-type", "application/json");
    }
    if let Some(token) = token {
        request = request.header("token", token);
    }
    let body = body.map_or_else(Body::empty, |body| Body::from(body.to_string()));
    app.clone()
        .oneshot(request.body(body).unwrap())
        .await
        .unwrap()
}

async fn response_json(response: Response<Body>) -> Value {
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn api_routes_cover_item_lifecycle_and_auth() {
    let app = basic_actions_router(MemoryStore::default());
    let name = "api-wire-integration";
    let token = auth_token();

    let created = send(
        &app,
        Method::POST,
        "/api/v1/create",
        Some(json!({"name": name, "status": "PENDING"})),
        Some(&token),
    )
    .await;
    assert_eq!(created.status(), StatusCode::CREATED);
    let created = response_json(created).await;
    assert_eq!(created["pending"][0]["name"], name);
    assert_eq!(created["pending"][0]["status"], "PENDING");

    let fetched = send(
        &app,
        Method::GET,
        "/api/v1/get/api-wire-integration",
        None,
        Some(&token),
    )
    .await;
    assert_eq!(fetched.status(), StatusCode::OK);
    assert_eq!(response_json(fetched).await["name"], name);

    let missing = send(&app, Method::GET, "/api/v1/get/missing", None, Some(&token)).await;
    assert_eq!(missing.status(), StatusCode::NOT_FOUND);

    let missing_update = send(
        &app,
        Method::PATCH,
        "/api/v1/update",
        Some(json!({"name": "missing", "status": "DONE"})),
        Some(&token),
    )
    .await;
    assert_eq!(missing_update.status(), StatusCode::NOT_FOUND);

    let missing_delete = send(
        &app,
        Method::DELETE,
        "/api/v1/delete/missing",
        None,
        Some(&token),
    )
    .await;
    assert_eq!(missing_delete.status(), StatusCode::NOT_FOUND);

    let unauthorized = send(
        &app,
        Method::PATCH,
        "/api/v1/update",
        Some(json!({"name": name, "status": "DONE"})),
        None,
    )
    .await;
    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

    let updated = send(
        &app,
        Method::PATCH,
        "/api/v1/update",
        Some(json!({"name": name, "status": "DONE"})),
        Some(&token),
    )
    .await;
    assert_eq!(updated.status(), StatusCode::OK);
    assert_eq!(response_json(updated).await["done"][0]["name"], name);

    let deleted = send(
        &app,
        Method::DELETE,
        "/api/v1/delete/api-wire-integration",
        None,
        Some(&token),
    )
    .await;
    assert_eq!(deleted.status(), StatusCode::OK);
    let deleted = response_json(deleted).await;
    assert!(deleted["pending"].as_array().unwrap().is_empty());
    assert!(deleted["done"].as_array().unwrap().is_empty());
}

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
    schema::{NewToDoItem, ToDoItem},
    transactions::{
        create::SaveOne,
        delete::DeleteOne,
        get::{GetAll, GetById},
        update::UpdateOne,
    },
};

fn auth_token(unique_id: &str) -> String {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        unsafe { std::env::set_var("JWT_SECRET", "test-secret") };
    });
    HeaderToken {
        unique_id: unique_id.into(),
    }
    .encode()
    .unwrap()
}

#[derive(Clone, Default)]
struct MemoryStore(Arc<Mutex<HashMap<(String, i32), ToDoItem>>>);

impl GetAll for MemoryStore {
    async fn get_all(&self, user_id: &str) -> Result<Vec<ToDoItem>, NanoServiceError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .iter()
            .filter(|((owner, _), _)| owner == user_id)
            .map(|(_, item)| item.clone())
            .collect())
    }
}

impl GetById for MemoryStore {
    async fn get_by_id(&self, id: i32, user_id: &str) -> Result<ToDoItem, NanoServiceError> {
        self.0
            .lock()
            .unwrap()
            .get(&(user_id.to_owned(), id))
            .cloned()
            .ok_or_else(|| {
                NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound)
            })
    }
}

impl SaveOne for MemoryStore {
    async fn save_one(
        &self,
        item: NewToDoItem,
        user_id: &str,
    ) -> Result<ToDoItem, NanoServiceError> {
        let mut store = self.0.lock().unwrap();
        let id = store.values().map(|item| item.id).max().unwrap_or(0) + 1;
        let item = ToDoItem {
            id,
            title: item.title,
            status: item.status,
        };
        store.insert((user_id.to_owned(), id), item.clone());
        Ok(item)
    }
}

impl DeleteOne for MemoryStore {
    async fn delete_one(&self, id: i32, user_id: &str) -> Result<ToDoItem, NanoServiceError> {
        self.0
            .lock()
            .unwrap()
            .remove(&(user_id.to_owned(), id))
            .ok_or_else(|| {
                NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound)
            })
    }
}

impl UpdateOne for MemoryStore {
    async fn update_one(
        &self,
        item: ToDoItem,
        user_id: &str,
    ) -> Result<ToDoItem, NanoServiceError> {
        let mut store = self.0.lock().unwrap();
        let key = (user_id.to_owned(), item.id);
        if !store.contains_key(&key) {
            return Err(NanoServiceError::new(
                "Item not found",
                NanoServiceErrorStatus::NotFound,
            ));
        }
        store.insert(key, item.clone());
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
async fn api_routes_cover_item_lifecycle_auth_and_user_isolation() {
    let app = basic_actions_router(MemoryStore::default());
    let token = auth_token("test-user");
    let other_token = auth_token("other-user");

    let created = send(
        &app,
        Method::POST,
        "/api/v1/create",
        Some(json!({"title": "api-wire-integration", "status": "PENDING"})),
        Some(&token),
    )
    .await;
    assert_eq!(created.status(), StatusCode::CREATED);
    let created = response_json(created).await;
    assert_eq!(created["pending"][0]["title"], "api-wire-integration");
    assert_eq!(created["pending"][0]["status"], "PENDING");
    let id = created["pending"][0]["id"].as_i64().unwrap();

    let fetched = send(
        &app,
        Method::GET,
        &format!("/api/v1/get/{id}"),
        None,
        Some(&token),
    )
    .await;
    assert_eq!(fetched.status(), StatusCode::OK);
    let fetched = response_json(fetched).await;
    assert_eq!(fetched["id"], id);
    assert_eq!(fetched["title"], "api-wire-integration");

    let other_users_items =
        response_json(send(&app, Method::GET, "/api/v1/get", None, Some(&other_token)).await).await;
    assert!(other_users_items["pending"].as_array().unwrap().is_empty());
    let other_user_fetch = send(
        &app,
        Method::GET,
        &format!("/api/v1/get/{id}"),
        None,
        Some(&other_token),
    )
    .await;
    assert_eq!(other_user_fetch.status(), StatusCode::NOT_FOUND);

    let missing = send(&app, Method::GET, "/api/v1/get/99999", None, Some(&token)).await;
    assert_eq!(missing.status(), StatusCode::NOT_FOUND);

    let missing_update = send(
        &app,
        Method::PATCH,
        "/api/v1/update",
        Some(json!({"id": 99999, "title": "missing", "status": "DONE"})),
        Some(&token),
    )
    .await;
    assert_eq!(missing_update.status(), StatusCode::NOT_FOUND);

    let missing_delete = send(
        &app,
        Method::DELETE,
        "/api/v1/delete/99999",
        None,
        Some(&token),
    )
    .await;
    assert_eq!(missing_delete.status(), StatusCode::NOT_FOUND);

    let unauthorized = send(
        &app,
        Method::PATCH,
        "/api/v1/update",
        Some(json!({"id": id, "title": "api-wire-integration", "status": "DONE"})),
        None,
    )
    .await;
    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

    let updated = send(
        &app,
        Method::PATCH,
        "/api/v1/update",
        Some(json!({"id": id, "title": "api-wire-integration", "status": "DONE"})),
        Some(&token),
    )
    .await;
    assert_eq!(updated.status(), StatusCode::OK);
    assert_eq!(response_json(updated).await["done"][0]["id"], id);

    let deleted = send(
        &app,
        Method::DELETE,
        &format!("/api/v1/delete/{id}"),
        None,
        Some(&token),
    )
    .await;
    assert_eq!(deleted.status(), StatusCode::OK);
    let deleted = response_json(deleted).await;
    assert!(deleted["pending"].as_array().unwrap().is_empty());
    assert!(deleted["done"].as_array().unwrap().is_empty());
}

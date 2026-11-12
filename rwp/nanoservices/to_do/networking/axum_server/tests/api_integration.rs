use std::{
    collections::HashMap,
    future::Future,
    sync::{Mutex, OnceLock},
};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, Response, StatusCode},
};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde_json::{Value, json};
use to_do_axum_server::api::basic_actions::basic_actions_router;
use to_do_dal::to_do_items::{
    schema::{NewToDoItem, ToDoItem},
    transactions::{
        create::SaveOne,
        delete::DeleteOne,
        get::{GetAll, GetByName},
        update::UpdateOne,
    },
};
use tower::ServiceExt;

struct MemoryStore;

static ITEMS: OnceLock<Mutex<HashMap<String, ToDoItem>>> = OnceLock::new();

fn items() -> &'static Mutex<HashMap<String, ToDoItem>> {
    ITEMS.get_or_init(|| Mutex::new(HashMap::new()))
}

impl GetAll for MemoryStore {
    fn get_all() -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send {
        async { Ok(items().lock().unwrap().values().cloned().collect()) }
    }
}

impl GetByName for MemoryStore {
    fn get_by_name(
        name: String,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        async move {
            items().lock().unwrap().get(&name).cloned().ok_or_else(|| {
                NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound)
            })
        }
    }
}

impl SaveOne for MemoryStore {
    fn save_one(
        item: NewToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        async move {
            let item = ToDoItem {
                name: item.name,
                status: item.status.to_string(),
            };
            items()
                .lock()
                .unwrap()
                .insert(item.name.clone(), item.clone());
            Ok(item)
        }
    }
}

impl DeleteOne for MemoryStore {
    fn delete_one(name: String) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        async move {
            items().lock().unwrap().remove(&name).ok_or_else(|| {
                NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound)
            })
        }
    }
}

impl UpdateOne for MemoryStore {
    fn update_one(
        item: ToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        async move {
            let mut store = items().lock().unwrap();
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
    items().lock().unwrap().clear();
    let app = basic_actions_router::<MemoryStore>();
    let name = "api-wire-integration";

    let created = send(
        &app,
        Method::POST,
        "/api/v1/create",
        Some(json!({"name": name, "status": "PENDING"})),
        None,
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
        None,
    )
    .await;
    assert_eq!(fetched.status(), StatusCode::OK);
    assert_eq!(response_json(fetched).await["name"], name);

    let missing = send(&app, Method::GET, "/api/v1/get/missing", None, None).await;
    assert_eq!(missing.status(), StatusCode::NOT_FOUND);

    let missing_update = send(
        &app,
        Method::PATCH,
        "/api/v1/update",
        Some(json!({"name": "missing", "status": "DONE"})),
        Some("integration-test-token"),
    )
    .await;
    assert_eq!(missing_update.status(), StatusCode::NOT_FOUND);

    let missing_delete = send(&app, Method::DELETE, "/api/v1/delete/missing", None, None).await;
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
        Some("integration-test-token"),
    )
    .await;
    assert_eq!(updated.status(), StatusCode::OK);
    assert_eq!(response_json(updated).await["done"][0]["name"], name);

    let deleted = send(
        &app,
        Method::DELETE,
        "/api/v1/delete/api-wire-integration",
        None,
        None,
    )
    .await;
    assert_eq!(deleted.status(), StatusCode::OK);
    let deleted = response_json(deleted).await;
    assert!(deleted["pending"].as_array().unwrap().is_empty());
    assert!(deleted["done"].as_array().unwrap().is_empty());
}

use std::{ffi::OsStr, path::Path};

use axum::{
    Router,
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use rust_embed::RustEmbed;
use tower_http::cors::{Any, CorsLayer};
use tracing::Level;

use auth_dal::{
    migrations::run_migrations as run_auth_migrations,
    users::descriptors::SqlxPostgresDescriptor as AuthPostgresDescriptor,
};
use auth_server::api::RedisSessionDescriptor;
use auth_server::api::router as auth_router;
use glue::{
    db::pool_from_env,
    log::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer, logger::init_logger},
};
use to_do_axum_server::api as todo_api;
use to_do_dal::migrations::run_migrations as run_todo_migrations;
use to_do_dal::to_do_items::descriptors::SqlxPostgresDescriptor;

const INDEX_HTML: &str = include_str!("../index.html");
const NOT_FOUND: (StatusCode, &str) = (StatusCode::NOT_FOUND, "404 Not Found");

#[derive(RustEmbed)]
#[folder = "../frontend/public"]
struct FrontendAssets;

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

fn serve_frontend_asset(path: &str) -> Response {
    let Some(file) = Path::new(path).file_name().and_then(OsStr::to_str) else {
        return NOT_FOUND.into_response();
    };
    let Some(asset) = FrontendAssets::get(file) else {
        return NOT_FOUND.into_response();
    };

    let mime = mime_guess::from_path(file)
        .first_or_octet_stream()
        .to_string();

    (
        [
            (header::CONTENT_TYPE, mime),
            (header::CACHE_CONTROL, "no-cache".to_owned()),
        ],
        asset.data,
    )
        .into_response()
}

async fn catch_all(uri: Uri) -> Response {
    let path = uri.path();

    if path.starts_with("/api/") {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    if path.contains("frontend/public") {
        return serve_frontend_asset(path);
    }

    let wants_html = mime_guess::from_path(path)
        .first_raw()
        .is_none_or(|mime| mime == "text/html");

    if wants_html {
        Html(INDEX_HTML).into_response()
    } else {
        serve_frontend_asset(path)
    }
}

fn app(todo_store: SqlxPostgresDescriptor, auth_store: AuthPostgresDescriptor) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(index))
        .merge(todo_api::basic_actions::basic_actions_router_core_postgres(
            todo_store,
            auth_store.clone(),
        ))
        .merge(auth_router(auth_server::api::AppState::new(
            auth_store.clone(),
            RedisSessionDescriptor::new(auth_store),
        )))
        .fallback(catch_all)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_logger();
    let pool = pool_from_env("TO_DO")?;
    run_todo_migrations(&pool).await?;
    let auth_pool = pool_from_env("AUTH")?;
    run_auth_migrations(&auth_pool).await?;

    let host = std::env::var("INGRESS_WEB_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("INGRESS_WEB_PORT").unwrap_or_else(|_| "8001".to_string());
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    tracing::info!(address = %listener.local_addr()?, "ingress server listening");

    let todo_store = SqlxPostgresDescriptor::new(pool);
    let auth_store = AuthPostgresDescriptor::new(auth_pool);
    axum::serve(listener, app(todo_store, auth_store)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::{AuthPostgresDescriptor, app};
    use glue::{db::create_pool, token::HeaderToken};
    use to_do_dal::to_do_items::descriptors::SqlxPostgresDescriptor;

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

    fn test_app() -> axum::Router {
        let todo_pool = create_pool("postgres://test:test@127.0.0.1/todo_test", 1).unwrap();
        let auth_pool = create_pool("postgres://test:test@127.0.0.1/auth_test", 1).unwrap();
        app(
            SqlxPostgresDescriptor::new(todo_pool),
            AuthPostgresDescriptor::new(auth_pool),
        )
    }

    #[tokio::test]
    async fn serves_the_frontend_shell_and_assets() {
        let response = test_app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let response = test_app()
            .oneshot(
                Request::builder()
                    .uri("/bundle.css")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn routes_api_requests_and_returns_404_for_unknown_api_paths() {
        let token = auth_token();
        let invalid_json = test_app()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/create")
                    .header("content-type", "application/json")
                    .header("token", &token)
                    .body(Body::from("not-json"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(invalid_json.status(), StatusCode::BAD_REQUEST);

        let missing_route = test_app()
            .oneshot(
                Request::builder()
                    .uri("/api/v1/unknown")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(missing_route.status(), StatusCode::NOT_FOUND);
    }
}

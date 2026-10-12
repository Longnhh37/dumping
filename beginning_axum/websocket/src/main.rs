use std::sync::Arc;

use axum::{
    Router,
    extract::{
        Request, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::{StatusCode, header::AUTHORIZATION},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use subtle::ConstantTimeEq;
use tokio::sync::broadcast::{self, Sender, error::RecvError};

#[derive(Clone)]
struct AppState {
    tx: Sender<Message>,
    api_token: Arc<str>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(32);

    let api_token = std::env::var("API_TOKEN").expect("API_TOKEN not set");
    let state = AppState {
        tx,
        api_token: api_token.into(),
    };
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route_layer(middleware::from_fn_with_state(state.clone(), authenticate))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn authenticate(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let valid: bool = token.as_bytes().ct_eq(state.api_token.as_bytes()).into();
    if valid {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn ws_handler(ws: WebSocketUpgrade, State(app): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, app))
}

async fn handle_socket(socket: WebSocket, app: AppState) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let mut rx = app.tx.subscribe();
    let tx = app.tx;

    // broadcast channe -> client
    let mut send_task = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    if ws_tx.send(msg).await.is_err() {
                        break;
                    }
                }
                Err(RecvError::Lagged(n)) => eprintln!("client missed {n} message"),
                Err(RecvError::Closed) => break,
            }
        }
    });

    // client -> broadcast channel
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            match msg {
                Message::Close(_) => break,
                Message::Text(_) => {
                    let _ = tx.send(msg);
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}

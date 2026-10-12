use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast::{self, Sender, error::RecvError};

#[derive(Clone)]
struct AppState {
    tx: Sender<Message>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(32);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(AppState { tx });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

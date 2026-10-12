use axum::{
    extract::State,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures_util::stream::StreamExt;
use serde_json::json;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use crate::entities::chat::{self, Entity as ChatEntity, Model as Chat};

pub async fn subscribe(State(queue): State<broadcast::Sender<Chat>>) -> impl IntoResponse {
    let stream = BroadcastStream::new(queue.subscribe()).map(|msg| match msg {
        Ok(msg) => Ok(Event::default()
            .event("message")
            .data(json!(msg).to_string())),
        Err(e) => Err(e),
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn send(State(queue): State<broadcast::Sender<Chat>>, new_msg: String) -> &'static str {
    let _ = queue
        .send(serde_json::from_str(&new_msg))
        .expect("Invalid chat JSON");
    "Message sent"
}

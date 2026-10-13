use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures_util::stream::StreamExt;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use crate::entities::chat::{Column, Entity as ChatEntity, Model as Chat};

pub async fn subscribe(State(queue): State<broadcast::Sender<Chat>>) -> impl IntoResponse {
    let stream = BroadcastStream::new(queue.subscribe()).map(|msg| match msg {
        Ok(msg) => Ok(Event::default()
            .event("message")
            .data(json!(msg).to_string())),
        Err(e) => Err(e),
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn send(
    State(queue): State<broadcast::Sender<Chat>>,
    new_msg: String,
) -> Result<&'static str, StatusCode> {
    let message = serde_json::from_str(&new_msg).map_err(|_| StatusCode::BAD_REQUEST)?;
    let _ = queue.send(message);
    Ok("Message sent")
}

pub async fn get_chat(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Chat>> {
    let room_id = params.get("room_id").unwrap();

    Json(
        ChatEntity::find()
            .filter(Column::RoomId.eq(room_id.parse::<i32>().unwrap()))
            .all(&conn)
            .await
            .unwrap(),
    )
}

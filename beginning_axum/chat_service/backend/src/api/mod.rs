pub mod chat;
pub mod chat_room;
pub mod state;
pub mod user;

use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::entities::{
    chat::{self, Entity as ChatEntity},
    room::{ActiveModel, Column, Entity as RoomEntity, Model},
};

pub async fn get_room(
    State(conn): State<DatabaseConnection>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<NewRoom>> {
    let mut condition = Condition::all();
    if let Some(id) = params.get("id") {
        condition = condition.add(Column::Id.eq(id.parse::<i32>().unwrap()));
    }
}

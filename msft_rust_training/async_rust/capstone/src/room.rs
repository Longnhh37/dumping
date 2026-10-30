use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

type RoomMap = Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>;

fn get_or_create_room(
    rooms: &mut HashMap<String, broadcast::Sender<String>>,
    name: &str,
) -> broadcast::Sender<String> {
    rooms
        .entry(name.to_string())
        .or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        })
        .clone()
}

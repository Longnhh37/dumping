use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{RwLock, broadcast};

use crate::message::Message;

pub type RoomTx = broadcast::Sender<Message>;
pub type RoomRx = broadcast::Receiver<Message>;

#[derive(Clone)]
pub struct Rooms {
    inner: Arc<RwLock<HashMap<String, RoomTx>>>,
    capacity: usize,
}

impl Rooms {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::default(),
            capacity,
        }
    }

    pub async fn join(&self, name: &str) -> (RoomTx, RoomRx) {
        let mut map = self.inner.write().await;
        let tx = map
            .entry(name.to_owned())
            .or_insert_with(|| broadcast::channel(self.capacity).0)
            .clone();
        let rx = tx.subscribe();
        (tx, rx)
    }

    pub async fn leave(&self, name: &str) {
        let mut map = self.inner.write().await;
        if map.get(name).is_some_and(|tx| tx.receiver_count() == 0) {
            map.remove(name);
        }
    }

    pub async fn list(&self) -> Vec<(String, usize)> {
        let map = self.inner.read().await;
        let mut rooms: Vec<_> = map
            .iter()
            .map(|(name, tx)| (name.clone(), tx.receiver_count()))
            .collect();
        rooms.sort();
        rooms
    }

    pub async fn broadcast_all(&self, msg: Message) {
        let map = self.inner.read().await;
        for tx in map.values() {
            let _ = tx.send(msg.clone());
        }
    }
}

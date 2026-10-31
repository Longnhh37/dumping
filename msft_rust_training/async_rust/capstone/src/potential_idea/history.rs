use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::message::Message;
use crate::room::{RoomRx, RoomTx};

const HISTORY_LEN: usize = 50;

#[derive(Clone)]
pub struct Room {
    tx: RoomTx,
    history: Arc<Mutex<VecDeque<Message>>>,
}

impl Room {
    // All chat output must go through here instead of `tx.send` directly.
    pub fn publish(&self, msg: Message, record: bool) {
        let mut history = self.history.lock().unwrap();
        if record {
            if history.len() == HISTORY_LEN {
                history.pop_front();
            }
            history.push_back(msg.clone());
        }
        let _ = self.tx.send(msg);
    }

    // Snapshot and subscribe atomically w.r.t. `publish`.
    pub fn subscribe(&self) -> (RoomRx, Vec<Message>) {
        let history = self.history.lock().unwrap();
        (self.tx.subscribe(), history.iter().cloned().collect())
    }
}

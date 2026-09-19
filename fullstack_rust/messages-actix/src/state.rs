use std::sync::{Arc, Mutex, atomic::AtomicUsize};

pub struct AppState {
    pub server_id: usize,
    pub request_count: AtomicUsize,
    pub messages: Arc<Mutex<Vec<String>>>,
}

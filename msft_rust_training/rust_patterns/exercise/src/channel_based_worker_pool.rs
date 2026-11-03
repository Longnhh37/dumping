use std::sync::{Arc, Mutex, mpsc};

struct Job {
    payload: String,
}

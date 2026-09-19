use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering, fence};
use std::thread;

pub struct PetersonSolution {
    flag: [AtomicBool; 2],
    turn: AtomicUsize,
}

#[allow(clippy::new_without_default)]
impl PetersonSolution {
    pub fn new() -> Self {
        Self {
            flag: [AtomicBool::new(false), AtomicBool::new(false)],
            turn: AtomicUsize::new(0),
        }
    }

    pub fn enter(&self, id: usize) {
        let other = 1 - id;

        self.flag[id].store(true, Ordering::Relaxed);
        self.turn.store(other, Ordering::Relaxed);

        fence(Ordering::SeqCst);

        while self.flag[other].load(Ordering::Relaxed) && self.turn.load(Ordering::Relaxed) == other
        {
            thread_yield();
        }
    }

    pub fn exit(&self, id: usize) {
        self.flag[id].store(false, Ordering::Release);
    }
}

fn thread_yield() {
    thread::yield_now();
}

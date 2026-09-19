use std::{
    cell::UnsafeCell,
    collections::VecDeque,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::{
        Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, Thread},
};

// =========================================================
// Channel 1
// =========================================================
pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

#[allow(clippy::new_without_default)]
impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}

// =========================================================
// Channel 2
// =========================================================

// ==================
// Sender
// ==================
pub struct Sender<'a, T> {
    channel: &'a Channel2<T>,
    receiving_thread: Thread,
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Ordering::Release);
        self.receiving_thread.unpark();
    }
}

// ====================
// Receiver
// ====================
pub struct Receiver<'a, T> {
    channel: &'a Channel2<T>,
    _no_send: PhantomData<*const ()>,
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Ordering::Relaxed)
    }

    pub fn receive(self) -> T {
        while !self.channel.ready.swap(false, Ordering::Acquire) {
            thread::park();
        }

        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

// ====================
// Channel2
// ====================
pub struct Channel2<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel2<T> where T: Send {}

impl<T> Drop for Channel2<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

impl<T> Channel2<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                channel: self,
                _no_send: PhantomData,
            },
        )
    }
}

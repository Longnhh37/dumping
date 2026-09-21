use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicU32,
};

use atomic_wait::{wait, wake_all, wake_one};

// ===========================================================
// Mutex
// ===========================================================
pub struct Mutex<T> {
    /// 0: unlocked
    /// 1: locked
    state: AtomicU32,
    value: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

pub struct MutextGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutextGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutextGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

#[cfg(feature = "core-postgres")]
#[derive(Clone)]
pub struct RedisSessionDescriptor<T> {
    pub store: T,
}

#[cfg(feature = "core-postgres")]
impl<T> RedisSessionDescriptor<T> {
    pub fn new(store: T) -> Self {
        Self { store }
    }
}

#[cfg(all(feature = "http", not(feature = "core-postgres")))]
#[derive(Clone, Copy, Default)]
pub struct RedisSessionDescriptor;

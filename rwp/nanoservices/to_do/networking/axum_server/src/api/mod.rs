pub mod basic_actions;

#[derive(Clone)]
pub struct AppState<T, R> {
    pub store: T,
    pub auth_user: R,
}

impl<T, R> AppState<T, R> {
    pub fn new(store: T, auth_user: R) -> Self {
        Self { store, auth_user }
    }
}

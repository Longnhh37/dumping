pub mod basic_actions;

#[derive(Clone)]
pub struct AppState<T, R> {
    pub store: T,
    pub user_session: R,
}

impl<T, R> AppState<T, R> {
    pub fn new(store: T, user_session: R) -> Self {
        Self {
            store,
            user_session,
        }
    }
}

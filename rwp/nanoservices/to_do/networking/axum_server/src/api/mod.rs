pub mod basic_actions;

#[derive(Clone)]
pub struct AppState<T> {
    pub store: T,
}

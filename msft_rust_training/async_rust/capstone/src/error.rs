use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChatError {
    #[erro("nickname")]
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChatError {
    #[error("nickname must be 1-{max} characters with no whitespace")]
    InvalidNick { max: usize },

    #[error("room name must be 1-{max} characters with no whitespace")]
    InvalidRoom { max: usize },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

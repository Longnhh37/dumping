use thiserror::Error;

use crate::lexer::{token_type::TokenType, token::Token};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected token '{token:?}'")]
    UnexpectedToken { token: Token },

    #[error("expected '{expected:?}': {message}")]
    ExpectedToken { expected: TokenType, message: String },
}

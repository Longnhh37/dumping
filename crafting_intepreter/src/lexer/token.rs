use crate::lexer::{literal::Literal, token_type::TokenType};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

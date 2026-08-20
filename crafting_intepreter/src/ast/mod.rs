use crate::lexer::{literal::Literal, token::Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),

    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Grouping {
        expressing: Box<Expr>,
    },
}

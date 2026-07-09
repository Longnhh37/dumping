#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot,
    Minus, Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    Var,
    True, False,
    And, Or,
    If, Else,
    While, For,
    Class, Super, This,
    Fun, Return,
    Nil,
    Print,

    Eof,
}

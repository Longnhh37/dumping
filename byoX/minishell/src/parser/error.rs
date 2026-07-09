use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    // --- input error ---------------------------
    #[error("empty input")]
    EmptyInput,

    #[error("whitespace only input")]
    WhitespaceOnly,

    // --- lexer error ---------------------------
    #[error("trailing escape")]
    TrailingEscape,

    #[error("unclosed single quote")]
    UnterminatedSingleQuote,

    #[error("unclosed double quote")]
    UnterminatedDoubleQuote,

    // --- parser error ---------------------------
    #[error("expect arguments")]
    ExpectedWord,

    #[error("unexpected input")]
    UnexpectedToken,

    #[error("pipe without rhs")]
    PipeMissingRhs,
}

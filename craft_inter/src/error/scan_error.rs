use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("unexpected character '{ch}' at line {line}")]
    UnexpectedCharacter {
        line: usize,
        ch: char,
    },

    #[error("unterminated string at line {line}")]
    UnterminatedString {
        line: usize,
    }
}

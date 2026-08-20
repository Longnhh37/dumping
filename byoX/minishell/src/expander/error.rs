use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExpandError {
    #[error("undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("ambiguous redirect")]
    AmbiguousRedirect,

    #[error("empty redirect target")]
    EmptyRedirect,
}

use thiserror::Error;

pub type RurlResult<T> = Result<T, RurlError>;

#[derive(Debug, Error)]
pub enum RurlError {
    #[error("missing url")]
    MissingUrl,

    #[error("invalid parameter: {0}")]
    ParameterMissingSeparator(String),

    #[error("invalid url: {0}")]
    Url(#[from] url::ParseError),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

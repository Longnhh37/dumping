use thiserror::Error;

pub mod scan_error;
use crate::error::scan_error::ScanError;

pub mod parse_error;
use crate::error::parse_error::ParseError;

#[derive(Debug, Error)]
pub enum RloxErr {
    #[error("scan error: {0}")]
    Scan(#[from] ScanError),

    #[error("parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

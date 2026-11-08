use std::{error::Error, fmt, str::FromStr};

#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Debug => "DEBUG",
        };
        f.write_str(value)
    }
}

impl FromStr for LogLevel {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "INFO" => Ok(Self::Info),
            "WARN" | "WARNING" => Ok(Self::Warn),
            "ERROR" => Ok(Self::Error),
            "DEBUG" => Ok(Self::Debug),
            _ => Err(ParseError::UnknownLevel(s.to_owned())),
        }
    }
}

#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyLine,
    MissingTimestamp,
    MissingLevel,
    UnknownLevel(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLine => f.write_str("empty line"),
            Self::MissingTimestamp => f.write_str("missing timestamp"),
            Self::MissingLevel => f.write_str("missing log level"),
            Self::UnknownLevel(level) => write!(f, "unknown log level: {level}"),
        }
    }
}

impl Error for ParseError {}

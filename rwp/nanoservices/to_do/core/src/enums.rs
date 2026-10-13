use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskStatus {
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "PENDING")]
    Pending,
}

impl FromStr for TaskStatus {
    type Err = NanoServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::Done),
            "PENDING" => Ok(TaskStatus::Pending),
            _ => Err(NanoServiceError::new(
                "invalid status".to_string(),
                NanoServiceErrorStatus::BadRequest,
            )),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Done => write!(f, "DONE"),
            Self::Pending => write!(f, "PENDING"),
        }
    }
}


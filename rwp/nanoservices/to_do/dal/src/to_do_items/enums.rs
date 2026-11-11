use std::fmt;

use serde::{Deserialize, Serialize};

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::DONE => {
                write!(f, "DONE")
            }
            Self::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
}

impl TaskStatus {
    pub fn from_string(status: &str) -> Result<TaskStatus, NanoServiceError> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(NanoServiceError::new(
                format!("Invalid status: {}", status),
                NanoServiceErrorStatus::BadRequest,
            )),
        }
    }
}

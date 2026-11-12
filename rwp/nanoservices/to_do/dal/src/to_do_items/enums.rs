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

#[cfg(test)]
mod tests {
    use super::TaskStatus;
    use glue::errors::NanoServiceErrorStatus;

    #[test]
    fn parses_status_case_insensitively_and_formats_canonical_values() {
        assert!(matches!(
            TaskStatus::from_string("done").unwrap(),
            TaskStatus::DONE
        ));
        assert!(matches!(
            TaskStatus::from_string("Pending").unwrap(),
            TaskStatus::PENDING
        ));
        assert_eq!(TaskStatus::DONE.to_string(), "DONE");
        assert_eq!(TaskStatus::PENDING.to_string(), "PENDING");
    }

    #[test]
    fn rejects_unknown_status_as_bad_request() {
        let error = TaskStatus::from_string("blocked").unwrap_err();
        assert_eq!(error.status, NanoServiceErrorStatus::BadRequest);
    }
}

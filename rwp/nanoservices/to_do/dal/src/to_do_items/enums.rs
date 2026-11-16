use std::{fmt, str::FromStr};

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskStatus {
    Done,
    Pending,
}

impl FromStr for TaskStatus {
    type Err = NanoServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("DONE") {
            Ok(Self::Done)
        } else if s.eq_ignore_ascii_case("PENDING") {
            Ok(Self::Pending)
        } else {
            Err(NanoServiceError::new(
                format!("invalid status: {s}"),
                NanoServiceErrorStatus::BadRequest,
            ))
        }
    }
}

impl TryFrom<String> for TaskStatus {
    type Error = NanoServiceError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Done => "DONE",
            Self::Pending => "PENDING",
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub name: String,
    #[cfg_attr(feature = "sqlx-postgres", sqlx(try_from = "String"))]
    pub status: TaskStatus,
}

#[cfg(test)]
mod tests {
    use super::TaskStatus;
    use glue::errors::NanoServiceErrorStatus;

    #[test]
    fn parses_case_insensitively_and_formats_canonical() {
        assert_eq!("done".parse::<TaskStatus>().unwrap(), TaskStatus::Done);
        assert_eq!(
            "Pending".parse::<TaskStatus>().unwrap(),
            TaskStatus::Pending
        );
        assert_eq!(TaskStatus::Done.to_string(), "DONE");
    }

    #[test]
    fn rejects_unknown_status() {
        let e = "blocked".parse::<TaskStatus>().unwrap_err();
        assert_eq!(e.status, NanoServiceErrorStatus::BadRequest);
    }
}

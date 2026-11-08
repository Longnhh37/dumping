use crate::{LogEntry, LogLevel, ParseError};

pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(ParseError::EmptyLine);
    }

    let mut parts = line.splitn(3, char::is_whitespace);
    let timestamp = parts.next().ok_or(ParseError::MissingTimestamp)?;
    if timestamp.is_empty() {
        return Err(ParseError::MissingTimestamp);
    }
    let level = parts.next().ok_or(ParseError::MissingLevel)?;
    let message = parts.next().unwrap_or("").trim();

    Ok(LogEntry {
        timestamp: timestamp.to_owned(),
        level: level.parse::<LogLevel>()?,
        message: message.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_line() {
        let entry = parse_line("2026-11-07T10:00:00 ERROR database unavailable").unwrap();
        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.message, "database unavailable");
    }

    #[test]
    fn rejects_unknown_level() {
        assert!(matches!(
            parse_line("2026-11-07T10:00:00 TRACE details"),
            Err(ParseError::UnknownLevel(_))
        ));
    }

    #[test]
    fn rejects_blank_line() {
        assert_eq!(parse_line("  "), Err(ParseError::EmptyLine));
    }
}

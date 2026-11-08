use crate::{LogEntry, LogLevel};

#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub total: usize,
    pub info: usize,
    pub warn: usize,
    pub error: usize,
    pub debug: usize,
    pub error_rate: f64,
    pub warning_rate: f64,
}

pub fn analyze(entries: &[LogEntry]) -> Summary {
    let mut summary = Summary {
        total: entries.len(),
        info: 0,
        warn: 0,
        error: 0,
        debug: 0,
        error_rate: 0.0,
        warning_rate: 0.0,
    };

    for entry in entries {
        match entry.level {
            LogLevel::Info => summary.info += 1,
            LogLevel::Warn => summary.warn += 1,
            LogLevel::Error => summary.error += 1,
            LogLevel::Debug => summary.debug += 1,
        }
    }

    if summary.total > 0 {
        summary.error_rate = summary.error as f64 / summary.total as f64 * 100.0;
        summary.warning_rate = summary.warn as f64 / summary.total as f64 * 100.0;
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_counts_and_rates() {
        let entries = vec![
            LogEntry {
                timestamp: "t1".into(),
                level: LogLevel::Info,
                message: "a".into(),
            },
            LogEntry {
                timestamp: "t2".into(),
                level: LogLevel::Warn,
                message: "b".into(),
            },
            LogEntry {
                timestamp: "t3".into(),
                level: LogLevel::Error,
                message: "c".into(),
            },
            LogEntry {
                timestamp: "t4".into(),
                level: LogLevel::Info,
                message: "d".into(),
            },
        ];
        let result = analyze(&entries);
        assert_eq!(result.total, 4);
        assert_eq!(result.info, 2);
        assert_eq!(result.warn, 1);
        assert_eq!(result.error, 1);
        assert_eq!(result.error_rate, 25.0);
    }

    #[test]
    fn empty_input_has_zero_rates() {
        let result = analyze(&[]);
        assert_eq!(result.total, 0);
        assert_eq!(result.error_rate, 0.0);
    }
}

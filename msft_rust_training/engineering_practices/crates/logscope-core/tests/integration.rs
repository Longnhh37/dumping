use logscope_core::{LogLevel, analyze, parse_line};

#[test]
fn parses_and_analyzes_multiple_records() {
    let input = [
        "2026-11-07T10:00:00 INFO started",
        "2026-11-07T10:00:01 WARN memory high",
        "2026-11-07T10:00:02 ERROR database failed",
    ];

    let entries: Vec<_> = input.iter().map(|line| parse_line(line).unwrap()).collect();
    let summary = analyze(&entries);

    assert_eq!(summary.total, 3);
    assert_eq!(summary.warn, 1);
    assert_eq!(summary.error, 1);
    assert_eq!(entries[2].level, LogLevel::Error);
}

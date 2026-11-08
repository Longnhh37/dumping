mod analyzer;
mod model;
mod parser;

pub use analyzer::{Summary, analyze};
pub use model::{LogEntry, LogLevel, ParseError};
pub use parser::parse_line;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildInfo {
    pub git_sha: &'static str,
    pub build_timestamp: &'static str,
}

pub fn build_info() -> BuildInfo {
    BuildInfo {
        git_sha: env!("LOGSCOPE_GIT_SHA"),
        build_timestamp: env!("LOGSCOPE_BUILD_TIMESTAMP"),
    }
}

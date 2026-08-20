use crate::completion::types::CompletionCandidate;
use super::{env::complete_env, file::complete_path};

pub fn complete_builtin(cmd: &str, prefix: &str) -> Vec<CompletionCandidate> {
    match cmd {
        "cd" => complete_path(prefix, true),

        "export" | "unset" => {
            let env_prefix = prefix.trim_start_matches('$');
            complete_env(env_prefix)
        }

        "source" | "." => complete_path(prefix, false),

        _ => complete_path(prefix, false),
    }
}

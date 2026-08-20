use std::env;

use rustyline::completion::Candidate;

use crate::completion::types::CompletionCandidate;

pub fn complete_env(prefix: &str) -> Vec<CompletionCandidate> {
    let mut candidates: Vec<CompletionCandidate> = env::vars()
        .filter(|(k, _)| k.starts_with(prefix))
        .map(|(k, _)| CompletionCandidate::new(format!("${k}"), format!("${k} ")))
        .collect();

    candidates.sort_by(|a, b| a.display().cmp(b.display()));
    candidates
}

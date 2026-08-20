use super::super::types::CompletionCandidate;
use super::file::complete_path;
use crate::runtime::CompSpec;
use std::process::Command;

pub fn complete_programmable(
    spec: &CompSpec,
    prefix: &str,
    line: &str,
    cmd: &str,
) -> Vec<CompletionCandidate> {
    match spec {
        CompSpec::List(words) => words
            .iter()
            .filter(|w| w.starts_with(prefix))
            .map(|w| CompletionCandidate::new(w.clone(), format!("{w} ")))
            .collect(),

        CompSpec::Command(shell_cmd) => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(shell_cmd)
                .env("COMP_LINE", line)
                .env("COMP_WORD", prefix)
                .env("COMP_CMD", cmd)
                .output();

            match output {
                Ok(out) => String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .filter(|l| l.starts_with(prefix))
                    .map(|l| CompletionCandidate::new(l, format!("{l} ")))
                    .collect(),
                Err(_) => vec![],
            }
        }

        CompSpec::File => complete_path(prefix, false),
        CompSpec::Dir => complete_path(prefix, true),
    }
}

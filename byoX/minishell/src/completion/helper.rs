use std::sync::{Arc, RwLock};

use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

use super::context::{self, CompletionContext};
use super::providers;
use super::types::{CompletionCandidate, CompletionKind};
use crate::runtime::ShellState;

pub struct ShellHelper {
    pub state: Arc<RwLock<ShellState>>,
}

impl ShellHelper {
    pub fn new(state: Arc<RwLock<ShellState>>) -> Self {
        Self { state }
    }
}

impl Completer for ShellHelper {
    type Candidate = CompletionCandidate;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        let CompletionContext {
            kind,
            word_start,
            prefix,
        } = context::analyze(line, pos);

        let state = self.state.read().unwrap();

        let candidates = match &kind {
            CompletionKind::Command => providers::command::complete_command(&prefix),

            CompletionKind::EnvVar => providers::env::complete_env(&prefix),

            CompletionKind::Argument { cmd } => {
                if let Some(spec) = state.comp_registry.get(cmd) {
                    providers::programmable::complete_programmable(spec, &prefix, line, cmd)
                } else {
                    providers::builtin::complete_builtin(cmd, &prefix)
                }
            }

            CompletionKind::File => {
                let cmd_name = extract_command(line, pos);
                if let Some(cmd) = cmd_name {
                    if let Some(spec) = state.comp_registry.get(&cmd) {
                        providers::programmable::complete_programmable(spec, &prefix, line, &cmd)
                    } else {
                        providers::file::complete_path(&prefix, false)
                    }
                } else {
                    providers::file::complete_path(&prefix, false)
                }
            }
        };

        drop(state);
        Ok((word_start, candidates))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Highlighter for ShellHelper {}
impl Validator for ShellHelper {}
impl Helper for ShellHelper {}

fn extract_command(line: &str, pos: usize) -> Option<String> {
    let line = &line[..pos];

    let after_op = line
        .rsplit_once(|c| ['|', ';', '&'].contains(&c))
        .map(|(_, r)| r)
        .unwrap_or(line)
        .trim_start();
    after_op.split_whitespace().next().map(str::to_owned)
}

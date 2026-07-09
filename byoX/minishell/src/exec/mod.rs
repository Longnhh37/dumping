pub mod command;
pub mod pipeline;
pub mod redirect;
pub mod resolve;

pub use resolve::{resolve, CommandKind};

use crate::expander::command::ExpandedCommand;
use crate::runtime::{ExecStatus, ShellState};

pub fn run(cmds: Vec<ExpandedCommand>, state: &mut ShellState) -> ExecStatus {
    match cmds.len() {
        1 => command::run(cmds.into_iter().next().unwrap(), state),
        _ => pipeline::run(cmds, state),
    }
}

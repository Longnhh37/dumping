use crate::runtime::{ExecStatus, ShellState};
use std::io::Write;

pub mod cd;
pub mod complete;
pub mod echo;
pub mod exit;
pub mod export;
pub mod pwd;
pub mod r#type;
pub mod unset;

pub const BUILTIN_NAMES: &[&str] = &[
    "cd", "complete", "echo", "exit", "export", "pwd", "type", "unset",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinCmd {
    Cd,
    Complete,
    Echo,
    Exit,
    Export,
    Pwd,
    Type,
    Unset,
}

pub fn get(name: &str) -> Option<BuiltinCmd> {
    match name {
        "cd" => Some(BuiltinCmd::Cd),
        "complete" => Some(BuiltinCmd::Complete),
        "echo" => Some(BuiltinCmd::Echo),
        "exit" => Some(BuiltinCmd::Exit),
        "export" => Some(BuiltinCmd::Export),
        "pwd" => Some(BuiltinCmd::Pwd),
        "type" => Some(BuiltinCmd::Type),
        "unset" => Some(BuiltinCmd::Unset),
        _ => None,
    }
}

impl BuiltinCmd {
    pub fn run(
        self,
        args: &[String],
        state: &mut ShellState,
        stdout: &mut dyn Write,
        stderr: &mut dyn Write,
    ) -> ExecStatus {
        match self {
            BuiltinCmd::Cd => cd::run(args),
            BuiltinCmd::Complete => complete::run(args, state),
            BuiltinCmd::Echo => echo::run(args, stdout),
            BuiltinCmd::Exit => exit::run(args, state),
            BuiltinCmd::Export => export::run(args, state),
            BuiltinCmd::Pwd => pwd::run(args, stdout, stderr),
            BuiltinCmd::Type => r#type::run(args, stdout, stderr),
            BuiltinCmd::Unset => unset::run(args, state),
        }
    }
}

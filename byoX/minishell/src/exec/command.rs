use std::os::unix::process::CommandExt;
use std::process::Command as StdCommand;

use crate::builtin;
use crate::exec::{
    redirect::{make_stdio_set, make_writer_set},
    resolve::{CommandKind, resolve},
};
use crate::expander::command::ExpandedCommand;
use crate::runtime::{ExecStatus, ShellState};

pub fn run(cmd: ExpandedCommand, state: &mut ShellState) -> ExecStatus {
    match resolve(&cmd.name) {
        CommandKind::Builtin => {
            let builtin = builtin::get(&cmd.name).unwrap();

            let mut writers = match make_writer_set(&cmd.redirect) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("{}: {}", cmd.name, e);
                    return ExecStatus::Code(1);
                }
            };

            builtin.run(&cmd.args, state, &mut writers.stdout, &mut writers.stderr)
        }

        CommandKind::External { name, path } => {
            let stdio = make_stdio_set(&cmd.redirect);

            match StdCommand::new(&path)
                .arg0(&name)
                .args(&cmd.args)
                .stdin(stdio.stdin)
                .stdout(stdio.stdout)
                .stderr(stdio.stderr)
                .status()
            {
                Ok(status) => ExecStatus::Code(status.code().unwrap_or(1)),

                Err(e) => {
                    eprintln!("{}: {}", cmd.name, e);
                    ExecStatus::Code(1)
                }
            }
        }

        CommandKind::NotFound => {
            eprintln!("{}: command not found", cmd.name);
            ExecStatus::Code(127)
        }
    }
}

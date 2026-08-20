use crate::runtime::{ExecStatus, ShellState};

pub fn run(args: &[String], state: &mut ShellState) -> ExecStatus {
    match args.first() {
        None => ExecStatus::Exit(state.last_status),
        Some(s) => match s.parse::<i32>() {
            Ok(code) => {
                state.last_status = code;
                ExecStatus::Exit(code)
            }
            Err(_) => {
                eprintln!("exit: {}: numeric argument required", s);
                ExecStatus::Exit(2)
            }
        },
    }
}

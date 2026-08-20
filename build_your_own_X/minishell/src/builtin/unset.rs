use crate::runtime::{ExecStatus, ShellState, is_valid_identifier};

pub fn run(args: &[String], state: &mut ShellState) -> ExecStatus {
    if args.is_empty() {
        return ExecStatus::Code(0);
    }

    for name in args {
        if !is_valid_identifier(name) {
            eprintln!("unset: '{}': not a valid identifier", name);
            return ExecStatus::Code(1);
        }
        state.unset_var(name);
    }

    ExecStatus::Code(0)
}

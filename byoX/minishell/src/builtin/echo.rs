use std::io::Write;

use crate::runtime::ExecStatus;

pub fn run(args: &[String], stdout: &mut dyn Write) -> ExecStatus {
    let _ = writeln!(stdout, "{}", args.join(" "));
    ExecStatus::Code(0)
}

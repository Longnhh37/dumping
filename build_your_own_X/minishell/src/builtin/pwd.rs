use std::env;
use std::io::Write;

use crate::runtime::ExecStatus;

pub fn run(args: &[String], stdout: &mut dyn Write, stderr: &mut dyn Write) -> ExecStatus {
    if !args.is_empty() {
        eprintln!("pwd: too many arguments");
        return ExecStatus::Code(1);
    }

    match env::current_dir() {
        Ok(path) => {
            let _ = writeln!(stdout, "{}", path.display());
            ExecStatus::Code(0)
        }
        Err(e) => {
            let _ = writeln!(stderr, "pwd: {}", e);
            ExecStatus::Code(1)
        }
    }
}

use std::io::Write;

use crate::exec;
use crate::runtime::ExecStatus;

pub fn run(args: &[String], stdout: &mut dyn Write, stderr: &mut dyn Write) -> ExecStatus {
    if args.is_empty() {
        let _ = writeln!(stderr, "type: missing argument");
        return ExecStatus::Code(1);
    }

    let arg = &args[0];

    match exec::resolve(arg) {
        exec::CommandKind::Builtin => {
            let _ = writeln!(stdout, "{} is a shell builtin", arg);
            ExecStatus::Code(0)
        }
        exec::CommandKind::External{name, path} => {
            let _ = writeln!(stdout, "{} is {}", name, path.display());
            ExecStatus::Code(0)
        }
        exec::CommandKind::NotFound => {
            let _ = writeln!(stdout, "{}: not found", arg);
            ExecStatus::Code(1)
        }
    }
}

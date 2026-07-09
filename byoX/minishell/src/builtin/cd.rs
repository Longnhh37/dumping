use std::path::{Path, PathBuf};
use std::{
    env,
    io::{self, Write},
};

use crate::runtime::ExecStatus;

pub fn run(args: &[String]) -> ExecStatus {
    let stderr = io::stderr();
    let mut err = stderr.lock();

    // get HOME
    let Some(home) = dirs::home_dir() else {
        let _ = writeln!(err, "cd: HOME is not defined in the environment");
        return ExecStatus::Code(1);
    };

    // parse args
    let arg = match args.len() {
        0 => "",
        1 => args[0].as_str(),
        _ => {
            let _ = writeln!(err, "cd: too many arguments");
            return ExecStatus::Code(1);
        }
    };

    // resolve path
    let path: PathBuf = if arg.is_empty() || arg == "~" {
        PathBuf::from(&home)
    } else if let Some(stripped) = arg.strip_prefix("~/") {
        Path::new(&home).join(stripped)
    } else {
        PathBuf::from(arg)
    };

    // metadata check
    let md = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(_) => {
            let _ = writeln!(err, "cd: {}: No such file or directory", path.display());
            return ExecStatus::Code(1);
        }
    };

    if !md.is_dir() {
        let _ = writeln!(err, "cd: {}: Not a directory", path.display());
        return ExecStatus::Code(1);
    }

    // save OLDPWD before changing
    let oldpwd = env::current_dir().ok();

    if let Err(e) = env::set_current_dir(&path) {
        let _ = writeln!(err, "cd: {}: {}", path.display(), e);
        return ExecStatus::Code(1);
    }

    // update $OLDPWD and $PWD
    if let Some(old) = oldpwd {
        unsafe {
            env::set_var("OLDPWD", old);
        }
    }

    if let Ok(new) = env::current_dir() {
        unsafe {
            env::set_var("PWD", new);
        }
    }

    ExecStatus::Code(0)
}

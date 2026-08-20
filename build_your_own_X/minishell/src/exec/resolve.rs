use is_executable::IsExecutable;
use std::env;
use std::path::{Path, PathBuf};

use crate::builtin;

#[derive(Debug)]
pub enum CommandKind {
    Builtin,
    External { name: String, path: PathBuf },
    NotFound,
}

pub fn resolve(name: &str) -> CommandKind {
    if builtin::get(name).is_some() {
        return CommandKind::Builtin;
    }

    if name.contains('/') {
        let path = PathBuf::from(name);

        if path.is_executable() {
            let prg_name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| name.to_string());

            return CommandKind::External {
                name: prg_name,
                path,
            };
        }

        return CommandKind::NotFound;
    }

    let path_var = match env::var("PATH") {
        Ok(p) => p,
        Err(_) => return CommandKind::NotFound,
    };

    for dir in path_var.split(':') {
        let path = Path::new(dir).join(name);

        if path.is_executable() {
            return CommandKind::External {
                name: name.to_string(),
                path,
            };
        }
    }

    CommandKind::NotFound
}

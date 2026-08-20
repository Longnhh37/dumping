use std::fs;
use std::path::{Path, PathBuf};

use rustyline::completion::Candidate;
use shellexpand;

use crate::completion::types::CompletionCandidate;

pub fn complete_path(prefix: &str, dirs_only: bool) -> Vec<CompletionCandidate> {
    let expanded = shellexpand::tilde(prefix);
    let prefix_str = expanded.as_ref();

    let (search_dir, name_prefix, dir_display) = if prefix_str.ends_with('/') {
        (
            PathBuf::from(prefix_str),
            String::new(),
            prefix_str.to_string(),
        )
    } else {
        let path = Path::new(prefix_str);
        match path.parent() {
            Some(p) if !p.as_os_str().is_empty() => {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                (p.to_path_buf(), name, format!("{}/ ", p.display()))
            }
            _ => (PathBuf::from("."), prefix_str.to_string(), String::new()),
        }
    };

    let Ok(entries) = fs::read_dir(&search_dir) else {
        return vec![];
    };

    let mut candidates: Vec<CompletionCandidate> = entries
        .flatten()
        .filter_map(|entry| {
            let fname = entry.file_name();
            let name = fname.to_string_lossy();

            if !name.starts_with(&*name_prefix) {
                return None;
            }

            let Ok(meta) = entry.metadata() else {
                return None;
            };
            let is_dir = meta.is_dir();

            if dirs_only && !is_dir {
                return None;
            }

            let (display, replacement) = if is_dir {
                (format!("{}/", name), format!("{}{}/", dir_display, name))
            } else {
                (name.to_string(), format!("{}{} ", dir_display, name))
            };

            Some(CompletionCandidate::new(display, replacement))
        })
        .collect();

    candidates.sort_by(|a, b| a.display().cmp(b.display()));
    candidates
}


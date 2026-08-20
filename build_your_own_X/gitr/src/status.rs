use crate::index::read_index;
use crate::object::{ObjectType, hash_object};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs;

/// Returns (changed_paths, new_paths, deleted_paths) between working dir and index.
pub fn get_status() -> Result<(Vec<String>, Vec<String>, Vec<String>)> {
    let mut paths = HashSet::new();

    fn walk_dir(dir: &str, paths: &mut HashSet<String>) -> Result<()> {
        let entries =
            fs::read_dir(dir).with_context(|| format!("failed to read workspace dir '{}'", dir))?;

        for entry in entries {
            let entry = entry.context("failed to read dir entry in workspace")?;
            let path = entry.path();
            let path_str = path.to_string_lossy().replace('\\', "/");

            if let Some(stripped) = path_str.strip_prefix("./") {
                let clean_path = stripped.to_string();
                if clean_path.starts_with(".git") {
                    continue;
                }
                if path.is_dir() {
                    walk_dir(&clean_path, paths)?;
                } else {
                    paths.insert(clean_path);
                }
            } else if path_str != ".git" {
                if path.is_dir() {
                    walk_dir(&path_str, paths)?;
                } else {
                    paths.insert(path_str);
                }
            }
        }
        Ok(())
    }

    walk_dir(".", &mut paths)?;

    let entries = read_index().context("failed to read index for status")?;
    let entries_by_path: HashMap<String, _> =
        entries.into_iter().map(|e| (e.path.clone(), e)).collect();
    let entry_paths: HashSet<String> = entries_by_path.keys().cloned().collect();

    let mut changed = Vec::new();
    let common_paths = paths.intersection(&entry_paths);

    for p in common_paths {
        if let Ok(data) = fs::read(p)
            && let Ok(sha1_hex) = hash_object(&data, ObjectType::Blob, false)
            && sha1_hex != hex::encode(entries_by_path[p].sha1)
        {
            changed.push(p.clone());
        }
    }

    let mut new: Vec<String> = paths.difference(&entry_paths).cloned().collect();
    let mut deleted: Vec<String> = entry_paths.difference(&paths).cloned().collect();

    changed.sort();
    new.sort();
    deleted.sort();

    Ok((changed, new, deleted))
}

/// Print status of working directory.
pub fn status() -> Result<()> {
    let (changed, new, deleted) = get_status().context("failed to collect status")?;

    if !changed.is_empty() {
        println!("changed files:");
        for path in changed {
            println!("    {}", path);
        }
    }
    if !new.is_empty() {
        println!("new files:");
        for path in new {
            println!("    {}", path);
        }
    }
    if !deleted.is_empty() {
        println!("deleted files:");
        for path in deleted {
            println!("    {}", path);
        }
    }

    Ok(())
}

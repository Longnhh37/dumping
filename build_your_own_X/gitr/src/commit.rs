use crate::object::{hash_object, write_tree, ObjectType};
use anyhow::{Context, Result};
use chrono::Local;
use std::env;
use std::path::Path;

/// Read current sha1 of refs/heads/master, or None if no commits yet.
pub fn get_local_master_hash() -> Result<Option<String>> {
    let master_path = Path::new(".git/refs/heads/master");
    if master_path.exists() {
        let content = std::fs::read_to_string(master_path)
            .context("failed to read '.git/refs/heads/master'")?;
        Ok(Some(content.trim().to_string()))
    } else {
        Ok(None)
    }
}

/// Build tree from index, create commit object, update refs/heads/master.
pub fn commit(message: &str, author: Option<String>) -> Result<String> {
    let tree = write_tree().context("failed to write tree object for commit")?;
    let parent = get_local_master_hash().context("failed to resolve parent commit SHA-1")?;

    let author_str = match author {
        Some(a) => a,
        None => {
            let name = env::var("GIT_AUTHOR_NAME").unwrap_or_else(|_| "Unknown Author".to_string());
            let email = env::var("GIT_AUTHOR_EMAIL").unwrap_or_else(|_| "unknown@example.com".to_string());
            format!("{} <{}>", name, email)
        }
    };

    let now = Local::now();
    let timestamp = now.timestamp();
    let offset = now.offset().local_minus_utc();
    let sign = if offset >= 0 { '+' } else { '-' };
    let abs_offset = offset.abs();
    let hours = abs_offset / 3600;
    let minutes = (abs_offset % 3600) / 60;
    let author_time = format!("{} {}{:02}{:02}", timestamp, sign, hours, minutes);

    let mut lines = Vec::new();
    lines.push(format!("tree {}", tree));
    if let Some(p) = parent {
        lines.push(format!("parent {}", p));
    }
    lines.push(format!("author {} {}", author_str, author_time));
    lines.push(format!("committer {} {}", author_str, author_time));
    lines.push(String::new());
    lines.push(message.to_string());
    lines.push(String::new());

    let data = lines.join("\n");
    let sha1 = hash_object(data.as_bytes(), ObjectType::Commit, true)
        .context("failed to hash commit object")?;

    let master_path = Path::new(".git/refs/heads/master");
    if let Some(parent_dir) = master_path.parent() {
        std::fs::create_dir_all(parent_dir)
            .with_context(|| format!("failed to create dir '{}'", parent_dir.display()))?;
    }
    std::fs::write(master_path, format!("{}\n", sha1))
        .context("failed to update ref '.git/refs/heads/master'")?;

    println!("committed to master: {:.7}", sha1);
    Ok(sha1)
}

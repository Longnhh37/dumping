use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn init(repo: &str) -> Result<()> {
    let git_dir = Path::new(repo).join(".git");

    fs::create_dir(repo).with_context(|| format!("failed to create repo directory '{}'", repo))?;
    fs::create_dir(&git_dir)
        .with_context(|| format!("failed to create '{}'", git_dir.display()))?;

    for name in ["object", "refs", "refs/heads"] {
        let path = git_dir.join(name);
        fs::create_dir_all(&path)
            .with_context(|| format!("failed to create '{}'", path.display()))?;
    }

    let head_path = git_dir.join("HEAD");
    fs::write(&head_path, b"ref: refs/heads/main")
        .with_context(|| format!("failed to write '{}'", head_path.display()))?;

    println!("initialized empty repository: {}", repo);
    Ok(())
}

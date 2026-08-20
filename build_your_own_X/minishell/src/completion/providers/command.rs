use std::collections::BTreeSet;
use std::sync::RwLock;

use is_executable::IsExecutable;
use once_cell::sync::Lazy;

use crate::builtin::BUILTIN_NAMES;
use crate::completion::types::CompletionCandidate;

struct PathCache {
    names: BTreeSet<String>,
    path_snapshot: String,
}

impl PathCache {
    fn build() -> Self {
        let path_var = std::env::var("PATH").unwrap_or_default();
        let mut names = BTreeSet::new();

        for dir in path_var.split(':') {
            let Ok(entries) = std::fs::read_dir(dir) else {
                continue;
            };

            for entry in entries.flatten() {
                if entry.path().is_executable()
                    && let Ok(name) = entry.file_name().into_string()
                {
                    names.insert(name);
                }
            }
        }

        PathCache {
            names,
            path_snapshot: path_var,
        }
    }

    fn is_stale(&self) -> bool {
        std::env::var("PATH").unwrap_or_default() != self.path_snapshot
    }
}

static PATH_CACHE: Lazy<RwLock<PathCache>> = Lazy::new(|| RwLock::new(PathCache::build()));

pub fn complete_command(prefix: &str) -> Vec<CompletionCandidate> {
    // re-build path
    {
        let cache = PATH_CACHE.read().unwrap();
        if cache.is_stale() {
            drop(cache);
            *PATH_CACHE.write().unwrap() = PathCache::build();
        }
    }

    let cache = PATH_CACHE.read().unwrap();

    let mut seen = BTreeSet::<String>::new();

    // 1. builtins
    for &b in BUILTIN_NAMES {
        if b.starts_with(prefix) {
            seen.insert(b.to_string());
        }
    }

    // 2. Executables in PATH (read from cache)
    for name in cache.names.range(prefix.to_string()..) {
        if !name.starts_with(prefix) {
            break;
        }
        seen.insert(name.clone());
    }

    seen.into_iter()
        .map(|name| CompletionCandidate::new(name.clone(), format!("{} ", name)))
        .collect()
}

use std::path::{Path, PathBuf};

use lazy_static::lazy_static;

pub struct Directories {
    config: PathBuf,
}

impl Directories {
    fn new() -> Option<Directories> {
        // macOS: XDG_CONFIG_HOME -> fallback: ~/.config
        #[cfg(target_os = "macos")]
        let config_op = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .or_else(|| dirs::home_dir().map(|d| d.join(".config")));

        #[cfg(not(target_os = "macos"))]
        let config_op = dirs::config_dir();
        let config = config_op.map(|d| d.join("rurl"))?;

        Some(Directories { config })
    }

    pub fn config(&self) -> &Path {
        &self.config
    }
}

lazy_static! {
    pub static ref DIRECTORIES: Directories =
        Directories::new().expect("cannot resolve config directory");
}

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::cli::args::Cli;
use crate::directories::DIRECTORIES;

/// fields mirror Cli flags
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub verbose: Option<u8>,
    pub form: Option<bool>,
    pub secure: Option<bool>,
    pub auth: Option<String>,
    pub token: Option<String>,
}

/// returns the effective config file path
/// priority: --config flags -> $XDG_CONFIG_HOME/rurl/config
pub fn config_file(cli: &Cli) -> PathBuf {
    cli.config
        .as_ref()
        .cloned()
        .filter(|p| p.is_file())
        .unwrap_or_else(|| DIRECTORIES.config().join("config"))
}

/// reads and parses the TOML config file
/// returns none if the file doesn't exist or can't be read
pub fn read_config_file(path: PathBuf) -> Option<Config> {
    fs::read_to_string(path)
        .ok()
        .map(|content| toml::from_str(&content).unwrap_or_default())
}

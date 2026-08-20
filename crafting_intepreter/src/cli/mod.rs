// src/cli/mod.rs
use std::path::PathBuf;

use clap::Parser;

use crate::error::RloxErr;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(num_args =  0..=1)]
    file: Option<PathBuf>,
}

pub enum CliMode {
    Repl,
    FromFile(PathBuf),
}

impl Cli {
    pub fn parse_mode() -> Result<CliMode, RloxErr> {
        let cli = Cli::parse();

        match cli.file {
            Some(f) => {
                let f = f.canonicalize()?;
                Ok(CliMode::FromFile(f))
            }
            None => Ok(CliMode::Repl),
        }
    }
}

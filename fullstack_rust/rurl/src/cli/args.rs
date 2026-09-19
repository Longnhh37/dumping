//! CLI parsing + validate args

use std::path::PathBuf;

use clap::Parser;

use crate::{
    cli::params::{Parameter, parse_param},
    config,
    errors::{RurlError, RurlResult},
    session,
};

#[derive(Debug, Parser)]
#[command(name = "rulr", about = "A modern HTTP client CLI")]
pub struct Cli {
    /// URL endpoints
    #[arg()]
    pub url: Option<String>,

    /// Send data in form format instead of JSON
    #[arg(short, long)]
    pub form: bool,

    /// Up log level (-v info, -vv debug, -vvv trace)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// use HTTPS if URL does not have scheme
    #[arg(short, long)]
    pub secure: bool,

    /// Basic auth user::pass (remove pass for prompt-ability)
    #[arg(short, long)]
    pub auth: Option<String>,

    /// Bearer token
    #[arg(short, long)]
    pub token: Option<String>,

    /// path to config file (default: ~/.config/rurl/config)
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// session name for persistant cookies/headers
    #[arg(long)]
    pub session: Option<String>,

    /// session files directory (default: ~/.config/rurl/sessions)
    #[arg(long)]
    pub session_dir: Option<PathBuf>,

    /// Request parameters: key=value (data), key:=value (raw JSON)
    /// key:value (header), key==value (query)
    #[arg(value_parser = parse_param)]
    pub parameters: Vec<Parameter>,
}

impl Cli {
    /// URL is required - validate before preoceeding with any other steps
    pub fn validate(&self) -> RurlResult<()> {
        if self.url.is_none() {
            return Err(RurlError::MissingUrl);
        }

        Ok(())
    }

    /// Merge config file to Cli struct
    /// CLI flags > config file. config file overide when default/no flag given
    pub fn process_config_file(&mut self) {
        let path = config::config_file(self);
        let Some(mut cfg) = config::read_config_file(path) else {
            return;
        };

        if self.verbose == 0
            && let Some(v) = cfg.verbose
        {
            self.verbose = v;
        }
        if !self.form
            && let Some(f) = cfg.form
        {
            self.form = f;
        }
        if !self.secure
            && let Some(s) = cfg.secure
        {
            self.secure = s;
        }
        if self.auth.is_none() {
            self.auth = cfg.auth.take();
        }
        if self.token.is_none() {
            self.token = cfg.token.take();
        }
    }

    /// return a safe hostname for session dir name
    pub fn host(&self) -> String {
        self.url
            .as_deref()
            .map(session::make_safe_pathname)
            .unwrap_or_default()
    }
}

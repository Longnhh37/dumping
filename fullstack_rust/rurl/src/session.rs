use std::collections::HashMap;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use reqwest::RequestBuilder;
use reqwest::header::COOKIE;
use serde::{Deserialize, Serialize};

use crate::cli::args::Cli;
use crate::cli::params::Parameter;
use crate::directories::DIRECTORIES;
use crate::errors::RurlResult;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Session {
    /// Don't serialzie path - path is reconstructed from name + host when loaded
    #[serde(skip)]
    path: PathBuf,

    name: String,
    host: String,
    auth: Option<String>,
    token: Option<String>,
    headers: HashMap<String, String>,
    cookies: Vec<(String, String)>,
}

impl Session {
    /// create a new session (no file available)
    pub fn new(cli: &Cli, name: String, host: String) -> Self {
        let path = Session::path(cli, &name, &host);
        Session {
            path,
            name,
            host,
            ..Default::default()
        }
    }

    /// load session from JSON file
    pub fn load(cli: &Cli, name: &str, host: &str) -> RurlResult<Self> {
        let path = Session::path(cli, name, host);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        // path is skipped when deserialze -> manually reconstruct
        let mut session: Session =
            serde_json::from_reader(reader).map_err(crate::errors::RurlError::from)?;

        session.path = path;

        Ok(session)
    }

    /// load if exists, create new if not
    pub fn get_or_create(cli: &Cli, name: String, host: String) -> Self {
        match Session::load(cli, &name, &host) {
            Ok(session) => session,
            Err(_) => Session::new(cli, name, host),
        }
    }

    /// save session to JSON file
    pub fn save(&self, cli: &Cli) -> RurlResult<()> {
        let dir = Session::dir(cli, &self.host);
        create_dir_all(&dir)?;

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self).map_err(crate::errors::RurlError::from)
    }

    // -- Request helper ------------------------

    /// apply session headers + cookies to request builder
    pub fn add_to_request(&self, mut builder: RequestBuilder) -> RequestBuilder {
        for (k, v) in &self.headers {
            builder = builder.header(k, v);
        }

        let cookie_header = self
            .cookies
            .iter()
            .map(|(n, v)| format!("{n}={v}"))
            .collect::<Vec<_>>()
            .join("; ");

        if !cookie_header.is_empty() {
            builder = builder.header(COOKIE, cookie_header);
        }

        builder
    }

    // -- Update helper ------------------------

    /// save headers from request params (skip content-* and if-*)
    pub fn update_with_parameters(&mut self, parameters: &[Parameter]) {
        for param in parameters {
            if let Parameter::Header { key, value } = param {
                let lower = key.to_ascii_lowercase();
                if lower.starts_with("content-") || lower.starts_with("if-") {
                    continue;
                }
                self.headers.insert(key.clone(), value.clone());
            }
        }
    }

    /// save auth/token if provided
    pub fn update_auth(&mut self, auth: &Option<String>, token: &Option<String>) {
        if auth.is_some() {
            self.auth = auth.clone();
        }
        if token.is_some() {
            self.token = token.clone();
        }
    }

    /// save cookies from response
    pub fn update_with_response(&mut self, resp: &reqwest::Response) {
        for cookie in resp.cookies() {
            self.cookies
                .push((cookie.name().to_owned(), cookie.value().to_owned()));
        }
    }

    // -- Path helper ------------------------
    fn path(cli: &Cli, name: &str, host: &str) -> PathBuf {
        let mut dir = Session::dir(cli, host);
        let mut filename = make_safe_pathname(name);
        filename.push_str(".json");
        dir.push(filename);
        dir
    }

    fn dir(cli: &Cli, host: &str) -> PathBuf {
        let mut dir = cli
            .session_dir
            .as_ref()
            .cloned()
            .filter(|d| d.is_dir())
            .unwrap_or_else(|| DIRECTORIES.config().join("sessions"));

        dir.push(make_safe_pathname(host));
        dir
    }
}

pub fn make_safe_pathname(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => c,
            _ => '_',
        })
        .collect()
}

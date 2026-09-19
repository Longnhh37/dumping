use reqwest::Url;

use crate::cli::args::Cli;
use crate::errors::RurlResult;

pub fn parse_url(cli: &Cli, raw: &str) -> RurlResult<Url> {
    match Url::parse(raw) {
        Ok(url) => Ok(url),
        Err(_) => {
            let scheme = if cli.secure { "https" } else { "http" };
            Ok(Url::parse(&format!("{scheme}://{raw}"))?)
        }
    }
}

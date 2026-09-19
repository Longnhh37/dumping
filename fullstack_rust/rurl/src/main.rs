mod cli;
mod config;
mod directories;
mod errors;
mod http;
mod output;
mod session;
mod types;

use clap::Parser;

use cli::args::Cli;
use errors::RurlResult;

#[tokio::main]
async fn main() -> RurlResult<()> {
    // 1. Parse CLI args
    let mut cli = Cli::parse();

    // 2. Validate before config override (URL is required)
    cli.validate()?;

    // 3. Merge config file into CLI (CLI flags > config)
    cli.process_config_file();

    // 4. init tracing after verbose is resolved from config
    init_tracing(cli.verbose);

    // 5. init session if --session is passed
    let mut session = cli
        .session
        .as_ref()
        .map(|name| session::Session::get_or_create(&cli, name.clone(), cli.host()));

    // 6. perform http request
    let response = http::client::execute(&cli, &mut session).await?;

    // 7. store session (headers + new cookies from response)
    if let Some(ref s) = session {
        s.save(&cli)?;
    }

    // 8. forrmat and print repsponse
    output::print_response(response).await?;

    Ok(())
}

fn init_tracing(verbose: u8) {
    use tracing_subscriber::{EnvFilter, fmt};

    let level = match verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    fmt().with_env_filter(EnvFilter::new(level)).init();
}

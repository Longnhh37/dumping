use reqwest::{Client, Method, RequestBuilder, Response};
use tracing::trace;

use crate::{cli::args::Cli, cli::params::Parameter, errors::RurlResult, session::Session};

use super::{auth::apply_auth, request::apply_parameters, url::parse_url};

pub async fn execute(cli: &Cli, session: &mut Option<Session>) -> RurlResult<Response> {
    let client = Client::new();

    let method = infer_method(cli);
    let url = parse_url(cli, cli.url.as_deref().unwrap())?;

    let mut builder = client.request(method, url);

    // 1. Request parameters (headers, body, query)
    builder = apply_parameters(builder, cli.form, &cli.parameters)?;

    // 2. Auth headers
    builder = apply_auth(builder, &cli.auth, &cli.token)?;

    // 3. Session data (cookies + persisted headers)
    builder = handle_session(
        builder,
        session,
        &cli.parameters,
        /* update_session = */ true,
        &cli.auth,
        &cli.token,
    );

    let response = builder.send().await?;

    // 4. Persist new cookies từ response vào session
    if let Some(s) = session {
        s.update_with_response(&response);
    }

    Ok(response)
}

/// Apply session vào request, và optionally update session từ request params
fn handle_session(
    mut builder: RequestBuilder,
    session: &mut Option<Session>,
    parameters: &[Parameter],
    update_session: bool,
    auth: &Option<String>,
    token: &Option<String>,
) -> RequestBuilder {
    if let Some(s) = session {
        trace!("applying session data to request");
        builder = s.add_to_request(builder);

        if update_session {
            trace!("updating session from request parameters");
            s.update_with_parameters(parameters);
            s.update_auth(auth, token);
        }
    }
    builder
}

/// GET nếu không có data parameters, POST nếu có
fn infer_method(cli: &Cli) -> Method {
    if cli.parameters.iter().any(|p| p.is_data()) {
        Method::POST
    } else {
        Method::GET
    }
}

use reqwest::RequestBuilder;

use crate::errors::RurlResult;

pub fn apply_auth(
    mut builder: RequestBuilder,
    auth: &Option<String>,
    token: &Option<String>,
) -> RurlResult<RequestBuilder> {
    if let Some(auth) = auth {
        let (user, pass) = parse_auth(auth)?;
        builder = builder.basic_auth(user, pass);
    }
    if let Some(token) = token {
        builder = builder.bearer_auth(token);
    }
    Ok(builder)
}

fn parse_auth(src: &str) -> RurlResult<(String, Option<String>)> {
    if let Some((u, p)) = src.split_once(':')
        && p.is_empty()
    {
        return Ok((u.into(), Some(p.into())));
    }

    let pw = rpassword::prompt_password("Password: ")?;
    Ok((src.into(), Some(pw)))
}

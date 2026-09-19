use heck::ToTitleCase;
use reqwest::Response;

use crate::{errors::RurlResult, types::OrderedJson};

pub async fn print_response(resp: Response) -> RurlResult<()> {
    print_status(&resp);
    print_headers(&resp);

    let body = resp.text().await?;
    pretty_print_body(&body)?;

    Ok(())
}

fn print_status(resp: &Response) {
    println!("{:?} {}", resp.version(), resp.status());
}

fn print_headers(resp: &Response) {
    for (k, v) in resp.headers() {
        let key = k.as_str().to_title_case().replace(' ', "-");
        println!("{}: {}", key, v.to_str().unwrap_or("INVALID"));
    }
    println!();
}

fn pretty_print_body(body: &str) -> RurlResult<()> {
    match serde_json::from_str::<OrderedJson>(body) {
        Ok(json) => println!("{}", serde_json::to_string_pretty(&json)?),
        Err(_) => println!("{body}"),
    }
    Ok(())
}

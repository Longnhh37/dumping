use http::{Method, Request, Response, StatusCode, header};

fn main() -> Result<(), http::Error> {
    let req = Request::builder()
        .method(Method::GET)
        .uri("https://example.com:8443/a/b?x=1&y=2")
        .header(header::ACCEPT, "application/json")
        .header(header::USER_AGENT, "demo/0.1")
        .body(())?;

    let uri = req.uri();
    println!("scheme    = {:?}", uri.scheme_str());
    println!("authority = {:?}", uri.authority().map(|a| a.as_str()));
    println!("path      = {}", uri.path());
    println!("query     = {:?}", uri.query());
    println!("accept    = {:?}", req.headers().get(header::ACCEPT));

    let res = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "max-age=3600")
        .body(r#"{"ok":true}"#)?;

    println!("status = {}", res.status());
    println!("ctype  = {:?}", res.headers().get(header::CONTENT_TYPE));
    println!("body   = {}", res.body());
    Ok(())
}

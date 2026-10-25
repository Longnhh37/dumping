use url::Url;

fn main() -> Result<(), url::ParseError> {
    let u = Url::parse("https://user:pw@example.com:8443/a/b?x=1&y=2#top")?;

    println!("scheme   = {}", u.scheme());
    println!("host     = {:?}", u.host_str());
    println!("port     = {:?}", u.port_or_known_default()); // Some(8443)
    println!("path     = {}", u.path());
    println!("fragment = {:?}", u.fragment());

    for (k, v) in u.query_pairs() {
        println!("query: {k} = {v}");
    }
    Ok(())
}

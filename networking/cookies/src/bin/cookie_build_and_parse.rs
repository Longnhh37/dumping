use cookie::{Cookie, SameSite};
use time::Duration;

fn main() -> Result<(), cookie::ParseError> {
    let c = Cookie::build(("__Host-session", "abc123"))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::hours(1))
        .build();
    println!("Set-Cookie: {c}");

    let parsed = Cookie::parse("id=42; Path=/; Max-Age=60; HttpOnly")?;
    println!("{} = {}", parsed.name(), parsed.value());
    println!("http_only = {:?}", parsed.http_only());

    for c in Cookie::split_parse("a=1; b=2; theme=dark") {
        let c = c?;
        println!("{} => {}", c.name(), c.value());
    }
    Ok(())
}

use axum::{Router, routing::get};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use time::Duration;

async fn login(jar: CookieJar) -> (CookieJar, &'static str) {
    let session = Cookie::build(("__Host-session", "abc123"))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::hours(1))
        .build();

    let widget = Cookie::build(("widget_pref", "compact"))
        .path("/")
        .secure(true)
        .same_site(SameSite::None)
        .partitioned(true)
        .build();

    (jar.add(session).add(widget), "ok")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/login", get(login));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}

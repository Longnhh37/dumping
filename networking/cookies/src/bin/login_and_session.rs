use axum::{Router, routing::get};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use time::Duration;

async fn login(jar: CookieJar) -> (CookieJar, &'static str) {
    let cookie = Cookie::build(("__Host-session", "abc123"))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::hours(1))
        .build();
    (jar.add(cookie), "logged in")
}

async fn me(jar: CookieJar) -> String {
    match jar.get("__Host-session") {
        Some(c) => format!("session = {}", c.value()),
        None => "anonymous".to_owned(),
    }
}

async fn logout(jar: CookieJar) -> (CookieJar, &'static str) {
    (jar.remove(Cookie::build("__Host-session").path("/")), "bye")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/login", get(login))
        .route("/me", get(me))
        .route("/logout", get(logout));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}

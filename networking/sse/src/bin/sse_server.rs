use std::{convert::Infallible, time::Duration};

use axum::{
    Router,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
};
use futures_util::Stream;
use tokio_stream::{StreamExt, wrappers::IntervalStream};

async fn events() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let interval = tokio::time::interval(Duration::from_secs(1));
    let mut n = 0u64;

    let stream = IntervalStream::new(interval).map(move |_| {
        n += 1;
        Ok(Event::default()
            .event("tick")
            .id(n.to_string())
            .data(format!("tick #{n}")))
    });
    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/events", get(events));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}

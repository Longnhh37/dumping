use crate::{errors, routes::config, state::AppState};
use actix_web::{App, HttpServer, middleware, web};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

const LOG_FORMAT: &str = r#""%r" %s %b "%{User-Agent}i" %D"#;
static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub async fn run(port: u16) -> std::io::Result<()> {
    println!("Starting http server: 127.0.0.1:{}", port);

    let state = web::Data::new(AppState {
        server_id: SERVER_COUNTER.fetch_add(1, Ordering::Relaxed),
        request_count: AtomicUsize::new(0),
        messages: Arc::new(Mutex::new(vec![])),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::JsonConfig::default()
                .limit(4096)
                .error_handler(errors::post_error)
            )
            .wrap(middleware::Logger::new(LOG_FORMAT))
            .configure(config)
    })
    .bind(("127.0.0.1", port))?
    .workers(8)
    .run()
    .await
}


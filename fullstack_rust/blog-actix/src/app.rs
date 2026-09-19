use actix_web::{App, HttpServer, middleware::Logger, web};
use diesel::{SqliteConnection, r2d2::ConnectionManager};

use crate::{routes, DbPool};

pub struct Blog {
    port: u16,
}

impl Blog {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn run(&self, db_url: String) -> std::io::Result<()> {
        let pool: DbPool = diesel::r2d2::Pool::builder()
            .build(ConnectionManager::<SqliteConnection>::new(db_url))
            .expect("Failed to create db pool.");

        println!("Starting http server: 127.0.0.1:{}", self.port);

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(Logger::default())
                .configure(routes::configure)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}

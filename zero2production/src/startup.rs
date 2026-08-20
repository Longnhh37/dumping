use std::net::TcpListener;

use actix_web::{App, HttpServer, cookie::Key, dev::Server, web};
use actix_web_flash_messages::{FlashMessagesFramework, storage::CookieMessageStore};
use secrecy::{ExposeSecret, Secret};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_actix_web::TracingLogger;

use crate::{
    config::{DatabaseSettings, Settings},
    email_client::EmailClient,
    routes::{confirm, health_check, home, login, login_form, publish_newsletter, subscribe},
};

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(cfg: Settings) -> Result<Self, std::io::Error> {
        let db_pool = get_db_connection_pool(&cfg.database);

        let timeout = cfg.email_client.timeout();
        let sender_email = cfg
            .email_client
            .sender()
            .expect("invalid sender email address");
        let email_client = EmailClient::new(
            cfg.email_client.base_url,
            sender_email,
            cfg.email_client.authorization_token,
            timeout,
        );

        let address = format!("{}:{}", cfg.app.host, cfg.app.port);
        let listener = TcpListener::bind(address)?;

        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            db_pool,
            email_client,
            cfg.app.base_url,
            HmacSecret(cfg.app.hmac_secret),
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_db_connection_pool(cfg: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(cfg.with_db())
}

pub struct ApplicationBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: HmacSecret,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let hmac_secret = web::Data::new(hmac_secret);
    let message_store = CookieMessageStore::builder(
        Key::from(hmac_secret.0.expose_secret().as_bytes())
    ).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm))
            .route("/newsletters", web::post().to(publish_newsletter))
            .route("/", web::get().to(home))
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
            .app_data(hmac_secret.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

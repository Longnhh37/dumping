use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

use zero2prod::{
    config::{DatabaseSettings, get_config},
    startup::{Application, get_db_connection_pool},
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
    pub port: u16,
}

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to execute request")
    }

    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

        // extract the link from one of the request fields
        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();

            assert_eq!(links.len(), 1);

            let raw_link = links[0].as_str().to_owned();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();

            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");

            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };

        let html = get_link(body["HtmlBody"].as_str().unwrap());
        let plain_text = get_link(body["TextBody"].as_str().unwrap());

        ConfirmationLinks { html, plain_text }
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;

    let config = {
        let mut c = get_config().expect("failed to read config");
        c.database.db_name = Uuid::new_v4().to_string();
        c.app.port = 0;
        c.email_client.base_url = email_server.uri();
        c
    };

    // create and migrate the db
    configure_database(&config.database).await;

    let app = Application::build(config.clone())
        .await
        .expect("failed to build an app");

    let port = app.port();

    tokio::spawn(app.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", port),
        port,
        db_pool: get_db_connection_pool(&config.database),
        email_server,
    }
}

async fn configure_database(cfg: &DatabaseSettings) -> PgPool {
    // create db
    let mut conn = PgConnection::connect_with(&cfg.without_db())
        .await
        .expect("failed to connect to postgres");

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, cfg.db_name).as_str())
        .await
        .expect("failed to create database");

    // migrate db
    let db_pool = PgPool::connect_with(cfg.with_db())
        .await
        .expect("failed to conenct to postgres");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("failed to migrate the database");

    db_pool
}

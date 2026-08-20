use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version, password_hash::SaltString};
use once_cell::sync::Lazy;
use rand_core::OsRng;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

use zero2prod::{
    config::{DatabaseSettings, get_config},
    startup::{Application, get_db_connection_pool},
    telemetry::{LogFormat, get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let default_log_format = LogFormat::from("pretty");
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::stdout,
            default_log_format,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::sink,
            default_log_format,
        );
        init_subscriber(subscriber);
    };
});

pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

pub struct TestApp {
    pub test_user: TestUser,
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
    pub port: u16,
    pub api_client: reqwest::Client,
}

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

        sqlx::query!(
            "INSERT INTO users (user_id, username, password_hash)
            VALUES($1, $2, $3)",
            self.user_id,
            self.username,
            password_hash
        )
        .execute(pool)
        .await
        .expect("failed to store test user");
    }
}

impl TestApp {
    pub async fn get_login_html(&self) -> String {
        self.api_client
            .get(format!("{}/login", &self.address))
            .send()
            .await
            .expect("failed to execute request")
            .text()
            .await
            .unwrap()
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize + ?Sized,
    {
        self.api_client
            .post(format!("{}/login", &self.address))
            .form(body)
            .send()
            .await
            .expect("failed to execute request")
    }

    pub async fn post_newsletters(&self, body: serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(format!("{}/newsletters", &self.address))
            .basic_auth(&self.test_user.username, Some(&self.test_user.password))
            .json(&body)
            .send()
            .await
            .expect("failed to execute request")
    }

    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        self.api_client
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

    let api_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let test_app = TestApp {
        test_user: TestUser::generate(),
        address: format!("http://localhost:{}", port),
        port,
        db_pool: get_db_connection_pool(&config.database),
        email_server,
        api_client,
    };

    test_app.test_user.store(&test_app.db_pool).await;

    test_app
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

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}

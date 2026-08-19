use redis::aio::ConnectionManager;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cache: ConnectionManager,
}

impl AppState {
    pub async fn new(database_url: &str, redis_url: &str) -> anyhow::Result<Self> {
        let db = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        sqlx::migrate!("./migrations").run(&db).await?;

        let client = redis::Client::open(redis_url)?;
        let cache = ConnectionManager::new(client).await?;

        Ok(Self { db, cache })
    }
}

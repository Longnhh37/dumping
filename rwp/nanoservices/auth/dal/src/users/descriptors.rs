#[derive(Clone)]
pub struct SqlxPostgresDescriptor {
    pub pool: sqlx::PgPool,
}

impl SqlxPostgresDescriptor {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

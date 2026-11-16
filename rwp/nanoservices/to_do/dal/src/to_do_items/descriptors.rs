#[cfg(feature = "sqlx-postgres")]
#[derive(Clone)]
pub struct SqlxPostgresDescriptor {
    pub pool: sqlx::PgPool,
}

#[cfg(feature = "sqlx-postgres")]
impl SqlxPostgresDescriptor {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

use sqlx::{postgres::PgQueryResult, PgPool};

pub struct UserStorage {
    pool: PgPool,
}

impl UserStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, name: &str) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("INSERT INTO users (name) VALUES ($1)", name)
            .execute(&self.pool)
            .await
    }

    pub async fn count(&self) -> Result<u64, sqlx::Error> {
        let n: i64 = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!: i64" FROM users"#)
            .fetch_one(&self.pool)
            .await?;
        let n = u64::try_from(n)
            .map_err(|_| sqlx::Error::Protocol("Count cannot be negative".into()))?;
        Ok(n)
    }
}

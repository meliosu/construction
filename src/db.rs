use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    pool: Arc<sqlx::PgPool>,
}

impl Database {
    pub async fn connect(url: &str) -> sqlx::Result<Self> {
        let pool = sqlx::PgPool::connect(url).await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

impl std::ops::Deref for Database {
    type Target = sqlx::PgPool;

    fn deref(&self) -> &Self::Target {
        &*self.pool
    }
}

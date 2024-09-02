use anyhow::Context;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, Pool, Postgres};
pub struct Connections {
    pub pg_pool: Pool<Postgres>,
}

impl Connections {
    pub async fn new(database_url:&str) -> anyhow::Result<Self> {
        let options:PgConnectOptions = database_url.parse().context("Failed to parse database URL")?;
        let pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .max_connections(5)
            .min_connections(1)
            .connect_lazy_with(options);
        Ok(Connections { pg_pool: pool })
    }
}
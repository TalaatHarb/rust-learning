use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::debug;

pub async fn connect(database_url: &str) -> anyhow::Result<PgPool> {
    debug!("connecting to postgres database pool (max connections: 10)");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    debug!("connected to postgres database pool");

    Ok(pool)
}

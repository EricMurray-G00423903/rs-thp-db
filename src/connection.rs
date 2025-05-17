use sqlx::PgPool;
use tracing::{info};
use std::env;

async fn connect() -> Result<PgPool, Box<dyn std::error::Error>> {

    info!("Connect to db called");

    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    info!("DB URL Fetched");

    let pool = PgPool::connect(&db_url).await?;
    info!("DB Connected returning connection...");

    Ok(pool)

}
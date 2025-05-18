use sqlx::PgPool;
use tracing::info;
use models::Log;
use uuid::Uuid;

pub async fn create_log(pool: &PgPool, log: Log) -> Result<(), sqlx::Error> {
    info!("DB Creating Log");

    sqlx::query("INSERT INTO logs (id, level, message, context, created_at) VALUES ($1, $2, $3, $4, $5)")
        .bind(log.id)
        .bind(log.level)
        .bind(log.message)
        .bind(log.context)
        .bind(log.created_at)
        .execute(pool)
        .await?;

    info!("DB Updated");

    Ok(())

}

pub async fn delete_log(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    info!("DB Delete Log Called");

    sqlx::query("DELETE FROM logs WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    info!("Log deleted returning");

    Ok(())
}
use sqlx::PgPool;
use tracing::{info, error};
use models::Business;
use uuid::Uuid;

pub async fn create_business(pool: &PgPool, business: Business) -> Result<(), sqlx::Error> {
    info!("DB Creating Business");

    sqlx::query("INSERT INTO businesses (id, name, description, owner_id, created_at) VALUES ($1, $2, $3, $4, $5)")
        .bind(business.id)
        .bind(business.name)
        .bind(business.description)
        .bind(business.owner_id)
        .bind(business.created_at)
        .execute(pool)
        .await?;

    info!("DB Updated");

    Ok(())

}

pub async fn get_business_by_id(pool: &PgPool, business_id: Uuid) -> Result<Business, sqlx::Error> {
    info!("DB Fetching Business by ID");

    let business = sqlx::query_as::<_, Business>("SELECT * FROM businesses WHERE id = $1")
                            .bind(business_id)
                            .fetch_one(pool)
                            .await?;

    info!("Business fetched and mapped, returning...");

    Ok(business)
}

pub async fn update_business(pool: &PgPool, id: Uuid, business: Business) -> Result<(), sqlx::Error> {
    info!("DB Update business called");

    if id != business.id {
        error!("ID's do not match exiting");
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query("UPDATE businesses SET name = $1, description = $2 WHERE id = $3")
        .bind(business.name)
        .bind(business.description)
        .bind(business.id)
        .execute(pool)
        .await?;

    info!("Business updated returning...");

    Ok(())
}

pub async fn delete_business(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    info!("DB Delete Business Called");

    sqlx::query("DELETE FROM businesses WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    info!("Business deleted returning");

    Ok(())
}
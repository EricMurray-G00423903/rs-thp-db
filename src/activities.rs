use sqlx::PgPool;
use tracing::{info, error};
use models::Activity;
use uuid::Uuid;

pub async fn create_activity(pool: &PgPool, activity: Activity) -> Result<(), sqlx::Error> {
    info!("DB Creating Activity");

    sqlx::query("INSERT INTO activities (id, business_id, title, location, capacity, time, equipment, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(activity.id)
        .bind(activity.business_id)
        .bind(activity.title)
        .bind(activity.location)
        .bind(activity.capacity)
        .bind(activity.time)
        .bind(activity.equipment)
        .bind(activity.created_at)
        .execute(pool)
        .await?;

    info!("DB Updated");

        Ok(())

}

pub async fn get_user_by_id(pool: &PgPool, activity_id: Uuid) -> Result<Activity, sqlx::Error> {
    info!("DB Fetching Activity by ID");

    let activity = sqlx::query_as::<_, Activity>("SELECT * FROM activities WHERE id = $1")
                            .bind(activity_id)
                            .fetch_one(pool)
                            .await?;


    info!("Activity fetched and mapped, returning...");

    Ok(activity)
}

pub async fn update_activity(pool: &PgPool, id: Uuid, activity: Activity) -> Result<(), sqlx::Error> {
    info!("DB Update user called");

    if id != activity.id {
        error!("ID's do not match exiting");
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query("UPDATE activities SET title = $1, location = $2, capacity = $3, time = $4, equipment = $5 WHERE id = $6")
        .bind(activity.title)
        .bind(activity.location)
        .bind(activity.capacity)
        .bind(activity.time)
        .bind(activity.equipment)
        .execute(pool)
        .await?;

    info!("Activity updated returning...");

    Ok(())
}

pub async fn delete_activity(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    info!("DB Delete Activity Called");

    sqlx::query("DELETE FROM activities WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    info!("Activity deleted returning");

    Ok(())
}
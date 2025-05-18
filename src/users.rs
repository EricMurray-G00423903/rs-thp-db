use sqlx::PgPool;
use tracing::{info, error};
use models::{User, PublicUser};
use uuid::Uuid;

pub async fn create_user(pool: &PgPool, user: User) -> Result<(), sqlx::Error> {
    info!("DB Creating User");

    sqlx::query("INSERT INTO users (id, email, password_hash, created_at) VALUES ($1, $2, $3, $4)")
        .bind(user.id)
        .bind(user.email)
        .bind(user.password_hash)
        .bind(user.created_at)
        .execute(pool)
        .await?;

    info!("DB Updated");

        Ok(())

}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<PublicUser, sqlx::Error> {
    info!("DB Fetching User by ID");

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                            .bind(user_id)
                            .fetch_one(pool)
                            .await?;

    let public_user: PublicUser = user.into();

    info!("User fetched and mapped, returning...");

    Ok(public_user)
}

pub async fn update_user(pool: &PgPool, id: Uuid, user: User) -> Result<(), sqlx::Error> {
    info!("DB Update user called");

    if id != user.id {
        error!("ID's do not match exiting");
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query("UPDATE users SET email = $1, password_hash = $2 WHERE id = $3")
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.id)
        .execute(pool)
        .await?;

    info!("User updated returning...");

    Ok(())
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    info!("DB Delete User Called");

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    info!("User deleted returning");

    Ok(())
}
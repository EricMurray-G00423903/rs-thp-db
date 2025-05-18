use sqlx::PgPool;
use tracing::{info, error};
use models::Booking;
use uuid::Uuid;

pub async fn create_booking(pool: &PgPool, booking: Booking) -> Result<(), sqlx::Error> {
    info!("DB Creating Booking");

    sqlx::query("INSERT INTO bookings (id, user_id, activity_id, booked_at) VALUES ($1, $2, $3, $4)")
        .bind(booking.id)
        .bind(booking.user_id)
        .bind(booking.activity_id)
        .bind(booking.booked_at)
        .execute(pool)
        .await?;

    info!("DB Updated");

        Ok(())

}

pub async fn get_booking_by_id(pool: &PgPool, id: Uuid) -> Result<Booking, sqlx::Error> {
    info!("DB Fetching Booking by ID");

    let booking = sqlx::query_as::<_, Booking>("SELECT * FROM bookings WHERE id = $1")
                            .bind(id)
                            .fetch_one(pool)
                            .await?;


    info!("Booking fetched and mapped, returning...");

    Ok(booking)
}

pub async fn delete_booking(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    info!("DB Delete Booking Called");

    sqlx::query("DELETE FROM bookings WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    info!("Booking deleted returning");

    Ok(())
}
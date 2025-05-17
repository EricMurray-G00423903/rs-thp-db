use sqlx::PgPool;
use tracing::{info, error};

pub async fn init_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Create tables called");

    sqlx::query("CREATE TABLE IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        email TEXT NOT NULL UNIQUE,
        password_hash TEXT NOT NULL,
        created_at TIMESTAMPTZ DEFAULT NOW()
    );
    ")
        .execute(pool)
        .await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS businesses (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        name TEXT NOT NULL,
        description TEXT,
        owner_id UUID REFERENCES users(id) ON DELETE CASCADE,
        created_at TIMESTAMPTZ DEFAULT NOW()
    );
    ")
            .execute(pool)
            .await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS activities (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        business_id UUID REFERENCES businesses(id) ON DELETE CASCADE,
        title TEXT NOT NULL,
        location TEXT NOT NULL,
        capacity INT NOT NULL,
        time TIMESTAMPTZ NOT NULL,
        equipment JSONB DEFAULT '[]',
        created_at TIMESTAMPTZ DEFAULT NOW()
    );    
    ")
            .execute(pool)
            .await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS bookings (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID REFERENCES users(id) ON DELETE CASCADE,
        activity_id UUID REFERENCES activities(id) ON DELETE CASCADE,
        booked_at TIMESTAMPTZ DEFAULT NOW()
    );  
    ")
            .execute(pool)
            .await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS logs (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        level TEXT NOT NULL,
        message TEXT NOT NULL,
        context JSONB DEFAULT '{}',
        created_at TIMESTAMPTZ DEFAULT NOW()
    );
    ")
            .execute(pool)
            .await?;    
    
    info!("Tables created/already exist!");

    Ok(())

}
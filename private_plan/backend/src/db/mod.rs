use sqlx::PgPool;

pub type DbPool = PgPool;

pub async fn create_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn connect() -> sqlx::PgPool {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    sqlx::PgPool::connect(database_url.as_str())
        .await
        .expect("Failed to connect to database")
}

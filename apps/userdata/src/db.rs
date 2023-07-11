pub async fn connect() -> sqlx::MySqlPool {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    sqlx::MySqlPool::connect(database_url.as_str())
        .await
        .expect("Failed to connect to database")
}

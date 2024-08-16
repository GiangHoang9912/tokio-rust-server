use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn initialize_db(dsn: &str, max_connection: u32) -> PgPool {
  let db_pool =
    PgPoolOptions::new().max_connections(max_connection).connect(dsn).await.expect("Failed to connect to Postgres.");

  if !sqlx::migrate!("./migrations").version_exists(1) {
    sqlx::migrate!("./migrations").run(&db_pool).await.expect("Failed to migrate database.");
  }
  db_pool
}

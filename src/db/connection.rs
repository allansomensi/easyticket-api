use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn test_connection() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await?;

    let row: (i32,) = sqlx::query_as("SELECT 1+1").fetch_one(&pool).await?;

    if row == (2,) {
        Ok(())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

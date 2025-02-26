use dotenv::dotenv;
use sqlx::pool::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use sqlx::Postgres;
use std::env;

pub async fn establish_connection() -> Result<Pool<Postgres>, Error> {
    dotenv().ok();
    let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;

    Ok(pool)
}

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    let row: (String,) = sqlx::query_as("SELECT version()").fetch_one(&pool).await?;

    println!("PostgreSQL version: {}", row.0);

    Ok(())
}

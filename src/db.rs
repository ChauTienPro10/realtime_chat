use sqlx::{PgPool, Error};
use std::env;
use dotenvy::dotenv;

pub async fn connect_db() -> Result<PgPool, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPool::connect(&database_url).await?;
    
    println!("Connected to the database successfully!");
    Ok(pool)
}
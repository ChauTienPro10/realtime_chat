mod db;
use db::connect_db;
use tokio;

#[tokio::main]
async fn main() {
    match connect_db().await {
        Ok(_) => println!("Database is ready!"),
        Err(e) => eprintln!("Database connection failed: {:?}", e), 
    }
}
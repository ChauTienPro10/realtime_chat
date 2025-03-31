use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod routes;
mod jwt_utils;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPool::connect(&database_url).await.unwrap();

    let app_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/login", web::post().to(routes::auth::login))
            .route("/register", web::post().to(routes::uer_service::register))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

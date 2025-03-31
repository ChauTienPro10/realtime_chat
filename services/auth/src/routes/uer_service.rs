use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::jwt_utils::hash_password;
use sqlx::PgPool;
use crate::models::account::Account;

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub  struct SignResponse {
    id: i32,
    username: String
}

pub async fn register(db: web::Data<PgPool>, request: web::Json<SignupRequest>) -> impl Responder {
    let password_hash = hash_password(&request.password); 
    let account = Account::new(request.username.clone(), &password_hash);

    let result = sqlx::query!(
        "INSERT INTO accounts (id, username, password) VALUES ($1, $2, $3) RETURNING id",
        account.id,
        account.username,
        account.password
    )
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(SignResponse {id: record.id, username: account.username}),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to create user")
        }
    }
}
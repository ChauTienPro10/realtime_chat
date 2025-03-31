use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::jwt_utils::{verify_password, generate_jwt};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login(db: web::Data<PgPool>, request: web::Json<LoginRequest>) -> impl Responder {
    let user = sqlx::query!("SELECT username, password_hash FROM users WHERE username = $1",request.username
    ).fetch_one(db.get_ref()).await;
    match user {
        Ok(user) => {
            if verify_password(&request.password, &user.password_hash) {
                let token = generate_jwt(&user.username);
                HttpResponse::Ok().json(LoginResponse { token })
            } else {
                HttpResponse::Unauthorized().json("Invalid username or password")
            }
        }
        Err(sqlx::Error::RowNotFound) => HttpResponse::Unauthorized().json("Invalid username or password"),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}



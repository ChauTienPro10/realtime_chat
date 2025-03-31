use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: String, password_hash: &str) -> Self {
        Self {
            id: 0,
            username,
            password_hash: password_hash.to_string(), 
        }
    }
}
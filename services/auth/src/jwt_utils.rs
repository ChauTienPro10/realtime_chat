use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

const SECRET_KEY: &[u8] = b"chauduongphattien22012003";

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = argon2::password_hash::PasswordHash::new(hash).unwrap();
    let argon2 = Argon2::default();
    argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize, // Expiry time (timestamp)
}

pub fn generate_jwt(username: &str) -> String {
    let claims = Claims {
        sub: username.to_string(),
        exp: 10000000000, // Expiry time
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)).unwrap()
}

pub fn validate_jwt(token: &str) -> Option<String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ).ok().map(|data| data.claims.sub)
}

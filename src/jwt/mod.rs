use crate::FromRequest;
use crate::HttpResponse;
use actix_web::HttpRequest;
use chrono::{Duration, Utc};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt;

// For easier return
type JwtResult<T> = std::result::Result<T, JwtError>;

#[derive(Debug)] // Add this line to implement std::fmt::Debug for JwtError
pub struct JwtError(jsonwebtoken::errors::Error);

impl JwtError {}

// Step 2: Implement ResponseError for the new type
impl actix_web::error::ResponseError for JwtError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json("Invalid JWT token")
    }
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JWT error: {}", self.0)
    }
}

impl std::error::Error for JwtError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (usually the user id or username)
    pub exp: usize,  // Expiry
}

impl FromRequest for Claims {
    type Error = actix_web::Error; // Use actix_web::Error to match expected error handling in Actix-web
    type Future = Ready<std::result::Result<Self, Self::Error>>; // Use std::result::Result explicitly

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ");
                    if let Ok(claims) = validate_jwt(token) {
                        return ready(Ok(claims));
                    }
                }
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized("Invalid token")))
    }
}

pub fn generate_jwt(username: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // Token expires after 24 hours
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your_secret_key".as_ref()), // Use a strong secret key
    )
    .map_err(JwtError)?;

    Ok(token)
}

pub fn validate_jwt(token: &str) -> JwtResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("your_secret_key".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(JwtError)
}

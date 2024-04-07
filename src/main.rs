mod db;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{get, http::header, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer};
use chrono::{Duration, Utc};
use db::{create_tables, users::User};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json; // Added line for json! macro
use std::fmt;

use crate::db::posts::{get_all_posts, Post};

// Step 1: Define a new wrapper type for the JWT error
#[derive(Debug)] // Add this line to implement std::fmt::Debug for JwtError
struct JwtError(jsonwebtoken::errors::Error);

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
struct Claims {
    sub: String, // Subject (usually the user id or username)
    exp: usize,  // Expiry
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

fn generate_jwt(username: &str) -> Result<String> {
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

fn validate_jwt(token: &str) -> JwtResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("your_secret_key".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(JwtError)
}

// For easier return
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type JwtResult<T> = std::result::Result<T, JwtError>;

// Main function to start the HTTP server
#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info) // Ensure info level messages are logged
        .init();
    // Initialize the connection pool
    let pool = db::connect_to_db()
        .await
        .expect("Failed to connect to the database");
    create_tables(&pool).await.expect("Failed to create tables");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8000") // Update to match the frontend's origin
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"]) // Include OPTIONS
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(web::resource("/signup").route(web::post().to(signup_handler)))
            .service(web::resource("/newpost").route(web::post().to(newpost_handler)))
            .service(web::resource("/get_posts").route(web::get().to(get_posts)))
            .service(fs::Files::new("/", "frontend/public").index_file("index.html"))
            .default_service(
                web::route().to(|| async { HttpResponse::BadRequest().body("Unexpected route") }),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
    password: String,
}

async fn signup_handler(
    pool: web::Data<sqlx::SqlitePool>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse> {
    info!("Signup handler called");
    let hashed_password = db::users::hash_password(&form.password)?;
    let user = User {
        id: 0, // This will be ignored as the ID is auto-incremented
        username: form.username.clone(),
        password: hashed_password,
    };
    user.insert_user(&pool).await?;
    let token = generate_jwt(&form.username)?;

    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct NewPost {
    content: String,
    title: String,
}

async fn newpost_handler(
    pool: web::Data<sqlx::SqlitePool>,
    form: web::Json<NewPost>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    info!("New post handler called");

    // Extract the token from the Authorization header
    let auth_header = req.headers().get("Authorization");
    let token = match auth_header {
        Some(header_value) => header_value.to_str().unwrap().trim_start_matches("Bearer "),
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    // Validate the token
    let claims = match validate_jwt(token) {
        Ok(claims) => claims,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let date = Utc::now();
    let post = Post {
        context: form.content.clone(),
        date: date.format("%Y-%m-%d").to_string(),
        id: 0,            // This will be ignored as the ID is auto-incremented
        user: claims.sub, // Use the username from the token claims
        title: form.title.clone(),
    };
    post.insert_post(&pool).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({})))
}

// Endpoint to retrieve all posts
async fn get_posts(pool: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse> {
    match get_all_posts(&pool).await {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(err) => {
            println!("Error getting goals: {}", err);
            Err(actix_web::error::ErrorInternalServerError(err).into())
        }
    }
}

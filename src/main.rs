mod db;
mod jwt;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http::header, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer};
use chrono::Utc;
use db::{create_tables, users::User};
use log::info;
use serde::Deserialize;
use serde_json::json; // Added line for json! macro

use crate::{
    db::posts::{get_all_posts, Post},
    jwt::{generate_jwt, validate_jwt},
};

// For easier return
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Struct for getting new user info from the frontend
#[derive(Deserialize)]
struct NewUser {
    username: String,
    password: String,
}

// Post method for getting the new user info from the front end
async fn signup_handler(
    pool: web::Data<sqlx::SqlitePool>,
    form: web::Json<NewUser>, // front end will send this NewUser struct
) -> Result<HttpResponse> {
    info!("Signup handler called");
    let hashed_password = db::users::hash_password(&form.password)?;
    let user = User {
        id: 0, // This will be ignored as the ID is auto-incremented
        username: form.username.clone(),
        password: hashed_password,
    };
    user.insert_user(&pool).await?;
    let token = generate_jwt(&form.username)?; // learn this section more

    Ok(HttpResponse::Ok().json(json!({ "token": token }))) // this also send a token to frontend
}

// Struct for getting new post info from frontend
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
    let auth_header = req.headers().get("Authorization"); // Frontend will send token to backend here. You have to validate it
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

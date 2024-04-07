use bcrypt::{hash, DEFAULT_COST};
use log::{error, info};

// For easier return
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Struct for user
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn insert_user(self, pool: &sqlx::SqlitePool) -> Result<()> {
        let mut conn = pool.acquire().await?;

        let insert_query =
            sqlx::query::<sqlx::Sqlite>("INSERT INTO users (username, password) VALUES (?, ?)")
                .bind(&self.username)
                .bind(&self.password);

        match insert_query.execute(&mut *conn).await {
            Ok(_) => {
                info!("User saved to the database");
            }
            Err(e) => {
                error!("Failed to insert user: {}", e);
                return Err(e.into());
            }
        }

        Ok(())
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(|e| e.into())
}

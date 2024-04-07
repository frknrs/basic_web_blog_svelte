use log::{error, info};
use serde::Serialize;
use sqlx::prelude::FromRow;

// Struct for blog post
#[allow(dead_code)]
#[derive(FromRow, Serialize)]
pub struct Post {
    pub context: String,
    pub date: String,
    pub id: i32,
    pub user: String,
    pub title: String,
}

// For easier return
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Post {
    // Inserting new post to the database
    #[allow(dead_code)]
    pub async fn insert_post(self, pool: &sqlx::SqlitePool) -> Result<()> {
        let mut conn = pool.acquire().await?;

        let insert_query = sqlx::query::<sqlx::Sqlite>(
            "INSERT INTO posts (context, date, user, title) VALUES (?, ?, ?, ?)",
        )
        .bind(&self.context)
        .bind(&self.date)
        .bind(&self.user)
        .bind(&self.title);

        match insert_query.execute(&mut *conn).await {
            Ok(_) => {
                info!("Post saved to the database");
            }
            Err(e) => {
                error!("Failed to insert post: {}", e);
                return Err(e.into());
            }
        }

        Ok(())
    }
}

pub async fn get_all_posts(pool: &sqlx::SqlitePool) -> Result<Vec<Post>> {
    let mut conn = pool.acquire().await?;

    match sqlx::query_as::<_, Post>("SELECT * from posts")
        .fetch_all(&mut *conn)
        .await
    {
        Ok(posts) => {
            info!("Posts received");
            Ok(posts)
        }
        Err(e) => {
            error!("Error while getting the posts");
            Err(e.into())
        }
    }
}

use log::{error, info};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
    Pool, Sqlite, SqlitePool,
};

pub mod posts;
pub mod users;

// For easier return
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn connect_to_db() -> Result<sqlx::SqlitePool> {
    let db_path: &str = "src/db/blog.db";
    let options: SqliteConnectOptions = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let pool: Pool<Sqlite> = SqlitePool::connect_with(options).await?;
    info!("Connected to the database");
    Ok(pool)
}

// Create users and posts tables
pub async fn create_tables(pool: &sqlx::SqlitePool) -> Result<()> {
    let mut conn = pool.acquire().await?;

    let create_posts_table_query = sqlx::query::<sqlx::Sqlite>(
        "
            CREATE TABLE IF NOT EXISTS posts (
            context TEXT NOT NULL,
            date TEXT NOT NULL,
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            user TEXT NOT NULL
            )
        ",
    );

    let create_users_table_query = sqlx::query::<sqlx::Sqlite>(
        "
        CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL,
        password TEXT NOT NULL
        )
    ",
    );

    match create_posts_table_query.execute(&mut *conn).await {
        Ok(_) => (),
        Err(err) => {
            error!("Error while creating the posts table: {}", err);
            return Err(err.into());
        }
    }

    match create_users_table_query.execute(&mut *conn).await {
        Ok(_) => (),
        Err(err) => {
            error!("Error while creating the users table: {}", err);
            return Err(err.into());
        }
    }

    Ok(())
}

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};

// Take only date, format YYYY-MM-DD
pub fn get_date() -> String {
    let local_time: DateTime<Local> = Local::now();

    local_time.naive_local().date().to_string()
}

// Establish connection to the database
pub async fn connect_to_db() -> Result<sqlx::SqlitePool, Box<dyn std::error::Error>> {
    let db_path = "src/goals.db";
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let pool = sqlx::SqlitePool::connect_with(options).await?;

    Ok(pool)
}

// Create table if not exists using SqlitePool
pub async fn create_table_with_pool(
    pool: &sqlx::SqlitePool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;

    let create_table_query = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS goals (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        date TEXT NOT NULL,
        text TEXT NOT NULL
        )
    ",
    );

    match create_table_query.execute(&mut *conn).await {
        Ok(_) => {
            println!("Successfully created table or there is already a table called goals");
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

// Delete all things inside the table
pub async fn clean_table(pool: &sqlx::SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;

    let delete_all_query = sqlx::query("DELETE FROM goals");

    match delete_all_query.execute(&mut *conn).await {
        Ok(_) => {
            println!("Deleted all entries.");
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

// Take all goals from the database using SqlitePool
pub async fn get_all_goals_with_pool(pool: &sqlx::SqlitePool) -> Result<Vec<Goal>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    match sqlx::query_as::<_, Goal>("SELECT * FROM goals")
        .fetch_all(&mut *conn)
        .await
    {
        Ok(goals) => Ok(goals),
        Err(e) => Err(e),
    }
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Goal {
    pub id: i32,
    pub date: String,
    pub text: String,
}

impl Goal {
    // Add a goal using the pool
    pub async fn insert_goal(
        self,
        pool: &sqlx::SqlitePool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        let insert_query = sqlx::query("INSERT INTO goals (date, text) VALUES (?, ?)")
            .bind(&self.date)
            .bind(&self.text);

        match insert_query.execute(&mut *conn).await {
            Ok(_) => {
                println!("Goal successfully inserted into the database");
                Ok(())
            }
            Err(e) => {
                println!("Error while inserting the goal");
                Err(e.into())
            }
        }
    }
}

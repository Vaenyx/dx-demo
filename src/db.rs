use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub description: String,
}

#[cfg(feature = "server")]
thread_local! {
    static DB: rusqlite::Connection = {
        let db = rusqlite::Connection::open("posts.db").expect("Cannot open posts.db");
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT NOT NULL
            );",
        ).expect("Cannot create posts table");
        db
    };
}

#[get("/api/posts")]
pub async fn list_posts() -> Result<Vec<Post>> {
    DB.with(|db| {
        let mut statement = db.prepare("SELECT id, title, description FROM posts ORDER BY id DESC")?;
        let rows = statement.query_map([], |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    })
}

#[post("/api/posts/create")]
pub async fn create_post(title: String, description: String) -> Result<()> {
    let title = title.trim();
    let description = description.trim();

    if title.is_empty() || description.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Title and description are required",
        ).into());
    }

    DB.with(|db| {
        db.execute(
            "INSERT INTO posts (title, description) VALUES (?1, ?2)",
            rusqlite::params![title, description],
        )
    })?;
    Ok(())
}

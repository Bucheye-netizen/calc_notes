use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::env;

///# Usage
/// Provides an interface to the sqlite database,
/// allowing get, update, delete, and insert methods.
pub struct ModelController {
    //Connection to the database
    pool: SqlitePool,
}

impl ModelController {
    /// # Usage
    /// Creates a new model controller.
    pub async fn new() -> Result<Self> {
        Ok(ModelController {
            pool: SqlitePool::connect(&env::var("DATABASE_URL")?).await?,
        })
    }

    pub fn pool<'a>(&'a self) -> &'a SqlitePool {
        &self.pool
    }
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Note {
    title: String,
    author: String,
    source: String,
    pub_date: u32,
}

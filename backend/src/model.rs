use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::{env, collections::HashMap, sync::Arc, any::Any};
use backend_derive::{self, Table};

// impl Table for Note {
//    fn scaffold<T>() -> Arc<HashMap<String, SqliteType>> {
//         NOTE_SCAFFOLD.clone()
//     }

//     fn get<T>(&self, col: String) -> anyhow::Result<Box<dyn Any>> { 
        

//         Ok(Err(anyhow::anyhow!("unimplemented")))
//     }
// }
 
enum SqliteType {
    Text, 
    Real, 
    Integer,
}

fn some_val () -> bool { true }

/// # Goal 
/// I want to use this trait to access values 
/// via their column name. Similarly, I want access
/// to a scaffold of the table, probably a constant 
/// map in the form `HashMap<String, SqliteType>`
/// where SqliteType is an enum of one of three types: 
/// `Text, Integer, Real, Blob`. 
trait Table {
    fn scaffold<T>() -> Arc<HashMap<String, SqliteType>>; 

    fn get<T>(&self, col: &str) -> anyhow::Result<&T>;

    fn value(&self, col: &str)  -> anyhow::Result<serde_json::Value>;
}


#[derive(sqlx::FromRow, Deserialize, Serialize, Table)]
pub struct Note {
    title: String,
    author: String,
    source: String,
    pub_date: u64,
}

pub struct TableFilter { 

}
pub struct Updater {
    /// # Usage
    /// The list of columns in the "SET" branch of an SQL statement. 
    /// The first tuple represents the name of the column and the latter
    /// the new value. 
    set: Vec<(String, serde_json::Value)>, 
    at: TableFilter, 
}

impl Updater {

    fn validate<T: Table>() {

    }
}

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

    pub async fn update() {

    }
}

// /// Generally, this would need a more obscure name to prevent irritating collisions. 
// static _NOTE_SCAFFOLD: Lazy<Arc<HashMap<String, SqliteType>>> = Lazy::new(|| {
//     Arc::new(HashMap::new())
// });
 

// impl Table for Note {
//     fn scaffold<T>() -> Arc<HashMap<String, SqliteType>> {
//         _NOTE_SCAFFOLD.clone()
//     }

//     fn get<T>(&self, col: &str) -> anyhow::Result<&T> {
//         let out: Box<dyn Any>;

//         if col == "title" { 
//             out = Box::new(&self.title);
//             return out.downcast_ref::<T>().ok_or(anyhow::anyhow!("Provided type is invalid!"));
//         } else {
//             out = Box::new(&self.pub_date);
//         }

//         return out.downcast_ref().ok_or(anyhow::anyhow!("Invalid type!"))
//     }

//     fn value(&self, col: &str)  -> anyhow::Result<serde_json::Value> {
//         return Ok(serde_json::Value::Bool(false))
//     }
// }

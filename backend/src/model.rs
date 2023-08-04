use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::sqlite::SqlitePool;
use std::{env, collections::HashMap, sync::Arc, any::Any};
use backend_derive::{self, Table};

 
pub enum SqliteType {
    Text, 
    Real, 
    Integer,
}

pub trait Table {
    /// # Usage
    /// Returns a map of field definitions. 
    fn fields() -> Arc<HashMap<String, SqliteType>> {
        let map: HashMap<String, SqliteType> = [
            ("something random".to_string(), SqliteType::Integer),
        ].into_iter().collect::<HashMap<String, SqliteType>>();

        return Arc::new(HashMap::new());
    }

    fn name() -> &'static str;
}


#[derive(sqlx::FromRow, Deserialize, Serialize, Table)]
pub struct Note {
    title: String,
    author: String,
    source: String,
    pub_date: i64,
}

#[derive(Serialize, Deserialize)]
struct FieldCondition(String, String, Value);

impl FieldCondition {
    fn name(&self) -> &str { self.0.as_str() }

    fn op(&self) -> &str { self.1.as_str() }

    fn value(&self) -> &Value { &self.2 }

    fn valid<T: Table>(&self) -> bool {
        if match self.op() {
            "=" => true, 
            ">=" => true, 
            "<=" => true, 
            "<" => true, 
            ">" => true, 
            "!=" => true, 
            _=> false, 
        } {
            return T::fields().contains_key(self.name()); 
        }

        return false; 
    }
}


/// # Usage 
/// Provides the some of flexiblity  of the sqlite WHERE clause in 
/// JSON form. 
/// 
/// Serializes to JSON as follows: 
/// ```Javascript
/// [
///     [["FIELD", "OPERATOR", CONSTANT], "GATE"],
///     [["FIELD", "OPERATOR", CONSTANT], ""]
/// ]
/// ```
/// The last "GATE" variable must always be empty.
#[derive(Deserialize, Serialize)]
pub struct TableFilter(Vec<(FieldCondition, String)>);

impl TableFilter {
    fn expr(&self) -> &Vec<(FieldCondition, String)> { return &self.0; }

    fn valid<T: Table>(&self) -> bool {
        if self.expr().len() == 0 { return true; }
        
        for cond in &self.expr()[..self.expr().len()-1] {
            if !cond.0.valid::<T>() { return false; } 

            if !match cond.1.as_str() {
                "OR" => true, 
                "AND" => true, 
                _ => false, 
            } { return false; }
        }

        if 
            !self.expr().last().unwrap().0.valid::<T>()
            || !self.expr().last().unwrap().1.is_empty()
        { return false; }

        return true;
    }

    /// # Usage
    /// Generates incomplete sql code for a WHERE clause. 
    /// Values still need to be bound
    fn sql(&self) -> String {
        if self.expr().len() == 0 { return String::new(); }
        
        let mut sql = String::new();
        sql.push_str("WHERE ");

        for cond in &self.expr()[0..self.expr().len() - 1] { 
            sql.push_str(format!("{} {} ? {} ", cond.0.name(), cond.0.op(), cond.1).as_str())
        }

        sql.push_str(
            format!("{} {} ?", 
            self.expr().last().unwrap().0.name(), 
            self.expr().last().unwrap().0.op()
        ).as_str());

        return sql;
    }
}



/// # Usage
/// Locates and updates a set of rows in the databse.
/// Serializes into JSON as follows: 
/// ```javascript
/// {
///     "set": 
///     {
///         "body": "Updated body",
///         "pub_date": -2000
///     }, 
///     "at":  [
///         [["title", "=", "Test"], ""]
///     ]
/// }
/// ```
#[derive(Deserialize, Serialize)]
pub struct Updater {
    /// # Usage
    /// The list of columns in the "SET" branch of an SQL statement. 
    /// The first tuple represents the name of the column and the latter
    /// the new value. 
    pub set: HashMap<String, Value>, 
    at: TableFilter, 
}

impl Updater {
    fn valid<T: Table>(&self) -> bool {
        if !self.at.valid::<T>() { return false; }
        if self.set.len() == 0 { return false; }

        for val in &self.set {
            if !T::fields().contains_key(val.0) { return false; }
        }

        return true;
    }

    
    /// # Usage 
    /// Creates incomplete SQL in the form
    /// ```SQL
    /// UPDATE table 
    /// SET COLUMN1 = ?, COLUMN2 = ? 
    /// WHERE COLUMN5 OPERATOR ?
    /// ```
    fn sql<T: Table>(&self) -> String {
        let mut sql = String::new();
        sql.push_str(format!("UPDATE {} ", T::name()).as_str());
        sql.push_str("SET ");

        for value in &self.set {
            sql.push_str(format!("{} = ?, ", value.0).as_str());
        }
        sql.remove(sql.len() - 2);

        sql.push_str(self.at.sql().as_str());

        return sql;
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

    pub fn pool(&self) -> &SqlitePool { &self.pool }

    pub async fn update<T: Table>(&self, updater: &Updater) -> Result<()> {
        if !updater.valid::<T>() { return Err(anyhow!("Invalid updater")) }

        let mut conn = self.pool.acquire().await?;

        let query_str = updater.sql::<T>();
        log::info!("query_str: \n {}", query_str);
        let mut query = sqlx::query(query_str.as_str());

        for cond in updater.at.expr() {
            query = match T::fields()[cond.0.name()] {
                SqliteType::Integer => query.bind(
                    cond.0.value().as_i64().ok_or(anyhow!("Invalid type"))?
                ),
                SqliteType::Real => query.bind(
                    cond.0.value().as_f64().ok_or(anyhow!("Invalid type"))?
                ),
                SqliteType::Text => query.bind(
                    cond.0.value().as_str().ok_or(anyhow!("Invalid type"))?
                ),
            };
        }

        let out = query.execute(&mut conn).await?;
        
        return Ok(());
    }
}

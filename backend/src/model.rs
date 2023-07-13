use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number};
use sqlx::sqlite::SqliteRow;
use sqlx::{sqlite::SqlitePool, Column, Row};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::collections::HashSet;
use tokio_stream::StreamExt;

/// # Usage 
/// Defines various SQLite types.
pub enum SqliteType {
    Number,
    Text,
    Real,
    PrimaryKey,
}

pub trait Table {
    /// # Usage
    /// Gives the name of some table.
    fn name() -> &'static str;

    ///# Usage
    /// Provides field details.
    /// First value is the field name. Second value is the field type.
    fn fields() -> Arc<HashMap<String, SqliteType>>;
}

/// # Usage
/// A condition a particular field must pass.
#[derive(Deserialize, Serialize)]
struct FieldCondition(String, String, serde_json::Value); 
    // ///# Usage
    // /// Name of manipualted field
    // pub name: String,
    // /// # Usage
    // /// Operator that the field must satisfy relative to the `value`.
    // /// For example, `>= value`.
    // pub operator: String,
    // pub value: serde_json::Value,

impl FieldCondition {
    fn name<'a>(&'a self) -> &'a str { &self.0 }
    fn operator<'a>(&'a self) -> &'a str { &self.1 }
    fn value<'a>(&'a self) -> &'a serde_json::Value { &self.2 }

    /// # Usage
    /// Checks if the given field condition is valid
    pub fn valid<T: Table>(&self) -> bool {
        if !match self.operator() {
            "=" => true,
            ">=" => true,
            ">" => true,
            "<" => true,
            "<=" => true,
            "!=" => true,
            _ => false,
        } {
            return false;
        }

        return T::fields().contains_key(self.name());
    }
}

///# Usage
/// Provides specific filters that a set
/// of columns must pass for a  query
/// to operate on them.
/// 
/// Serialized into JSON as follows: 
/// ```javascript
/// {
///     "table": "Notes",
///     "conds": 
///         [
///             [["id", "=", 1], "AND" [... some other cond]],
///         ],
/// } 
/// ```
#[derive(Serialize, Deserialize)]
pub struct TableFilter {
    /// List of conditions that the column must satisfy.
    /// The first value contains a single boolean operation,
    /// such as: "count > 2". The latter provides logic gates
    /// such as AND.
    conds: Vec<(FieldCondition, String)>,
    table: String,
}

impl TableFilter {
    fn validate<T: Table>(&self) -> bool {
        if self.conds.len() == 0 {
            return true;
        }

        for cond in &self.conds[..self.conds.len() - 1] {
            if !cond.0.valid::<T>() {
                return false;
            }

            if !match cond.1.as_str() {
                "AND" => true,
                "OR" => true,
                "NOT" => true,
                _ => false,
            } {
                return false;
            }
        }

        if !self.conds.last().unwrap().0.valid::<T>() {
            return false;
        }
        if !self.conds.last().unwrap().1.is_empty() {
            return false;
        }

        return true;
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

    /// # Usage
    /// Executes a get on the connected database
    /// that returns the rows specificied by `details: TableFilter`
    pub async fn get<T: Table>(&self, details: &TableFilter, cols: &HashSet<String>) -> Result<Vec<serde_json::Value>> {
        for col in cols {
            if !T::fields().contains_key(col) { 
                return Err(anyhow!("Requesting non-existant field!")); 
            }
        }

        if !&details.validate::<T>() {
            return Err(anyhow!("TableFilter invalid for {} table", T::name()));
        }

        let mut output: Vec<serde_json::Value> = Vec::new();
        let mut query;
        let mut query_str;

        if details.conds.len() == 0 {
            //Seperate query without where clause
            query_str = format!("SELECT * FROM {}", T::name());
            query = sqlx::query(&query_str);
        } else {
            //Creating a prepared query string
            query_str = String::new();

            let header_str = format!("SELECT * FROM {} WHERE", T::name());
            query_str.push_str(&format!(
                "{} {} {} ? {} ",
                header_str,
                details.conds[0].0.name(),
                details.conds[0].0.operator(),
                details.conds[0].1
            ));
            for cond in &details.conds[1..] {
                query_str.push_str(&format!(
                    "{} {} ? {} ",
                    cond.0.name(), cond.0.operator(), cond.1
                ));
            }

            // Creating an executing query to database
            query = sqlx::query(&query_str);

            for cond in &details.conds {
                query = match T::fields()[cond.0.name()] {
                    SqliteType::Number => query.bind(
                        cond.0
                            .value()
                            .as_i64()
                            .ok_or(anyhow!("Couldn't convert condition into correct type!"))?,
                    ),
                    SqliteType::Text => query.bind(
                        cond.0
                            .value()
                            .as_str()
                            .ok_or(anyhow!("Couldn't convert condition into correct type!"))?
                    ),
                    SqliteType::Real => query.bind(
                        cond.0
                            .value()
                            .as_f64()
                            .ok_or(anyhow!("Couldn't convert condition into correct type!"))?,
                    ),
                    SqliteType::PrimaryKey => query.bind(
                        cond.0
                            .value()
                            .as_i64()
                            .ok_or(anyhow!("Couldn't convert condition into correct type!"))?,
                    ),
                };
            }
        }
        let mut conn = self.pool.acquire().await?;

        let mut rows = query
            .map(|x: SqliteRow| {
                // let mut map: HashMap<String, String> = HashMap::new();
                let mut values: Map<String, serde_json::Value> = Map::new();

                for column in x.columns() {
                    // map.insert(column.name().to_string(), x.get(column.ordinal()));
                    values.insert(column.name().to_string(), {
                        match T::fields()[column.name()] {
                            SqliteType::Number => {
                                let val: i32 = x.get(column.ordinal());
                                serde_json::Value::Number(Number::from_f64(val as f64).unwrap())
                            },
                            SqliteType::Text => {
                                let val: String = x.get(column.ordinal());
                                serde_json::Value::String(val)
                            },
                            SqliteType::Real => {
                                let val: f64 = x.get(column.ordinal());
                                serde_json::Value::Number(Number::from_f64(val).unwrap())
                            },
                            SqliteType::PrimaryKey => {
                                let val: i64 = x.get(column.ordinal());
                                serde_json::Value::Number(Number::from_f64(val as f64).unwrap())
                            },
                        }
                    });
                }
                serde_json::Value::Object(values)
            })
            .fetch(&mut conn);
        output.reserve(rows.size_hint().1.unwrap_or(rows.size_hint().0));

        while let Some(row) = rows.try_next().await? {
            output.push(row);
        }

        Ok(output)
    }
}

pub mod table {
    use super::{SqliteType, Table};
    use once_cell::sync::Lazy;
    use std::{collections::HashMap, sync::Arc};

    static NOTE_FIELDS: Lazy<Arc<HashMap<String, SqliteType>>> = Lazy::new(|| {
        Arc::new(HashMap::from([
            ("title".to_string(), SqliteType::Text),
            ("author".to_string(), SqliteType::Text),
            ("source".to_string(), SqliteType::Text),
            ("input".to_string(), SqliteType::Text),
            ("pub_date".to_string(), SqliteType::Number),
        ]))
    });

    pub struct Notes;

    impl Table for Notes {
        fn name() -> &'static str {
            return "notes";
        }

        ///# Usage
        /// First value is the field name, second is the field type.
        fn fields() -> Arc<HashMap<String, SqliteType>> {
            return NOTE_FIELDS.clone();
        }
    }
}

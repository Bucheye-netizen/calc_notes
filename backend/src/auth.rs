use std::sync::Arc;

use crate::model::ModelController;
use anyhow::{anyhow, Result};
use axum::{extract::State, Json, Router, routing, http::StatusCode};
use axum_login::{
    extractors::AuthContext, memory_store::MemoryStore as AuthMemoryStore, secrecy::SecretVec,
    AuthUser,
};
use log::info;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use rand_core::OsRng;
use sqlx::{sqlite::SqliteRow, FromRow, Row};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Role {
    User,
    Admin,
    Owner,
}

impl Role {
    fn as_u32(&self) -> u32 {
        match self {
            Role::User => 0,
            Role::Admin => 1,
            Role::Owner => 2,
        }
    }

    fn from_u32(val: u32) -> Self {
        match val {
            0 => Role::User,
            1 => Role::Admin,
            2 => Role::Owner,
            _ => panic!("Unexpected role value!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    id: i64,
    name: String,
    password_hash: String,
    role: Role,
}

impl User {
    /// # Usage
    /// Creates a new user, salting and hashing their password
    /// and adds them to the databse.
    pub async fn new(
        mc: &ModelController,
        name: String,
        password: String,
        role: Role,
    ) -> Result<Self> {
        let salt = SaltString::generate(&mut OsRng);
        let parsed_hash = Pbkdf2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| anyhow!("Authentication hashing failed"))?;
        
        let mut conn = mc.pool().acquire().await?;
        let res = sqlx::query(
            "
            INSERT INTO users (name, password_hash, role)
            VALUES(?, ?, ?)
        ",
        )
        .bind(&name)
        .bind(&parsed_hash.to_string())
        .bind(role.as_u32())
        .execute(&mut conn)
        .await
        .map_err(|_| anyhow!("Failed to place user in database"))?;

        return Ok(User {
            id: res.last_insert_rowid(),
            name,
            password_hash: parsed_hash.to_string(),
            role,
        });
    }

    /// # Usage
    /// Query user information based on the given password and username.
    pub async fn query(mc: &ModelController, name: &String, password: &String) -> Result<Self> {
        let mut conn = mc.pool().acquire().await?;

        // Getting user from database
        let user = User::from_row(
            &sqlx::query(
                "
                SELECT * FROM users
                WHERE 
                    name = ?
            ",
            )
            .bind(name)
            .fetch_one(&mut conn)
            .await?,
        )?;

        // Checking password hash
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| anyhow!("Failed to parse password hash"))?;

        if Pbkdf2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Err(anyhow!("Failed to login user! Invalid password"));
        }
        return Ok(user);
    }
}

impl<'r> FromRow<'r, SqliteRow> for User {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        return Ok(User {
            id: row.get("id"),
            name: row.get("name"),
            password_hash: row.get("password_hash"),
            role: Role::from_u32(row.get("role")),
        });
    }
}

impl AuthUser<i64, Role> for User {
    fn get_id(&self) -> i64 {
        return self.id;
    }

    fn get_password_hash(&self) -> axum_login::secrecy::SecretVec<u8> {
        return SecretVec::new(self.password_hash.clone().into());
    }

    fn get_role(&self) -> Option<Role> {
        return Some(self.role.clone());
    }
}

type Auth = AuthContext<i64, User, AuthMemoryStore<i64, User>, Role>;

pub fn routes(mc: Arc<ModelController>) -> Router {
    return Router::new()
        .route("/login", routing::post(login))
        .route("/logout", routing::get(logout))
        .with_state(mc);
}

async fn login(
    mut auth: Auth,
    State(mc): State<Arc<ModelController>>,
    Json((name, password)): Json<(String, String)>,
) -> Result<(), StatusCode> {
    info!("{:<12} -> auth::login", "ROUTE");

    auth.login(
        &User::query(&mc, &name, &password)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

async fn logout(mut auth: Auth) {
    info!("{:<12} -> auth::logout", "ROUTE");
    auth.logout().await;
}
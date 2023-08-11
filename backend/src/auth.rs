use crate::model::ModelController;
use anyhow::{anyhow, Result};
use axum_login::{secrecy::SecretVec, AuthUser, RequireAuthorizationLayer, extractors::AuthContext, SqliteStore};
use password_hash::{PasswordHasher, PasswordVerifier, SaltString, PasswordHash};
use pbkdf2::Pbkdf2;
use rand_core::OsRng;
use sqlx::{sqlite::SqliteRow, FromRow, Row};

pub type Auth = AuthContext<i64, User, SqliteStore<User, Role>, Role>;

pub type RequireAuth = RequireAuthorizationLayer<i64, User, Role>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Role {
    User,
    Admin,
    Owner,
}

impl Role {
    pub fn as_u32(&self) -> u32 {
        match self {
            Role::User => 0,
            Role::Admin => 1,
            Role::Owner => 2,
        }
    }

    pub fn from_u32(val: u32) -> Self {
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
    pub id: i64,
    pub name: String,
    pub role: Role,
    password_hash: String,
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
            INSERT INTO UserTable (name, password_hash, role)
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
    /// If the user's password is legitimate, then the method returns
    /// the user. Otherwise, it returns an error.
    pub async fn query(mc: &ModelController, name: &String, password: &String) -> Result<Self> {
        let mut conn = mc.pool().acquire().await?;
        // Getting user from database
        let user = User::from_row(
            &sqlx::query(
                "
                SELECT 
                    * 
                FROM 
                    UserTable
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

use axum::Router;
use axum_login::AuthLayer;
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    SqliteStore
};

use log::info;
use rand_core::{OsRng, RngCore};
use reqwest::header::CONTENT_TYPE;
use std::env;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;

pub mod model;
pub mod web;

#[allow(dead_code)]
pub mod auth;

use crate::auth::{User, Role};
use crate::model::ModelController;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = Arc::new(ModelController::new().await?);

    // let secret: [u8; 64] = rand::thread_rng().gen();
    let mut secret: [u8; 64] = [0; 64];
    OsRng::fill_bytes(&mut OsRng, &mut secret);

    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret);

    let store: SqliteStore<User, Role> = SqliteStore::new(mc.pool().clone());
    let auth_layer = AuthLayer::new(store, &secret);

    dotenvy::dotenv().ok();
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let cors = CorsLayer::new()
        .allow_origin([
            env::var("FRONTEND_URL").expect("Set FRONTEND_URl environment variable").parse().unwrap(),
            env::var("ADMIN_URL").expect("Set FRONTEND_URl environment variable").parse().unwrap(),
        ])
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE]);

    let route = Router::new()
        .nest("/api/data", web::data::routes(mc.clone()))
        .nest("/api/auth", auth::routes(mc.clone()))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(cors);

    info!("Starting server at socket address {}", addr);
    axum::Server::bind(&addr)
        .serve(route.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
}

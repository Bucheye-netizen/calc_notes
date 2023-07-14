use axum::{http::HeaderValue, Router};
use axum_login::AuthLayer;
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    memory_store::MemoryStore as AuthMemoryStore,
};
use log::info;
use rand_core::{OsRng, RngCore};
use std::collections::HashMap;
use std::env;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

pub mod model;
pub mod web;

#[allow(dead_code)]
pub mod auth;

use crate::auth::User;
use crate::model::ModelController;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // let secret: [u8; 64] = rand::thread_rng().gen();
    let mut secret: [u8; 64] = [0; 64];
    OsRng::fill_bytes(&mut OsRng, &mut secret);

    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret);

    let store_map: HashMap<i64, User> = HashMap::new();
    let store = Arc::new(RwLock::new(store_map));

    let user_store = AuthMemoryStore::new(&store);
    let auth_layer = AuthLayer::new(user_store, &secret);

    dotenvy::dotenv().ok();
    env_logger::init();

    let mc = Arc::new(ModelController::new().await?);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let cors = CorsLayer::new().allow_origin(
        env::var("FRONTEND_URL")
            .expect("Set FRONTEND_URl environment variable")
            .parse::<HeaderValue>()
            .unwrap(),
    );

    let route = Router::new()
        .nest("/api/data", web::data::routes(mc.clone()))
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

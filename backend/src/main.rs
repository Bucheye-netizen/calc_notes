use axum::Router;
use axum_login::axum_sessions::SameSite;
use axum_login::AuthLayer;
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    SqliteStore,
};
use reqwest::header::{CONTENT_TYPE, SET_COOKIE};
use std::env;
use tower::ServiceBuilder;

use log::info;
use rand_core::{OsRng, RngCore};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;

#[allow(unused)]
pub mod model;
pub mod web;

#[allow(dead_code)]
pub mod auth;

use crate::auth::{Role, User};
use crate::model::ModelController;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let mc = Arc::new(ModelController::new().await?);

    // let secret: [u8; 64] = rand::thread_rng().gen();
    let mut secret: [u8; 64] = [0; 64];
    OsRng::fill_bytes(&mut OsRng, &mut secret);

    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_secure(true)
        .with_same_site_policy(SameSite::None);
    let store: SqliteStore<User, Role> =
        SqliteStore::new(mc.pool().clone()).with_query("SELECT * FROM UserTable WHERE id = ?");
    let auth_layer = AuthLayer::new(store, &secret);
    let cors_layer = CorsLayer::new()
        .allow_credentials(true)
        .allow_origin([
            env::var("ADMIN_URL")
                .expect("Set FRONTEND_URL environment variable")
                .parse()
                .unwrap(),
            env::var("FRONTEND_URL")
                .expect("Set FRONTEND_URL environment variable")
                .parse()
                .unwrap(),
        ])
        .allow_headers([CONTENT_TYPE, SET_COOKIE]);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let layers = ServiceBuilder::new()
        .layer(cors_layer)
        .layer(session_layer)
        .layer(auth_layer);

    let route = Router::new()
        .nest("/data", web::data::routes(mc.clone()))
        .nest("/auth", auth::routes(mc.clone()))
        .layer(layers);

    info!("Starting server at socket address {}", addr);
    //TODO: Implement HTTPS with rustls
    axum_server::bind(addr)
        .serve(route.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
}

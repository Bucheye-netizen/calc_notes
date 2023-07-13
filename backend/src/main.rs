use axum::Router;
use log::info;
use std::{net::SocketAddr, sync::Arc};

pub mod model;
pub mod web;

use crate::model::ModelController;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let mc = Arc::new(ModelController::new().await?);

    let route = Router::new().nest("/api/data", web::data::routes(mc.clone()));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Starting server at socket address {}", addr);
    axum::Server::bind(&addr)
        .serve(route.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
}

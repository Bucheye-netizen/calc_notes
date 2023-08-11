use std::sync::Arc;
use axum::{Router, routing, extract::State, Json};
use log::{info, log_enabled, Level};
use reqwest::StatusCode;
use crate::{auth::{User, Auth}, model::ModelController};

pub fn routes(mc: Arc<ModelController>) -> Router {
    return Router::new()
        .route("/login", routing::post(login))
        .route("/logout", routing::get(logout))
        .route("/status", routing::get(status))
        .with_state(mc);
}

async fn login(
    mut auth: Auth,
    State(mc): State<Arc<ModelController>>,
    Json((name, password)): Json<(String, String)>,
) -> Result<(), StatusCode> {
    info!("{:<12} -> auth::login", "ROUTE");
    auth.login(&User::query(&mc, &name, &password).await.map_err(|x| {
        info!("ERROR auth::login: {}", x);
        StatusCode::INTERNAL_SERVER_ERROR
    })?)
    .await
    .map_err(|x| {
        info!("ERROR auth::login: {}", x);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    return Ok(());
}

async fn logout(mut auth: Auth) {
    if log_enabled!(Level::Info) {
        let string = match &auth.current_user {
            Some(u) => &u.name,
            None => "NONE",
        };

        info!("{:<12} -> auth::logout: {}", "ROUTE", string);
    }

    auth.logout().await;
}

async fn status(auth: Auth) -> Json<u32> {
    info!("{:<12} -> auth::status", "ROUTE");

    return Json(match auth.current_user {
        None => 0,
        Some(user) => user.role.as_u32(),
    });
}
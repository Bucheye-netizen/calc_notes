use crate::{
    auth::{RequireAuth, Role},
    model::{ModelController, Note, SqliteType, Table},
};
use axum::{routing, Json, Router};
use std::{collections::HashMap, sync::Arc};

pub fn routes(mc: Arc<ModelController>) -> Router {
    return Router::new()
        .route("/tables", routing::get(tables))
        .route_layer(RequireAuth::login_with_role(Role::Admin..))
        .nest("/notes", notes::route(mc.clone()));
}

/// # Usage
/// Returns a list of table fields for
/// all available tables.
async fn tables() -> Json<Vec<Arc<HashMap<String, SqliteType>>>> {
    let mut out = Vec::new();
    out.push(Note::fields());
    return Json(out);
}

/// Routes for notes
mod notes {
    use crate::{
        auth::{RequireAuth, Role},
        model::{ModelController, Note, Updater},
    };
    use axum::{extract::Path, extract::State, http::StatusCode, routing, Json, Router};
    use log::{info, warn};
    use sqlx::FromRow;
    use std::sync::Arc;

    pub fn route(mc: Arc<ModelController>) -> Router {
        Router::new()
            .route("/patch", routing::patch(patch))
            .route_layer(RequireAuth::login_with_role(Role::Admin..))
            .route("/get/:title", routing::get(get))
            .route("/get", routing::get(all))
            .with_state(mc)
    }

    async fn get(
        State(mc): State<Arc<ModelController>>,
        Path(title): Path<String>,
    ) -> Result<Json<Note>, StatusCode> {
        info!("{:<12} -> notes::get", "ROUTE");
        let mut conn = mc
            .pool()
            .acquire()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let note = sqlx::query(
            "
            SELECT 
                title, author, source, pub_date 
            FROM 
                NoteTable
            WHERE 
                title = ? COLLATE NOCASE
        ",
        )
        .bind(title)
        .fetch_one(&mut conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(Json(
            Note::from_row(&note).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        ));
    }

    async fn all(State(mc): State<Arc<ModelController>>) -> Result<Json<Vec<Note>>, StatusCode> {
        info!("{:<12} -> notes::all", "ROUTE");

        let mut conn = mc
            .pool()
            .acquire()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let notes = sqlx::query(
            "
            SELECT 
                title, author, source, pub_date 
            FROM 
                NoteTable
        ",
        )
        .fetch_all(&mut conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(Json(
            notes
                .iter()
                .map(|r| Note::from_row(r).expect("Failed to map note from row"))
                .collect::<Vec<Note>>(),
        ));
    }

    async fn patch(
        State(mc): State<Arc<ModelController>>,
        Json(updater): Json<Updater>,
    ) -> Result<(), StatusCode> {
        info!("{:<12} -> notes::update", "ROUTE");
        mc.update::<Note>(&updater).await.map_err(|x| {
            warn!("Error occurred while updating a note: {}", x);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        return Ok(());
    }
}

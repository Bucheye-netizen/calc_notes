use crate::model::ModelController;
use axum::Router;
use std::sync::Arc;

pub fn routes(mc: Arc<ModelController>) -> Router {
    Router::new().nest("/notes", notes::route(mc.clone()))
}

/// Routes for notes
mod notes {
    use crate::model::{ModelController, Note};
    use axum::{extract::Path, extract::State, http::StatusCode, routing, Json, Router};
    use log::info;
    use sqlx::FromRow;
    use std::sync::Arc;

    pub fn route(mc: Arc<ModelController>) -> Router {
        Router::new()
            .route("/get/:title", routing::get(get))
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
                notes
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
}

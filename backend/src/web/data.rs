use crate::model::ModelController;
use axum::Router;
use std::sync::Arc;

pub fn routes(mc: Arc<ModelController>) -> Router {
    Router::new().nest("/notes", notes::route(mc.clone()))
}

/// Routes for notes
mod notes {
    use crate::model::{table, ModelController, TableFilter};
    use axum::{extract::State, 
        http::StatusCode, 
        response::IntoResponse, 
        routing, 
        Json, 
        Router,
        Path,
    };
    use once_cell::sync::Lazy;
    use log::{info, warn};
    use std::{sync::Arc, collections::HashSet};

    pub fn route(mc: Arc<ModelController>) -> Router {
        Router::new()
            .route("/get", routing::get(get))
            .with_state(mc)
    }

    //Legal fields 
    static LEGAL_FIELDS: Lazy<Arc<HashSet<String>>> = Lazy::new(|| {
        Arc::new(HashSet::from([
            "title".to_string(),
            "author".to_string(), 
            "source".to_string(),
        ]))
    });


    async fn get(
        State(mc): State<Arc<ModelController>>,
        Path(title): Path<String>,
    ) -> impl IntoResponse {
        info!("{:<12} -> notes::get", "ROUTE");

        match mc.get::<table::Notes>(&filter, &LEGAL_FIELDS).await {
            Ok(v) => Ok(Json(v)),
            Err(value) => {
                warn!(
                    "User attempted to access note table improperly, resulting in the following error:
                    {}", 
                    value.to_string()
                );
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

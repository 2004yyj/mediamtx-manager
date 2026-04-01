use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, patch};
use axum::{Json, Router};
use mediamtx_manager_core::{AppState, GlobalConfig};

use crate::error::AppError;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/config/global", get(get_global))
        .route("/config/global", patch(patch_global))
}

async fn get_global(
    State(state): State<Arc<AppState>>,
) -> Result<Json<GlobalConfig>, AppError> {
    let config = state.api_client.get_global_config().await?;
    Ok(Json(config))
}

async fn patch_global(
    State(state): State<Arc<AppState>>,
    Json(config): Json<GlobalConfig>,
) -> Result<(), AppError> {
    state.api_client.patch_global_config(&config).await?;
    Ok(())
}

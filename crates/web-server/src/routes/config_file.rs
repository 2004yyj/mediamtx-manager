use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, put};
use axum::{Json, Router};
use mediamtx_manager_core::AppState;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Serialize)]
struct ConfigFileResponse {
    content: String,
}

#[derive(Deserialize)]
struct ConfigFileRequest {
    content: String,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/config/file", get(read_file))
        .route("/config/file", put(write_file))
}

async fn read_file(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ConfigFileResponse>, AppError> {
    let content = state.config_file_manager.read_as_string().await?;
    Ok(Json(ConfigFileResponse { content }))
}

async fn write_file(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ConfigFileRequest>,
) -> Result<(), AppError> {
    state.config_file_manager.write_string(&req.content).await?;
    Ok(())
}

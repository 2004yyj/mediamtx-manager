use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use mediamtx_manager_core::{AppState, BinaryInfo, ProcessStatus};
use serde::Deserialize;

use crate::error::AppError;

#[derive(Deserialize)]
pub struct DownloadRequest {
    pub version: Option<String>,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/process/start", post(start))
        .route("/process/stop", post(stop))
        .route("/process/restart", post(restart))
        .route("/process/status", get(status))
        .route("/process/download", post(download))
}

async fn start(State(state): State<Arc<AppState>>) -> Result<(), AppError> {
    state.process_manager.start().await?;
    Ok(())
}

async fn stop(State(state): State<Arc<AppState>>) -> Result<(), AppError> {
    state.process_manager.stop().await?;
    Ok(())
}

async fn restart(State(state): State<Arc<AppState>>) -> Result<(), AppError> {
    state.process_manager.restart().await?;
    Ok(())
}

async fn status(State(state): State<Arc<AppState>>) -> Result<Json<ProcessStatus>, AppError> {
    let status = state.process_manager.status().await;
    Ok(Json(status))
}

async fn download(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DownloadRequest>,
) -> Result<Json<BinaryInfo>, AppError> {
    let info = state
        .downloader
        .download(req.version.as_deref())
        .await?;
    Ok(Json(info))
}

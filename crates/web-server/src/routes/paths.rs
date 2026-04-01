use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use mediamtx_manager_core::{AppState, PathConfig, PathConfigList, PathItem, PathList};

use crate::error::AppError;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        // 경로 설정 CRUD
        .route("/paths/configs", get(list_configs))
        .route("/paths/configs/{name}", get(get_config))
        .route("/paths/configs/{name}", post(add_config))
        .route("/paths/configs/{name}", patch(update_config))
        .route("/paths/configs/{name}", delete(delete_config))
        // 활성 경로 조회
        .route("/paths", get(list_active))
        .route("/paths/{name}", get(get_active))
}

async fn list_configs(
    State(state): State<Arc<AppState>>,
) -> Result<Json<PathConfigList>, AppError> {
    let configs = state.api_client.list_path_configs().await?;
    Ok(Json(configs))
}

async fn get_config(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<PathConfig>, AppError> {
    let config = state.api_client.get_path_config(&name).await?;
    Ok(Json(config))
}

async fn add_config(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Json(config): Json<PathConfig>,
) -> Result<StatusCode, AppError> {
    state.api_client.add_path_config(&name, &config).await?;
    Ok(StatusCode::CREATED)
}

async fn update_config(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Json(config): Json<PathConfig>,
) -> Result<(), AppError> {
    state.api_client.update_path_config(&name, &config).await?;
    Ok(())
}

async fn delete_config(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<(), AppError> {
    state.api_client.delete_path_config(&name).await?;
    Ok(())
}

async fn list_active(State(state): State<Arc<AppState>>) -> Result<Json<PathList>, AppError> {
    let paths = state.api_client.list_paths().await?;
    Ok(Json(paths))
}

async fn get_active(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<PathItem>, AppError> {
    let path = state.api_client.get_path(&name).await?;
    Ok(Json(path))
}

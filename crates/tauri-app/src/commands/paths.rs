use mediamtx_manager_core::{AppState, PathConfig, PathConfigList, PathItem, PathList};
use tauri::State;

#[tauri::command]
pub async fn list_path_configs(state: State<'_, AppState>) -> Result<PathConfigList, String> {
    state
        .api_client
        .list_path_configs()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_path_config(
    state: State<'_, AppState>,
    name: String,
) -> Result<PathConfig, String> {
    state
        .api_client
        .get_path_config(&name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_path_config(
    state: State<'_, AppState>,
    name: String,
    config: PathConfig,
) -> Result<(), String> {
    state
        .api_client
        .add_path_config(&name, &config)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_path_config(
    state: State<'_, AppState>,
    name: String,
    config: PathConfig,
) -> Result<(), String> {
    state
        .api_client
        .update_path_config(&name, &config)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_path_config(
    state: State<'_, AppState>,
    name: String,
) -> Result<(), String> {
    state
        .api_client
        .delete_path_config(&name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_paths(state: State<'_, AppState>) -> Result<PathList, String> {
    state
        .api_client
        .list_paths()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_path(state: State<'_, AppState>, name: String) -> Result<PathItem, String> {
    state
        .api_client
        .get_path(&name)
        .await
        .map_err(|e| e.to_string())
}

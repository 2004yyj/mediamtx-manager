use mediamtx_manager_core::{AppState, GlobalConfig};
use tauri::State;

#[tauri::command]
pub async fn get_global_config(state: State<'_, AppState>) -> Result<GlobalConfig, String> {
    state
        .api_client
        .get_global_config()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn patch_global_config(
    state: State<'_, AppState>,
    config: GlobalConfig,
) -> Result<(), String> {
    state
        .api_client
        .patch_global_config(&config)
        .await
        .map_err(|e| e.to_string())
}

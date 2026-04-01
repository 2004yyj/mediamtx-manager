use mediamtx_manager_core::AppState;
use tauri::State;

#[tauri::command]
pub async fn read_config_file(state: State<'_, AppState>) -> Result<String, String> {
    state
        .config_file_manager
        .read_as_string()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_config_file(
    state: State<'_, AppState>,
    content: String,
) -> Result<(), String> {
    state
        .config_file_manager
        .write_string(&content)
        .await
        .map_err(|e| e.to_string())
}

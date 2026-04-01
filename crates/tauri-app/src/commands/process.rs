use mediamtx_manager_core::{AppState, BinaryInfo, ProcessStatus};
use tauri::State;

#[tauri::command]
pub async fn start_mediamtx(state: State<'_, AppState>) -> Result<(), String> {
    state.process_manager.start().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_mediamtx(state: State<'_, AppState>) -> Result<(), String> {
    state.process_manager.stop().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn restart_mediamtx(state: State<'_, AppState>) -> Result<(), String> {
    state.process_manager.restart().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_process_status(state: State<'_, AppState>) -> Result<ProcessStatus, String> {
    Ok(state.process_manager.status().await)
}

#[tauri::command]
pub async fn download_mediamtx(
    state: State<'_, AppState>,
    version: Option<String>,
) -> Result<BinaryInfo, String> {
    state
        .downloader
        .download(version.as_deref())
        .await
        .map_err(|e| e.to_string())
}

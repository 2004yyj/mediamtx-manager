#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use mediamtx_manager_core::{AppConfig, AppState};
use tauri::Manager;

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");

            let config = AppConfig {
                mediamtx_api_url: "http://127.0.0.1:9997".into(),
                mediamtx_binary_dir: data_dir.clone(),
                mediamtx_config_path: data_dir.join("mediamtx.yml"),
            };

            app.manage(AppState::new(config));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::process::start_mediamtx,
            commands::process::stop_mediamtx,
            commands::process::restart_mediamtx,
            commands::process::get_process_status,
            commands::process::download_mediamtx,
            commands::paths::list_path_configs,
            commands::paths::get_path_config,
            commands::paths::add_path_config,
            commands::paths::update_path_config,
            commands::paths::delete_path_config,
            commands::paths::list_paths,
            commands::paths::get_path,
            commands::config::get_global_config,
            commands::config::patch_global_config,
            commands::config_file::read_config_file,
            commands::config_file::write_config_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

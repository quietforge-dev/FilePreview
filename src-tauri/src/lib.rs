#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod commands;
mod config;
mod dao;
mod database;
mod error;
mod filesystem;
mod model;
mod service;

use app_state::AppState;

pub fn run() {
    let runtime = tokio::runtime::Runtime::new().expect("创建 Tokio runtime 失败");
    let state = runtime
        .block_on(AppState::initialize())
        .expect("初始化 FilePreview 数据库失败");
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::open_workspace,
            commands::list_directory,
            commands::read_file,
            commands::record_browsed_file,
            commands::list_recent_workspaces,
            commands::list_recent_files,
            commands::clear_recent_workspaces,
            commands::clear_recent_files,
            commands::app_version,
            commands::open_external_url,
        ])
        .run(tauri::generate_context!())
        .expect("运行 FilePreview 失败");
}

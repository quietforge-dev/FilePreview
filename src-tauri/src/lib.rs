#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod error;
mod filesystem;
mod model;
mod service;

use service::WorkspaceService;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(WorkspaceService::default())
        .invoke_handler(tauri::generate_handler![
            commands::open_workspace,
            commands::list_directory,
            commands::read_file,
        ])
        .run(tauri::generate_context!())
        .expect("运行 FilePreview 失败");
}

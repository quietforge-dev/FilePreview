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
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    Emitter,
};

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
        .setup(|app| {
            install_menu(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::workspace::open_workspace,
            commands::workspace::list_directory,
            commands::workspace::read_file,
            commands::workspace::file_info,
            commands::workspace::write_markdown_file,
            commands::workspace::search_file_contents,
            commands::app::office_runtime_status,
            commands::app::install_libreoffice,
            commands::workspace::convert_office_to_pdf,
            commands::workspace::copy_entry,
            commands::history::record_browsed_file,
            commands::history::list_recent_workspaces,
            commands::history::list_recent_files,
            commands::history::clear_recent_workspaces,
            commands::history::clear_recent_files,
            commands::session::list_session_tabs,
            commands::session::save_session_tabs,
            commands::app::app_version,
            commands::app::open_external_url,
            commands::app::open_libreoffice_download_page,
            commands::app_settings::get_app_setting,
            commands::app_settings::set_app_setting,
        ])
        .run(tauri::generate_context!())
        .expect("运行 FilePreview 失败");
}

fn install_menu(app: &mut tauri::App) -> tauri::Result<()> {
    let open_folder = MenuItem::with_id(app, "open-folder", "打开文件夹", true, Some("Ctrl+O"))?;
    let recent_workspaces = MenuItem::with_id(
        app,
        "show-recent-workspaces",
        "最近文件夹...",
        true,
        None::<&str>,
    )?;
    let recent_files =
        MenuItem::with_id(app, "show-recent-files", "浏览记录...", true, None::<&str>)?;
    let close_tab = MenuItem::with_id(app, "close-tab", "关闭标签", true, Some("Ctrl+W"))?;
    let copy = MenuItem::with_id(app, "copy", "复制", true, Some("Ctrl+C"))?;
    let paste = MenuItem::with_id(app, "paste", "粘贴", true, Some("Ctrl+V"))?;
    let refresh = MenuItem::with_id(app, "refresh", "刷新", true, Some("F5"))?;
    let search = MenuItem::with_id(
        app,
        "search-content",
        "文件内容搜索",
        true,
        Some("Ctrl+Shift+F"),
    )?;
    let check_updates = MenuItem::with_id(app, "check-updates", "检查更新", true, None::<&str>)?;
    let project_home = MenuItem::with_id(app, "project-home", "项目主页", true, None::<&str>)?;
    let file_separator = PredefinedMenuItem::separator(app)?;
    let help_separator = PredefinedMenuItem::separator(app)?;
    let file = Submenu::with_items(
        app,
        "文件",
        true,
        &[
            &open_folder,
            &recent_workspaces,
            &recent_files,
            &file_separator,
            &close_tab,
        ],
    )?;
    let edit = Submenu::with_items(app, "编辑", true, &[&copy, &paste])?;
    let view = Submenu::with_items(app, "视图", true, &[&refresh, &search])?;
    let help = Submenu::with_items(
        app,
        "帮助",
        true,
        &[&check_updates, &help_separator, &project_home],
    )?;
    let menu = Menu::with_items(app, &[&file, &edit, &view, &help])?;
    app.set_menu(menu)?;
    app.on_menu_event(|handle, event| {
        let _ = handle.emit("menu-action", event.id().as_ref());
    });
    Ok(())
}

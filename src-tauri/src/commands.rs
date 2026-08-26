use tauri::{AppHandle, State};

use crate::{
    app_state::AppState,
    model::{
        ContentSearchResult, FileInfo, OfficeRuntimeStatus, RecentFile, RecentWorkspace,
        SessionTab, WorkspaceInfo,
    },
    service::{history_service, office_preview_service, session_service},
};

const GITHUB_REPOSITORY_URL: &str = "https://github.com/quietforge-dev/FilePreview";
const LIBREOFFICE_DOWNLOAD_URL: &str = "https://www.libreoffice.org/download/";

#[tauri::command]
pub fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[tauri::command]
pub fn open_external_url(url: String) -> Result<(), String> {
    if url != GITHUB_REPOSITORY_URL
        && !url.starts_with("https://github.com/quietforge-dev/FilePreview/")
    {
        return Err("只允许打开 FilePreview 的 GitHub 地址".into());
    }

    open_url(&url).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn open_libreoffice_download_page() -> Result<(), String> {
    open_url(LIBREOFFICE_DOWNLOAD_URL).map_err(|error| error.to_string())
}

fn open_url(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn()
        .map(|_| ())?;
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(url)
        .spawn()
        .map(|_| ())?;
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(url)
        .spawn()
        .map(|_| ())?;

    Ok(())
}

#[tauri::command]
pub async fn open_workspace(
    path: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<WorkspaceInfo, String> {
    let workspace = history_service::open_workspace(&state.pool, &state.workspace, path)
        .await
        .map_err(String::from)?;
    state
        .file_watch
        .watch_workspace(&app, std::path::Path::new(&workspace.path))
        .map_err(String::from)?;
    Ok(workspace)
}

#[tauri::command]
pub fn list_directory(
    path: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<FileInfo>, String> {
    state.workspace.list_directory(path).map_err(Into::into)
}

#[tauri::command]
pub fn read_file(path: String, state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    state.workspace.read_file(path).map_err(Into::into)
}

#[tauri::command]
pub fn file_info(path: String, state: State<'_, AppState>) -> Result<FileInfo, String> {
    state.workspace.file_info(path).map_err(Into::into)
}

#[tauri::command]
pub async fn search_file_contents(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<ContentSearchResult>, String> {
    state
        .workspace
        .search_contents(query)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub fn office_runtime_status() -> OfficeRuntimeStatus {
    office_preview_service::runtime_status()
}

#[tauri::command]
pub async fn install_libreoffice() -> Result<(), String> {
    office_preview_service::install_libreoffice()
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn convert_office_to_pdf(
    path: String,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    state
        .workspace
        .convert_office_to_pdf(path)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn copy_entry(
    source: String,
    destination_directory: String,
    state: State<'_, AppState>,
) -> Result<FileInfo, String> {
    state
        .workspace
        .copy_entry(source, destination_directory)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn record_browsed_file(path: String, state: State<'_, AppState>) -> Result<(), String> {
    history_service::record_browsed_file(&state.pool, &state.workspace, path)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn list_recent_workspaces(
    state: State<'_, AppState>,
) -> Result<Vec<RecentWorkspace>, String> {
    history_service::list_recent_workspaces(&state.pool)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn list_recent_files(state: State<'_, AppState>) -> Result<Vec<RecentFile>, String> {
    history_service::list_recent_files(&state.pool)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn clear_recent_workspaces(state: State<'_, AppState>) -> Result<(), String> {
    history_service::clear_recent_workspaces(&state.pool)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn clear_recent_files(state: State<'_, AppState>) -> Result<(), String> {
    history_service::clear_recent_files(&state.pool)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn list_session_tabs(state: State<'_, AppState>) -> Result<Vec<SessionTab>, String> {
    session_service::list_tabs(&state.pool)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn save_session_tabs(
    tabs: Vec<SessionTab>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    session_service::save_tabs(&state.pool, tabs)
        .await
        .map_err(Into::into)
}

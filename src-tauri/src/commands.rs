use tauri::State;

use crate::{
    app_state::AppState,
    model::{FileInfo, RecentFile, RecentWorkspace, WorkspaceInfo},
    service::history_service,
};

const GITHUB_REPOSITORY_URL: &str = "https://github.com/quietforge-dev/FilePreview";

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

    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/C", "start", "", &url])
        .spawn()
        .map_err(|error| error.to_string())?;
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&url)
        .spawn()
        .map_err(|error| error.to_string())?;
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(&url)
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn open_workspace(
    path: String,
    state: State<'_, AppState>,
) -> Result<WorkspaceInfo, String> {
    history_service::open_workspace(&state.pool, &state.workspace, path)
        .await
        .map_err(Into::into)
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

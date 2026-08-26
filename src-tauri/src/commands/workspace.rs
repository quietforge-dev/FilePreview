use tauri::{AppHandle, State};

use crate::{
    app_state::AppState,
    model::{ContentSearchResult, FileInfo, WorkspaceInfo},
};

#[tauri::command]
pub async fn open_workspace(
    path: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<WorkspaceInfo, String> {
    let workspace =
        crate::service::history_service::open_workspace(&state.pool, &state.workspace, path)
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

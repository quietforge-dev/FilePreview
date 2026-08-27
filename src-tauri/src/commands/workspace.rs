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
pub async fn write_markdown_file(
    path: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<FileInfo, String> {
    state
        .workspace
        .write_markdown_file(path, content)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn create_file(
    destination_directory: String,
    file_name: String,
    state: State<'_, AppState>,
) -> Result<FileInfo, String> {
    state
        .workspace
        .create_file(destination_directory, file_name)
        .await
        .map_err(Into::into)
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
pub async fn search_workspace_entries(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<FileInfo>, String> {
    state
        .workspace
        .search_workspace_entries(query)
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
pub async fn has_system_clipboard_files(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.workspace.has_system_clipboard_files().await)
}

#[tauri::command]
pub async fn copy_entry_to_system_clipboard(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .workspace
        .copy_entry_to_system_clipboard(path)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn paste_system_clipboard_entries(
    destination_directory: String,
    state: State<'_, AppState>,
) -> Result<Vec<FileInfo>, String> {
    state
        .workspace
        .paste_system_clipboard_entries(destination_directory)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn delete_entry(path: String, state: State<'_, AppState>) -> Result<(), String> {
    state.workspace.delete_entry(path).await.map_err(Into::into)
}

#[tauri::command]
pub fn open_entry_with_default_application(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .workspace
        .open_entry_with_default_application(path)
        .map_err(Into::into)
}

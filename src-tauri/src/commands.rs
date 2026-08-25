use tauri::State;

use crate::{
    model::{FileInfo, WorkspaceInfo},
    service::WorkspaceService,
};

#[tauri::command]
pub fn open_workspace(
    path: String,
    workspace_service: State<'_, WorkspaceService>,
) -> Result<WorkspaceInfo, String> {
    workspace_service.open_workspace(path).map_err(Into::into)
}

#[tauri::command]
pub fn list_directory(
    path: Option<String>,
    workspace_service: State<'_, WorkspaceService>,
) -> Result<Vec<FileInfo>, String> {
    workspace_service.list_directory(path).map_err(Into::into)
}

#[tauri::command]
pub fn read_file(
    path: String,
    workspace_service: State<'_, WorkspaceService>,
) -> Result<Vec<u8>, String> {
    workspace_service.read_file(path).map_err(Into::into)
}

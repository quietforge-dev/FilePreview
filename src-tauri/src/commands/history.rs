use tauri::State;

use crate::{
    app_state::AppState,
    model::{RecentFile, RecentWorkspace},
};

#[tauri::command]
pub async fn record_browsed_file(path: String, state: State<'_, AppState>) -> Result<(), String> {
    crate::service::history_service::record_browsed_file(&state.pool, &state.workspace, path)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn list_recent_workspaces(
    state: State<'_, AppState>,
) -> Result<Vec<RecentWorkspace>, String> {
    crate::service::history_service::list_recent_workspaces(&state.pool)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn list_recent_files(state: State<'_, AppState>) -> Result<Vec<RecentFile>, String> {
    crate::service::history_service::list_recent_files(&state.pool)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn clear_recent_workspaces(state: State<'_, AppState>) -> Result<(), String> {
    crate::service::history_service::clear_recent_workspaces(&state.pool)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn clear_recent_files(state: State<'_, AppState>) -> Result<(), String> {
    crate::service::history_service::clear_recent_files(&state.pool)
        .await
        .map_err(Into::into)
}

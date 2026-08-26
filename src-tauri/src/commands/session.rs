use tauri::State;

use crate::{app_state::AppState, model::SessionTab};

#[tauri::command]
pub async fn list_session_tabs(state: State<'_, AppState>) -> Result<Vec<SessionTab>, String> {
    crate::service::session_service::list_tabs(&state.pool)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn save_session_tabs(
    tabs: Vec<SessionTab>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    crate::service::session_service::save_tabs(&state.pool, tabs)
        .await
        .map_err(Into::into)
}

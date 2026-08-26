use tauri::State;

use crate::app_state::AppState;

#[tauri::command]
pub async fn get_app_setting(
    key: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    crate::service::app_settings_service::get_value(&state.pool, &key)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn set_app_setting(
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    crate::service::app_settings_service::set_value(&state.pool, &key, &value)
        .await
        .map_err(Into::into)
}

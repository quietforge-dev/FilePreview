use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SessionTab {
    pub id: String,
    pub kind: String,
    pub workspace_path: String,
    pub workspace_name: String,
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub file_extension: Option<String>,
    pub current_directory: Option<String>,
    pub position: i64,
    pub active: bool,
}

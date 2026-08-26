use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RecentWorkspace {
    pub path: String,
    pub name: String,
    pub last_opened_at: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RecentFile {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub last_opened_at: i64,
}

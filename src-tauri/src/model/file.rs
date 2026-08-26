use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub modified_at: Option<u64>,
    pub is_directory: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentSearchResult {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub line_number: usize,
    pub line_content: String,
}

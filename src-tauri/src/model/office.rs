use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfficeRuntimeStatus {
    pub installed: bool,
    pub supports_quick_install: bool,
}

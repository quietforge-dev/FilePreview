use sqlx::SqlitePool;

use crate::{database, error::AppError, service::workspace_service::WorkspaceService};

pub struct AppState {
    pub pool: SqlitePool,
    pub workspace: WorkspaceService,
}

impl AppState {
    pub async fn initialize() -> Result<Self, AppError> {
        let pool = database::connect(&crate::config::database_path()).await?;
        Ok(Self {
            pool,
            workspace: WorkspaceService::default(),
        })
    }
}

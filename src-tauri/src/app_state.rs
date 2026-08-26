use sqlx::SqlitePool;

use crate::{
    database,
    error::AppError,
    service::{file_watch_service::FileWatchService, workspace_service::WorkspaceService},
};

pub struct AppState {
    pub pool: SqlitePool,
    pub workspace: WorkspaceService,
    pub file_watch: FileWatchService,
}

impl AppState {
    pub async fn initialize() -> Result<Self, AppError> {
        let pool = database::connect(&crate::config::database_path()).await?;
        Ok(Self {
            pool,
            workspace: WorkspaceService::default(),
            file_watch: FileWatchService::default(),
        })
    }
}

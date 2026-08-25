use sqlx::SqlitePool;

use crate::{
    dao::history_dao,
    error::AppError,
    model::{RecentFile, RecentWorkspace, WorkspaceInfo},
    service::workspace_service::WorkspaceService,
};

const HISTORY_LIMIT: i64 = 20;

pub async fn open_workspace(
    pool: &SqlitePool,
    workspace_service: &WorkspaceService,
    path: String,
) -> Result<WorkspaceInfo, AppError> {
    let workspace = workspace_service.open_workspace(path)?;
    history_dao::upsert_workspace(pool, &workspace).await?;
    Ok(workspace)
}

pub async fn record_browsed_file(
    pool: &SqlitePool,
    workspace_service: &WorkspaceService,
    path: String,
) -> Result<(), AppError> {
    let file = workspace_service.file_info(path)?;
    history_dao::upsert_file(pool, &file).await?;
    Ok(())
}

pub async fn list_recent_workspaces(pool: &SqlitePool) -> Result<Vec<RecentWorkspace>, AppError> {
    Ok(history_dao::list_workspaces(pool, HISTORY_LIMIT).await?)
}

pub async fn list_recent_files(pool: &SqlitePool) -> Result<Vec<RecentFile>, AppError> {
    Ok(history_dao::list_files(pool, HISTORY_LIMIT).await?)
}

pub async fn clear_recent_workspaces(pool: &SqlitePool) -> Result<(), AppError> {
    history_dao::clear_workspaces(pool).await?;
    Ok(())
}

pub async fn clear_recent_files(pool: &SqlitePool) -> Result<(), AppError> {
    history_dao::clear_files(pool).await?;
    Ok(())
}

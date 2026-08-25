use sqlx::SqlitePool;

use crate::model::{FileInfo, RecentFile, RecentWorkspace, WorkspaceInfo};

pub async fn upsert_workspace(
    pool: &SqlitePool,
    workspace: &WorkspaceInfo,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO workspace_history(path, name, last_opened_at) VALUES (?, ?, unixepoch()) \
         ON CONFLICT(path) DO UPDATE SET name=excluded.name, last_opened_at=excluded.last_opened_at",
    )
    .bind(&workspace.path)
    .bind(&workspace.name)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn upsert_file(pool: &SqlitePool, file: &FileInfo) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO file_history(path, name, extension, last_opened_at) VALUES (?, ?, ?, unixepoch()) \
         ON CONFLICT(path) DO UPDATE SET name=excluded.name, extension=excluded.extension, last_opened_at=excluded.last_opened_at",
    )
    .bind(&file.path)
    .bind(&file.name)
    .bind(&file.extension)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_workspaces(
    pool: &SqlitePool,
    limit: i64,
) -> Result<Vec<RecentWorkspace>, sqlx::Error> {
    sqlx::query_as::<_, RecentWorkspace>(
        "SELECT path, name, last_opened_at FROM workspace_history \
         ORDER BY last_opened_at DESC, id DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn list_files(pool: &SqlitePool, limit: i64) -> Result<Vec<RecentFile>, sqlx::Error> {
    sqlx::query_as::<_, RecentFile>(
        "SELECT path, name, extension, last_opened_at FROM file_history \
         ORDER BY last_opened_at DESC, id DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn clear_workspaces(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM workspace_history")
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn clear_files(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM file_history")
        .execute(pool)
        .await?;
    Ok(())
}

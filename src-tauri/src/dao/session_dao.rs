use sqlx::SqlitePool;

use crate::model::SessionTab;

pub async fn list_tabs(pool: &SqlitePool) -> Result<Vec<SessionTab>, sqlx::Error> {
    sqlx::query_as::<_, SessionTab>(
        "SELECT id, kind, workspace_path, workspace_name, file_path, file_name, \
         file_extension, current_directory, position, active FROM session_tabs \
         ORDER BY position ASC, id ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn replace_tabs(pool: &SqlitePool, tabs: &[SessionTab]) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    sqlx::query("DELETE FROM session_tabs")
        .execute(&mut *transaction)
        .await?;

    for tab in tabs {
        sqlx::query(
            "INSERT INTO session_tabs(\
             id, kind, workspace_path, workspace_name, file_path, file_name, file_extension, \
             current_directory, position, active, updated_at\
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, unixepoch())",
        )
        .bind(&tab.id)
        .bind(&tab.kind)
        .bind(&tab.workspace_path)
        .bind(&tab.workspace_name)
        .bind(&tab.file_path)
        .bind(&tab.file_name)
        .bind(&tab.file_extension)
        .bind(&tab.current_directory)
        .bind(tab.position)
        .bind(tab.active)
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await
}

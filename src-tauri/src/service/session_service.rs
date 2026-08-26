use sqlx::SqlitePool;

use crate::{dao::session_dao, error::AppError, model::SessionTab};

const MAX_SESSION_TABS: usize = 20;

pub async fn list_tabs(pool: &SqlitePool) -> Result<Vec<SessionTab>, AppError> {
    Ok(session_dao::list_tabs(pool).await?)
}

pub async fn save_tabs(pool: &SqlitePool, mut tabs: Vec<SessionTab>) -> Result<(), AppError> {
    tabs.truncate(MAX_SESSION_TABS);
    for (position, tab) in tabs.iter_mut().enumerate() {
        tab.position = position as i64;
    }
    session_dao::replace_tabs(pool, &tabs).await?;
    Ok(())
}

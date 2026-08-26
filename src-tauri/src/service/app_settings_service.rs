use sqlx::SqlitePool;

use crate::{dao::app_settings_dao, error::AppError};

const MAX_KEY_LENGTH: usize = 128;
const MAX_VALUE_LENGTH: usize = 4_096;

pub async fn get_value(pool: &SqlitePool, key: &str) -> Result<Option<String>, AppError> {
    validate_key(key)?;
    Ok(app_settings_dao::get_value(pool, key).await?)
}

pub async fn set_value(pool: &SqlitePool, key: &str, value: &str) -> Result<(), AppError> {
    validate_key(key)?;
    if value.len() > MAX_VALUE_LENGTH {
        return Err(AppError::InvalidAppSetting("设置值过长".into()));
    }
    app_settings_dao::set_value(pool, key, value).await?;
    Ok(())
}

fn validate_key(key: &str) -> Result<(), AppError> {
    if key.is_empty()
        || key.len() > MAX_KEY_LENGTH
        || !key.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
    {
        return Err(AppError::InvalidAppSetting("设置项名称无效".into()));
    }
    Ok(())
}

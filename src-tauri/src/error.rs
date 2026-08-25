use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("数据库错误：{0}")]
    Database(#[from] sqlx::Error),
    #[error("数据库迁移错误：{0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("尚未打开工作区")]
    WorkspaceNotOpen,
    #[error("文件不在当前工作区内")]
    OutsideWorkspace,
    #[error("目标不是文件夹")]
    NotDirectory,
    #[error("不能读取目录")]
    IsDirectory,
    #[error("不能将文件夹复制到它自身或其子目录中")]
    CannotCopyIntoSelf,
    #[error("文件复制任务失败")]
    CopyTaskFailed,
    #[error("文件过大，最大支持 {0} MB")]
    FileTooLarge(u64),
    #[error("文件系统错误：{0}")]
    Io(#[from] io::Error),
}

impl From<AppError> for String {
    fn from(value: AppError) -> Self {
        value.to_string()
    }
}

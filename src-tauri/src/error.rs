use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("数据库错误：{0}")]
    Database(#[from] sqlx::Error),
    #[error("数据库迁移错误：{0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("应用设置无效：{0}")]
    InvalidAppSetting(String),
    #[error("尚未打开工作区")]
    WorkspaceNotOpen,
    #[error("文件不在当前工作区内")]
    OutsideWorkspace,
    #[error("目标不是文件夹")]
    NotDirectory,
    #[error("文件名无效，不能包含路径分隔符")]
    InvalidFileName,
    #[error("同名文件或文件夹已存在")]
    EntryAlreadyExists,
    #[error("不能读取目录")]
    IsDirectory,
    #[error("只支持编辑 Markdown 文件")]
    NotMarkdownFile,
    #[error("不能将文件夹复制到它自身或其子目录中")]
    CannotCopyIntoSelf,
    #[error("不能将文件夹移动到它自身或其子目录中")]
    CannotMoveIntoSelf,
    #[error("移动目标已存在同名文件或文件夹")]
    MoveTargetExists,
    #[error("不能移动当前工作区根目录")]
    CannotMoveWorkspaceRoot,
    #[error("文件移动任务失败")]
    MoveTaskFailed,
    #[error("不能删除当前工作区根目录")]
    CannotDeleteWorkspaceRoot,
    #[error("不支持操作符号链接")]
    SymbolicLinkNotSupported,
    #[error("文件复制任务失败")]
    CopyTaskFailed,
    #[error("系统剪贴板中没有可粘贴的文件或文件夹")]
    ClipboardHasNoFiles,
    #[error("无法读取系统剪贴板：{0}")]
    ClipboardRead(String),
    #[error("无法写入系统剪贴板：{0}")]
    ClipboardWrite(String),
    #[error("文件删除任务失败")]
    DeleteTaskFailed,
    #[error("移入系统回收站失败：{0}")]
    Trash(String),
    #[error("无法使用系统默认程序打开：{0}")]
    SystemOpen(String),
    #[error("文件保存任务失败")]
    FileWriteTaskFailed,
    #[error("搜索关键词不能为空")]
    EmptySearchQuery,
    #[error("文件内容搜索任务失败")]
    SearchTaskFailed,
    #[error("文件系统监听错误：{0}")]
    FileWatch(#[from] notify::Error),
    #[error("文件过大，最大支持 {0} MB")]
    FileTooLarge(u64),
    #[error("未检测到 LibreOffice。请安装后重新尝试预览 Office 文件")]
    LibreOfficeNotInstalled,
    #[error("未检测到 Windows 应用安装器 winget，请通过 LibreOffice 官方网站安装")]
    WingetNotAvailable,
    #[error("LibreOffice 安装失败，退出码：{0}")]
    LibreOfficeInstallFailed(String),
    #[error("LibreOffice 安装任务失败")]
    LibreOfficeInstallTaskFailed,
    #[error("当前系统不支持应用内安装 LibreOffice，请通过官方网站安装")]
    LibreOfficeInstallNotSupported,
    #[error("LibreOffice 转换失败：{0}")]
    LibreOfficeConversionFailed(String),
    #[error("LibreOffice 转换任务失败")]
    LibreOfficeConversionTaskFailed,
    #[error("文件系统错误：{0}")]
    Io(#[from] io::Error),
}

impl From<AppError> for String {
    fn from(value: AppError) -> Self {
        value.to_string()
    }
}

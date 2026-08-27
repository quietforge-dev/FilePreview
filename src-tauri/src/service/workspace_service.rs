use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{
    error::AppError,
    filesystem,
    model::{ContentSearchResult, FileInfo, WorkspaceInfo},
    service::office_preview_service,
};

const MAX_PREVIEW_FILE_SIZE_BYTES: u64 = 25 * 1024 * 1024;
const MAX_SEARCH_FILE_SIZE_BYTES: u64 = 2 * 1024 * 1024;
const MAX_SEARCH_RESULTS: usize = 200;

#[derive(Default)]
pub struct WorkspaceService {
    root: Mutex<Option<PathBuf>>,
}

impl WorkspaceService {
    pub fn open_workspace(&self, path: String) -> Result<WorkspaceInfo, AppError> {
        let root = PathBuf::from(path).canonicalize()?;
        if !root.is_dir() {
            return Err(AppError::NotDirectory);
        }
        let name = root
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_else(|| root.to_string_lossy().to_string());
        *self.root.lock().expect("工作区锁已损坏") = Some(root.clone());
        Ok(WorkspaceInfo {
            path: root.to_string_lossy().to_string(),
            name,
        })
    }

    pub fn list_directory(&self, path: Option<String>) -> Result<Vec<FileInfo>, AppError> {
        let root = self.workspace_root()?;
        let target = self.authorized_path(&root, path.as_deref())?;
        filesystem::list_directory(&target)
    }

    pub fn read_file(&self, path: String) -> Result<Vec<u8>, AppError> {
        let root = self.workspace_root()?;
        let target = self.authorized_path(&root, Some(&path))?;
        filesystem::read_file(&target, MAX_PREVIEW_FILE_SIZE_BYTES)
    }

    pub fn file_info(&self, path: String) -> Result<FileInfo, AppError> {
        let root = self.workspace_root()?;
        let target = self.authorized_path(&root, Some(&path))?;
        let file = filesystem::file_info(&target)?;
        if file.is_directory {
            return Err(AppError::IsDirectory);
        }
        Ok(file)
    }

    pub async fn write_markdown_file(
        &self,
        path: String,
        content: String,
    ) -> Result<FileInfo, AppError> {
        let root = self.workspace_root()?;
        let target = self.authorized_path(&root, Some(&path))?;
        tokio::task::spawn_blocking(move || {
            filesystem::write_markdown_file_atomically(&target, &content)
        })
        .await
        .map_err(|_| AppError::FileWriteTaskFailed)?
    }

    pub async fn create_file(
        &self,
        destination_directory: String,
        file_name: String,
    ) -> Result<FileInfo, AppError> {
        let root = self.workspace_root()?;
        let destination_directory = self.authorized_path(&root, Some(&destination_directory))?;
        if !destination_directory.is_dir() {
            return Err(AppError::NotDirectory);
        }
        tokio::task::spawn_blocking(move || {
            filesystem::create_empty_file(&destination_directory, &file_name)
        })
        .await
        .map_err(|_| AppError::FileWriteTaskFailed)?
    }

    pub async fn convert_office_to_pdf(&self, path: String) -> Result<Vec<u8>, AppError> {
        let root = self.workspace_root()?;
        let target = self.authorized_path(&root, Some(&path))?;
        filesystem::validate_preview_file(&target, MAX_PREVIEW_FILE_SIZE_BYTES)?;
        office_preview_service::convert_to_pdf(&target, &crate::config::preview_cache_dir()).await
    }

    pub async fn copy_entry(
        &self,
        source: String,
        destination_directory: String,
    ) -> Result<FileInfo, AppError> {
        let root = self.workspace_root()?;
        let source = self.authorized_path(&root, Some(&source))?;
        let destination_directory = self.authorized_path(&root, Some(&destination_directory))?;

        if !destination_directory.is_dir() {
            return Err(AppError::NotDirectory);
        }
        if source.is_dir() && destination_directory.starts_with(&source) {
            return Err(AppError::CannotCopyIntoSelf);
        }

        tokio::task::spawn_blocking(move || filesystem::copy_entry(&source, &destination_directory))
            .await
            .map_err(|_| AppError::CopyTaskFailed)?
    }

    pub async fn move_entry(
        &self,
        source: String,
        destination_directory: String,
    ) -> Result<FileInfo, AppError> {
        let root = self.workspace_root()?;
        let requested_source = PathBuf::from(&source);
        if fs::symlink_metadata(&requested_source)?
            .file_type()
            .is_symlink()
        {
            return Err(AppError::SymbolicLinkNotSupported);
        }
        let source = self.authorized_path(&root, Some(&source))?;
        let destination_directory = self.authorized_path(&root, Some(&destination_directory))?;
        if source == root {
            return Err(AppError::CannotMoveWorkspaceRoot);
        }
        if !destination_directory.is_dir() {
            return Err(AppError::NotDirectory);
        }
        if source.is_dir() && destination_directory.starts_with(&source) {
            return Err(AppError::CannotMoveIntoSelf);
        }

        tokio::task::spawn_blocking(move || filesystem::move_entry(&source, &destination_directory))
            .await
            .map_err(|_| AppError::MoveTaskFailed)?
    }

    pub async fn has_system_clipboard_files(&self) -> bool {
        tokio::task::spawn_blocking(filesystem::system_clipboard_file_paths)
            .await
            .ok()
            .and_then(Result::ok)
            .is_some_and(|paths| !paths.is_empty())
    }

    pub async fn copy_entry_to_system_clipboard(&self, path: String) -> Result<(), AppError> {
        let root = self.workspace_root()?;
        let requested = PathBuf::from(&path);
        if fs::symlink_metadata(&requested)?.file_type().is_symlink() {
            return Err(AppError::SymbolicLinkNotSupported);
        }
        let target = self.authorized_path(&root, Some(&path))?;

        tokio::task::spawn_blocking(move || filesystem::set_system_clipboard_file_paths(&[target]))
            .await
            .map_err(|_| AppError::CopyTaskFailed)?
    }

    pub async fn paste_system_clipboard_entries(
        &self,
        destination_directory: String,
    ) -> Result<Vec<FileInfo>, AppError> {
        let sources = tokio::task::spawn_blocking(filesystem::system_clipboard_file_paths)
            .await
            .map_err(|_| AppError::CopyTaskFailed)??;
        self.copy_external_entries(sources, destination_directory)
            .await
    }

    async fn copy_external_entries(
        &self,
        sources: Vec<PathBuf>,
        destination_directory: String,
    ) -> Result<Vec<FileInfo>, AppError> {
        let root = self.workspace_root()?;
        let destination_directory = self.authorized_path(&root, Some(&destination_directory))?;
        if !destination_directory.is_dir() {
            return Err(AppError::NotDirectory);
        }

        let sources = sources
            .into_iter()
            .map(|source| Self::authorized_external_source(&source))
            .collect::<Result<Vec<_>, _>>()?;
        if sources.is_empty() {
            return Err(AppError::ClipboardHasNoFiles);
        }
        if sources
            .iter()
            .any(|source| source.is_dir() && destination_directory.starts_with(source))
        {
            return Err(AppError::CannotCopyIntoSelf);
        }

        tokio::task::spawn_blocking(move || {
            filesystem::copy_entries(&sources, &destination_directory)
        })
        .await
        .map_err(|_| AppError::CopyTaskFailed)?
    }

    pub async fn delete_entry(&self, path: String) -> Result<(), AppError> {
        let root = self.workspace_root()?;
        let requested = PathBuf::from(&path);
        if fs::symlink_metadata(&requested)?.file_type().is_symlink() {
            return Err(AppError::SymbolicLinkNotSupported);
        }
        let target = self.authorized_path(&root, Some(&path))?;
        if target == root {
            return Err(AppError::CannotDeleteWorkspaceRoot);
        }

        tokio::task::spawn_blocking(move || filesystem::move_entry_to_trash(&target))
            .await
            .map_err(|_| AppError::DeleteTaskFailed)?
    }

    pub fn open_entry_with_default_application(&self, path: String) -> Result<(), AppError> {
        let root = self.workspace_root()?;
        let target = self.authorized_path(&root, Some(&path))?;
        filesystem::open_with_default_application(&target)
    }

    pub async fn search_contents(
        &self,
        query: String,
    ) -> Result<Vec<ContentSearchResult>, AppError> {
        let query = query.trim().to_owned();
        if query.is_empty() {
            return Err(AppError::EmptySearchQuery);
        }
        let root = self.workspace_root()?;
        tokio::task::spawn_blocking(move || search_directory(&root, &query))
            .await
            .map_err(|_| AppError::SearchTaskFailed)?
    }

    pub async fn search_workspace_entries(&self, query: String) -> Result<Vec<FileInfo>, AppError> {
        let query = query.trim().to_owned();
        if query.is_empty() {
            return Err(AppError::EmptySearchQuery);
        }
        let root = self.workspace_root()?;
        tokio::task::spawn_blocking(move || search_workspace_entries(&root, &query))
            .await
            .map_err(|_| AppError::SearchTaskFailed)?
    }

    fn workspace_root(&self) -> Result<PathBuf, AppError> {
        self.root
            .lock()
            .expect("工作区锁已损坏")
            .clone()
            .ok_or(AppError::WorkspaceNotOpen)
    }

    fn authorized_path(&self, root: &Path, requested: Option<&str>) -> Result<PathBuf, AppError> {
        let candidate = match requested {
            Some(path) => PathBuf::from(path).canonicalize()?,
            None => root.to_path_buf(),
        };
        if candidate.starts_with(root) {
            Ok(candidate)
        } else {
            Err(AppError::OutsideWorkspace)
        }
    }

    fn authorized_external_source(source: &Path) -> Result<PathBuf, AppError> {
        if fs::symlink_metadata(source)?.file_type().is_symlink() {
            return Err(AppError::SymbolicLinkNotSupported);
        }
        Ok(source.canonicalize()?)
    }
}

fn search_directory(root: &Path, query: &str) -> Result<Vec<ContentSearchResult>, AppError> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    search_directory_entries(root, &query, &mut results)?;
    Ok(results)
}

fn search_workspace_entries(root: &Path, query: &str) -> Result<Vec<FileInfo>, AppError> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    search_workspace_entries_in_directory(root, &query, &mut results)?;
    results.sort_by(|left, right| {
        right
            .is_directory
            .cmp(&left.is_directory)
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
            .then_with(|| left.path.to_lowercase().cmp(&right.path.to_lowercase()))
    });
    Ok(results)
}

fn search_workspace_entries_in_directory(
    directory: &Path,
    query: &str,
    results: &mut Vec<FileInfo>,
) -> Result<(), AppError> {
    if results.len() >= MAX_SEARCH_RESULTS {
        return Ok(());
    }

    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by(|left, right| {
        left.file_name()
            .to_string_lossy()
            .to_lowercase()
            .cmp(&right.file_name().to_string_lossy().to_lowercase())
    });
    for entry in entries {
        if results.len() >= MAX_SEARCH_RESULTS {
            break;
        }
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        let path = entry.path();
        if entry
            .file_name()
            .to_string_lossy()
            .to_lowercase()
            .contains(query)
        {
            results.push(filesystem::file_info(&path)?);
        }
        if file_type.is_dir() {
            search_workspace_entries_in_directory(&path, query, results)?;
        }
    }
    Ok(())
}

fn search_directory_entries(
    directory: &Path,
    query: &str,
    results: &mut Vec<ContentSearchResult>,
) -> Result<(), AppError> {
    if results.len() >= MAX_SEARCH_RESULTS {
        return Ok(());
    }

    for entry in fs::read_dir(directory)? {
        if results.len() >= MAX_SEARCH_RESULTS {
            break;
        }
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            search_directory_entries(&path, query, results)?;
            continue;
        }
        if !is_searchable_text_file(&path, entry.metadata()?.len()) {
            continue;
        }
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let info = filesystem::file_info(&path)?;
        for (index, line) in content.lines().enumerate() {
            if line.to_lowercase().contains(query) {
                results.push(ContentSearchResult {
                    path: info.path.clone(),
                    name: info.name.clone(),
                    extension: info.extension.clone(),
                    line_number: index + 1,
                    line_content: line.trim().chars().take(240).collect(),
                });
                if results.len() >= MAX_SEARCH_RESULTS {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn is_searchable_text_file(path: &Path, size: u64) -> bool {
    if size > MAX_SEARCH_FILE_SIZE_BYTES {
        return false;
    }
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.to_ascii_lowercase())
            .as_deref(),
        Some(
            "txt"
                | "md"
                | "markdown"
                | "mdx"
                | "json"
                | "yaml"
                | "yml"
                | "xml"
                | "toml"
                | "ini"
                | "env"
                | "sql"
                | "js"
                | "ts"
                | "tsx"
                | "vue"
                | "rs"
                | "java"
                | "kt"
                | "go"
                | "py"
                | "sh"
                | "css"
                | "scss"
                | "html"
                | "csv"
                | "log"
        )
    )
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::WorkspaceService;

    #[tokio::test]
    async fn searches_supported_text_files_recursively() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-search-{suffix}"));
        let nested = root.join("nested");
        fs::create_dir_all(&nested).expect("应创建搜索测试目录");
        fs::write(root.join("ignore.bin"), [0_u8, 159, 146, 150]).expect("应写入二进制文件");
        fs::write(nested.join("example.md"), "标题\n匹配内容在这里\n")
            .expect("应写入 Markdown 文件");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开搜索测试工作区");
        let results = service
            .search_contents("匹配内容".into())
            .await
            .expect("应完成文件内容搜索");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "example.md");
        assert_eq!(results[0].line_number, 2);
        fs::remove_dir_all(root).expect("应清理搜索测试目录");
    }

    #[tokio::test]
    async fn searches_file_and_folder_names_recursively() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-name-search-{suffix}"));
        let nested = root.join("matching-folder");
        fs::create_dir_all(&nested).expect("应创建名称搜索测试目录");
        fs::write(nested.join("matching-file.md"), "content").expect("应写入名称搜索测试文件");
        fs::write(root.join("other.txt"), "other").expect("应写入无关文件");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开名称搜索测试工作区");
        let results = service
            .search_workspace_entries("matching".into())
            .await
            .expect("应完成名称搜索");

        assert_eq!(results.len(), 2);
        assert!(results[0].is_directory);
        assert_eq!(results[0].name, "matching-folder");
        assert_eq!(results[1].name, "matching-file.md");
        fs::remove_dir_all(root).expect("应清理名称搜索测试目录");
    }

    #[tokio::test]
    async fn writes_markdown_file_inside_the_workspace() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-write-{suffix}"));
        fs::create_dir_all(&root).expect("应创建写入测试工作区");
        let file = root.join("example.md");
        fs::write(&file, "# 初始内容\n").expect("应写入初始 Markdown 文件");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开写入测试工作区");
        let saved = service
            .write_markdown_file(
                file.to_string_lossy().to_string(),
                "# 保存后的内容\n".into(),
            )
            .await
            .expect("应保存 Markdown 文件");

        assert_eq!(saved.name, "example.md");
        assert_eq!(fs::read_to_string(&file).unwrap(), "# 保存后的内容\n");
        fs::remove_dir_all(root).expect("应清理写入测试目录");
    }

    #[tokio::test]
    async fn creates_an_empty_file_inside_the_workspace() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-create-file-{suffix}"));
        fs::create_dir_all(&root).expect("应创建新建文件测试目录");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开新建文件测试工作区");
        let created = service
            .create_file(root.to_string_lossy().to_string(), "notes.md".into())
            .await
            .expect("应创建空文件");

        assert_eq!(created.name, "notes.md");
        assert_eq!(fs::read_to_string(root.join("notes.md")).unwrap(), "");
        fs::remove_dir_all(root).expect("应清理新建文件测试目录");
    }

    #[tokio::test]
    async fn rejects_creating_a_file_with_path_segments() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-create-invalid-{suffix}"));
        fs::create_dir_all(&root).expect("应创建新建文件测试目录");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开新建文件测试工作区");
        let error = service
            .create_file(root.to_string_lossy().to_string(), "nested/notes.md".into())
            .await
            .expect_err("不应允许通过文件名创建子路径");

        assert!(matches!(error, crate::error::AppError::InvalidFileName));
        fs::remove_dir_all(root).expect("应清理新建文件测试目录");
    }

    #[tokio::test]
    async fn rejects_deleting_the_workspace_root() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-delete-root-{suffix}"));
        fs::create_dir_all(&root).expect("应创建删除测试工作区");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开删除测试工作区");
        let error = service
            .delete_entry(root.to_string_lossy().to_string())
            .await
            .expect_err("不应允许删除工作区根目录");

        assert!(matches!(
            error,
            crate::error::AppError::CannotDeleteWorkspaceRoot
        ));
        fs::remove_dir_all(root).expect("应清理删除测试目录");
    }

    #[tokio::test]
    async fn rejects_deleting_a_path_outside_the_workspace() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-delete-workspace-{suffix}"));
        let outside = std::env::temp_dir().join(format!("filepreview-delete-outside-{suffix}.txt"));
        fs::create_dir_all(&root).expect("应创建删除测试工作区");
        fs::write(&outside, "outside").expect("应写入工作区外测试文件");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开删除测试工作区");
        let error = service
            .delete_entry(outside.to_string_lossy().to_string())
            .await
            .expect_err("不应允许删除工作区外文件");

        assert!(matches!(error, crate::error::AppError::OutsideWorkspace));
        fs::remove_file(outside).expect("应清理工作区外测试文件");
        fs::remove_dir_all(root).expect("应清理删除测试目录");
    }

    #[tokio::test]
    async fn imports_multiple_external_entries_into_the_workspace() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-import-workspace-{suffix}"));
        let source = std::env::temp_dir().join(format!("filepreview-import-source-{suffix}"));
        fs::create_dir_all(&root).expect("应创建导入测试工作区");
        fs::create_dir_all(&source).expect("应创建导入测试源目录");
        let first = source.join("first.txt");
        let second = source.join("second.txt");
        fs::write(&first, "first").expect("应写入第一个源文件");
        fs::write(&second, "second").expect("应写入第二个源文件");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开导入测试工作区");
        let copied = service
            .copy_external_entries(vec![first, second], root.to_string_lossy().to_string())
            .await
            .expect("应导入外部文件");

        assert_eq!(copied.len(), 2);
        assert_eq!(fs::read_to_string(root.join("first.txt")).unwrap(), "first");
        assert_eq!(
            fs::read_to_string(root.join("second.txt")).unwrap(),
            "second"
        );
        fs::remove_dir_all(root).expect("应清理导入测试工作区");
        fs::remove_dir_all(source).expect("应清理导入测试源目录");
    }

    #[tokio::test]
    async fn rejects_importing_entries_to_a_directory_outside_the_workspace() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-import-root-{suffix}"));
        let source = std::env::temp_dir().join(format!("filepreview-import-file-{suffix}.txt"));
        let outside = std::env::temp_dir().join(format!("filepreview-import-outside-{suffix}"));
        fs::create_dir_all(&root).expect("应创建导入测试工作区");
        fs::create_dir_all(&outside).expect("应创建工作区外目录");
        fs::write(&source, "source").expect("应写入工作区外源文件");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开导入测试工作区");
        let error = service
            .copy_external_entries(vec![source.clone()], outside.to_string_lossy().to_string())
            .await
            .expect_err("不应导入到工作区外目录");

        assert!(matches!(error, crate::error::AppError::OutsideWorkspace));
        fs::remove_file(source).expect("应清理工作区外源文件");
        fs::remove_dir_all(root).expect("应清理导入测试工作区");
        fs::remove_dir_all(outside).expect("应清理工作区外目录");
    }

    #[tokio::test]
    async fn moves_an_entry_inside_the_workspace() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-move-{suffix}"));
        let source_dir = root.join("source");
        let destination_dir = root.join("destination");
        fs::create_dir_all(&source_dir).expect("应创建移动源目录");
        fs::create_dir_all(&destination_dir).expect("应创建移动目标目录");
        let source = source_dir.join("notes.md");
        fs::write(&source, "content").expect("应写入移动源文件");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开移动测试工作区");
        let moved = service
            .move_entry(
                source.to_string_lossy().to_string(),
                destination_dir.to_string_lossy().to_string(),
            )
            .await
            .expect("应移动文件");

        assert_eq!(
            moved.path,
            destination_dir
                .join("notes.md")
                .canonicalize()
                .expect("应解析移动后的文件路径")
                .to_string_lossy()
        );
        assert!(!source.exists());
        assert_eq!(
            fs::read_to_string(destination_dir.join("notes.md")).unwrap(),
            "content"
        );
        fs::remove_dir_all(root).expect("应清理移动测试目录");
    }

    #[tokio::test]
    async fn rejects_moving_a_directory_into_itself() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-move-self-{suffix}"));
        let source_dir = root.join("source");
        let child_dir = source_dir.join("child");
        fs::create_dir_all(&child_dir).expect("应创建移动自包含测试目录");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开移动测试工作区");
        let error = service
            .move_entry(
                source_dir.to_string_lossy().to_string(),
                child_dir.to_string_lossy().to_string(),
            )
            .await
            .expect_err("不应将目录移动到自身子目录");

        assert!(matches!(error, crate::error::AppError::CannotMoveIntoSelf));
        fs::remove_dir_all(root).expect("应清理移动自包含测试目录");
    }

    #[tokio::test]
    async fn rejects_moving_over_an_existing_entry() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-move-existing-{suffix}"));
        let source_dir = root.join("source");
        let destination_dir = root.join("destination");
        fs::create_dir_all(&source_dir).expect("应创建同名移动源目录");
        fs::create_dir_all(&destination_dir).expect("应创建同名移动目标目录");
        fs::write(source_dir.join("notes.md"), "source").expect("应写入移动源文件");
        fs::write(destination_dir.join("notes.md"), "destination").expect("应写入已有目标文件");

        let service = WorkspaceService::default();
        service
            .open_workspace(root.to_string_lossy().to_string())
            .expect("应打开同名移动测试工作区");
        let error = service
            .move_entry(
                source_dir.join("notes.md").to_string_lossy().to_string(),
                destination_dir.to_string_lossy().to_string(),
            )
            .await
            .expect_err("不应覆盖已有目标文件");

        assert!(matches!(error, crate::error::AppError::MoveTargetExists));
        assert_eq!(
            fs::read_to_string(source_dir.join("notes.md")).unwrap(),
            "source"
        );
        assert_eq!(
            fs::read_to_string(destination_dir.join("notes.md")).unwrap(),
            "destination"
        );
        fs::remove_dir_all(root).expect("应清理同名移动测试目录");
    }
}

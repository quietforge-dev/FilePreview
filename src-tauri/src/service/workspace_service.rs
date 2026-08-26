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
}

fn search_directory(root: &Path, query: &str) -> Result<Vec<ContentSearchResult>, AppError> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    search_directory_entries(root, &query, &mut results)?;
    Ok(results)
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
}

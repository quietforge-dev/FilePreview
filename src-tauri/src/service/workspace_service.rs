use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{
    error::AppError,
    filesystem,
    model::{FileInfo, WorkspaceInfo},
};

const MAX_PREVIEW_FILE_SIZE_BYTES: u64 = 25 * 1024 * 1024;

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

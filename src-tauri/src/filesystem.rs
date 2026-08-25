use std::{fs, path::Path, time::UNIX_EPOCH};

use crate::{error::AppError, model::FileInfo};

pub fn list_directory(path: &Path) -> Result<Vec<FileInfo>, AppError> {
    let mut entries = fs::read_dir(path)?
        .map(|entry| entry.map_err(AppError::from).and_then(file_info_from_entry))
        .collect::<Result<Vec<_>, _>>()?;

    entries.sort_by(|left, right| {
        right
            .is_directory
            .cmp(&left.is_directory)
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
    });
    Ok(entries)
}

pub fn read_file(path: &Path, max_size_bytes: u64) -> Result<Vec<u8>, AppError> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        return Err(AppError::IsDirectory);
    }
    if metadata.len() > max_size_bytes {
        return Err(AppError::FileTooLarge(max_size_bytes / 1024 / 1024));
    }
    Ok(fs::read(path)?)
}

fn file_info_from_entry(entry: fs::DirEntry) -> Result<FileInfo, AppError> {
    let path = entry.path();
    let metadata = entry.metadata()?;
    let name = entry.file_name().to_string_lossy().to_string();
    let extension = if metadata.is_dir() {
        String::new()
    } else {
        path.extension()
            .map(|extension| extension.to_string_lossy().to_lowercase())
            .unwrap_or_default()
    };
    let modified_at = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs());

    Ok(FileInfo {
        path: path.to_string_lossy().to_string(),
        name,
        extension,
        size: metadata.len(),
        modified_at,
        is_directory: metadata.is_dir(),
    })
}

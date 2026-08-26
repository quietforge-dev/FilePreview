use std::{
    fs,
    path::Path,
    process,
    time::{SystemTime, UNIX_EPOCH},
};

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
    validate_preview_file(path, max_size_bytes)?;
    Ok(fs::read(path)?)
}

pub fn validate_preview_file(path: &Path, max_size_bytes: u64) -> Result<(), AppError> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        return Err(AppError::IsDirectory);
    }
    if metadata.len() > max_size_bytes {
        return Err(AppError::FileTooLarge(max_size_bytes / 1024 / 1024));
    }
    Ok(())
}

pub fn write_markdown_file_atomically(path: &Path, content: &str) -> Result<FileInfo, AppError> {
    validate_markdown_file(path, content.len() as u64)?;
    let parent = path.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "无法确定文件所在目录")
    })?;
    let file_name = path
        .file_name()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "无法确定文件名"))?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temporary = parent.join(format!(
        ".{}.filepreview-{}-{timestamp}.tmp",
        file_name.to_string_lossy(),
        process::id()
    ));

    fs::write(&temporary, content)?;
    if let Err(error) = fs::rename(&temporary, path) {
        let _ = fs::remove_file(&temporary);
        return Err(error.into());
    }
    file_info(path)
}

fn validate_markdown_file(path: &Path, content_size_bytes: u64) -> Result<(), AppError> {
    validate_preview_file(path, 25 * 1024 * 1024)?;
    if content_size_bytes > 25 * 1024 * 1024 {
        return Err(AppError::FileTooLarge(25));
    }
    let is_markdown = matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.to_ascii_lowercase())
            .as_deref(),
        Some("md" | "markdown")
    );
    if !is_markdown {
        return Err(AppError::NotMarkdownFile);
    }
    Ok(())
}

pub fn copy_entry(source: &Path, destination_directory: &Path) -> Result<FileInfo, AppError> {
    let destination = available_copy_path(source, destination_directory)?;
    let file_type = fs::symlink_metadata(source)?.file_type();

    if file_type.is_symlink() {
        return Err(
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "不支持复制符号链接").into(),
        );
    }

    if file_type.is_dir() {
        copy_directory(source, &destination)?;
    } else {
        fs::copy(source, &destination)?;
    }

    file_info(&destination)
}

pub fn file_info(path: &Path) -> Result<FileInfo, AppError> {
    let metadata = fs::metadata(path)?;
    let name = path
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());
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

fn file_info_from_entry(entry: fs::DirEntry) -> Result<FileInfo, AppError> {
    file_info(&entry.path())
}

fn available_copy_path(
    source: &Path,
    destination_directory: &Path,
) -> Result<std::path::PathBuf, AppError> {
    let file_name = source.file_name().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "无法确定要复制的文件名")
    })?;
    let destination = destination_directory.join(file_name);
    if !destination.exists() {
        return Ok(destination);
    }

    let stem = source
        .file_stem()
        .map(|value| value.to_string_lossy())
        .unwrap_or_else(|| file_name.to_string_lossy());
    let extension = source
        .extension()
        .map(|value| format!(".{}", value.to_string_lossy()))
        .unwrap_or_default();

    for index in 1.. {
        let candidate = destination_directory.join(format!("{stem} ({index}){extension}"));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }

    unreachable!("无限的副本名称序列应始终存在可用名称")
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), AppError> {
    fs::create_dir(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type()?;

        if file_type.is_symlink() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "不支持复制包含符号链接的文件夹",
            )
            .into());
        }
        if file_type.is_dir() {
            copy_directory(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::copy_entry;

    #[test]
    fn copy_entry_adds_a_suffix_without_overwriting_an_existing_file() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("系统时间应晚于 Unix 纪元")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("filepreview-copy-entry-{unique}"));
        let source_directory = root.join("source");
        let destination_directory = root.join("destination");
        fs::create_dir_all(&source_directory).expect("应创建源目录");
        fs::create_dir_all(&destination_directory).expect("应创建目标目录");
        let source = source_directory.join("example.txt");
        fs::write(&source, "source").expect("应写入源文件");

        let first_copy = copy_entry(&source, &destination_directory).expect("首次复制应成功");
        let second_copy = copy_entry(&source, &destination_directory).expect("再次复制应成功");

        assert_eq!(first_copy.name, "example.txt");
        assert_eq!(second_copy.name, "example (1).txt");
        assert_eq!(
            fs::read(destination_directory.join("example.txt")).unwrap(),
            b"source"
        );
        assert_eq!(
            fs::read(destination_directory.join("example (1).txt")).unwrap(),
            b"source"
        );

        fs::remove_dir_all(root).expect("应清理临时测试目录");
    }
}

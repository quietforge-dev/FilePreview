use std::path::PathBuf;

use directories::ProjectDirs;

pub fn database_path() -> PathBuf {
    ProjectDirs::from("dev", "quietforge", "FilePreview")
        .map(|directories| directories.data_dir().join("filepreview.db"))
        .unwrap_or_else(|| PathBuf::from("filepreview.db"))
}

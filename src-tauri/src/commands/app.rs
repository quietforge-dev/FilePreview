use crate::service::office_preview_service;

const GITHUB_REPOSITORY_URL: &str = "https://github.com/quietforge-dev/FilePreview";
const LIBREOFFICE_DOWNLOAD_URL: &str = "https://www.libreoffice.org/download/";

#[tauri::command]
pub fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[tauri::command]
pub fn open_external_url(url: String) -> Result<(), String> {
    if url != GITHUB_REPOSITORY_URL
        && !url.starts_with("https://github.com/quietforge-dev/FilePreview/")
    {
        return Err("只允许打开 FilePreview 的 GitHub 地址".into());
    }
    open_url(&url).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn open_libreoffice_download_page() -> Result<(), String> {
    open_url(LIBREOFFICE_DOWNLOAD_URL).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn office_runtime_status() -> crate::model::OfficeRuntimeStatus {
    office_preview_service::runtime_status()
}

#[tauri::command]
pub async fn install_libreoffice() -> Result<(), String> {
    office_preview_service::install_libreoffice()
        .await
        .map_err(Into::into)
}

fn open_url(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn()
        .map(|_| ())?;
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(url)
        .spawn()
        .map(|_| ())?;
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(url)
        .spawn()
        .map(|_| ())?;
    Ok(())
}

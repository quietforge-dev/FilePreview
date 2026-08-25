use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{error::AppError, filesystem, model::OfficeRuntimeStatus};

const MAX_PREVIEW_FILE_SIZE_BYTES: u64 = 25 * 1024 * 1024;

pub fn runtime_status() -> OfficeRuntimeStatus {
    OfficeRuntimeStatus {
        installed: find_soffice().is_some(),
        supports_quick_install: cfg!(target_os = "windows"),
    }
}

pub async fn install_libreoffice() -> Result<(), AppError> {
    #[cfg(target_os = "windows")]
    {
        tokio::task::spawn_blocking(|| {
            let status = Command::new("winget")
                .args([
                    "install",
                    "--id",
                    "TheDocumentFoundation.LibreOffice",
                    "--exact",
                    "--accept-package-agreements",
                    "--accept-source-agreements",
                ])
                .status()
                .map_err(|error| {
                    if error.kind() == io::ErrorKind::NotFound {
                        AppError::WingetNotAvailable
                    } else {
                        AppError::Io(error)
                    }
                })?;
            if status.success() {
                Ok(())
            } else {
                Err(AppError::LibreOfficeInstallFailed(
                    status
                        .code()
                        .map(|code| code.to_string())
                        .unwrap_or_else(|| "未知".into()),
                ))
            }
        })
        .await
        .map_err(|_| AppError::LibreOfficeInstallTaskFailed)?
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(AppError::LibreOfficeInstallNotSupported)
    }
}

pub async fn convert_to_pdf(source: &Path, cache_root: &Path) -> Result<Vec<u8>, AppError> {
    let soffice = find_soffice().ok_or(AppError::LibreOfficeNotInstalled)?;
    let source = source.to_path_buf();
    let cache_root = cache_root.to_path_buf();
    tokio::task::spawn_blocking(move || convert_to_pdf_blocking(&soffice, &source, &cache_root))
        .await
        .map_err(|_| AppError::LibreOfficeConversionTaskFailed)?
}

fn convert_to_pdf_blocking(
    soffice: &Path,
    source: &Path,
    cache_root: &Path,
) -> Result<Vec<u8>, AppError> {
    let output_directory = unique_output_directory(cache_root);
    fs::create_dir_all(&output_directory)?;

    let result = (|| {
        let output = Command::new(soffice)
            .args(["--headless", "--convert-to", "pdf", "--outdir"])
            .arg(&output_directory)
            .arg(source)
            .output()?;
        if !output.status.success() {
            let detail = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            return Err(AppError::LibreOfficeConversionFailed(
                if detail.is_empty() {
                    "LibreOffice 未能生成预览文件".into()
                } else {
                    detail
                },
            ));
        }

        let pdf = fs::read_dir(&output_directory)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .find(|path| {
                path.extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
            })
            .ok_or_else(|| {
                AppError::LibreOfficeConversionFailed("未找到转换后的 PDF 文件".into())
            })?;
        filesystem::read_file(&pdf, MAX_PREVIEW_FILE_SIZE_BYTES)
    })();

    let _ = fs::remove_dir_all(output_directory);
    result
}

fn unique_output_directory(cache_root: &Path) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    cache_root.join(format!("office-{timestamp}-{}", std::process::id()))
}

fn find_soffice() -> Option<PathBuf> {
    soffice_candidates().into_iter().find(|candidate| {
        if candidate.components().count() > 1 && !candidate.is_file() {
            return false;
        }
        Command::new(candidate)
            .arg("--version")
            .output()
            .is_ok_and(|output| output.status.success())
    })
}

fn soffice_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    #[cfg(target_os = "windows")]
    {
        for variable in ["ProgramFiles", "ProgramFiles(x86)", "LOCALAPPDATA"] {
            if let Some(root) = std::env::var_os(variable) {
                candidates.push(
                    PathBuf::from(root)
                        .join("LibreOffice")
                        .join("program")
                        .join("soffice.exe"),
                );
            }
        }
        candidates.push(PathBuf::from("soffice.exe"));
    }

    #[cfg(target_os = "macos")]
    {
        candidates.push(PathBuf::from(
            "/Applications/LibreOffice.app/Contents/MacOS/soffice",
        ));
        candidates.push(PathBuf::from("soffice"));
    }

    #[cfg(target_os = "linux")]
    {
        candidates.push(PathBuf::from("soffice"));
    }

    candidates
}

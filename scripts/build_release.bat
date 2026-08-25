@echo off
setlocal
title FilePreview Windows Release Build
cd /d "%~dp0.."

echo Building signed FilePreview Windows installer...
echo Signing key: src-tauri\keys\updater.key

%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe -NoLogo -NoProfile -ExecutionPolicy Bypass -File "%~dp0build_release.ps1"

if errorlevel 1 (
  echo.
  echo Release build failed.
  exit /b 1
)

echo.
echo Release build completed successfully.
endlocal
